mod auth;

pub use auth::*;
use typescript_definitions::TypeScriptifyTrait;

use crate::{
    error::{ApiError, ApiResult},
    schema::{Item, SpotifyId, SpotifyObjectType, SpotifyUri, TaggedItem},
};
use lazy_static::lazy_static;
use mongodb::{
    bson::{self, Bson, Document},
    Cursor,
};
use regex::Regex;
use rocket::{http::RawStr, request::FromParam};
use serde::{
    de::{DeserializeOwned, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    backtrace::Backtrace,
    collections::HashMap,
    fmt::{self, Display},
    str::FromStr,
};
use tokio::stream::StreamExt;

/// Deserialize a [Document] into a specific type
pub fn from_doc<T: DeserializeOwned>(doc: Document) -> ApiResult<T> {
    Ok(bson::from_bson(Bson::Document(doc))?)
}

/// Collect a stream of Mongo documents into a Vec
pub async fn from_cursor<T: DeserializeOwned>(
    cursor: Cursor,
) -> ApiResult<Vec<T>> {
    cursor.map(|doc| from_doc::<T>(doc?)).collect().await
}

// SpotifyObjectType

impl Display for SpotifyObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Track => "track",
            Self::Album => "album",
            Self::Artist => "artist",
            Self::User => "user",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for SpotifyObjectType {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "track" => Ok(SpotifyObjectType::Track),
            "album" => Ok(SpotifyObjectType::Album),
            "artist" => Ok(SpotifyObjectType::Artist),
            "user" => Ok(SpotifyObjectType::User),
            _ => Err(ApiError::ParseError {
                message: format!("Unknown Spotify object type: {}", s),
                backtrace: Backtrace::capture(),
            }),
        }
    }
}

// SpotifyUri implementation

impl From<SpotifyUri> for Bson {
    fn from(other: SpotifyUri) -> Self {
        other.to_string().into()
    }
}

impl FromStr for SpotifyUri {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new("^spotify:([^:]+):(.+)$").unwrap();
        }

        match RE.captures(s) {
            None => Err(ApiError::ParseError {
                message: format!("Invalid Spotify URI: {}", s),
                backtrace: Backtrace::capture(),
            }),
            Some(cap) => Ok(Self {
                obj_type: cap[1].parse()?,
                id: cap[2].into(),
            }),
        }
    }
}

struct SpotifyUriVisitor;

impl<'de> Visitor<'de> for SpotifyUriVisitor {
    type Value = SpotifyUri;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string of the format `spotify:<type>:<id>`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        value.parse().map_err(serde::de::Error::custom)
    }
}

impl Serialize for SpotifyUri {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for SpotifyUri {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SpotifyUriVisitor)
    }
}

impl<'r> FromParam<'r> for SpotifyUri {
    type Error = ApiError;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        param.percent_decode()?.parse()
    }
}

impl TypeScriptifyTrait for SpotifyUri {
    fn type_script_ify() -> std::borrow::Cow<'static, str> {
        // real shitty but it works
        "export type SpotifyUri = string;".into()
    }
}

// Item implementation

impl Item {
    pub fn id(&self) -> &SpotifyId {
        match self {
            Self::Track(track) => &track.id,
            Self::Album(album) => &album.id,
            Self::Artist(artist) => &artist.id,
        }
    }

    /// Join tag data with a series of items. For each item, it does a lookup
    /// in the tag map and, if tags are found, joins them with the item into
    /// a single wrapper struct. This is useful when combined with
    /// [TaggedItemDocument::group_tags].
    pub fn join_tags<T: Into<Self>>(
        mut tags_map: HashMap<SpotifyId, Vec<String>>,
        items: impl IntoIterator<Item = T>,
    ) -> impl Iterator<Item = TaggedItem> {
        items.into_iter().map(move |el| {
            let item: Self = el.into();
            let tags = tags_map.remove(item.id()).unwrap_or_else(Vec::new);
            TaggedItem { item, tags }
        })
    }
}
