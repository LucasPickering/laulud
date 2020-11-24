use crate::{
    db::{CollectionName, DbHandler, TrackDocument},
    error::{ApiError, ApiResult},
    schema::{TagDetails, TaggedTrack},
    spotify::Spotify,
    util,
};
use mongodb::bson::doc;
use rocket::{get, State};
use rocket_contrib::json::Json;
use std::{backtrace::Backtrace, collections::HashMap};
use tokio::stream::StreamExt;

#[get("/tags", format = "json")]
pub async fn route_get_tags(
    mut spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<Vec<TagDetails>>> {
    let coll = db_handler.collection(CollectionName::Tracks);
    let user_id = spotify.get_user_id().await?;
    let doc = coll
        .find_one(doc! { "tags": &track_id, "user_id": user_id }, None)
        .await?;
    let tags = doc
        .map::<ApiResult<Vec<String>>, _>(|doc| {
            Ok(util::from_doc::<TrackDocument>(doc)?.tags)
        })
        .transpose()? // Option<Result> -> Result<Option>
        .unwrap_or_else(Vec::new);

    Ok(Json(TaggedTrack {
        track: spotify_track,
        tags,
    }))
}
