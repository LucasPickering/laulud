mod auth;

pub use auth::*;

use crate::error::ApiResult;
use mongodb::{
    bson::{self, Bson, Document},
    Cursor,
};
use serde::de::DeserializeOwned;
use tokio::stream::StreamExt;

/// Deserialize a [Document] into a specific type
pub fn from_doc<T: DeserializeOwned>(doc: Document) -> ApiResult<T> {
    Ok(bson::from_bson(Bson::Document(doc))?)
}

pub async fn from_cursor<T: DeserializeOwned>(cursor: Cursor) -> ApiResult<Vec<T>> {
    cursor.map(|doc| from_doc::<T>(doc?)).collect().await
}
