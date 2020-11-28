use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typescript_definitions::TypeScriptify;
use validator::Validate;

/// All types that get serialized over the wire live here

// ========== SPOTIFY API TYPES ==========

/// Any object type that can get a URI
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    TypeScriptify,
)]
#[serde(rename_all = "lowercase")]
pub enum SpotifyObjectType {
    Track,
    Album,
    Artist,
    User,
}

/// https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids
#[derive(
    Clone,
    Debug,
    Display,
    From,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    TypeScriptify,
)]
#[from(forward)]
pub struct SpotifyId(pub String);

/// https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids
#[derive(Clone, Debug, Display, PartialEq, Eq, Hash)]
#[display(fmt = "spotify:{}:{}", obj_type, id)]
pub struct SpotifyUri {
    pub obj_type: SpotifyObjectType,
    pub id: SpotifyId,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#image-object
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct Image {
    pub url: String,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

/// https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct CurrentUser {
    pub id: SpotifyId,
    pub href: String,
    pub uri: SpotifyUri,
    pub display_name: Option<String>,
    pub images: Vec<Image>,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#external-id-object
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct ExternalIds(HashMap<String, String>);

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#external-url-object
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct ExternalUrls(HashMap<String, String>);

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-simplified
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct ArtistSimplified {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: SpotifyId,
    pub name: String,
    pub uri: SpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-full
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    // skipping followers
    pub genres: Vec<String>,
    pub href: String,
    pub id: SpotifyId,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: usize,
    pub uri: SpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#album-object-simplified
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct AlbumSimplified {
    pub album_group: Option<String>,
    pub album_type: String,
    pub artists: Vec<ArtistSimplified>,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: SpotifyId,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    // Skipping `restrictions`
    pub uri: SpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-link
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct TrackLink {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: SpotifyId,
    pub uri: SpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct Track {
    pub album: AlbumSimplified,
    pub artists: Vec<ArtistSimplified>,
    pub available_markets: Vec<String>,
    pub disc_number: usize,
    pub duration_ms: usize,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: SpotifyId,
    pub is_playable: Option<bool>,
    pub linked_from: Option<TrackLink>,
    pub name: String,
    pub popularity: usize,
    pub preview_url: Option<String>,
    pub track_number: usize,
    pub uri: SpotifyUri,
}

// ========== CUSTOM API TYPES ==========

/// Anything that can be tagged
/// impl is in the util folder
#[derive(Clone, Debug, From, Serialize, Deserialize, TypeScriptify)]
#[serde(tag = "type", content = "data", rename_all = "lowercase")]
pub enum Item {
    Track(Track),
    Album(AlbumSimplified),
    Artist(Artist),
}

/// A taggable item, with its assigned tags
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct TaggedItem {
    pub item: Item,
    pub tags: Vec<String>,
}

/// Response for a search query
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct ItemSearchResponse {
    pub tracks: Vec<TaggedItem>,
    pub albums: Vec<TaggedItem>,
    pub artists: Vec<TaggedItem>,
}

/// Summary information for a tag
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct TagSummary {
    pub tag: String,
    /// The number of items that have this tag assigned
    pub num_items: usize,
}

/// Details for a tag
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct TagDetails {
    pub tag: String,
    pub items: Vec<TaggedItem>,
}

/// POST input for tagging a track
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify, Validate)]
pub struct CreateTagBody {
    #[validate(length(min = 1))]
    pub tag: String,
}
