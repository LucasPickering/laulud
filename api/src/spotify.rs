use crate::{
    error::{ApiError, ApiResult},
    util::IdentityState,
};
use async_trait::async_trait;
use oauth2::{basic::BasicTokenResponse, TokenResponse};
use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use rocket::{http::CookieJar, request::FromRequest};
use serde::{de::DeserializeOwned, Deserialize};
use std::backtrace::Backtrace;

const SPOTIFY_BASE_URL: &str = "https://api.spotify.com";

// Below are simple types mapped from the Spotify API

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#image-object
#[derive(Clone, Debug, Deserialize)]
pub struct Image {
    pub url: String,
    pub width: Option<usize>,
    pub height: Option<usize>,
}

/// https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/
#[derive(Clone, Debug, Deserialize)]
pub struct CurrentUser {
    pub id: String,
    pub href: String,
    pub uri: String,
    pub display_name: Option<String>,
    pub images: Vec<Image>,
}

/// https://developer.spotify.com/documentation/web-api/reference/object-model/#track-object-full
#[derive(Clone, Debug, Deserialize)]
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
#[derive(Clone, Debug, Deserialize)]
pub struct TracksSearchResult {
    tracks: Vec<Track>,
}

/// A client for accessing the Spotify web API
pub struct Spotify {
    /// The access token response from the OAuth2 cycle. This can be used to
    /// get the access token, or to refresh for a new one.
    token_response: BasicTokenResponse,
    /// HTTP request client
    req_client: Client,
}

impl Spotify {
    pub fn new(token_response: BasicTokenResponse) -> ApiResult<Self> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Bearer: {}",
                token_response.access_token().secret()
            ))?,
        );
        let req_client =
            Client::builder().default_headers(default_headers).build()?;

        Ok(Self {
            token_response,
            req_client,
        })
    }

    /// Helper method for doing a GET request against a Spotify endpoint. Parses
    /// the result as JSON.
    async fn get_endpoint<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> ApiResult<T> {
        let url = format!("{}{}", SPOTIFY_BASE_URL, endpoint);
        let response = self.req_client.get(&url).query(params).send().await?;
        Ok(response.json().await?)
    }

    /// https://developer.spotify.com/documentation/web-api/reference/users-profile/get-current-users-profile/
    pub async fn get_current_user(&self) -> ApiResult<CurrentUser> {
        self.get_endpoint("/v1/me", &[]).await
    }

    /// Search restricted to tracks
    /// https://developer.spotify.com/documentation/web-api/reference/search/search/
    pub async fn search_tracks(&self, query: &str) -> ApiResult<Vec<Track>> {
        // TODO maybe need to encode the query?
        self.get_endpoint("/v1/search", &[("q", query), ("type", "tracks")])
            .await
    }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for Spotify {
    type Error = ApiError;

    async fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        fn ass(cookies: &CookieJar<'_>) -> ApiResult<Spotify> {
            // Read the user's access token from the auth cookie
            let token_response = match IdentityState::from_cookies(cookies) {
                Some(IdentityState::PostAuth { token_response }) => {
                    Ok(token_response)
                }
                _ => Err(ApiError::Unauthenticated {
                    backtrace: Backtrace::capture(),
                }),
            }?;

            let spotify = Spotify::new(token_response)?;
            Ok(spotify)
        }

        match ass(request.cookies()) {
            Ok(spotify) => rocket::request::Outcome::Success(spotify),
            Err(err) => {
                rocket::request::Outcome::Failure((err.to_status(), err))
            }
        }
    }
}
