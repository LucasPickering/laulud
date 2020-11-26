use crate::{
    db::{CollectionName, DbHandler, TrackDocument},
    error::ApiResult,
    schema::{TagDetails, TaggedTrack},
    spotify::Spotify,
    util,
};
use mongodb::bson::doc;
use rocket::{get, State};
use rocket_contrib::json::Json;

// #[get("/tags", format = "json")]
// pub async fn route_get_tags(
//     mut spotify: Spotify,
//     db_handler: State<'_, DbHandler>,
// ) -> ApiResult<Json<Vec<TagDetails>>> {
//     let coll = db_handler.collection(CollectionName::Tracks);
//     let user_id = spotify.get_user_id().await?;
//     let doc = coll
//         .find(doc! { "tags": &track_id, "user_id": user_id }, None)
//         .await?;
//     let tags = doc
//         .map::<ApiResult<Vec<String>>, _>(|doc| Ok(util::from_doc::<TrackDocument>(doc)?.tags))
//         .transpose()? // Option<Result> -> Result<Option>
//         .unwrap_or_else(Vec::new);

//     Ok(Json(TaggedTrack {
//         track: spotify_track,
//         tags,
//     }))
// }

#[get("/tags/<tag>", format = "json")]
pub async fn route_get_tag(
    tag: String,
    mut spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<TagDetails>> {
    let user_id = spotify.get_user_id().await?;

    // Look up the relevant tracks in the DB
    let cursor = db_handler
        .collection(CollectionName::Tracks)
        .find(doc! {"tags": &tag, "user_id": &user_id}, None)
        .await?;
    let db_tracks: Vec<TrackDocument> = util::from_cursor(cursor).await?;

    // Saturate the Spotify data with the tags from mongo
    let spotify_tracks = spotify
        .get_tracks(
            db_tracks
                .iter()
                .map(|track| track.track_id.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        )
        .await?;

    // Join the datasets. Spotify returns tracks in the same order we request,
    // so we can just pair them together. If a track returns None from spotify,
    // then we'll exclude it
    let joined_tracks = db_tracks
        .into_iter()
        .zip(spotify_tracks)
        // If Spotify returns None for a track, just skip it
        .filter_map(|(db_track, spotify_track)| {
            spotify_track.map(|spotify_track| TaggedTrack {
                track: spotify_track,
                tags: db_track.tags,
            })
        })
        .collect();

    Ok(Json(TagDetails {
        tag,
        tracks: joined_tracks,
    }))
}
