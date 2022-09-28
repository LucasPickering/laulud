use std::convert::TryInto;

use crate::{
    graphql::{
        core::PageInfo, internal::GenericEdge, item::TaggedItemConnection,
        Cursor, Node, RequestContext, Tag,
    },
    spotify::SpotifyUri,
};
use async_graphql::{Context, FieldResult, Object};
use derive_more::Deref;

/// A user-defined tag. Tags have a many-to-many relationship with Spotify
/// items, and all tag data is stored in the local DB. The items associated
/// with this tag can be loaded (preloaded) or lazily (loaded from the DB
/// when requested).
#[derive(Clone, Debug)]
pub struct TagNode {
    pub tag: Tag,
    /// `None` means lazy-load the list of item URIs. This will map to a lazy
    /// version of [TaggedItemConnection], which will only load the list of
    /// items as needed. `Some` means the list of item URIs is preloaded
    /// and [TaggedNodeConnection] won't have to make any DB queries to get the
    /// list of items that match the tag. Note that in either case, we're only
    /// loading item **URIs**, not the full item data. So either way, the
    /// full item data won't be preloaded from the Spotify API, we're just
    /// saving a DB query in the eager case.
    pub item_uris: Option<Vec<SpotifyUri>>,
}

#[Object]
impl TagNode {
    pub async fn id(
        &self,
        context: &Context<'_>,
    ) -> FieldResult<async_graphql::ID> {
        // We have to wrap this struct in a `Node` first, because that type
        // defines how to map each of its variants to an ID
        let node: Node = self.clone().into();
        Ok(node.get_id(context).await?)
    }

    async fn tag(&self) -> &Tag {
        &self.tag
    }

    /// Lazily fetch items for this tag node
    /// TODO support pagination on this
    async fn items(&self) -> TaggedItemConnection {
        match &self.item_uris {
            // We have URIs already, so we can skip the DB query to fetch them
            Some(item_uris) => TaggedItemConnection::ByUris {
                uris: item_uris.clone(),
            },
            // URIs haven't been loaded yet, TaggedItemConnection will have to
            // do a DB query to get them before doing anything else
            None => TaggedItemConnection::ByTag {
                tag: self.tag.clone(),
            },
        }
    }
}

#[derive(Clone, Debug, Deref)]
pub struct TagEdge(GenericEdge<TagNode>);

#[Object]
impl TagEdge {
    async fn node(&self) -> &TagNode {
        self.0.node()
    }

    async fn cursor(&self) -> &Cursor {
        self.0.cursor()
    }
}

/// "Connection" is a concept from Relay. Read more: https://graphql.org/learn/pagination/
/// This struct provides data about a particular collection of tags. The list
/// of tags may be loaded eagerly or lazily. See individual variants for the
/// possible options.
pub enum TagConnection {
    /// The list of tags is preloaded from the DB. The first level of
    /// field resolutions for this variant will be immediate, and not require
    /// any I/O (nested fields may require additional I/O, but that beyond
    /// the concern of this struct).
    ///
    /// This variant should be used whenever tag data is already present, but
    /// you shouldn't prefetch data just for the purposes of using this
    /// variant. In those cases, use one of the lazily loaded variants instead.
    Preloaded { tags: Vec<Tag> },

    /// Lazily load tag data for **all** tags defined by this user. The list of
    /// tags that this user has created will be fetched lazily, as needed.
    ///
    /// This variant currently doesn't support pagination, but that can be
    /// added if necessary.
    All,

    /// Lazily load tag data, where the list of tags is defined by an item URI.
    /// Any tag that is applied to the item will be included.
    ///
    /// This variant currently doesn't support pagination, but that can be
    /// added if necessary.
    ByItem { item_uri: SpotifyUri },
}

#[Object]
impl TagConnection {
    async fn total_count(&self, context: &Context<'_>) -> FieldResult<usize> {
        let context = context.data::<RequestContext>()?;
        let collection = context.db_handler.collection_tagged_items();

        let total_count = match self {
            Self::Preloaded { tags } => tags.len(),
            // Count all tags in the DB for this user
            Self::All => {
                collection.count_tags(&context.user_id).await?.try_into()?
            }
            // Count all tags in the DB for a single user+item
            Self::ByItem { item_uri } => collection
                .count_tags_by_item(&context.user_id, item_uri)
                .await?
                .try_into()?,
        };

        Ok(total_count)
    }

    async fn page_info(&self, context: &Context<'_>) -> FieldResult<PageInfo> {
        // We don't actually support paginating through tags in any way yet,
        // so the offset is always 0 on these
        let page_info = match self {
            Self::Preloaded { tags } => PageInfo {
                offset: 0,
                page_len: tags.len(),
                has_previous_page: false,
                has_next_page: false,
            },

            // This variant doesn't support pagination, so offset is always 0
            Self::All { .. } | Self::ByItem { .. } => {
                // In either case, this will hit the DB to count matches
                let total_count = self.total_count(&context).await?;
                PageInfo {
                    offset: 0,
                    page_len: total_count,
                    has_previous_page: false,
                    has_next_page: false,
                }
            }
        };

        Ok(page_info)
    }

    async fn edges(&self, context: &Context<'_>) -> FieldResult<Vec<TagEdge>> {
        let context = context.data::<RequestContext>()?;
        let collection = context.db_handler.collection_tagged_items();

        // Get a list of raw tags, whether it pre-loaded or we have to go
        // to the DB
        let tags: Vec<Tag> = match self {
            // Tags have been loaded eagerly, so no I/O required here
            Self::Preloaded { tags } => tags.clone(),

            // Tags haven't been loaded yet, fetch all of them
            Self::All => collection.find_tags(&context.user_id).await?,

            // Tags haven't been loaded yet, so fetch them now, filtered by a
            // single item
            Self::ByItem { item_uri } => {
                collection
                    .find_tags_by_item(&context.user_id, item_uri)
                    .await?
            }
        };

        // Map individual tags into graphql edges
        let edges = TagEdge::from_nodes(
            tags.into_iter().map(|tag| TagNode {
                tag,
                // Defer loading the items for this tag until needed
                item_uris: None,
            }),
            0,
        );
        Ok(edges)
    }
}
