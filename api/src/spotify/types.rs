//! Rust mappings of Spotify API types, plus some extra utility types that
//! relate closely to the Spotify API. Everything in this module will be
//! exported to the entire crate!

use crate::{
    error::{ApiError, InputValidationError},
    graphql::{SpotifyId, SpotifyUri},
    util::Validate,
};
use derive_more::Display;
use mongodb::bson::Bson;
use serde::{Deserialize, Serialize};
use std::{backtrace::Backtrace, convert::TryFrom, str::FromStr};

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-simplified
#[derive(Clone, Debug, Deserialize)]
pub struct ArtistSimplified {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: SpotifyId,
    pub name: String,
    pub uri: ValidSpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-full
#[derive(Clone, Debug, Deserialize)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub genres: Vec<String>,
    pub href: String,
    pub id: SpotifyId,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: i32,
    pub uri: ValidSpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#album-object-simplified
#[derive(Clone, Debug, Deserialize)]
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
    pub uri: ValidSpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full
#[derive(Clone, Debug, Deserialize)]
pub struct Track {
    pub album: AlbumSimplified,
    pub artists: Vec<ArtistSimplified>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: SpotifyId,
    pub is_playable: Option<bool>,
    pub name: String,
    pub popularity: i32,
    pub preview_url: Option<String>,
    pub track_number: i32,
    pub uri: ValidSpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-externalurlobject
#[derive(Clone, Debug, Deserialize)]
pub struct ExternalUrls {
    pub spotify: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#image-object
#[derive(Clone, Debug, Deserialize)]
pub struct Image {
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-privateuserobject
#[derive(Clone, Debug, Deserialize)]
pub struct PrivateUser {
    pub id: SpotifyId,
    pub href: String,
    pub uri: ValidSpotifyUri,
    pub display_name: Option<String>,
    pub images: Vec<Image>,
}

/// https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-tracks/
#[derive(Clone, Debug, Deserialize)]
pub struct TracksResponse {
    pub tracks: Vec<Option<Track>>,
}

/// https://developer.spotify.com/documentation/web-api/reference/albums/get-several-albums/
#[derive(Clone, Debug, Deserialize)]
pub struct AlbumsResponse {
    /// The response actually includes full album objects, but we use
    /// simplified here to keep compatibility with the search endpoint
    pub albums: Vec<Option<AlbumSimplified>>,
}

/// https://developer.spotify.com/documentation/web-api/reference/artists/get-several-artists/
#[derive(Clone, Debug, Deserialize)]
pub struct ArtistsResponse {
    pub artists: Vec<Option<Artist>>,
}

/// https://developer.spotify.com/documentation/web-api/reference/#category-search
/// The search method is hard-coded to always request these item categories,
/// so we can hard-code them here. If we wanted to make that dynamic though, we
/// could use a HashMap instead of this struct.
#[derive(Clone, Debug, Deserialize)]
pub struct SearchResponse {
    pub tracks: PaginatedResponse<Track>,
    pub albums: PaginatedResponse<AlbumSimplified>,
    pub artists: PaginatedResponse<Artist>,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#paging-object
#[derive(Clone, Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub href: String,
    pub limit: usize,
    pub offset: usize,
    pub total: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub items: Vec<T>,
}

impl<T> PaginatedResponse<T> {
    /// Create a new struct instance that has all the same values as this
    /// instance, except the `items` field has had the mapper function applied
    /// to it. Useful for type transformations on the `items` field.
    pub fn map_items<U>(
        self,
        mut mapper: impl FnMut(Vec<T>) -> Vec<U>,
    ) -> PaginatedResponse<U> {
        PaginatedResponse {
            // Can't use .. syntax because the generic param changes
            href: self.href,
            limit: self.limit,
            offset: self.offset,
            total: self.total,
            next: self.next,
            previous: self.previous,
            items: mapper(self.items),
        }
    }
}

/// Any item type that can get a URI
///
/// Note: we don't actually support every Spotify type here yet, just the ones
/// we use. Add more as needed.
#[derive(
    Copy, Clone, Debug, Display, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum SpotifyItemType {
    #[display("track")]
    Track,
    #[display("album")]
    Album,
    #[display("artist")]
    Artist,
    #[display("user")]
    User,
}

impl FromStr for SpotifyItemType {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(SpotifyItemType::Track),
            "album" => Ok(SpotifyItemType::Album),
            "artist" => Ok(SpotifyItemType::Artist),
            "user" => Ok(SpotifyItemType::User),
            _ => Err(ApiError::ParseError {
                message: format!("Unknown Spotify object type: {}", s),
                backtrace: Backtrace::capture(),
            }),
        }
    }
}

/// A parsed and validated Spotify URI. This should be used for any internal
/// logic that passes around URIs, so that we always know they're valid. It
/// can be converted _from_ [SpotifyUri] fallibly, and _to_ [SpotifyUri]
/// infallibly. Note that in this context, "valid" just means it's not
/// _malformed_. It **doesn't** mean that the URI actually exists in Spotify.
///
/// This can only be constructed via its [Validate] implementation.
#[derive(Clone, Debug, Display, Serialize, Deserialize)]
#[display(fmt = "spotify:{}:{}", item_type, id)]
#[serde(try_from = "SpotifyUri", into = "SpotifyUri")]
pub struct ValidSpotifyUri {
    item_type: SpotifyItemType,
    id: SpotifyId,
}

impl ValidSpotifyUri {
    pub fn item_type(&self) -> SpotifyItemType {
        self.item_type
    }

    pub fn id(&self) -> &SpotifyId {
        &self.id
    }
}

impl Validate for SpotifyUri {
    type Output = ValidSpotifyUri;

    /// Parse the raw Spotify ID into an item type+ID. See
    /// https://developer.spotify.com/documentation/web-api/ for a description
    /// of URIs.
    fn validate(
        self,
        field: &str,
    ) -> Result<Self::Output, InputValidationError> {
        // Expect URIs of the format "spotify:<type>:<id>"
        // We have to generate errors as strings first, then map to a proper
        // error type, cause borrowck
        let parsed: Result<ValidSpotifyUri, String> =
            match self.split(':').collect::<Vec<&str>>().as_slice() {
                ["spotify", item_type, id] => {
                    // Parse item type. It's possible we get a valid item type
                    // that we just don't support, just going to treat those
                    // as invalid for now.
                    match item_type.parse::<SpotifyItemType>() {
                        Ok(item_type) => Ok(ValidSpotifyUri {
                            item_type,
                            id: (*id).into(),
                        }),
                        // Big NG
                        Err(_) => Err(format!(
                            "Invalid Spotify URI: unknown item type {}",
                            item_type
                        )),
                    }
                }
                _ => Err("Invalid Spotify URI: invalid format".into()),
            };
        parsed.map_err(|message| InputValidationError {
            field: field.into(),
            message,
            value: self.into(),
        })
    }
}

impl From<ValidSpotifyUri> for SpotifyUri {
    fn from(uri: ValidSpotifyUri) -> Self {
        uri.to_string()
    }
}

impl From<&ValidSpotifyUri> for Bson {
    fn from(uri: &ValidSpotifyUri) -> Self {
        uri.to_string().into()
    }
}

impl TryFrom<SpotifyUri> for ValidSpotifyUri {
    type Error = InputValidationError;

    fn try_from(value: SpotifyUri) -> Result<Self, Self::Error> {
        // This is kinda bullshit but just assume the field name. Most of the
        // time, we're going to be using this when deserializing from the
        // Spotify API or DB so the field name matches
        value.validate("uri")
    }
}
