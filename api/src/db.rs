use std::{backtrace::Backtrace, collections::HashMap};

use crate::{
    error::{ApiError, ApiResult},
    schema::{SpotifyId, SpotifyObjectType, SpotifyUri},
    util::UserId,
    LauludConfig,
};
use mongodb::{options::ClientOptions, Client, Collection, Database};
use serde::{Deserialize, Serialize};

const DATABASE_NAME: &str = "laulud";

#[derive(Copy, Clone, Debug)]
pub enum CollectionName {
    TaggedItems,
}

impl CollectionName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TaggedItems => "taggedItems",
        }
    }
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
        self.database().collection(collection_name.as_str())
    }
}

// ===== DB Schema =====
// Below is the schema for each collection in the DB

/// A document in [CollectionName::TaggedItems]. Any item type can be tagged.
/// The item type can be grabbed via `uri.uri_type`. We avoid storing any data
/// beyond the URI because it can change, and there's probably legal shit around
/// it too. So we just fetch it from Spotify on-demand whenever it's needed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaggedItemDocument {
    pub user_id: UserId,
    pub tags: Vec<String>,
    /// Uniquely identifies both the document type (track, album, etc.) and the
    /// document itself.
    /// https://developer.spotify.com/documentation/web-api/#spotify-uris-and-ids
    pub uri: SpotifyUri,
}

impl TaggedItemDocument {
    /// Convert a list of docs into three different mappings of URI:tags, split
    /// by doc type (tracks, albums, artists)
    #[allow(clippy::clippy::type_complexity)] // fuck a lint warning
    pub fn group_tags(
        docs: impl IntoIterator<Item = Self>,
    ) -> ApiResult<(
        HashMap<SpotifyId, Vec<String>>,
        HashMap<SpotifyId, Vec<String>>,
        HashMap<SpotifyId, Vec<String>>,
    )> {
        // Each one is a mapping of ID:tags
        let mut track_tags: HashMap<SpotifyId, Vec<String>> = HashMap::new();
        let mut album_tags: HashMap<SpotifyId, Vec<String>> = HashMap::new();
        let mut artist_tags: HashMap<SpotifyId, Vec<String>> = HashMap::new();

        // Group the docs by type
        for doc in docs {
            let id_map = match doc.uri.obj_type {
                SpotifyObjectType::Track => &mut track_tags,
                SpotifyObjectType::Album => &mut album_tags,
                SpotifyObjectType::Artist => &mut artist_tags,
                // We don't support tagging any other object types
                obj_type => {
                    return Err(ApiError::UnsupportedObjectType {
                        obj_type,
                        backtrace: Backtrace::capture(),
                    })
                }
            };
            id_map.insert(doc.uri.id, doc.tags);
        }

        Ok((track_tags, album_tags, artist_tags))
    }
}
