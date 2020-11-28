use crate::{
    db::TaggedItemDocument,
    error::{ApiError, ApiResult},
    schema::{
        AlbumSimplified, Artist, CurrentUser, Item, SpotifyId,
        SpotifyObjectType, SpotifyUri, TaggedItem, Track,
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
use std::{backtrace::Backtrace, convert::identity, sync::Arc};
use tokio::try_join;

const SPOTIFY_BASE_URL: &str = "https://api.spotify.com";

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#paging-object
#[derive(Clone, Debug, Deserialize)]
pub struct PaginatedResponse<T> {
    pub href: String,
    pub limit: usize,
    pub offset: usize,
    pub total: usize,
    pub next: Option<String>,
    pub previos: Option<String>,
    pub items: Vec<T>,
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

/// https://developer.spotify.com/documentation/web-api/reference/search/search/
#[derive(Clone, Debug, Deserialize)]
pub struct SearchResponse {
    pub tracks: PaginatedResponse<Track>,
    pub albums: PaginatedResponse<AlbumSimplified>,
    pub artists: PaginatedResponse<Artist>,
}
/// Customization options for requests we make to the Spotify API
#[derive(Copy, Clone, Debug, Default)]
struct RequestOptions<'a> {
    /// Query params
    params: &'a [(&'a str, &'a str)],
    /// These error status codes will be propagated up, instead of being
    /// converted to a 500
    propagate_errors: &'a [StatusCode],
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
    pub async fn get_current_user(&self) -> ApiResult<CurrentUser> {
        self.get_endpoint("/v1/me", RequestOptions::default()).await
    }

    /// Get an item of any type from the API
    pub async fn get_item(&self, uri: &SpotifyUri) -> ApiResult<Item> {
        let options = RequestOptions {
            // a 404 (hopefully) indicates an invalid ID
            propagate_errors: &[StatusCode::NOT_FOUND],
            ..Default::default()
        };
        let item = match uri.obj_type {
            // https://developer.spotify.com/documentation/web-api/reference/tracks/get-track/
            SpotifyObjectType::Track => self
                .get_endpoint::<Track>(
                    &format!("/v1/tracks/{}", uri.id),
                    options,
                )
                .await?
                .into(),
            // https://developer.spotify.com/documentation/web-api/reference/albums/get-album/
            SpotifyObjectType::Album => self
                .get_endpoint::<AlbumSimplified>(
                    &format!("/v1/albums/{}", uri.id),
                    options,
                )
                .await?
                .into(),
            // https://developer.spotify.com/documentation/web-api/reference/artists/get-artist/
            SpotifyObjectType::Artist => self
                .get_endpoint::<Artist>(
                    &format!("/v1/artists/{}", uri.id),
                    options,
                )
                .await?
                .into(),
            // We don't support tagging any other object types
            obj_type => {
                return Err(ApiError::UnsupportedObjectType {
                    obj_type,
                    backtrace: Backtrace::capture(),
                })
            }
        };
        Ok(item)
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
                    track_ids.map(|id| id.0.as_str()).join(",").as_str(),
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
                    album_ids.map(|id| id.0.as_str()).join(",").as_str(),
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
                    artists_ids.map(|id| id.0.as_str()).join(",").as_str(),
                )],
                ..Default::default()
            },
        )
        .await
    }

    /// Search restricted to taggable items
    /// https://developer.spotify.com/documentation/web-api/reference/search/search/
    pub async fn search_items(&self, query: &str) -> ApiResult<SearchResponse> {
        self.get_endpoint(
            "/v1/search",
            RequestOptions {
                params: &[("q", query), ("type", "track,album,artist")],
                ..Default::default()
            },
        )
        .await
    }

    /// Given an array of tagged items that came from Mongo, this looks up
    /// each item's metadata from Spotify and joins it into each document.
    /// For example, for a track this will add in title, artist, album, etc.
    pub async fn saturated_tagged_items(
        &self,
        docs: Vec<TaggedItemDocument>,
    ) -> ApiResult<Vec<TaggedItem>> {
        // Group the docs by type
        let (track_tags, album_tags, artist_tags) =
            TaggedItemDocument::group_tags(docs)?;

        // Fetch all the data from Spotify. For any item type, if we have no
        // documents for that type, then skip it.
        let (tracks_resp, albums_resp, artists_resp) = try_join!(
            async {
                if track_tags.is_empty() {
                    Ok(None)
                } else {
                    self.get_tracks(track_tags.keys()).await.map(Some)
                }
            },
            async {
                if album_tags.is_empty() {
                    Ok(None)
                } else {
                    self.get_albums(album_tags.keys()).await.map(Some)
                }
            },
            async {
                if artist_tags.is_empty() {
                    Ok(None)
                } else {
                    self.get_artists(artist_tags.keys()).await.map(Some)
                }
            },
        )?;

        // Join the tag data into each item we got from spotify
        let mut items: Vec<TaggedItem> = Vec::new();
        if let Some(track_objs) = tracks_resp {
            items.extend(Item::join_tags(
                track_tags,
                track_objs.tracks.into_iter().filter_map(identity),
            ));
        }
        if let Some(album_objs) = albums_resp {
            items.extend(Item::join_tags(
                album_tags,
                album_objs.albums.into_iter().filter_map(identity),
            ));
        }
        if let Some(artist_objs) = artists_resp {
            items.extend(Item::join_tags(
                artist_tags,
                artist_objs.artists.into_iter().filter_map(identity),
            ));
        }

        Ok(items)
    }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for Spotify {
    type Error = ApiError;

    async fn from_request(
        request: &'a rocket::Request<'r>,
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
