use mongodb::bson::{self, doc, Bson};
use rocket::{get, post, routes, Route, State};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::{CollectionName, DbHandler};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Track {
    track_id: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateTagBody {
    tags: Vec<String>,
}

struct UserTrackTags {
    user_id: String,
    track_id: String,
    tags: Vec<String>,
}

/// Function that exports all routes in this file
pub fn all_routes() -> Vec<Route> {
    routes![route_get_track, route_create_tag]
}

#[get("/tracks/<track_id>", format = "json")]
async fn route_get_track(
    track_id: String,
    db_handler: State<'_, DbHandler>,
) -> Json<Option<Track>> {
    let coll = db_handler.collection(CollectionName::Tracks);
    let track_doc = coll
        .find_one(dbg!(doc! { "track_id": &track_id }), None)
        .await
        .unwrap();

    let track =
        track_doc.map(|doc| bson::from_bson(Bson::Document(doc)).unwrap());
    Json(track)
}

#[post("/tracks/<track_id>/tags", format = "json", data = "<body>")]
async fn route_create_tag(
    track_id: String,
    body: Json<CreateTagBody>,
    db_handler: State<'_, DbHandler>,
) -> Json<Track> {
    let CreateTagBody { tags } = body.to_owned();

    let coll = db_handler.collection(CollectionName::Tracks);
    coll.insert_one(
        doc! {
            "track_id": &track_id,
            "tags": &tags,
        },
        None,
    )
    .await
    .unwrap();

    let track = Track { track_id, tags };
    Json(track)
}
