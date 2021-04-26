//! All GraphQL bindings for Spotify types are defined here. The types
//! themselves are provided by the [crate::spotify] module.

use crate::{
    graphql::{
        AlbumSimplifiedFields, ArtistFields, ArtistSimplifiedFields,
        ExternalUrlsFields, ImageFields, PrivateUserFields, RequestContext,
        SpotifyId, SpotifyUri, TrackFields,
    },
    spotify::{
        AlbumSimplified, Artist, ArtistSimplified, ExternalUrls, Image,
        PrivateUser, Track,
    },
};
use juniper::Executor;
use juniper_from_schema::{QueryTrail, Walked};

impl ArtistSimplifiedFields for ArtistSimplified {
    fn field_external_urls(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, ExternalUrls, Walked>,
    ) -> &ExternalUrls {
        &self.external_urls
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

    fn field_name(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.name
    }

    fn field_uri(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> SpotifyUri {
        self.uri.to_string()
    }
}

impl ArtistFields for Artist {
    fn field_external_urls(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, ExternalUrls, Walked>,
    ) -> &ExternalUrls {
        &self.external_urls
    }

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
    ) -> SpotifyUri {
        self.uri.to_string()
    }
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

    fn field_external_urls(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, ExternalUrls, Walked>,
    ) -> &ExternalUrls {
        &self.external_urls
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
    ) -> SpotifyUri {
        self.uri.to_string()
    }
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

    fn field_external_urls(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
        _trail: &QueryTrail<'_, ExternalUrls, Walked>,
    ) -> &ExternalUrls {
        &self.external_urls
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
    ) -> SpotifyUri {
        self.uri.to_string()
    }
}

impl ExternalUrlsFields for ExternalUrls {
    fn field_spotify(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> &String {
        &self.spotify
    }
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
    ) -> SpotifyUri {
        self.uri.to_string()
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
