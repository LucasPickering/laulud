use log::error;
use mongodb::bson;
use oauth2::basic::BasicErrorResponse;
use rocket::{http::Status, response::Responder, Request};
use std::backtrace::Backtrace;
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

    /// CSRF failure during auth
    #[error("CSRF token was not provided or did not match the expected value")]
    CsrfError { backtrace: Backtrace },

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
            // 401
            Self::Unauthenticated { .. }
            | Self::CsrfError { .. }
            | Self::OauthErrorResponse { .. } => Status::Unauthorized,

            // 404
            Self::NotFound { .. } => Status::NotFound,

            // 500
            Self::BsonDeserialize { .. }
            | Self::Mongo { .. }
            | Self::Reqwest { .. }
            | Self::InvalidHeaderValue { .. }
            | Self::Unknown { .. } => Status::InternalServerError,
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
