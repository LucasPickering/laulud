use crate::spotify::SpotifyItemType;
use async_graphql::{ErrorExtensions, FieldError};
use log::{log, Level};
use mongodb::bson;
use oauth2::basic::BasicErrorResponse;
use rocket::{http::Status, response::Responder, Request};
use std::{backtrace::Backtrace, error::Report};
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
        /// Boxed to reduce variant size. Otherwise we get a clippy warning
        /// about having one very large variant.
        #[from]
        source: Box<mongodb::error::Error>,
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

    /// Failed to deserialize data from a Spotify API response. Indicates a bug
    /// in one of our local Spotify type definitions
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

    #[error("{source}")]
    InputOutput {
        #[from]
        source: tokio::io::Error,
        backtrace: Backtrace,
    },

    /// Error parsing
    #[error("{source}")]
    Parse {
        #[from]
        source: ParseError,
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

    /// Tried to tag an item of an unsupported type
    #[error("Tagging not supported for item of type: {item_type}")]
    UnsupportedItemType {
        item_type: SpotifyItemType,
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
    /// Convert this error to an HTTP status code. For most types the exact
    /// code doesn't matter because we don't actually return HTTP errors in
    /// GraphQL. So in most cases, the only thing that matters is 4xx vs 5xx to
    /// determine the logging level. For error types that can be returned
    /// from oauth routes though, the exact code matters because those are pure
    /// HTTP (no GraphQL).
    pub fn to_status(&self) -> Status {
        match self {
            // 400
            Self::UnsupportedItemType { .. }
            | Self::Parse { .. } => Status::BadRequest,

            // 401
            Self::Unauthenticated { .. }
            | Self::NoRefreshToken { .. }
            | Self::CsrfError { .. }
            | Self::OauthErrorResponse { .. } => Status::Unauthorized,

            // 500
            Self::BsonDeserialize { .. }
            | Self::Mongo { .. }
            | Self::Reqwest { .. }
            | Self::SpotifyApiDeserialization { .. }
            // In some cases, Spotify errors indicate user error (e.g. 404 for
            // an invalid ID), but those will be handled on a case-by-case
            // case by the surrounding code. So generically, Spotify errors are
            // 500s
            | Self::SpotifyApiHttp { .. }
            | Self::InvalidHeaderValue { .. }
            | Self::InputOutput { .. }
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

        let report = Report::new(self).pretty(true).show_backtrace(true);
        log!(log_level, "API Error: {}", report);
    }
}

// Mongo errors get boxed so we need a handwritten From impl to do that
impl From<mongodb::error::Error> for ApiError {
    fn from(other: mongodb::error::Error) -> Self {
        Box::new(other).into()
    }
}

// Allow error to be used in Rocket response
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(
        self,
        _: &'r Request<'_>,
    ) -> rocket::response::Result<'static> {
        self.log();
        Err(self.to_status())
    }
}

impl ErrorExtensions for ApiError {
    fn extend(&self) -> FieldError {
        // TODO see if there's any more info we can add here
        // TODO log errors
        FieldError::new(self.to_string())
    }
}

/// An error occurred while parsing some input. This is typically generated by
/// general purpose functions, so it's hard to attach much context here. If
/// this occurred while parsing user input, this should be mapped into an
/// [InputValidationError].
#[derive(Debug, Error)]
#[error("Error parsing {value}: {message}")]
pub struct ParseError {
    /// Description of the error
    pub message: String,
    /// The value being parsed
    pub value: String,
}
