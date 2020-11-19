use log::error;
use mongodb::bson;
use rocket::{http::Status, response::Responder, Request};
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

/// Type to capture all errors that can happen throughout the app.
/// TODO add backtrace
#[derive(Debug, Error)]
pub enum ApiError {
    /// Error deserializing BSON data, which most likely came from the DB. This
    /// is a server error because it indicates a mismatch between what's store
    /// in the DB and what we expected.
    #[error("BSON serialization error: {0}")]
    BsonDeserialize(#[from] bson::de::Error),

    /// Mongo DB error
    #[error("Database error: {0}")]
    Mongo(#[from] mongodb::error::Error),

    /// Spotify API error
    #[error("Spotify error: {0}")]
    Spotify(failure::Error),

    /// Action cannot be performed because the user is not authenticated.
    #[error("Not logged in")]
    Unauthenticated,

    /// CSRF failure during auth
    #[error("CSRF token was not provided or did not match the expected value")]
    CsrfError,

    /// Wrapper for an OpenID token error, which can occur while validating a
    /// token submitted by a user. BasicErrorType isn't an Error so we have
    /// to stringify it first.
    #[error("{0}")]
    OauthErrorResponse(String),

    /// When we do token exchange with an OpenID provider, we always expect to
    /// get an `id_token` field back in the response. If we don't for some
    /// reason (either we fucked up or the provider fucked up), use this error.
    #[error("id_token field was not in OpenID response")]
    MissingIdToken,

    /// User requested a resource that doesn't exist. String is the unknown
    /// identifier.
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Catch-all error, should have a descriptive message
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl ApiError {
    /// Convert this error to an HTTP status code
    fn to_status(&self) -> Status {
        match self {
            // 401
            Self::Unauthenticated
            | Self::CsrfError
            | Self::OauthErrorResponse(_) => Status::Unauthorized,

            // 404
            Self::NotFound(_) => Status::NotFound,

            // 500
            Self::BsonDeserialize(_)
            | Self::Mongo(_)
            | Self::Spotify(_)
            | Self::MissingIdToken
            | Self::Unknown(_) => Status::InternalServerError,
        }
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(
        self,
        _: &'r Request<'_>,
    ) -> rocket::response::Result<'static> {
        let status = self.to_status();

        // Log 5xx error messages
        if status.code >= 500 {
            error!("HTTP {}: {}", status, self);
        }

        Err(status)
    }
}
