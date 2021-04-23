//! Utility types that are used for Spotify API interactions, but don't need
//! to be exported outside the [crate::spotify] module.

use crate::{
    graphql::Item,
    spotify::{AlbumSimplified, Artist, Track},
};
use serde::Deserialize;

/// A struct to define how to deserialize items from the Spotify API. This is
/// essentially the same thing as the [Item] type that's generated for the
/// API, but we can't define macro attributes on auto-generated types, so we
/// need this mirror type to be able to use serde's attributes.
///
/// We could get around this if
/// https://github.com/davidpdrsn/juniper-from-schema/issues/139 happens.
#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ItemDeserialize {
    Track(Track),
    Album(AlbumSimplified),
    Artist(Artist),
}

// Convert from a Spotify API item to a GraphQL item. These are basically the
// same thing, just with different types
impl From<ItemDeserialize> for Item {
    fn from(other: ItemDeserialize) -> Self {
        match other {
            ItemDeserialize::Track(track) => Item::Track(track),
            ItemDeserialize::Album(album) => Item::AlbumSimplified(album),
            ItemDeserialize::Artist(artist) => Item::Artist(artist),
        }
    }
}
