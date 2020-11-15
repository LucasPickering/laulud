use crate::{
    db::{CollectionName, DbHandler},
    error::{ApiError, ApiResult},
    util,
};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
};
use rocket::{get, post, routes, Route, State};
use rocket_contrib::json::Json;
use rspotify::{client::Spotify, senum::SearchType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Track {
    track_id: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateTagBody {
    tags: Vec<String>,
}

/// Function that exports all routes in this file
pub fn all_routes() -> Vec<Route> {
    routes![route_get_track, route_search_tracks, route_create_tag]
}

#[get("/tracks/<track_id>", format = "json")]
async fn route_get_track(
    track_id: String,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<Option<Track>>> {
    let coll = db_handler.collection(CollectionName::Tracks);
    let track_doc = coll.find_one(doc! { "track_id": &track_id }, None).await?;

    match track_doc {
        None => Err(ApiError::NotFound(track_id)),
        Some(doc) => {
            let track = util::from_doc(doc)?;
            Ok(Json(track))
        }
    }
}

#[get("/tracks/search/<query>", format = "json")]
async fn route_search_tracks(
    query: String,
    db_handler: State<'_, DbHandler>,
    spotify: State<'_, Spotify>,
) -> ApiResult<Json<Vec<Track>>> {
    let search_results = spotify
        .search(&query, SearchType::Track, 0, 0, None, None)
        .await
        .map_err(ApiError::Spotify)?;
    dbg!(search_results);
    Ok(Json(Vec::new()))
}

#[post("/tracks/<track_id>/tags", format = "json", data = "<body>")]
async fn route_create_tag(
    track_id: String,
    body: Json<CreateTagBody>,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<Track>> {
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
        None => {
            Err(ApiError::Unknown("No result from findOneAndUpdate".into()))
        }
        Some(doc) => {
            let track = util::from_doc(doc)?;
            Ok(Json(track))
        }
    }
}
