use crate::{
    error::{ApiError, ApiResult},
    schema::{CurrentUser, Track, TracksSearchResponse},
    util::{IdentityState, OAuthHandler},
};
use async_trait::async_trait;
use log::trace;
use oauth2::basic::BasicClient;
use reqwest::{
    header::{self, HeaderValue},
    Client,
};
use rocket::{http::CookieJar, request::FromRequest, State};
use serde::de::DeserializeOwned;
use std::{backtrace::Backtrace, sync::Arc};

const SPOTIFY_BASE_URL: &str = "https://api.spotify.com";

// Below are simple types mapped from the Spotify API

/// A client for accessing the Spotify web API
#[derive(Debug)]
pub struct Spotify {
    /// The access token response from the OAuth2 cycle. This can be used to
    /// get the access token, or to refresh for a new one.
    oauth_handler: OAuthHandler,
    /// HTTP request client
    req_client: Client,
    /// Authenticated user's ID, will be populated after
    /// [Self::get_current_user] is called for the first time. i.e. this is
    /// just a cache for it.
    user_id: Option<String>,
}

impl Spotify {
    pub fn new(oauth_handler: OAuthHandler) -> Self {
        Self {
            oauth_handler,
            req_client: Client::new(),
            user_id: None,
        }
    }

    /// Helper method for doing a GET request against a Spotify endpoint. Parses
    /// the result as JSON.
    async fn get_endpoint<T: DeserializeOwned>(
        &mut self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> ApiResult<T> {
        // Make sure our auth token is up to date first
        self.oauth_handler.refresh_if_needed().await?;

        let url = format!("{}{}", SPOTIFY_BASE_URL, endpoint);
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
            .query(params)
            .send()
            .await?;

        // Some convoluted logic here to get around the response's ownership.
        // If it's a success, parse the body as JSON and return.
        // If it's an error, get the body text and create an error obj with that
        let response_error = response.error_for_status_ref().err();
        let body = response.text().await?;
        trace!("Spotify request URL: {}; Response: {}", &url, &body);
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

    /// Get the ID of the authenticated user. This will make a network request
    /// if this is the first time it's being fetched for this client.
    pub async fn get_user_id(&mut self) -> ApiResult<&str> {
        if self.user_id.is_none() {
            // This will populate self.user_id
            self.get_current_user().await?;
        }

        // Now we know the cache is populated
        Ok(self.user_id.as_deref().unwrap())
    }

    /// https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/
    pub async fn get_current_user(&mut self) -> ApiResult<CurrentUser> {
        let user: CurrentUser = self.get_endpoint("/v1/me", &[]).await?;
        self.user_id = Some(user.id.clone()); // caching!
        Ok(user)
    }

    /// https://developer.spotify.com/documentation/web-api/reference/tracks/get-track/
    pub async fn get_track(&mut self, track_id: &str) -> ApiResult<Track> {
        self.get_endpoint(&format!("/v1/tracks/{}", track_id), &[])
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
            .get_endpoint("/v1/search", &[("q", query), ("type", "track")])
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
            cookies: &CookieJar<'_>,
        ) -> ApiResult<Spotify> {
            // Read the user's access token from the auth cookie
            let identity_state = IdentityState::from_cookies(cookies)?;
            let oauth_handler =
                OAuthHandler::from_identity_state(oauth_client, identity_state)
                    .await?;

            Ok(Spotify::new(oauth_handler))
        }

        let oauth_client =
            match request.guard::<State<'_, Arc<BasicClient>>>().await {
                rocket::request::Outcome::Success(oauth_client) => {
                    oauth_client.inner()
                }
                // software engineering!
                _ => panic!("Couldn't get OAuth client"),
            };

        match build_client(oauth_client.clone(), request.cookies()).await {
            Ok(spotify) => rocket::request::Outcome::Success(spotify),
            Err(err) => {
                rocket::request::Outcome::Failure((err.to_status(), err))
            }
        }
    }
}
