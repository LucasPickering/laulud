use log::info;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::File,
    io::{self, Write},
};
use typescript_definitions::{TypeScriptify, TypeScriptifyTrait};
use validator::Validate;

use crate::LauludConfig;

/// All types that get serialized over the wire live here

const TS_DEFINITION_GENERATION_FUNCS: &[&dyn Fn() -> Cow<'static, str>] = &[
    // Hack for generic structs, the type it generates is still generic
    &<PaginatedResponse<i32>>::type_script_ify,
    &Image::type_script_ify,
    &CurrentUser::type_script_ify,
    &ExternalIds::type_script_ify,
    &ExternalUrls::type_script_ify,
    &ArtistSimplified::type_script_ify,
    &AlbumSimplified::type_script_ify,
    &TrackLink::type_script_ify,
    &Track::type_script_ify,
    &TracksSearchResponse::type_script_ify,
    &TaggedTrack::type_script_ify,
    &CreateTagBody::type_script_ify,
    // Make sure any new types get added here
];

pub fn generate_ts_definitions(config: &LauludConfig) -> io::Result<()> {
    if let Some(path) = &config.ts_definitions_file {
        let mut file = File::with_options().create(true).write(true).open(path)?;

        for func in TS_DEFINITION_GENERATION_FUNCS {
            file.write_all(b"\n")?;
            file.write_all(func().as_bytes())?;
            file.write_all(b"\n")?;
        }

        file.sync_all()?;
        info!(
            "Generated TypeScript definitions at {}",
            path.to_str().unwrap()
        );
    }
    Ok(())
}

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
    pub is_playable: bool,
    pub linked_from: Option<TrackLink>,
    pub name: String,
    pub popularity: usize,
    pub preview_url: Option<String>,
    pub track_number: usize,
    pub uri: String,
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
