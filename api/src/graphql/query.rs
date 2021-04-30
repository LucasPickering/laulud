//! This module holds the top-level GraphQL query object. All query subtypes
//! should be defined elsewhere.

use crate::{
    error::{ApiError, ApiResult, InputValidationError},
    graphql::{
        internal::{LimitOffset, NodeType},
        Cursor, Item, ItemSearch, Node, QueryFields, RequestContext,
        SpotifyUri, TagConnection, TagNode, TaggedItemConnection,
        TaggedItemNode,
    },
    spotify::{PaginatedResponse, PrivateUser, ValidSpotifyUri},
    util::Validate,
};
use async_trait::async_trait;
use juniper::{futures::StreamExt, Executor};
use juniper_from_schema::{QueryTrail, Walked};
use mongodb::bson::doc;
use serde::Deserialize;
use std::backtrace::Backtrace;

/// Root GraphQL query
pub struct Query;

#[async_trait]
impl QueryFields for Query {
    /// Get a node of any type by UUID.
    async fn field_node<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, Node, Walked>,
        id: juniper::ID,
    ) -> ApiResult<Option<Node>> {
        let context = executor.context();
        let (node_type, value_id, user_id) = NodeType::parse_id(&id)
            .map_err(|err| err.into_input_validation_error("id".into()))?;

        // Nice try, Satan
        if user_id != context.user_id {
            // Another user owns the node, pretend like it doesn't exist
            return Ok(None);
        }

        let node = match node_type {
            NodeType::TaggedItemNode => {
                // For items, the value ID is the URI. Look up the item in the
                // Spotify API
                let item_uri: ValidSpotifyUri = value_id.validate("")?;
                let item_opt = context.spotify.get_item(&item_uri).await?;
                item_opt.map(|item| TaggedItemNode { item, tags: None }.into())
            }
            NodeType::TagNode => {
                let tag = value_id;
                Some(
                    TagNode {
                        tag,
                        item_uris: None,
                    }
                    .into(),
                )
            }
        };
        Ok(node)
    }

    async fn field_current_user<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, PrivateUser, Walked>,
    ) -> ApiResult<PrivateUser> {
        executor.context().spotify.get_current_user().await
    }

    async fn field_item<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, TaggedItemNode, Walked>,
        uri: SpotifyUri,
    ) -> ApiResult<Option<TaggedItemNode>> {
        let context = executor.context();
        let uri = uri.validate("uri")?;
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
    async fn field_item_search<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, ItemSearch, Walked>,
        query: String,
        first: Option<i32>,
        after: Option<Cursor>,
    ) -> ApiResult<ItemSearch> {
        let context = executor.context();

        // Validate params
        if query.is_empty() {
            return Err(InputValidationError {
                field: "query".into(),
                message: "Search query must not be empty".into(),
                value: query.into(),
                backtrace: Backtrace::capture(),
            }
            .into());
        }
        let limit_offset = LimitOffset::try_from_first_after(first, after)?;

        // Run the search query through spotify. This returns a mapping of
        // results, grouped by item type. i.e. one PaginatedResponse for each
        // type (track/album/artist)
        let mut search_response = context
            .spotify
            .search_items(&query, limit_offset.limit(), limit_offset.offset())
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
    fn field_tags<'s, 'r, 'a>(
        &'s self,
        _executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, TagConnection, Walked>,
    ) -> TagConnection {
        TagConnection::All
    }

    /// Get info for a particular tag. If the tag doesn't exist in the DB, we'll
    /// pretend like it does and just return a node with no tagged items. Item
    /// data will be loaded lazily, when requested from [TaggedItemConnection].
    async fn field_tag<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, TagNode, Walked>,
        tag: String,
    ) -> ApiResult<TagNode> {
        let context = executor.context();

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

        // The rest of the item's data will be loaded lazily by
        // TaggedItemConnection
        Ok(TagNode {
            tag,
            item_uris: Some(item_uris),
        })
    }
}

/// A summary of tag information, generated by an unwind query.
#[derive(Clone, Debug, Deserialize)]
struct TagSummary {
    tag: String,
    /// The number of items that have this tag assigned
    num_items: usize,
}
