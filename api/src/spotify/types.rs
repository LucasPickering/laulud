//! Rust mappings of Spotify API types, plus some extra utility types that
//! relate closely to the Spotify API. Everything in this module will be
//! exported to the entire crate!

use crate::error::{InputValidationError, ParseError};
use async_graphql::{scalar, Interface, ScalarType, SimpleObject};
use derive_more::Display;
use mongodb::bson::Bson;
use serde::{Deserialize, Serialize};
use std::{backtrace::Backtrace, str::FromStr};

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-simplified
#[derive(Clone, Debug, Deserialize, SimpleObject)]
pub struct ArtistSimplified {
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub name: String,
    pub uri: SpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-full
#[derive(Clone, Debug, Deserialize, SimpleObject)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub genres: Vec<String>,
    pub href: String,
    pub id: String,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: i32,
    pub uri: SpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#album-object-simplified
#[derive(Clone, Debug, Deserialize, SimpleObject)]
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
    pub uri: SpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-audiofeaturesobject
#[derive(Clone, Debug, Deserialize, SimpleObject)]
pub struct AudioFeatures {
    pub acousticness: f64,
    pub analysis_url: String,
    pub danceability: f64,
    pub duration_ms: i32,
    pub energy: f64,
    pub id: String,
    pub instrumentalness: f64,
    pub key: i32,
    pub liveness: f64,
    pub loudness: f64,
    pub mode: i32,
    pub speechiness: f64,
    pub tempo: f64,
    pub time_signature: i32,
    pub track_href: String,
    pub uri: SpotifyUri,
    pub valence: f64,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full
#[derive(Clone, Debug, Deserialize, SimpleObject)]
pub struct Track {
    pub album: AlbumSimplified,
    pub artists: Vec<ArtistSimplified>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i32,
    pub explicit: bool,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub name: String,
    pub popularity: i32,
    pub preview_url: Option<String>,
    pub track_number: i32,
    pub uri: SpotifyUri,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-externalurlobject
#[derive(Clone, Debug, Deserialize, SimpleObject)]
pub struct ExternalUrls {
    pub spotify: String,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#image-object
#[derive(Clone, Debug, Deserialize, SimpleObject)]
pub struct Image {
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-privateuserobject
#[derive(Clone, Debug, Deserialize, SimpleObject)]
pub struct PrivateUser {
    pub id: String,
    pub href: String,
    pub uri: SpotifyUri,
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
    #[display(fmt = "track")]
    Track,
    #[display(fmt = "album")]
    Album,
    #[display(fmt = "artist")]
    Artist,
    #[display(fmt = "user")]
    User,
}

impl FromStr for SpotifyItemType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(SpotifyItemType::Track),
            "album" => Ok(SpotifyItemType::Album),
            "artist" => Ok(SpotifyItemType::Artist),
            "user" => Ok(SpotifyItemType::User),
            _ => Err(ParseError {
                message: "Unknown Spotify object type".into(),
                value: s.into(),
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
///
/// TODO update comment
#[derive(Clone, Debug, Display, Serialize, Deserialize)]
#[display(fmt = "spotify:{}:{}", item_type, id)]
#[serde(try_from = "String", into = "String")]
pub struct SpotifyUri {
    item_type: SpotifyItemType,
    id: String,
}

impl SpotifyUri {
    pub fn item_type(&self) -> SpotifyItemType {
        self.item_type
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

// Declare this as a GraphQL scalar
scalar!(SpotifyUri);

impl From<SpotifyUri> for String {
    fn from(uri: SpotifyUri) -> Self {
        uri.to_string()
    }
}

impl From<&SpotifyUri> for Bson {
    fn from(uri: &SpotifyUri) -> Self {
        uri.to_string().into()
    }
}

impl FromStr for SpotifyUri {
    type Err = InputValidationError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // Expect URIs of the format "spotify:<type>:<id>"
        // We have to generate errors as strings first, then map to a proper
        // error type, cause borrowck
        let parsed: Result<SpotifyUri, String> =
            match value.split(':').collect::<Vec<&str>>().as_slice() {
                ["spotify", item_type, id] => {
                    // Parse item type. It's possible we get a valid item type
                    // that we just don't support, just going to treat those
                    // as invalid for now.
                    match item_type.parse::<SpotifyItemType>() {
                        Ok(item_type) => Ok(SpotifyUri {
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
            // This is kinda bullshit but just assume the field name. Most of
            // the time, we're going to be using this when
            // deserializing from the Spotify API or DB so the field
            // name matches
            field: "uri".into(),
            message,
            value: value.into(),
            backtrace: Backtrace::capture(),
        })
    }
}

/// An item is a polymorphic type that includes anything that can be fetched
/// from Spotify and tagged in the API.
#[derive(Clone, Debug, Deserialize, Interface)]
#[graphql(
    field(name = "externalUrls", type = "ExternalUrls"),
    field(name = "href", type = "String"),
    field(name = "id", type = "String"),
    field(name = "uri", type = "SpotifyUri")
)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)] // don't change external API for micro-opt
pub enum Item {
    Track(Track),
    Album(AlbumSimplified),
    Artist(Artist),
}
