use mongodb::bson::{self, Bson, Document};
use serde::de::DeserializeOwned;

use crate::error::ApiResult;

/// Deserialize a [Document] into a specific type
pub fn from_doc<T: DeserializeOwned>(doc: Document) -> ApiResult<T> {
    Ok(bson::from_bson(Bson::Document(doc))?)
}
