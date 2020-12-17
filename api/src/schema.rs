use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typescript_definitions::TypeScriptify;
use validator::Validate;

/// All types that get serialized over the wire live here

// ========== SPOTIFY API TYPES ==========

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#paging-object
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct PaginatedResponse<T> {
    pub href: String,
    pub limit: usize,
    pub offset: usize,
    pub total: usize,
    pub next: Option<String>,
    pub previos: Option<String>,
    pub items: Vec<T>,
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
    pub id: String,
    pub href: String,
    pub uri: String,
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
    pub id: String,
    pub name: String,
    pub uri: String,
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
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    // Skipping `restrictions`
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-link
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct TrackLink {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub uri: String,
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
    pub id: String,
    pub is_playable: Option<bool>,
    pub linked_from: Option<TrackLink>,
    pub name: String,
    pub popularity: usize,
    pub preview_url: Option<String>,
    pub track_number: usize,
    pub uri: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-tracks/
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct TracksResponse {
    pub tracks: Vec<Option<Track>>,
}

/// https://developer.spotify.com/documentation/web-api/reference/search/search/
#[derive(Clone, Debug, Serialize, Deserialize, TypeScriptify)]
pub struct TracksSearchResponse {
    pub tracks: PaginatedResponse<Track>,
}

// ========== CUSTOM API TYPES ==========

/// A track that we've annotated with tag metadata
#[derive(Debug, Clone, Serialize, Deserialize, TypeScriptify)]
pub struct TaggedTrack {
    pub track: Track,
    pub tags: Vec<String>,
}

/// Summary informatoin for a tag
#[derive(Debug, Clone, Serialize, Deserialize, TypeScriptify)]
pub struct TagSummary {
    pub tag: String,
    /// The number of tracks that have this tag assigned
    pub num_tracks: usize,
}

/// Details for a tag
#[derive(Debug, Clone, Serialize, Deserialize, TypeScriptify)]
pub struct TagDetails {
    pub tag: String,
    /// IDs of all tracks with this tag
    pub tracks: Vec<TaggedTrack>,
}

/// POST input for tagging a track
#[derive(Debug, Clone, Serialize, Deserialize, TypeScriptify, Validate)]
pub struct CreateTagBody {
    #[validate(length(min = 1))]
    pub tag: String,
}
