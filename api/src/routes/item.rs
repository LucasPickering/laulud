use crate::{
    db::{CollectionName, DbHandler, TaggedItemDocument},
    error::{ApiError, ApiResult},
    schema::{CreateTagBody, Item, ItemSearchResponse, SpotifyUri, TaggedItem},
    spotify::Spotify,
    util::{self, UserId},
};
use mongodb::{
    bson::{doc, Document},
    options::{FindOneAndUpdateOptions, ReturnDocument},
};
use rocket::{delete, get, post, State};
use rocket_contrib::json::Json;
use std::backtrace::Backtrace;
use validator::Validate;

/// Parse a document from [CollectionName::TaggedItem] and get its tags from it.
/// If the doc is missing, return an empty list.
fn to_tags(doc: Option<Document>) -> ApiResult<Vec<String>> {
    Ok(doc
        .map::<ApiResult<Vec<String>>, _>(|doc| {
            Ok(util::from_doc::<TaggedItemDocument>(doc)?.tags)
        })
        .transpose()? // Option<Result> -> Result<Option>
        .unwrap_or_else(Vec::new))
}

#[get("/items/<uri>")]
pub async fn route_get_item(
    uri: SpotifyUri,
    user_id: UserId,
    spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<TaggedItem>> {
    // Look up the item in Spotify
    let spotify_item = spotify.get_item(&uri).await?;

    // Look up tags in the DB
    let doc = db_handler
        .collection(CollectionName::TaggedItems)
        .find_one(doc! { "uri": &uri, "user_id": &user_id }, None)
        .await?;

    Ok(Json(TaggedItem {
        item: spotify_item,
        tags: to_tags(doc)?,
    }))
}

#[get("/items/search/<query>")]
pub async fn route_search_items(
    query: String,
    spotify: Spotify,
    user_id: UserId,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<ItemSearchResponse>> {
    // Search for the tracks on Spotify
    let response = spotify.search_items(&query).await?;
    let item_uris: Vec<&SpotifyUri> = response
        .tracks
        .items
        .iter()
        .map(|track| &track.uri)
        .chain(response.albums.items.iter().map(|album| &album.uri))
        .chain(response.artists.items.iter().map(|artist| &artist.uri))
        .collect();

    // Look up tags in the DB
    let cursor = db_handler
        .collection(CollectionName::TaggedItems)
        .find(
            doc! { "uri": {"$in": item_uris}, "user_id": &user_id },
            None,
        )
        .await?;
    let docs: Vec<TaggedItemDocument> = util::from_cursor(cursor).await?;
    let (track_tags, album_tags, artist_tags) =
        TaggedItemDocument::group_tags(docs)?;

    Ok(Json(ItemSearchResponse {
        tracks: Item::join_tags(track_tags, response.tracks.items).collect(),
        albums: Item::join_tags(album_tags, response.albums.items).collect(),
        artists: Item::join_tags(artist_tags, response.artists.items).collect(),
    }))
}

#[post("/items/<uri>/tags", format = "json", data = "<body>")]
pub async fn route_create_tag(
    uri: SpotifyUri,
    body: Json<CreateTagBody>,
    user_id: UserId,
    spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<TaggedItem>> {
    body.validate()?;
    let CreateTagBody { tag } = body.into_inner();

    // Look up the item in Spotify first, to get metadata/confirm it's real
    let spotify_item = spotify.get_item(&uri).await?;

    let doc = db_handler
        .collection(CollectionName::TaggedItems)
        .find_one_and_update(
            doc! {"uri": &uri, "user_id": &user_id},
            // Add each tag to the doc if it isn't present already
            doc! {"$addToSet": {"tags": &tag}},
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
    let item_doc: TaggedItemDocument = util::from_doc(doc)?;

    Ok(Json(TaggedItem {
        item: spotify_item,
        tags: item_doc.tags,
    }))
}

#[delete("/items/<uri>/tags/<tag>")]
pub async fn route_delete_tag(
    uri: SpotifyUri,
    tag: String,
    user_id: UserId,
    spotify: Spotify,
    db_handler: State<'_, DbHandler>,
) -> ApiResult<Json<TaggedItem>> {
    // Look up the item in Spotify first, to get metadata/confirm it's real
    let spotify_item = spotify.get_item(&uri).await?;

    // Look up tags in mongo
    let doc = db_handler
        .collection(CollectionName::TaggedItems)
        .find_one_and_update(
            doc! {"uri": &uri, "user_id": &user_id},
            // Remove the tag from the doc
            doc! {"$pull": {"tags": &tag}},
            Some(
                FindOneAndUpdateOptions::builder()
                    .return_document(ReturnDocument::After)
                    .build(),
            ),
        )
        .await?;

    Ok(Json(TaggedItem {
        item: spotify_item,
        tags: to_tags(doc)?,
    }))
}
