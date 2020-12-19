use crate::{
    db::{CollectionName, DbHandler, TaggedItemDocument},
    error::ApiResult,
    schema::{TagDetails, TagSummary},
    spotify::Spotify,
    util::{self, UserId},
};
use mongodb::bson::doc;
use rocket::{get, State};
use rocket_contrib::json::Json;

#[get("/tags")]
pub async fn route_get_tags(
    user_id: UserId,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<Vec<TagSummary>>> {
    let cursor = db_handler
        .collection(CollectionName::TaggedItems)
        .aggregate(
            vec![
                doc! {"$match":{"user_id": user_id}},
                doc! {"$unwind":"$tags"},
                doc! {"$group":{"_id":"$tags","num_items":{"$sum":1}}},
                doc! {"$project":{"tag": "$_id", "num_items": 1, "_id": 0}},
                doc! {"$sort": {"num_items": -1}},
            ],
            None,
        )
        .await?;
    let summaries: Vec<TagSummary> = util::from_cursor(cursor).await?;

    Ok(Json(summaries))
}

#[get("/tags/<tag>")]
pub async fn route_get_tag(
    tag: String,
    user_id: UserId,
    spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<TagDetails>> {
    // Look up the relevant items in the DB
    let cursor = db_handler
        .collection(CollectionName::TaggedItems)
        .find(doc! {"tags": &tag, "user_id": &user_id}, None)
        .await?;
    let db_items: Vec<TaggedItemDocument> = util::from_cursor(cursor).await?;

    // Pull data from Spotify to saturate our DB data
    let saturated_items = spotify.saturated_tagged_items(db_items).await?;
    Ok(Json(TagDetails {
        tag,
        items: saturated_items,
    }))
}
