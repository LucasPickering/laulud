use std::{backtrace::Backtrace, str::FromStr};

use crate::{
    error::ApiError,
    graphql::{
        AlbumSimplifiedFields, ArtistFields, ArtistSimplifiedFields,
        ImageFields, PrivateUserFields, RequestContext, SpotifyId, SpotifyUri,
        TrackFields,
    },
};
use derive_more::Display;
use juniper::Executor;
use juniper_from_schema::{QueryTrail, Walked};
use serde::{Deserialize, Serialize};

/// All Spotify API types get defined here. Some types are missing fields from
/// the API spec, because they were annoying to implement and not needed. See
/// the GraphQL schema file for exactly which fields are omitted.

impl ArtistSimplifiedFields for ArtistSimplified {
    fn field_href(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.href
    }

    fn field_id(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &SpotifyId {
        &self.id
    }

    fn field_name(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.name
    }

    fn field_uri(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &SpotifyUri {
        &self.uri
    }
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-full
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub genres: Vec<String>,
    pub href: String,
    pub id: SpotifyId,
    pub images: Vec<Image>,
    pub name: String,
    pub popularity: i32,
    pub uri: SpotifyUri,
}

impl ArtistFields for Artist {
    fn field_genres(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &Vec<String> {
        &self.genres
    }

    fn field_href(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.href
    }

    fn field_id(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &SpotifyId {
        &self.id
    }

    fn field_images(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, Image, Walked>,
    ) -> &Vec<Image> {
        &self.images
    }

    fn field_name(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.name
    }

    fn field_popularity(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &i32 {
        &self.popularity
    }

    fn field_uri(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &SpotifyUri {
        &self.uri
    }
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#album-object-simplified
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlbumSimplified {
    pub album_group: Option<String>,
    pub album_type: String,
    pub artists: Vec<ArtistSimplified>,
    pub available_markets: Vec<String>,
    pub href: String,
    pub id: SpotifyId,
    pub images: Vec<Image>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub uri: SpotifyUri,
}

impl AlbumSimplifiedFields for AlbumSimplified {
    fn field_album_group(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &Option<String> {
        &self.album_group
    }

    fn field_album_type(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.album_type
    }

    fn field_artists(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, ArtistSimplified, Walked>,
    ) -> &Vec<ArtistSimplified> {
        &self.artists
    }

    fn field_available_markets(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &Vec<String> {
        &self.available_markets
    }

    fn field_href(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.href
    }

    fn field_id(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &SpotifyId {
        &self.id
    }

    fn field_images(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, Image, Walked>,
    ) -> &Vec<Image> {
        &self.images
    }

    fn field_name(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.name
    }

    fn field_release_date(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.release_date
    }

    fn field_release_date_precision(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.release_date_precision
    }

    fn field_uri(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &SpotifyUri {
        &self.uri
    }
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub album: AlbumSimplified,
    pub artists: Vec<ArtistSimplified>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i32,
    pub explicit: bool,
    pub href: String,
    pub id: SpotifyId,
    pub is_playable: Option<bool>,
    pub name: String,
    pub popularity: i32,
    pub preview_url: Option<String>,
    pub track_number: i32,
    pub uri: SpotifyUri,
}

impl TrackFields for Track {
    fn field_album(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, AlbumSimplified, Walked>,
    ) -> &AlbumSimplified {
        &self.album
    }

    fn field_artists(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, ArtistSimplified, Walked>,
    ) -> &Vec<ArtistSimplified> {
        &self.artists
    }

    fn field_available_markets(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &Vec<String> {
        &self.available_markets
    }

    fn field_disc_number(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &i32 {
        &self.disc_number
    }

    fn field_duration_ms(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &i32 {
        &self.duration_ms
    }

    fn field_explicit(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &bool {
        &self.explicit
    }

    fn field_href(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.href
    }

    fn field_id(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &SpotifyId {
        &self.id
    }

    fn field_is_playable(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &Option<bool> {
        &self.is_playable
    }

    fn field_name(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.name
    }

    fn field_popularity(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &i32 {
        &self.popularity
    }

    fn field_preview_url(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &Option<String> {
        &self.preview_url
    }

    fn field_track_number(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &i32 {
        &self.track_number
    }

    fn field_uri(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &SpotifyUri {
        &self.uri
    }
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#image-object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

impl ImageFields for Image {
    fn field_url(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.url
    }

    fn field_width(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &Option<i32> {
        &self.width
    }

    fn field_height(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &Option<i32> {
        &self.height
    }
}

/// https://developer.spotify.com/documentation/web-api/reference/#object-privateuserobject
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PrivateUser {
    pub id: SpotifyId,
    pub href: String,
    pub uri: SpotifyUri,
    pub display_name: Option<String>,
    pub images: Vec<Image>,
}

impl PrivateUserFields for PrivateUser {
    fn field_id(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &SpotifyId {
        &self.id
    }

    fn field_href(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.href
    }

    fn field_uri(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &SpotifyUri {
        &self.uri
    }

    fn field_display_name(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &Option<String> {
        &self.display_name
    }

    fn field_images(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, Image, Walked>,
    ) -> &Vec<Image> {
        &self.images
    }
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#artist-object-simplified
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtistSimplified {
    pub href: String,
    pub id: SpotifyId,
    pub name: String,
    pub uri: SpotifyUri,
}

/// Any object type that can get a URI
/// TODO rename to SpotifyItemType
#[derive(
    Copy, Clone, Debug, Display, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum SpotifyObjectType {
    #[display("track")]
    Track,
    #[display("album")]
    Album,
    #[display("artist")]
    Artist,
    #[display("user")]
    User,
}

impl SpotifyObjectType {
    /// Parse a Spotify URI into its components. URIs have the format:
    /// `spotify:<type>:<id>`, where the object type is one of the stringified
    /// values of [SpotifyObjectType].
    pub fn parse_uri(uri: &SpotifyUri) -> (Self, SpotifyId) {
        match uri.split(':').collect::<Vec<&str>>().as_slice() {
            ["spotify", object_type, id] => {
                (object_type.parse().unwrap(), (*id).to_owned())
            }
            // TODO figure out better way to handle this
            _ => panic!("Malformatted Spotify URI: {}", uri),
        }
    }
}

impl FromStr for SpotifyObjectType {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(SpotifyObjectType::Track),
            "album" => Ok(SpotifyObjectType::Album),
            "artist" => Ok(SpotifyObjectType::Artist),
            "user" => Ok(SpotifyObjectType::User),
            _ => Err(ApiError::ParseError {
                message: format!("Unknown Spotify object type: {}", s),
                backtrace: Backtrace::capture(),
            }),
        }
    }
}
