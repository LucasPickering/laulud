use crate::{
    error::{ApiError, ApiResult},
    schema::{CurrentUser, Track, TracksResponse, TracksSearchResponse},
    util::{IdentityState, OAuthHandler},
};
use async_trait::async_trait;
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
use serde::de::DeserializeOwned;
use std::{backtrace::Backtrace, sync::Arc};

const SPOTIFY_BASE_URL: &str = "https://api.spotify.com";

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
        &mut self,
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
                    self.oauth_handler.secret()
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
    pub async fn get_current_user(&mut self) -> ApiResult<CurrentUser> {
        self.get_endpoint("/v1/me", RequestOptions::default()).await
    }

    /// https://developer.spotify.com/documentation/web-api/reference/tracks/get-several-tracks/
    pub async fn get_tracks(
        &mut self,
        track_ids: &[&str],
    ) -> ApiResult<TracksResponse> {
        self.get_endpoint(
            "/v1/tracks",
            RequestOptions {
                params: &[("ids", track_ids.join(",").as_str())],
                ..Default::default()
            },
        )
        .await
    }

    /// https://developer.spotify.com/documentation/web-api/reference/tracks/get-track/
    pub async fn get_track(&mut self, track_id: &str) -> ApiResult<Track> {
        self.get_endpoint(
            &format!("/v1/tracks/{}", track_id),
            RequestOptions {
                // a 404 (hopefully) indicates an invalid track ID
                propagate_errors: &[StatusCode::NOT_FOUND],
                ..Default::default()
            },
        )
        .await
    }

    /// Search restricted to tracks
    /// https://developer.spotify.com/documentation/web-api/reference/search/search/
    pub async fn search_tracks(
        &mut self,
        query: &str,
    ) -> ApiResult<Vec<Track>> {
        // TODO maybe need to encode the query?
        let search_response: TracksSearchResponse = self
            .get_endpoint(
                "/v1/search",
                RequestOptions {
                    params: &[("q", query), ("type", "track")],
                    ..Default::default()
                },
            )
            .await?;
        Ok(search_response.tracks.items)
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
