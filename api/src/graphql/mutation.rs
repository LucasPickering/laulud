//! All types that are unique to GraphQL mutations

use crate::{
    error::{ApiError, ApiResult},
    graphql::{
        AddTagInput, AddTagPayloadFields, DeleteTagInput,
        DeleteTagPayloadFields, MutationFields, RequestContext, TaggedItemNode,
    },
};
use async_trait::async_trait;
use juniper::Executor;
use juniper_from_schema::{QueryTrail, Walked};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
};
use std::backtrace::Backtrace;

/// Root GraphQL mutation
pub struct Mutation;

#[async_trait]
impl MutationFields for Mutation {
    async fn field_add_tag<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, AddTagPayload, Walked>,
        input: AddTagInput,
    ) -> ApiResult<AddTagPayload> {
        let context = executor.context();
        // TODO input validation

        // Look up the item in Spotify first, to get metadata/confirm it's real
        // TODO return none if item doesn't exist
        let spotify_item = context.spotify.get_item(&input.item_uri).await?;

        // Do the update query
        let item_doc = context
            .db_handler
            .collection_tagged_items()
            .find_one_and_update(
                doc! {"uri": &input.item_uri, "user_id": &context.user_id},
                // Add each tag to the doc if it isn't present already
                doc! {"$addToSet": {"tags": &input.tag}},
                Some(
                    FindOneAndUpdateOptions::builder()
                        .upsert(true)
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?
            // Handle the None case - this shouldn't be possible because we have
            // upsert=true, but just to be safe
            .ok_or_else(|| ApiError::Unknown {
                message: ("No result from findOneAndUpdate".into()),
                backtrace: Backtrace::capture(),
            })?;

        Ok(AddTagPayload {
            item: Some(TaggedItemNode {
                item: spotify_item,
                // We get tag data preloaded for free from the query
                tags: Some(item_doc.tags),
            }),
        })
    }

    async fn field_delete_tag<'s, 'r, 'a>(
        &'s self,
        executor: &Executor<'r, 'a, RequestContext>,
        _trail: &QueryTrail<'r, DeleteTagPayload, Walked>,
        input: DeleteTagInput,
    ) -> ApiResult<DeleteTagPayload> {
        // TODO input validation
        let context = executor.context();

        // Look up the item in Spotify first, to get metadata/confirm it's real
        // TODO return none if item doesn't exist
        let spotify_item = context.spotify.get_item(&input.item_uri).await?;

        // Look up tags in mongo - will return None if item doesn't exist
        let item_doc_opt = context
            .db_handler
            .collection_tagged_items()
            .find_one_and_update(
                doc! {"uri": &input.item_uri, "user_id": &context.user_id},
                // Remove the tag from the doc
                doc! {"$pull": {"tags": &input.tag}},
                Some(
                    FindOneAndUpdateOptions::builder()
                        .return_document(ReturnDocument::After)
                        .build(),
                ),
            )
            .await?;
        // If the item doesn't exist, just pretend like the tag was deleted
        let tags = item_doc_opt
            .map(|item_doc| item_doc.tags)
            .unwrap_or_default();

        Ok(DeleteTagPayload {
            item: Some(TaggedItemNode {
                item: spotify_item,
                // We get tag data preloaded for free from the query
                tags: Some(tags),
            }),
        })
    }
}

pub struct AddTagPayload {
    pub item: Option<TaggedItemNode>,
}

impl AddTagPayloadFields for AddTagPayload {
    fn field_item(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, TaggedItemNode, Walked>,
    ) -> &Option<TaggedItemNode> {
        &self.item
    }
}

pub struct DeleteTagPayload {
    pub item: Option<TaggedItemNode>,
}

impl DeleteTagPayloadFields for DeleteTagPayload {
    fn field_item(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, TaggedItemNode, Walked>,
    ) -> &Option<TaggedItemNode> {
        &self.item
    }
}
