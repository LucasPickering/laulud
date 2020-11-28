use crate::error::{ApiError, ApiResult};
use async_trait::async_trait;
use mongodb::bson::Bson;
use oauth2::{
    basic::{BasicClient, BasicTokenResponse},
    CsrfToken, TokenResponse,
};
use rocket::{
    http::{Cookie, CookieJar, SameSite},
    request::{FromRequest, Outcome},
};
use serde::{Deserialize, Serialize};
use std::{backtrace::Backtrace, sync::Arc};
use time::{Duration, OffsetDateTime};

/// The name of the cookie that we store auth data in
const OAUTH_COOKIE_NAME: &str = "oauth-state";

/// Data related to a user's current auth state. This is meant to be serialized
/// and stored within a private cookie with rocket. That cookie is encrypted, so
/// any data we put in here is guaranteed to be secret and authentic.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum IdentityState {
    /// The state that we care about while the user is in the OAuth flow. This
    /// variant is only used while a user is in the login process. Once login
    /// is finished, we should replace the cookie with a `PostAuth` value.
    DuringAuth {
        /// Cross-site Request Forgery token. Used to reject unsolicited OAuth
        /// callbacks
        /// https://auth0.com/docs/protocols/state-parameters
        csrf_token: CsrfToken,
        /// The route to redirect the user to after finishing login
        next: Option<String>,
    },
    /// State that we track when the user is already logged in.
    PostAuth {
        token: AuthenticationToken,
        user_id: UserId,
    },
}

impl IdentityState {
    /// Serialize the identity state into a private auth cookie
    pub fn to_cookie(&self) -> Cookie<'static> {
        Cookie::build(OAUTH_COOKIE_NAME, self.serialize())
            .same_site(SameSite::Lax)
            .secure(true)
            .max_age(Duration::days(1))
            .finish()
    }

    /// Serialize this object into a string. Used to store the value in the
    /// identity cookie.
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Check the stored CSRF token against a token that was given as input.
    /// This should be done during the callback stage of auth, to make sure that
    /// the callback is coming from the user that requested the auth cycle.
    pub fn verify_csrf(&self, suspect_csrf_token: &str) -> ApiResult<()> {
        match self {
            // Check that we have a stored token, and that it matches the given
            Self::DuringAuth { csrf_token, .. }
                if suspect_csrf_token == csrf_token.secret() =>
            {
                Ok(())
            }
            _ => Err(ApiError::CsrfError {
                backtrace: Backtrace::capture(),
            }),
        }
    }

    /// Get the `next` param, which tells the API which route to redirect to
    /// after finishing the auth process. If the value isn't present in the
    /// state, just return the root route.
    pub fn next(&self) -> &str {
        match self {
            Self::DuringAuth {
                next: Some(next), ..
            } => next,
            _ => "/",
        }
    }

    /// Delete the stored ID state cookie (if present)
    pub fn remove_cookie(cookies: &CookieJar<'_>) {
        cookies.remove_private(Cookie::named(OAUTH_COOKIE_NAME));
    }
}

/// Load ID state from the request cookies
#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for IdentityState {
    type Error = ApiError;

    async fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> Outcome<Self, Self::Error> {
        // This cookie is encrypted+signed
        if let Some(cookie) = request.cookies().get_private(OAUTH_COOKIE_NAME) {
            if let Ok(identity_state) = serde_json::from_str(cookie.value()) {
                // Successfully deserialized
                return Outcome::Success(identity_state);
            } else {
                // Cookie was invalid/corrupt, so delete it
                Self::remove_cookie(request.cookies());
            }
        }

        let err = ApiError::Unauthenticated {
            backtrace: Backtrace::capture(),
        };
        Outcome::Failure((err.to_status(), err))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticationToken {
    /// The OAuth2 token we got for the user during auth flow. We'll use
    /// this to access the Spotify API on the user's behalf.
    token_response: BasicTokenResponse,
    /// The date/time when this token expires.
    expires_at: OffsetDateTime,
}

impl From<BasicTokenResponse> for AuthenticationToken {
    fn from(token_response: BasicTokenResponse) -> Self {
        let expires_at =
            OffsetDateTime::now_utc() + token_response.expires_in().unwrap();
        Self {
            token_response,
            expires_at,
        }
    }
}

/// A user's unique Spotify ID. We use a newtype so we can implement
/// `FromRequest`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId(String);

impl From<UserId> for Bson {
    fn from(other: UserId) -> Self {
        other.0.as_str().into()
    }
}

impl From<String> for UserId {
    fn from(other: String) -> Self {
        UserId(other)
    }
}

#[async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for UserId {
    type Error = ApiError;

    async fn from_request(
        request: &'a rocket::Request<'r>,
    ) -> Outcome<Self, Self::Error> {
        // TODO clean this up with utility funcs
        match IdentityState::from_request(request).await {
            Outcome::Success(IdentityState::PostAuth { user_id, .. }) => {
                Outcome::Success(user_id)
            }
            Outcome::Success(_) => {
                let err = ApiError::Unauthenticated {
                    backtrace: Backtrace::capture(),
                };
                Outcome::Failure((err.to_status(), err))
            }
            Outcome::Failure(err) => Outcome::Failure(err),
            Outcome::Forward(()) => Outcome::Forward(()),
        }
    }
}

#[derive(Debug)]
pub struct OAuthHandler {
    pub client: Arc<BasicClient>,
    pub token: AuthenticationToken,
}

impl OAuthHandler {
    pub fn secret(&self) -> &str {
        self.token.token_response.access_token().secret()
    }

    /// Move the auth token out of this object
    pub fn into_token(self) -> AuthenticationToken {
        self.token
    }

    pub async fn from_identity_state(
        client: Arc<BasicClient>,
        identity_state: IdentityState,
    ) -> ApiResult<Self> {
        match identity_state {
            IdentityState::PostAuth { token, .. } => {
                let mut rv = Self { client, token };
                // Make sure the access token is fresh
                rv.refresh_if_needed().await?;
                Ok(rv)
            }
            _ => Err(ApiError::Unauthenticated {
                backtrace: Backtrace::capture(),
            }),
        }
    }

    /// Check if the current access token is outdated, and if so, fetch a new
    /// one from the API. In most cases this will be very cheap, and will only
    /// make the HTTP call when necessary.
    pub async fn refresh_if_needed(&mut self) -> ApiResult<()> {
        // Give us a 1 minute buffer just to prevent race conditions or smth idk
        let threshold = self.token.expires_at - Duration::minutes(1);
        if OffsetDateTime::now_utc() > threshold {
            match self.token.token_response.refresh_token() {
                None => Err(ApiError::NoRefreshToken {
                    backtrace: Backtrace::capture(),
                }),
                Some(refresh_token) => {
                    let token_response = self
                        .client
                        .exchange_refresh_token(refresh_token)
                        .request_async(oauth2::reqwest::async_http_client)
                        .await?;
                    self.token = token_response.into();
                    // Successful refresh
                    Ok(())
                }
            }
        } else {
            Ok(())
        }
    }
}
