//! This module holds the top-level GraphQL query object. All query subtypes
//! should be defined elsewhere.

use crate::{
    error::{ApiError, ApiResult},
    graphql::{
        internal::NodeType, Cursor, ItemSearch, Node, RequestContext, Tag,
        TagConnection, TagNode, TaggedItemConnection, TaggedItemNode,
    },
    spotify::{Item, PaginatedResponse, PrivateUser, SpotifyUri},
};
use async_graphql::{Context, FieldResult, Object};
use futures::StreamExt;
use mongodb::bson::doc;
use std::backtrace::Backtrace;

/// Root GraphQL query
pub struct Query;

#[Object]
impl Query {
    /// Get a node of any type by UUID.
    async fn node(
        &self,
        context: &Context<'_>,
        id: async_graphql::ID,
    ) -> FieldResult<Option<Node>> {
        let context = context.data::<RequestContext>()?;
        let (node_type, value_id, user_id) = Node::parse_id(&id)?;

        // Nice try, Satan
        if user_id != context.user_id {
            // Another user owns the node, pretend like it doesn't exist
            return Ok(None);
        }

        let node = match node_type {
            NodeType::TaggedItemNode => {
                // For items, the value ID is the URI. Look up the item in the
                // Spotify API
                let item_uri: SpotifyUri = value_id.parse()?;
                let item_opt = context.spotify.get_item(&item_uri).await?;
                item_opt.map(|item| TaggedItemNode { item, tags: None }.into())
            }
            NodeType::TagNode => Some(
                // For tags, the value ID is just the tag
                TagNode {
                    tag: Tag::new(value_id),
                    item_uris: None,
                }
                .into(),
            ),
        };
        Ok(node)
    }

    async fn current_user(
        &self,
        context: &Context<'_>,
    ) -> FieldResult<PrivateUser> {
        Ok(context
            .data::<RequestContext>()?
            .spotify
            .get_current_user()
            .await?)
    }

    async fn item(
        &self,
        context: &Context<'_>,
        uri: SpotifyUri,
    ) -> FieldResult<Option<TaggedItemNode>> {
        let context = context.data::<RequestContext>()?;
        // Fetch the item from Spotify
        let node = context
            .spotify
            .get_item(&uri)
            .await?
            .map(|item| TaggedItemNode { item, tags: None });
        Ok(node)
    }

    /// Run a search term through spotify. We'll return items grouped by
    /// their type, which is how we get the data from Spotify. This only touches
    /// the Spotify API (not the DB), meaning we defer loading tags down the
    /// line.
    async fn item_search(
        &self,
        context: &Context<'_>,
        #[graphql(validator(min_length = 1))] query: String,
        first: Option<usize>,
        after: Option<Cursor>,
    ) -> FieldResult<ItemSearch> {
        let context = context.data::<RequestContext>()?;

        // Run the search query through spotify. This returns a mapping of
        // results, grouped by item type. i.e. one PaginatedResponse for each
        // type (track/album/artist)
        let mut search_response = context
            .spotify
            .search_items(
                &query,
                first,
                after.map(|cursor| cursor.after_offset()),
            )
            .await?;

        // Helper to pull a type out of the search response and error if missing
        let mut load_item_type =
            |field: &str| -> ApiResult<PaginatedResponse<Item>> {
                search_response
                    .remove(field)
                    .ok_or_else(|| ApiError::Unknown {
                        message: format!(
                            "Missing field {} in Spotify search response",
                            field
                        ),
                        backtrace: Backtrace::capture(),
                    })
            };

        // Pull out the item types we care about. This should always exhaust
        // the map (with no missing types), because the fields we return here
        // line up with the types requested from Spotify
        let rv = ItemSearch {
            tracks: TaggedItemConnection::Preloaded {
                paginated_response: load_item_type("tracks")?,
            },
            albums: TaggedItemConnection::Preloaded {
                paginated_response: load_item_type("albums")?,
            },
            artists: TaggedItemConnection::Preloaded {
                paginated_response: load_item_type("artists")?,
            },
        };

        // Sanity check to make sure we're not getting more data than we need
        debug_assert!(
            search_response.is_empty(),
            "Spotify search response has extra keys: {:?}",
            search_response.keys()
        );

        Ok(rv)
    }

    /// Get all tags. These are loaded lazily by [TagConnection]
    async fn tags(&self) -> TagConnection {
        TagConnection::All
    }

    /// Get info for a particular tag. If the tag doesn't exist in the DB, we'll
    /// pretend like it does and just return a node with no tagged items. Item
    /// data will be loaded lazily, when requested from [TaggedItemConnection].
    async fn tag(
        &self,
        context: &Context<'_>,
        tag: Tag,
    ) -> FieldResult<TagNode> {
        let context = context.data::<RequestContext>()?;

        // Look up the relevant items in the DB
        let mut cursor = context
            .db_handler
            .collection_tagged_items()
            .find_by_tag(&context.user_id, &tag)
            .await?;
        // Grab the URI for each item
        let mut item_uris = Vec::new();
        while let Some(doc) = cursor.next().await {
            item_uris.push(doc?.uri);
        }

        // The rest of the item data will be loaded lazily by
        // TaggedItemConnection
        Ok(TagNode {
            tag,
            item_uris: Some(item_uris),
        })
    }
}
