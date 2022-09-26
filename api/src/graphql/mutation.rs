//! All types that are unique to GraphQL mutations

use crate::{
    error::{ApiError, ApiResult},
    graphql::{
        RequestContext, Tag, TagEdge, TagNode, TaggedItemEdge, TaggedItemNode,
    },
    spotify::SpotifyUri,
};
use async_graphql::{InputObject, Object, SimpleObject};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
};
use std::backtrace::Backtrace;

/// Root GraphQL mutation
pub struct Mutation;

#[Object]
impl Mutation {
    async fn add_tag(&self, input: AddTagInput) -> ApiResult<AddTagPayload> {
        let context = executor.context();

        // Look up the item in Spotify first, to get metadata/confirm it's real
        let item_node = match context.spotify.get_item(&uri).await? {
            Some(spotify_item) => {
                // Do the update query
                let item_doc = context
                    .db_handler
                    .collection_tagged_items()
                    .find_one_and_update(
                        doc! {"uri": &uri, "user_id": &context.user_id},
                        // Add each tag to the doc if it isn't present already
                        doc! {"$addToSet": {"tags": &tag}},
                        Some(
                            FindOneAndUpdateOptions::builder()
                                .upsert(true)
                                .return_document(ReturnDocument::After)
                                .build(),
                        ),
                    )
                    .await?
                    // Handle the None case - this shouldn't be possible because
                    // we have upsert=true, but just to be safe
                    .ok_or_else(|| ApiError::Unknown {
                        message: ("No result from findOneAndUpdate".into()),
                        backtrace: Backtrace::capture(),
                    })?;

                Some(TaggedItemNode {
                    item: spotify_item,
                    // We get tag data preloaded for free from the query
                    tags: Some(item_doc.tags),
                })
            }
            // URI doesn't exist in spotify
            None => None,
        };
        let tag_node = TagNode {
            tag,
            item_uris: None,
        };

        Ok(AddTagPayload {
            item_edge: item_node.map(TaggedItemEdge::from),
            tag_edge: tag_node.into(),
        })
    }

    async fn delete_tag(
        &self,
        input: DeleteTagInput,
    ) -> ApiResult<DeleteTagPayload> {
        let context = executor.context();
        let tag = input.tag.validate("tag")?;

        // Look up the item in Spotify first, to get metadata/confirm it's real
        let uri = input.item_uri.validate("input.item_uri")?;
        let item_node = match context.spotify.get_item(&uri).await? {
            Some(spotify_item) => {
                // Look up tags in mongo - will return None if item doesn't
                // exist
                let item_doc_opt = context
                    .db_handler
                    .collection_tagged_items()
                    .find_one_and_update(
                        doc! {"uri": &uri, "user_id": &context.user_id},
                        // Remove the tag from the doc
                        doc! {"$pull": {"tags": &tag}},
                        Some(
                            FindOneAndUpdateOptions::builder()
                                .return_document(ReturnDocument::After)
                                .build(),
                        ),
                    )
                    .await?;
                // If the item doesn't exist, just pretend like the tag was
                // deleted
                let tags = item_doc_opt
                    .map(|item_doc| item_doc.tags)
                    .unwrap_or_default();

                Some(TaggedItemNode {
                    item: spotify_item,
                    // We get tag data preloaded for free from the query
                    tags: Some(tags),
                })
            }
            // URI doesn't exist in spotify
            None => None,
        };
        let tag_node = TagNode {
            tag,
            item_uris: None,
        };

        Ok(DeleteTagPayload {
            item_edge: item_node.map(TaggedItemEdge::from),
            tag_edge: tag_node.into(),
        })
    }
}

/// Input for the `addTag` mutation
#[derive(Clone, Debug, InputObject)]
pub struct AddTagInput {
    pub item_uri: SpotifyUri,
    pub tag: Tag,
}

/// Output for the `addTag` mutation
#[derive(Clone, Debug, SimpleObject)]
pub struct AddTagPayload {
    pub item_edge: Option<TaggedItemEdge>,
    pub tag_edge: TagEdge,
}

/// Input for the `deleteTag` mutation
#[derive(Clone, Debug, InputObject)]
pub struct DeleteTagInput {
    pub item_uri: SpotifyUri,
    pub tag: Tag,
}

/// Output for the `deleteTag` mutation
#[derive(Clone, Debug, SimpleObject)]
pub struct DeleteTagPayload {
    pub item_edge: Option<TaggedItemEdge>,
    pub tag_edge: TagEdge,
}
