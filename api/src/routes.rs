use mongodb::Client;
use rocket::{post, routes, Route, State};
use rocket_contrib::json::Json;
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

struct UserTrackTags {
    user_id: String,
    track_id: String,
    tags: Vec<String>,
}

/// Function that exports all routes in this file
pub fn all_routes() -> Vec<Route> {
    routes![route_get_track, route_create_tag]
}

#[post("/tracks/<track_id>/tags", format = "json", data = "<body>")]
async fn route_get_track(
    track_id: String,
    body: Json<CreateTagBody>,
    db_client: State<'_, Client>,
) -> Json<Track> {
    let tags = body.to_owned().tags;
    let track = Track { track_id, tags };
    Json(track)
}

#[post("/tracks/<track_id>/tags", format = "json", data = "<body>")]
async fn route_create_tag(
    track_id: String,
    body: Json<CreateTagBody>,
    db_client: State<'_, Client>,
) -> Json<Track> {
    let CreateTagBody { tags } = body.to_owned();

    let track = Track { track_id, tags };
    Json(track)
}
