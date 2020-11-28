use crate::{
    db::{CollectionName, DbHandler, TrackDocument},
    error::ApiResult,
    schema::{TagDetails, TagSummary, TaggedTrack},
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
        .collection(CollectionName::Tracks)
        .aggregate(
            vec![
                doc! {"$match":{"user_id": user_id}},
                doc! {"$unwind":"$tags"},
                doc! {"$group":{"_id":"$tags","num_tracks":{"$sum":1}}},
                doc! {"$project":{"tag": "$_id", "num_tracks": 1, "_id": 0}},
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
    mut spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<TagDetails>> {
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
        .zip(spotify_tracks.tracks)
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
