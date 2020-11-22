use crate::{error::ApiResult, LauludConfig};
use mongodb::{options::ClientOptions, Client, Collection, Database};
use serde::{Deserialize, Serialize};
use strum::IntoStaticStr;

const DATABASE_NAME: &str = "laulud";

#[derive(Copy, Clone, Debug, IntoStaticStr)]
pub enum CollectionName {
    #[strum(to_string = "tracks")]
    Tracks,
}

pub struct DbHandler {
    client: Client,
}

impl DbHandler {
    pub async fn connect(config: &LauludConfig) -> ApiResult<Self> {
        let options = ClientOptions::parse(&config.database_url).await?;
        let client = Client::with_options(options).unwrap();
        Ok(Self { client })
    }

    fn database(&self) -> Database {
        self.client.database(DATABASE_NAME)
    }

    pub fn collection(&self, collection_name: CollectionName) -> Collection {
        self.database().collection(collection_name.into())
    }
}

/// A document in [CollectionName::Tracks]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackDocument {
    pub track_id: String,
    pub tags: Vec<String>,
}
