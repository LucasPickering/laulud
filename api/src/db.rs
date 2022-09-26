use crate::{
    auth::UserId, error::ApiResult, graphql::Tag, spotify::SpotifyUri,
    LauludConfig,
};
use derive_more::{Deref, From};
use mongodb::{
    bson::{self, doc, Document},
    options::ClientOptions,
    Client, Collection, Cursor, Database,
};
use serde::{Deserialize, Serialize};

const DATABASE_NAME: &str = "laulud";

pub struct DbHandler {
    client: Client,
}

impl DbHandler {
    pub async fn connect(config: &LauludConfig) -> ApiResult<Self> {
        let options = ClientOptions::parse(&config.database_url).await?;
        let client = Client::with_options(options).unwrap();
        Ok(Self { client })
    }

    fn database(&self) -> Database {
        self.client.database(DATABASE_NAME)
    }

    /// Get a reference to the `taggedItems` collection from the DB. This can
    /// be used for any and all interactions with that collection. See the
    /// [TaggedItemsCollection] wrapper type for additional functionality
    /// provided beyond the stock Mongo functions.
    pub fn collection_tagged_items(&self) -> TaggedItemsCollection {
        self.database()
            .collection(TaggedItemsCollection::name())
            .into()
    }
}

/// A wrapper around the `taggedItems` collection that provides extra
/// functionality that's commonly needed on the collection. This is basically
/// a mini ORM for the collection.
///
/// The point of this is to encapsulate most (if not all) Mongo-specific logic
/// in one place, so that if we change the Mongo schema, we don't end up with
/// a bunch of broken queries around the app.
///
/// Right now the app doesn't support any cross-user interaction, so every
/// method on this filters by user ID. As such, we don't bother mentioning
/// the user in the method name, for brevity.
#[derive(Debug, Deref, From)]
pub struct TaggedItemsCollection {
    collection: Collection<TaggedItemDocument>,
}

impl TaggedItemsCollection {
    // Get the name of this collection, as defined in the DB
    pub fn name() -> &'static str {
        "taggedItems"
    }

    /// Filter this collection for documents owned by a particular user that
    /// have a particular tag applied
    pub async fn find_by_tag(
        &self,
        user_id: &UserId,
        tag: &Tag,
    ) -> ApiResult<Cursor<TaggedItemDocument>> {
        Ok(self
            .collection
            .find(Self::filter_by_tag(user_id, tag), None)
            .await?)
    }

    /// Count the number of documents owned by a particular user that
    /// have a particular tag applied
    pub async fn count_by_tag(
        &self,
        user_id: &UserId,
        tag: &Tag,
    ) -> ApiResult<i64> {
        Ok(self
            .collection
            .count_documents(Self::filter_by_tag(user_id, tag), None)
            .await?)
    }

    /// Count the number of unique tags that this user has created
    pub async fn count_tags(&self, user_id: &UserId) -> ApiResult<i64> {
        self.count_tags_helper(Self::filter_by_user(user_id)).await
    }

    /// Count the number of unique tags that a user has applied to a particular
    /// item
    pub async fn count_tags_by_item(
        &self,
        user_id: &UserId,
        item_uri: &SpotifyUri,
    ) -> ApiResult<i64> {
        self.count_tags_helper(Self::filter_by_item(user_id, item_uri))
            .await
    }

    /// Get a list of unique tags that this user has created
    pub async fn find_tags(&self, user_id: &UserId) -> ApiResult<Vec<Tag>> {
        self.find_tags_helper(Self::filter_by_user(user_id)).await
    }

    /// Get a list of unique tags that this user has applied to a particular
    /// item
    pub async fn find_tags_by_item(
        &self,
        user_id: &UserId,
        item_uri: &SpotifyUri,
    ) -> ApiResult<Vec<Tag>> {
        self.find_tags_helper(Self::filter_by_item(user_id, item_uri))
            .await
    }

    fn filter_by_user(user_id: &UserId) -> Document {
        doc! {"user_id": user_id}
    }

    fn filter_by_tag(user_id: &UserId, tag: &Tag) -> Document {
        doc! {"user_id": user_id, "tags":tag}
    }

    fn filter_by_item(user_id: &UserId, item_uri: &SpotifyUri) -> Document {
        doc! {"user_id": user_id, "uri": item_uri}
    }

    /// Internal helper function to count the number of unique tags in the DB
    /// that match a given filter
    async fn count_tags_helper(
        &self,
        match_filter: Document,
    ) -> ApiResult<i64> {
        let mut cursor = self
            .collection
            .aggregate(
                vec![
                    doc! {"$match": match_filter},
                    doc! {"$unwind": "$tags"},
                    doc! {"$count": "count"},
                ],
                None,
            )
            .await?;

        // This should return either 0 docs (no tags for this user)
        // or 1 doc (all other cases). If the doc is present, parse it
        // to get the tag count
        let count = match cursor.next().await {
            Some(doc) => {
                let count_doc: CountDocument =
                    mongodb::bson::from_document(doc?)?;
                count_doc.count
            }
            None => 0,
        };

        Ok(count)
    }

    /// Internal helper function to fetch and return a list of the unique tags
    /// in the DB that match a given filter
    async fn find_tags_helper(
        &self,
        match_filter: Document,
    ) -> ApiResult<Vec<Tag>> {
        let mut cursor = self
            .collection
            .aggregate(
                vec![
                    doc! {"$match": match_filter},
                    doc! {"$unwind": "$tags"},
                    doc! {"$group": {"_id": "$tags"}},
                    doc! {"$project": {"tag": "$_id", "_id": 0}},
                    // Sort tags alphabetically
                    doc! {"$sort": {"tag": 1}},
                ],
                None,
            )
            .await?;

        // Load and deserialize each doc
        let mut tags = Vec::new();
        while let Some(doc) = cursor.next().await {
            let doc: TagSummaryDocument = bson::from_document(doc?)?;
            tags.push(doc.tag)
        }

        Ok(tags)
    }
}

// ===== DB Schema =====
// Below is the schema for each collection in the DB

/// A document in [CollectionName::TaggedItems]. Any item type can be tagged.
/// The item type can be grabbed via `uri.uri_type`. We avoid storing any data
/// beyond the URI because it can change, and there's probably legal shit around
/// it too. So we just fetch it from Spotify on-demand whenever it's needed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaggedItemDocument {
    pub user_id: UserId,
    pub tags: Vec<Tag>,
    pub uri: SpotifyUri,
}

/// A Mongo document that counts a single `count` field. Useful when
/// deserializing the results of an aggregation that ends in a
/// `{$count:"count"}` step.
#[derive(Copy, Clone, Debug, Deserialize)]
struct CountDocument {
    /// Technically this could be a usize because counts should never be
    /// negative, but Mongo uses `i64` for the `count_*` methods, so we stick
    /// with that for consistency
    pub count: i64,
}

/// A summary of tag information, generated by an unwind query.
#[derive(Clone, Debug, Deserialize)]
struct TagSummaryDocument {
    tag: Tag,
}
