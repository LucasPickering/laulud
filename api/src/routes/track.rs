use crate::{
    db::{CollectionName, DbHandler},
    error::{ApiError, ApiResult},
    schema::{CreateTagBody, TaggedTrack},
    spotify::Spotify,
    util,
};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
};
use rocket::{get, post, State};
use rocket_contrib::json::Json;
use std::backtrace::Backtrace;

#[get("/tracks/<track_id>", format = "json")]
pub async fn route_get_track(
    track_id: String,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<Option<TaggedTrack>>> {
    let coll = db_handler.collection(CollectionName::Tracks);
    let track_doc = coll.find_one(doc! { "track_id": &track_id }, None).await?;

    match track_doc {
        None => Err(ApiError::NotFound {
            resource: (track_id),
            backtrace: Backtrace::capture(),
        }),
        Some(doc) => {
            let track = util::from_doc(doc)?;
            Ok(Json(track))
        }
    }
}

#[get("/tracks/search/<query>", format = "json")]
pub async fn route_search_tracks(
    query: String,
    mut spotify: Spotify,
) -> ApiResult<Json<Vec<TaggedTrack>>> {
    let data = spotify
        .search_tracks(&query)
        .await?
        .into_iter()
        .map(|track| TaggedTrack {
            track,
            tags: Vec::new(), // TODO
        })
        .collect();
    Ok(Json(data))
}

#[post("/tracks/<track_id>/tags", format = "json", data = "<body>")]
pub async fn route_create_tag(
    track_id: String,
    body: Json<CreateTagBody>,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<TaggedTrack>> {
    let CreateTagBody { tags } = body.to_owned();

    let coll = db_handler.collection(CollectionName::Tracks);
    let update_doc = coll
        .find_one_and_update(
            doc! {"track_id": &track_id},
            // Add each tag to the doc if it isn't present already
            doc! {"$addToSet": {"tags": {"$each": &tags}}},
            Some(
                FindOneAndUpdateOptions::builder()
                    .upsert(true)
                    .return_document(ReturnDocument::After)
                    .build(),
            ),
        )
        .await?;

    match update_doc {
        // This shouldn't be possible because we have upsert=true, but let's
        // handle it just to be safe
        None => Err(ApiError::Unknown {
            message: ("No result from findOneAndUpdate".into()),
            backtrace: Backtrace::capture(),
        }),
        Some(doc) => {
            let track = util::from_doc(doc)?;
            Ok(Json(track))
        }
    }
}
