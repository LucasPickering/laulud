use log::{log, Level};
use mongodb::bson;
use oauth2::basic::BasicErrorResponse;
use rocket::{http::Status, response::Responder, Request};
use std::{backtrace::Backtrace, error::Error};
use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

/// Type to capture all errors that can happen throughout the app.
#[derive(Debug, Error)]
pub enum ApiError {
    /// Error deserializing BSON data, which most likely came from the DB. This
    /// is a server error because it indicates a mismatch between what's store
    /// in the DB and what we expected.
    #[error("BSON serialization error: {source}")]
    BsonDeserialize {
        #[from]
        source: bson::de::Error,
        backtrace: Backtrace,
    },

    /// Mongo DB error
    #[error("Database error: {source}")]
    Mongo {
        #[from]
        source: mongodb::error::Error,
        backtrace: Backtrace,
    },

    /// Reqwest error
    #[error("{source}")]
    Reqwest {
        #[from]
        source: reqwest::Error,
        backtrace: Backtrace,
    },

    /// HTTP error from the Spotify API
    #[error("Spotify HTTP error: {source}; Body: {body}")]
    SpotifyApiHttp {
        source: reqwest::Error,
        body: String,
        backtrace: Backtrace,
    },

    /// Failed to deserialize data from a Spotify APi response
    #[error("Spotify deserialization error: {source}; Body: {body}")]
    SpotifyApiDeserialization {
        source: serde_json::Error,
        body: String,
        backtrace: Backtrace,
    },

    /// Reqwest error while creating a request header
    #[error("{source}")]
    InvalidHeaderValue {
        #[from]
        source: reqwest::header::InvalidHeaderValue,
        backtrace: Backtrace,
    },

    /// Wrapper for an OpenID token error, which can occur while validating a
    /// token submitted by a user.
    #[error("{source}")]
    OauthErrorResponse {
        #[from]
        source: oauth2::RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            BasicErrorResponse,
        >,
        backtrace: Backtrace,
    },

    /// Action cannot be performed because the user is not authenticated.
    #[error("Not logged in")]
    Unauthenticated { backtrace: Backtrace },

    /// We attempted to refresh the user state, but there was no refresh token
    #[error("No refresh token")]
    NoRefreshToken { backtrace: Backtrace },

    /// CSRF failure during auth
    #[error("CSRF token was not provided or did not match the expected value")]
    CsrfError { backtrace: Backtrace },

    #[error("Invalid input: {source}")]
    Validation {
        #[from]
        source: validator::ValidationErrors,
        backtrace: Backtrace,
    },

    /// User requested a resource that doesn't exist. `resource` is the unknown
    /// identifier.
    #[error("Resource not found: {resource}")]
    NotFound {
        resource: String,
        backtrace: Backtrace,
    },

    /// Catch-all error, should have a descriptive message
    #[error("Unknown error: {message}")]
    Unknown {
        message: String,
        backtrace: Backtrace,
    },
}

impl ApiError {
    /// Convert this error to an HTTP status code
    pub fn to_status(&self) -> Status {
        match self {
            // 400
            Self::Validation { .. } => Status::BadRequest,

            // 401
            Self::Unauthenticated { .. }
            | Self::NoRefreshToken { .. }
            | Self::CsrfError { .. }
            | Self::OauthErrorResponse { .. } => Status::Unauthorized,

            // 404
            Self::NotFound { .. } => Status::NotFound,

            // 500
            Self::BsonDeserialize { .. }
            | Self::Mongo { .. }
            | Self::Reqwest { .. }
            | Self::SpotifyApiHttp { .. }
            | Self::SpotifyApiDeserialization { .. }
            | Self::InvalidHeaderValue { .. }
            | Self::Unknown { .. } => Status::InternalServerError,
        }
    }

    /// Log this error. Logging level will be based on the status code
    pub fn log(&self) {
        let log_level = if self.to_status().code >= 500 {
            Level::Error
        } else {
            Level::Debug
        };

        log!(
            log_level,
            "API Error: {}\n{}",
            self,
            self.backtrace().expect("No backtrace available :(")
        );
    }
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(
        self,
        _: &'r Request<'_>,
    ) -> rocket::response::Result<'static> {
        self.log();
        Err(self.to_status())
    }
}
