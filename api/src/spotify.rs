use crate::{
    error::{ApiError, ApiResult},
    util::{IdentityState, OAuthHandler},
};
use async_trait::async_trait;
use oauth2::basic::BasicClient;
use reqwest::{
    header::{self, HeaderValue},
    Client,
};
use rocket::{http::CookieJar, request::FromRequest, State};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{backtrace::Backtrace, sync::Arc};

const SPOTIFY_BASE_URL: &str = "https://api.spotify.com";

// Below are simple types mapped from the Spotify API

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#paging-object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    href: String,
    limit: usize,
    offset: usize,
    total: usize,
    next: Option<String>,
    previos: Option<String>,
    items: Vec<T>,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#image-object
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

/// https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: String,
    pub href: String,
    pub uri: String,
    pub display_name: Option<String>,
    pub images: Vec<Image>,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    id: String,
    name: String,
    href: String,
    uri: String,
    explicit: bool,
    popularity: usize,
    track_number: usize,
}

/// https://developer.spotify.com/documentation/web-api/reference/search/search/
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TracksSearchResponse {
    tracks: PaginatedResponse<Track>,
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
    pub async fn get_current_user(&mut self) -> ApiResult<CurrentUser> {
        self.get_endpoint("/v1/me", &[]).await
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
