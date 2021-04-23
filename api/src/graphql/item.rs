use std::convert::TryInto;

use crate::{
    error::ApiResult,
    graphql::{
        internal::GenericEdge, Cursor, Item, ItemSearchFields, PageInfo,
        RequestContext, TagConnection, TaggedItemConnectionFields,
        TaggedItemEdgeFields, TaggedItemNodeFields,
    },
    spotify::{PaginatedResponse, ValidSpotifyUri},
    util,
};
use async_trait::async_trait;
use juniper::{futures::TryStreamExt, Executor};
use juniper_from_schema::{QueryTrail, Walked};
use mongodb::bson::doc;

/// A Spotify item with its applied tags. The item is always preloaded while the
/// tags can be fetched eagerly (preloaded) or lazily (loaded from the DB
/// when requested).
#[derive(Clone)]
pub struct TaggedItemNode {
    pub item: Item,
    /// `None` means lazy-load the tags. This will map to a lazy version of
    /// [TagConnection], which will only load data as needed. `Some` means the
    /// tags are all preloaded and [TagConnection] won't have to make any
    /// queries for its field resolutions.
    pub tags: Option<Vec<String>>,
}

impl TaggedItemNodeFields for TaggedItemNode {
    fn field_id(
        &self,
        executor: &Executor<'_, '_, RequestContext>,
    ) -> juniper::ID {
        // We have to wrap this struct in a `Node` first, because that type
        // defines how to map each of its variants to an ID
        let node: crate::graphql::Node = self.clone().into();
        node.id(&executor.context().user_id)
    }

    fn field_item(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, Item, Walked>,
    ) -> &Item {
        &self.item
    }

    fn field_tags(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, TagConnection, Walked>,
    ) -> TagConnection {
        // If we've already loaded the tags for this item, then we can pass them
        // to the TagConnection and skip a DB query. In some scenarios (e.g.
        // mutations), we can preload tags for free, but in others we
        // want to defer the DB query until it's actually necessary.
        match &self.tags {
            Some(tags) => TagConnection::Preloaded { tags: tags.clone() },
            None => TagConnection::ByItem {
                item_uri: self.item.uri().clone(),
            },
        }
    }
}

pub type TaggedItemEdge = GenericEdge<TaggedItemNode>;

impl TaggedItemEdgeFields for TaggedItemEdge {
    fn field_node(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, TaggedItemNode, Walked>,
    ) -> &TaggedItemNode {
        self.node()
    }

    fn field_cursor(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &Cursor {
        self.cursor()
    }
}

/// "Connection" is a concept from Relay. Read more: https://graphql.org/learn/pagination/
/// This struct provides data about a particular collection of tagged items.
/// The data may be loaded eagerly or lazily. See individual variants for the
/// possible options.
pub enum TaggedItemConnection {
    /// All item data is preloaded from the Spotify API. The first level of
    /// field resolutions for this variant will be immediate, and not require
    /// any I/O (nested fields may require additional I/O, but that's beyond
    /// the concern of this struct).
    ///
    /// This variant should be used whenever item data is already present, but
    /// you shouldn't prefetch data just for the purposes of using this
    /// variant. In those cases, use one of the lazily loaded variants instead.
    Preloaded {
        paginated_response: PaginatedResponse<Item>,
    },

    /// Lazily load item data, where the items in the collection are defined by
    /// a list of URIs. When item data is needed, all the items will be fetched
    /// from the Spotify API in a single request.
    ///
    /// This variant currently doesn't support pagination, but that can be
    /// added if necessary.
    ByUris { uris: Vec<ValidSpotifyUri> },

    /// Lazily load item data, where the items in the collection are defined by
    /// a single tag. When item data is needed, the list of items that match
    /// the tag will be fetched from the DB, _then_ those items will be fetched
    /// from the Spotify API.
    ///
    /// This variant currently doesn't support pagination, but that can be
    /// added if necessary.
    ByTag { tag: String },
}

#[async_trait]
impl TaggedItemConnectionFields for TaggedItemConnection {
    /// Get the total number of items in this connection, across all pages. If
    /// item data is preloaded, this will be fast. If we're in lazy mode, this
    /// will require a DB query.
    async fn field_total_count<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
    ) -> ApiResult<i32> {
        let context = executor.context();
        let total_count = match self {
            Self::Preloaded { paginated_response } => {
                util::to_i32(paginated_response.total)
            }
            // These URIs aren't paginated, they represent the full data set
            Self::ByUris { uris } => util::to_i32(uris.len()),
            // Count the number of matching docs in the DB
            Self::ByTag { tag } => util::to_i32(
                context
                    .db_handler
                    .collection_tagged_items()
                    .count_by_tag(&context.user_id, tag)
                    .await?,
            ),
        };
        Ok(total_count)
    }

    /// Get page info for these items. If item data is preloaded, this will
    /// be fast. If we're in lazy mode, this will require a DB query.
    async fn field_page_info<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, PageInfo, Walked>,
    ) -> ApiResult<PageInfo> {
        let page_info = match self {
            // This variant supports pagination via the Spotify API
            Self::Preloaded { paginated_response } => PageInfo {
                offset: paginated_response.offset,
                page_len: paginated_response.items.len(),
                has_previous_page: paginated_response.previous.is_some(),
                has_next_page: paginated_response.next.is_some(),
            },

            // This variant doesn't support pagination, so offset is always 0
            Self::ByUris { uris } => PageInfo {
                offset: 0,
                page_len: uris.len(),
                has_previous_page: false,
                has_next_page: false,
            },

            // This variant doesn't support pagination, so offset is always 0
            Self::ByTag { .. } => {
                // This will hit the DB to count matching records
                let total_count = self.field_total_count(executor).await?;
                PageInfo {
                    offset: 0,
                    // This conversion _shouldn't_ ever fail, but better safe
                    // than sorry
                    page_len: total_count.try_into()?,
                    has_previous_page: false,
                    has_next_page: false,
                }
            }
        };

        Ok(page_info)
    }

    async fn field_edges<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, TaggedItemEdge, Walked>,
    ) -> ApiResult<Vec<TaggedItemEdge>> {
        let context = executor.context();

        let (items, offset): (Vec<Item>, usize) = match self {
            // Items have already been loaded from spotify, so just return them
            Self::Preloaded { paginated_response } => {
                // We have to clone each item individually to return owned
                // values, so this clone here is unfortunate but not that bad
                (paginated_response.items.clone(), paginated_response.offset)
            }

            // We have a list of items, fetch the data from spotify
            Self::ByUris { uris } => {
                let items = context.spotify.get_items(uris.iter()).await?;
                // We don't support pagination on this variant yet, so offset
                // is always 0
                (items, 0)
            }

            // Fetch all the items for a tag, then fetch data for those items
            // from spotify
            Self::ByTag { tag } => {
                // Get URIs from DB
                let cursor = context
                    .db_handler
                    .collection_tagged_items()
                    .find_by_tag(&context.user_id, tag)
                    .await?;
                let uris: Vec<ValidSpotifyUri> =
                    cursor.map_ok(|doc| doc.uri).try_collect().await?;

                let items = context.spotify.get_items(uris.iter()).await?;
                // We don't support pagination on this variant yet, so offset
                // is always 0
                (items, 0)
            }
        };

        // Map items to nodes, then to edges
        let edges = TaggedItemEdge::from_nodes(
            items.into_iter().map(|item| {
                TaggedItemNode {
                    item,
                    // Tag data isn't present yet, defer loading it
                    tags: None,
                }
            }),
            offset,
        );

        Ok(edges)
    }
}

/// Result of running a search query among taggable items. This is the result of
/// a single Spotify API request, but Spotify returns the items grouped by type
/// so that's what we'll do. The 3 connections pagination in lockstep, i.e. they
/// use the same limit/offset.
/// https://developer.spotify.com/documentation/web-api/reference/#category-search
pub struct ItemSearch {
    pub tracks: TaggedItemConnection,
    pub albums: TaggedItemConnection,
    pub artists: TaggedItemConnection,
}

impl ItemSearchFields for ItemSearch {
    fn field_tracks(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, TaggedItemConnection, Walked>,
    ) -> &TaggedItemConnection {
        &self.tracks
    }

    fn field_albums(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, TaggedItemConnection, Walked>,
    ) -> &TaggedItemConnection {
        &self.albums
    }

    fn field_artists(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, TaggedItemConnection, Walked>,
    ) -> &TaggedItemConnection {
        &self.albums
    }
}
