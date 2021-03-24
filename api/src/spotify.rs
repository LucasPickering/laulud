//! A binding for the Spotify Web API. Most of the mapped types are defined
//! in GraphQL schema, then juniper_from_schema generates the Rust types that
//! we use here. Some extra types are defined here though, as well as the main
//! [Spotify] type that handles auth and wraps over common requests.
//!
//! The Spotify API has really good docs at
//! https://developer.spotify.com/documentation/web-api/reference/

use crate::{
    error::{ApiError, ApiResult},
    graphql::{
        AlbumSimplified, Artist, Item, PrivateUser, SpotifyId,
        SpotifyObjectType, SpotifyUri, Track,
    },
    util::{IdentityState, OAuthHandler},
};
use async_trait::async_trait;
use itertools::Itertools;
use log::{debug, trace};
use oauth2::basic::BasicClient;
use reqwest::{
    header::{self, HeaderValue},
    Client, StatusCode,
};
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    State,
};
use serde::{de::DeserializeOwned, Deserialize};
use std::{backtrace::Backtrace, collections::HashMap, sync::Arc};

const SPOTIFY_BASE_URL: &str = "https://api.spotify.com";

// TODO move the rest of these types below the main struct

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

/// A client for accessing the Spotify web API
#[derive(Debug)]
pub struct Spotify {
    /// The access token response from the OAuth2 cycle. This can be used to
    /// get the access token, or to refresh for a new one.
    oauth_handler: OAuthHandler,
    /// HTTP request client
    req_client: Client,
}

impl Spotify {
    pub fn new(oauth_handler: OAuthHandler) -> Self {
        Self {
            oauth_handler,
            req_client: Client::new(),
        }
    }

    /// Move the [OAuthHandler] out of this object
    pub fn into_oauth_handler(self) -> OAuthHandler {
        self.oauth_handler
    }

    /// Helper method for doing a GET request against a Spotify endpoint. Parses
    /// the result as JSON.
    async fn get_endpoint<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        options: RequestOptions<'_>,
    ) -> ApiResult<T> {
        // Make sure our auth token is up to date first
        self.oauth_handler.refresh_if_needed().await?;

        let url = format!("{}{}", SPOTIFY_BASE_URL, endpoint);
        let start_time = std::time::Instant::now();
        let response = self
            .req_client
            .get(&url)
            .header(
                header::AUTHORIZATION,
                HeaderValue::from_str(&format!(
                    "Bearer {}",
                    &self.oauth_handler.secret().await
                ))?,
            )
            .query(options.params)
            .send()
            .await?;

        // Some convoluted logic here to get around the response's ownership.
        // If it's a success, parse the body as JSON and return.
        // If it's an error, get the body text and create an error obj with that
        let response_error = response.error_for_status_ref().err();
        let body = response.text().await?;
        debug!(
            "Spotify request ({}) took {} ms",
            &url,
            start_time.elapsed().as_millis()
        );
        trace!("Spotify response: {}", &body);
        match response_error {
            None => Ok(serde_json::from_str(&body).map_err(|err| {
                ApiError::SpotifyApiDeserialization {
                    source: err,
                    body,
                    backtrace: Backtrace::capture(),
                }
            })?),
            Some(err) => {
                // Check if the caller requested that we propagate this error
                // with the actual status code, instead of 500
                let output_status = match err.status() {
                    Some(status)
                        if options.propagate_errors.contains(&status) =>
                    {
                        // Convert reqwest status to rocket status
                        Status::from_code(status.as_u16()).ok_or_else(|| {
                            ApiError::Unknown {
                                message: format!(
                                    "Unknown status code from reqwest: {}",
                                    status
                                ),
                                backtrace: Backtrace::capture(),
                            }
                        })?
                    }
                    _ => Status::InternalServerError,
                };
                Err(ApiError::SpotifyApiHttp {
                    source: err,
                    body,
                    backtrace: Backtrace::capture(),
                    output_status,
                })
            }
        }
    }

    /// https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/
    pub async fn get_current_user(&self) -> ApiResult<PrivateUser> {
        self.get_endpoint("/v1/me", RequestOptions::default()).await
    }

    /// https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-tracks/
    pub async fn get_tracks(
        &self,
        track_ids: impl Iterator<Item = &SpotifyId>,
    ) -> ApiResult<TracksResponse> {
        self.get_endpoint(
            "/v1/tracks",
            RequestOptions {
                params: &[(
                    "ids",
                    track_ids.map(|id| id.as_str()).join(",").as_str(),
                )],
                ..Default::default()
            },
        )
        .await
    }

    /// https://developer.spotify.com/documentation/web-api/reference/albums/get-several-albums/
    pub async fn get_albums(
        &self,
        album_ids: impl Iterator<Item = &SpotifyId>,
    ) -> ApiResult<AlbumsResponse> {
        self.get_endpoint(
            "/v1/albums",
            RequestOptions {
                params: &[(
                    "ids",
                    album_ids.map(|id| id.as_str()).join(",").as_str(),
                )],
                ..Default::default()
            },
        )
        .await
    }

    /// https://developer.spotify.com/documentation/web-api/reference/artists/get-several-artists/
    pub async fn get_artists(
        &self,
        artists_ids: impl Iterator<Item = &SpotifyId>,
    ) -> ApiResult<ArtistsResponse> {
        self.get_endpoint(
            "/v1/artists",
            RequestOptions {
                params: &[(
                    "ids",
                    artists_ids.map(|id| id.as_str()).join(",").as_str(),
                )],
                ..Default::default()
            },
        )
        .await
    }

    /// Search restricted to taggable items
    /// https://developer.spotify.com/documentation/web-api/reference/search/search/
    pub async fn search_items(
        &self,
        query: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> ApiResult<HashMap<String, PaginatedResponse<Item>>> {
        let responses: HashMap<String, PaginatedResponse<ItemDeserialize>> =
            self.get_endpoint(
                "/v1/search",
                RequestOptions {
                    // TODO pass limit+offset to spotify
                    params: &[("q", query), ("type", "track,album,artist")],
                    ..Default::default()
                },
            )
            .await?;
        // TODO comment this
        let responses = responses
            .into_iter()
            .map(|(item_type, paginated_response)| {
                (
                    item_type,
                    paginated_response.map_items(|items| {
                        items.into_iter().map(Item::from).collect()
                    }),
                )
            })
            .collect();
        Ok(responses)
    }

    /// Get an item of any type from the API. This will call the correct
    /// endpoint based on the item's type (track, album, etc.).
    pub async fn get_item(&self, uri: &SpotifyUri) -> ApiResult<Item> {
        let options = RequestOptions {
            // a 404 (hopefully) indicates an invalid ID
            propagate_errors: &[StatusCode::NOT_FOUND],
            ..Default::default()
        };
        let (object_type, id) = SpotifyObjectType::parse_uri(uri);
        let item = match object_type {
            // https://developer.spotify.com/documentation/web-api/reference/tracks/get-track/
            SpotifyObjectType::Track => self
                .get_endpoint::<Track>(&format!("/v1/tracks/{}", id), options)
                .await?
                .into(),
            // https://developer.spotify.com/documentation/web-api/reference/albums/get-album/
            SpotifyObjectType::Album => self
                .get_endpoint::<AlbumSimplified>(
                    &format!("/v1/albums/{}", id),
                    options,
                )
                .await?
                .into(),
            // https://developer.spotify.com/documentation/web-api/reference/artists/get-artist/
            SpotifyObjectType::Artist => self
                .get_endpoint::<Artist>(&format!("/v1/artists/{}", id), options)
                .await?
                .into(),
            // We don't support tagging any other object types
            object_type => {
                return Err(ApiError::UnsupportedObjectType {
                    object_type,
                    backtrace: Backtrace::capture(),
                })
            }
        };
        Ok(item)
    }

    /// Fetch data for a list of items of any type. This will make one request
    /// to the API per item _type_ in the input list, e.g. if your input URIs
    /// have 3 tracks, 2 albums, and 10 artists, this will still only make 3
    /// requests to the API.
    pub async fn get_items(
        &self,
        uris: impl Iterator<Item = &SpotifyUri>,
    ) -> ApiResult<Vec<Item>> {
        // Group URIs by type so we can make one request per type
        let ids_by_type: HashMap<SpotifyObjectType, Vec<SpotifyId>> =
            uris.map(SpotifyObjectType::parse_uri).into_group_map();

        // Make one request to the Spotify API for each item type
        // TODO run these requests concurrently with something like join_all
        let mut items: Vec<Item> = Vec::new();
        for (object_type, ids) in ids_by_type {
            // Each of these get_x methods returns a Vec<Option>, where the
            // order corresponds to the requested IDs and any element will be
            // null if nothing was found for that ID. We don't care about
            // those missing results though, so just filter those out
            match object_type {
                SpotifyObjectType::Track => {
                    let response = self.get_tracks(ids.iter()).await?;
                    items.extend(
                        response.tracks.into_iter().flatten().map(Item::from),
                    );
                }
                SpotifyObjectType::Album => {
                    let response = self.get_albums(ids.iter()).await?;
                    items.extend(
                        response.albums.into_iter().flatten().map(Item::from),
                    );
                }
                SpotifyObjectType::Artist => {
                    let response = self.get_artists(ids.iter()).await?;
                    items.extend(
                        response.artists.into_iter().flatten().map(Item::from),
                    );
                }
                _ => {
                    return Err(ApiError::UnsupportedObjectType {
                        object_type,
                        backtrace: Backtrace::capture(),
                    })
                }
            }
        }

        Ok(items)
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for Spotify {
    type Error = ApiError;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        /// Helper to build the client, which returns a result
        async fn build_client(
            oauth_client: Arc<BasicClient>,
            identity_state: IdentityState,
        ) -> ApiResult<Spotify> {
            let oauth_handler =
                OAuthHandler::from_identity_state(oauth_client, identity_state)
                    .await?;

            Ok(Spotify::new(oauth_handler))
        }

        // Read the user's ID state and access token from the request
        // TODO clean this up with utility funcs
        let identity_state = match IdentityState::from_request(request).await {
            Outcome::Success(identity_state) => identity_state,
            Outcome::Failure(err) => return Outcome::Failure(err),
            Outcome::Forward(()) => return Outcome::Forward(()),
        };
        let oauth_client =
            match request.guard::<State<'_, Arc<BasicClient>>>().await {
                rocket::request::Outcome::Success(oauth_client) => {
                    oauth_client.inner()
                }
                // software engineering!
                _ => panic!("Couldn't get OAuth client"),
            };

        match build_client(oauth_client.clone(), identity_state).await {
            Ok(spotify) => rocket::request::Outcome::Success(spotify),
            Err(err) => {
                rocket::request::Outcome::Failure((err.to_status(), err))
            }
        }
    }
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
    /// TODO
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

/// Customization options for requests we make to the Spotify API
#[derive(Copy, Clone, Debug, Default)]
struct RequestOptions<'a> {
    /// Query params - typically a slice of (&str, &str)
    params: &'a [(&'a str, &'a str)],
    /// These error status codes will be propagated up, instead of being
    /// converted to a 500
    propagate_errors: &'a [StatusCode],
}

/// A struct to define how to deserialize items from the Spotify API. This is
/// essentially the same thing as the [Item] type that's generated for the
/// API, but we can't define macro attributes on auto-generated types, so we
/// need this mirror type to be able to use serde's attributes.
///
/// We could get around this if
/// https://github.com/davidpdrsn/juniper-from-schema/issues/139 happens.
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum ItemDeserialize {
    Track(Track),
    Album(AlbumSimplified),
    Artist(Artist),
}

// Convert from a Spotify API item to a GraphQL item. These are basically the
// same thing, just with different types
impl From<ItemDeserialize> for Item {
    fn from(other: ItemDeserialize) -> Self {
        match other {
            ItemDeserialize::Track(track) => Item::Track(track),
            ItemDeserialize::Album(album) => Item::AlbumSimplified(album),
            ItemDeserialize::Artist(artist) => Item::Artist(artist),
        }
    }
}
