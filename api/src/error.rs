use crate::spotify::SpotifyObjectType;
use juniper::{graphql_value, FieldError, IntoFieldError};
use log::{log, Level};
use mongodb::bson;
use oauth2::basic::BasicErrorResponse;
use rocket::{http::Status, response::Responder, Request};
use std::{
    backtrace::Backtrace, error::Error, num::TryFromIntError, str::Utf8Error,
};
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

    /// User passed some invalid input via GraphQL
    #[error("{source}")]
    InvalidInput {
        #[from]
        source: InputValidationError,
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

    /// User passed in some bad UTF-8 string
    #[error("Invalid UTF-8 input")]
    Utf8Error {
        #[from]
        source: Utf8Error,
        backtrace: Backtrace,
    },

    /// Error while running some custom parsing
    #[error("Parse error: {message}")]
    ParseError {
        message: String,
        backtrace: Backtrace,
    },

    /// Tried to tag an object of an unsupported type
    #[error("Tagging not supported for object of type: {object_type}")]
    UnsupportedObjectType {
        object_type: SpotifyObjectType,
        backtrace: Backtrace,
    },

    /// Tried to convert an int to a different type but it didn't fit in the
    /// output type. Usually this indicates either an extremely large value
    /// (beyond what we ever expect to support, so that will never actually
    /// happen) or a failed assumption somewhere. Either way, safe to treat
    /// this as a server error.
    #[error("Number conversion error")]
    TryFromInt {
        #[from]
        source: TryFromIntError,
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
            Self::InvalidInput { .. }
            | Self::Utf8Error { .. }
            | Self::ParseError { .. }
            | Self::UnsupportedObjectType { .. } => Status::BadRequest,

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
            | Self::TryFromInt { .. }
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
            self.backtrace()
                .map(|bt| bt.to_string())
                .unwrap_or_else(String::new)
        );
    }
}

// Mongo errors get boxed so we need a handwritten From impl to do that
impl From<mongodb::error::Error> for ApiError {
    fn from(other: mongodb::error::Error) -> Self {
        Box::new(other).into()
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

// Juniper error
impl IntoFieldError for ApiError {
    fn into_field_error(self) -> FieldError {
        // Temporary method to log errors
        // TODO log this in juniper instead of when we generate the error
        // after https://github.com/graphql-rust/juniper/issues/423
        self.log();

        // TODO generate an error code for each variant that the UI can check
        match self {
            // Input validation errors are designed to be user-friendly, so we
            // can map those back to juniper errors easily
            Self::InvalidInput { source, .. } => FieldError::new(
                source.to_string(),
                graphql_value!({
                    // TODO map field names to camelCase
                    "field": (source.field),
                    "message": (source.message),
                    "value": (source.value),
                }),
            ),
            _ => FieldError::new(self.to_string(), juniper::Value::Null),
        }
    }
}

/// An error indicating the user passed invalid input via the GraphQL API. This
/// is its own type so we can be more specific about what errors we return from
/// validation procedures. This has a corresponding variant on [ApiError], so
/// it can easily be returned from the API.
#[derive(Debug, Error)]
#[error("Invalid input on field {field}: {message}")]
pub struct InputValidationError {
    /// The GraphQL field that contained the error. This will be reported back
    /// to the user to tell them what was invalid
    pub field: String,
    /// Description of what was invalid about the input
    pub message: String,
    /// The invalid input value, as passed by the user
    pub value: juniper::Value,
}
