//! Basic, generic GraphQL types, that aren't specific to any part of the API or
//! any particular data type

use crate::error::ParseError;
use async_graphql::{scalar, Object};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// An identifier determining where in a paginated sequence of data we are. A
/// cursor is just some offset value converted into a string, so this struct
/// represents the unconverted version.
#[derive(Copy, Clone, Debug, Display, Serialize, Deserialize)]
// TODO use a fancy encoding here like hex or base64 to look cool
#[display(fmt = "{}", self.offset)]
pub struct Cursor {
    offset: usize,
}

scalar!(Cursor);

impl Cursor {
    /// Get the pre-parsed offset for this cursor. A cursor is just an
    /// obfuscated number that indicates the element's offset into the
    /// collection. These offsets can be used with Spotify or Mongo to find
    /// the element.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Get the offset used when this cursor is provided as an `after`
    /// pagination param. In other words, get the offset of the item directly
    /// *after* this cursor in the sequence.
    pub fn after_offset(&self) -> usize {
        self.offset() + 1
    }

    /// Get a cursor for an edge based on the offset of the page that if came
    /// from and the index of the edge _within that page_. These two values
    /// together tell us the total offset of the edge, which is used to
    /// generate a cursor.
    pub fn from_offset_index(offset: usize, index: usize) -> Self {
        Self {
            offset: offset + index,
        }
    }
}

impl FromStr for Cursor {
    type Err = ParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // Parse the string as a plain number
        let offset = value.parse::<usize>().map_err(|_| ParseError {
            message: "Invalid pagination cursor".into(),
            value: value.into(),
        })?;
        Ok(Cursor { offset })
    }
}

/// GQL type to display information about a page of data. See the Relay
/// Connections spec: https://facebook.github.io/relay/graphql/connections.htm#sec-undefined.PageInfo
#[derive(Clone, Debug)]
pub struct PageInfo {
    pub offset: usize,
    pub page_len: usize,
    pub has_previous_page: bool,
    pub has_next_page: bool,
}

#[Object]
impl PageInfo {
    /// The spec says that the start and end cursors must be non-null, but that
    /// doesn't make sense because if the page is empty, then there is no
    /// possible value for either. So those fields should only be `None` iff
    /// the page is empty.
    async fn cursor(&self) -> Option<Cursor> {
        if self.page_len > 0 {
            Some(Cursor::from_offset_index(self.offset, 0))
        } else {
            None
        }
    }

    /// See start_cursor resolver above for why this is an option
    async fn end_cursor(&self) -> Option<Cursor> {
        if self.page_len > 0 {
            Some(Cursor::from_offset_index(self.offset, self.page_len - 1))
        } else {
            None
        }
    }

    async fn previous_page(&self) -> bool {
        self.has_previous_page
    }

    async fn next_page(&self) -> bool {
        self.has_next_page
    }
}
