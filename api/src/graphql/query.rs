//! This module holds the top-level GraphQL query object. All query subtypes
//! should be defined elsewhere.

use crate::{
    error::ApiResult,
    graphql::{
        internal::NodeType, Cursor, ItemSearch, PrivateUser, QueryFields,
        RequestContext, SpotifyUri, TagConnection, TagNode,
        TaggedItemConnection, TaggedItemNode,
    },
};
use async_trait::async_trait;
use juniper::{futures::StreamExt, Executor};
use juniper_from_schema::{QueryTrail, Walked};
use mongodb::bson::doc;
use serde::Deserialize;
use std::convert::TryInto;

/// Root GraphQL query
pub struct Query;

#[async_trait]
impl QueryFields for Query {
    /// Get a node of any type by UUID.
    ///
    /// For some reason, rust-analyzer shows an error if you import Node, so
    /// use the qualified path here just to get around that. Not really
    /// necessary, but makes working a tiny bit nicer.
    async fn field_node<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, crate::graphql::Node, Walked>,
        id: juniper::ID,
    ) -> ApiResult<Option<crate::graphql::Node>> {
        let context = executor.context();
        let (node_type, value_id, user_id) = NodeType::parse_id(&id)?;

        // Nice try, Satan
        if user_id != context.user_id {
            // Another user owns the node, pretend like it doesn't exist
            return Ok(None);
        }

        let node = match node_type {
            NodeType::TaggedItemNode => {
                // For items, the value ID is the URI. Look up the item in the
                // Spotify API
                let item_uri = value_id;
                // TODO figure out a clean way to map missing items to None
                let item = context.spotify.get_item(&item_uri).await?;
                Some(TaggedItemNode { item, tags: None }.into())
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
        // TODO figure out a clean way to map missing items to None
        let spotify_item = context.spotify.get_item(&uri).await?;

        Ok(Some(TaggedItemNode {
            item: spotify_item,
            tags: None,
        }))
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
        // We need to run the search through spotify, then join tag data
        let context = executor.context();
        // TODO replace unwrap with some input validation logic
        let limit: Option<usize> = first.map(|first| first.try_into().unwrap());
        let offset: Option<usize> = after.map(|cursor| cursor.offset());

        // Run the search query through spotify. This returns a mapping of
        // results, grouped by item type. i.e. one PaginatedResponse for each
        // type (track/album/artist)
        // TODO pass limit/offset
        let mut search_response =
            context.spotify.search_items(&query, limit, offset).await?;

        // Pull out the item types we care about. This should always exhaust
        // the map (with no missing types), because the fields we return here
        // line up with the types requested from Spotify
        let rv = ItemSearch {
            // TODO clean up unwraps
            tracks: TaggedItemConnection::Preloaded {
                paginated_response: search_response.remove("tracks").unwrap(),
            },
            albums: TaggedItemConnection::Preloaded {
                paginated_response: search_response.remove("albums").unwrap(),
            },
            artists: TaggedItemConnection::Preloaded {
                paginated_response: search_response.remove("artists").unwrap(),
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
