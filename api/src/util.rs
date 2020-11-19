use crate::error::{ApiError, ApiResult};
use mongodb::bson::{self, Bson, Document};
use oauth2::{basic::BasicTokenResponse, CsrfToken};
use rocket::http::{Cookie, CookieJar, SameSite};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use time::Duration;

/// The name of the cookie that we store auth data in
pub const OAUTH_COOKIE_NAME: &str = "oauth-state";

/// Deserialize a [Document] into a specific type
pub fn from_doc<T: DeserializeOwned>(doc: Document) -> ApiResult<T> {
    Ok(bson::from_bson(Bson::Document(doc))?)
}

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
        /// The OAuth2 token we got for the user during auth flow. We'll use
        /// this to access the Spotify API on the user's behalf.
        token_response: BasicTokenResponse,
    },
}

impl IdentityState {
    /// Read the identity state from the identity cookie. If the cookie isn't
    /// present/valid, or if the contents aren't deserializable, return `None`.
    pub fn from_cookies(cookies: &CookieJar<'_>) -> Option<Self> {
        let cookie = cookies.get_private(OAUTH_COOKIE_NAME)?;
        serde_json::from_str(cookie.value()).ok()
    }

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
            _ => Err(ApiError::CsrfError),
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
}
