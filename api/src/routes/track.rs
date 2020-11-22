use crate::{
    db::{CollectionName, DbHandler, TrackDocument},
    error::{ApiError, ApiResult},
    schema::{CreateTagBody, TaggedTrack, Track},
    spotify::Spotify,
    util,
};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
};
use rocket::{delete, get, post, State};
use rocket_contrib::json::Json;
use std::{backtrace::Backtrace, collections::HashMap};
use tokio::stream::StreamExt;

pub async fn load_track_tags(
    db_handler: &DbHandler,
    spotify_tracks: Vec<Track>,
) -> ApiResult<Vec<TaggedTrack>> {
    let ids: Vec<&str> = spotify_tracks
        .iter()
        .map(|track| track.id.as_str())
        .collect();

    // Look up the relevant tracks in the DB
    let coll = db_handler.collection(CollectionName::Tracks);
    let mut cursor = coll.find(doc! {"track_id": {"$in": ids}}, None).await?;

    // Build a mapping of track ID:tags
    let mut tagged_docs: HashMap<String, Vec<String>> = HashMap::new();
    while let Some(doc) = cursor.next().await {
        let track: TrackDocument = util::from_doc(doc?)?;
        tagged_docs.insert(track.track_id, track.tags);
    }

    // Join the datasets
    Ok(spotify_tracks
        .into_iter()
        .map(|track| {
            let tags = tagged_docs
                .remove(track.id.as_str())
                .unwrap_or_else(Vec::new);
            TaggedTrack { track, tags }
        })
        .collect())
}

#[get("/tracks/<track_id>", format = "json")]
pub async fn route_get_track(
    track_id: String,
    mut spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<TaggedTrack>> {
    // Look up the track in Spotify
    // TODO handle 404 here properly
    let spotify_track = spotify.get_track(&track_id).await?;

    let coll = db_handler.collection(CollectionName::Tracks);
    let doc = coll
        .find_one(doc! { "track_id": &track_id }, None)
        .await?
        .ok_or_else(|| ApiError::NotFound {
            resource: (track_id),
            backtrace: Backtrace::capture(),
        })?;
    let track_doc: TrackDocument = util::from_doc(doc)?;

    Ok(Json(TaggedTrack {
        track: spotify_track,
        tags: track_doc.tags,
    }))
}

#[get("/tracks/search/<query>", format = "json")]
pub async fn route_search_tracks(
    query: String,
    mut spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<Vec<TaggedTrack>>> {
    // Search for the tracks on Spotify
    let spotify_tracks = spotify.search_tracks(&query).await?;

    // Saturate the Spotify data with the tags from mongo
    let tagged_tracks = load_track_tags(&db_handler, spotify_tracks).await?;
    Ok(Json(tagged_tracks))
}

#[post("/tracks/<track_id>/tags", format = "json", data = "<body>")]
pub async fn route_create_tag(
    track_id: String,
    body: Json<CreateTagBody>,
    mut spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<TaggedTrack>> {
    // TODO valid input
    let CreateTagBody { tags } = body.into_inner();

    // Look up the track in Spotify first, to get metadata/confirm it's real
    // TODO handle 404 here properly
    let spotify_track = spotify.get_track(&track_id).await?;

    // let CreateTagBody { tags } = body.to_owned();
    let coll = db_handler.collection(CollectionName::Tracks);
    let doc = coll
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
        .await?
        // This shouldn't be possible because we have upsert=true, but let's
        // handle it just to be safe
        .ok_or_else(|| ApiError::Unknown {
            message: ("No result from findOneAndUpdate".into()),
            backtrace: Backtrace::capture(),
        })?;
    let track_doc: TrackDocument = util::from_doc(doc)?;

    Ok(Json(TaggedTrack {
        track: spotify_track,
        tags: track_doc.tags,
    }))
}

#[delete("/tracks/<track_id>/tags/<tag>")]
pub async fn route_delete_tag(
    track_id: String,
    tag: String,
    mut spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<TaggedTrack>> {
    // Look up the track in Spotify first, to get metadata/confirm it's real
    // TODO handle 404 here properly
    let spotify_track = spotify.get_track(&track_id).await?;

    // let CreateTagBody { tags } = body.to_owned();
    let coll = db_handler.collection(CollectionName::Tracks);
    let doc = coll
        .find_one_and_update(
            doc! {"track_id": &track_id},
            // Remove the tag from the doc
            doc! {"$pull": {"tags": &tag}},
            Some(
                FindOneAndUpdateOptions::builder()
                    .return_document(ReturnDocument::After)
                    .build(),
            ),
        )
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
