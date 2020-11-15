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
            // 404
            Self::NotFound(_) => Status::NotFound,
            // 500
            Self::BsonDeserialize(_)
            | Self::Mongo(_)
            | Self::Spotify(_)
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
