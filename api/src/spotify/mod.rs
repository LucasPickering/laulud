//! A binding for the Spotify Web API. The main struct here is [Spotify]. That
//! provides all the interactions you will need to do with the API. This module
//! also contains Rust definitions of the different types that the Spotify API
//! accepts and returns.
//!
//! The Spotify API has really good docs at
//! https://developer.spotify.com/documentation/web-api/reference/

mod types;

pub use types::*;

use crate::{
    auth::{IdentityState, OAuthHandler},
    error::{ApiError, ApiResult},
};
use futures::future::try_join_all;
use itertools::Itertools;
use log::{debug, trace};
use oauth2::basic::BasicClient;
use reqwest::{
    header::{self, HeaderValue},
    Client,
};
use rocket::{
    request::{FromRequest, Outcome},
    State,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{backtrace::Backtrace, collections::HashMap, sync::Arc};

const SPOTIFY_BASE_URL: &str = "https://api.spotify.com";

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
    /// the result as JSON. Query params are generally specified as a slice of
    /// string tuples, e.g.:
    ///
    /// ```
    /// &[("p1", "v1"), ("p2", "v2")]
    /// ```
    ///
    /// For requests with no query params, just pass `&[]`.
    async fn get_endpoint<P: Serialize, T: DeserializeOwned>(
        &self,
        endpoint: &str,
        query_params: P,
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
            .query(&query_params)
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
            Some(err) => Err(ApiError::SpotifyApiHttp {
                source: err,
                body,
                backtrace: Backtrace::capture(),
            }),
        }
    }

    /// https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/
    pub async fn get_current_user(&self) -> ApiResult<PrivateUser> {
        self.get_endpoint::<&[&str], _>("/v1/me", &[]).await
    }

    /// https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-tracks/
    pub async fn get_tracks(
        &self,
        mut track_ids: impl Iterator<Item = &str>,
    ) -> ApiResult<TracksResponse> {
        self.get_endpoint(
            "/v1/tracks",
            &[("ids", track_ids.join(",").as_str())],
        )
        .await
    }

    /// https://developer.spotify.com/documentation/web-api/reference/albums/get-several-albums/
    pub async fn get_albums(
        &self,
        mut album_ids: impl Iterator<Item = &str>,
    ) -> ApiResult<AlbumsResponse> {
        self.get_endpoint(
            "/v1/albums",
            &[("ids", album_ids.join(",").as_str())],
        )
        .await
    }

    /// https://developer.spotify.com/documentation/web-api/reference/artists/get-several-artists/
    pub async fn get_artists(
        &self,
        mut artists_ids: impl Iterator<Item = &str>,
    ) -> ApiResult<ArtistsResponse> {
        self.get_endpoint(
            "/v1/artists",
            &[("ids", artists_ids.join(",").as_str())],
        )
        .await
    }

    /// Search restricted to taggable items
    /// https://developer.spotify.com/documentation/web-api/reference/#category-search
    pub async fn search_items(
        &self,
        search_query: &str,
        limit: Option<usize>,
        offset: Option<usize>,
    ) -> ApiResult<HashMap<String, PaginatedResponse<Item>>> {
        let mut query_params = vec![
            ("q", search_query.to_owned()),
            ("type", "track,album,artist".to_owned()),
        ];

        if let Some(limit) = limit {
            query_params.push(("limit", limit.to_string()));
        }
        if let Some(offset) = offset {
            query_params.push(("offset", offset.to_string()));
        }

        let responses: HashMap<String, PaginatedResponse<Item>> =
            self.get_endpoint("/v1/search", &query_params).await?;
        Ok(responses)
    }

    /// Get an item of any type from the API. This will call the correct
    /// endpoint based on the item's type (track, album, etc.). Returns `None`
    /// iff the URI does not exist in Spotify (i.e. Spotify returned a 404).
    /// This makes the method more usable in GraphQL, where missing resources
    /// are typically returned as null.
    pub async fn get_item(&self, uri: &SpotifyUri) -> ApiResult<Option<Item>> {
        let result = match uri.item_type() {
            // https://developer.spotify.com/documentation/web-api/reference/tracks/get-track/
            SpotifyItemType::Track => self
                .get_endpoint::<&[&str], Track>(
                    &format!("/v1/tracks/{}", uri.id()),
                    &[],
                )
                .await.map(Item::from),
            // https://developer.spotify.com/documentation/web-api/reference/albums/get-album/
            SpotifyItemType::Album => self
                .get_endpoint::<&[&str], AlbumSimplified>(
                    &format!("/v1/albums/{}", uri.id()),
                    &[],
                )
                .await.map(Item::from),
            // https://developer.spotify.com/documentation/web-api/reference/artists/get-artist/
            SpotifyItemType::Artist => self
                .get_endpoint::<&[&str], Artist>(
                    &format!("/v1/artists/{}", uri.id()),
                    &[],
                )
                .await
                .map(Item::from)
                ,
            // We don't support tagging any other object types
            item_type => {
                 Err(ApiError::UnsupportedItemType {
                    item_type,
                    backtrace: Backtrace::capture(),
                })
            }
        };

        match result {
            Ok(item) => Ok(Some(item)),
            // Map 404 to None
            Err(ApiError::SpotifyApiHttp { source, .. })
                if source.status().map(|s| s.as_u16()) == Some(404) =>
            {
                Ok(None)
            }
            Err(err) => Err(err),
        }
    }

    /// Fetch data for a list of items of any type. This will make one request
    /// to the API per item _type_ in the input list, e.g. if your input URIs
    /// have 3 tracks, 2 albums, and 10 artists, this will still only make 3
    /// requests to the API.
    ///
    /// If any of the given URIs doesn't return a response from Spotify, then
    /// that item will simply not be included in the output. So the output will
    /// always be the length of the input minus the number of
    /// invalid/non-matching URIs.
    pub async fn get_items(
        &self,
        uris: impl Iterator<Item = &SpotifyUri>,
    ) -> ApiResult<Vec<Item>> {
        // Group URIs by type so we can make one request per type
        let ids_by_type: HashMap<SpotifyItemType, Vec<&str>> =
            uris.map(|uri| (uri.item_type(), uri.id())).into_group_map();

        /// Convert a list of search results of any type into a standardized
        /// list of items. For each item type, Spotify returns a list of options
        /// where the order corresponds to the URI request order and any
        /// element will be None if nothing matched the URI. We don't care
        /// about the Nones though, so filter those out here. At the same time,
        /// convert each result element into an [Item] so we can group them
        /// all together later.
        fn results_to_items<T>(results: Vec<Option<T>>) -> Vec<Item>
        where
            Item: From<T>,
        {
            results
                .into_iter()
                .flatten()
                .map(Item::from)
                .collect::<Vec<_>>()
        }

        // Make one request to the Spotify API for each item type. These will
        // run concurrently, hence the try_join_all down below
        let futures =
            ids_by_type.into_iter().map(|(item_type, ids)| async move {
                // Shortcut!
                if ids.is_empty() {
                    return Ok(Vec::new());
                }

                match item_type {
                    SpotifyItemType::Track => {
                        let response = self.get_tracks(ids.into_iter()).await?;
                        Ok(results_to_items(response.tracks))
                    }
                    SpotifyItemType::Album => {
                        let response = self.get_albums(ids.into_iter()).await?;
                        Ok(results_to_items(response.albums))
                    }
                    SpotifyItemType::Artist => {
                        let response =
                            self.get_artists(ids.into_iter()).await?;
                        Ok(results_to_items(response.artists))
                    }
                    _ => Err(ApiError::UnsupportedItemType {
                        item_type,
                        backtrace: Backtrace::capture(),
                    }),
                }
            });
        let vecs: Vec<Vec<Item>> = try_join_all(futures).await?;
        let items: Vec<Item> = vecs.into_iter().flatten().collect();

        Ok(items)
    }

    /// Get detailed analysis information for a single track. If the given IDd
    /// doesn't refer to a valid track, then this will error.
    ///
    /// https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-audio-features
    pub async fn get_audio_features(
        &self,
        id: &str,
    ) -> ApiResult<AudioFeatures> {
        self.get_endpoint::<&[&str], _>(
            &format!("/v1/audio-features/{}", id),
            &[],
        )
        .await
    }
}

// Make it easy to grab a spotify instance for any request handler
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Spotify {
    type Error = ApiError;

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        // We'll generate a Spotify instance by reading the user's spotify
        // creds from their cookies. If the cookies are missing/invalid, we'll
        // return an error.

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
        let identity_state = match IdentityState::from_request(request).await {
            Outcome::Success(identity_state) => identity_state,
            Outcome::Failure(err) => return Outcome::Failure(err),
            Outcome::Forward(()) => return Outcome::Forward(()),
        };
        let oauth_client =
            match request.guard::<&State<Arc<BasicClient>>>().await {
                rocket::request::Outcome::Success(oauth_client) => {
                    oauth_client.inner()
                }
                // software engineering!
                // This shouldn't be possible, as long as the oauth client is
                // available which is always will be
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
