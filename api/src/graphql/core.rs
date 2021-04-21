//! Basic, generic GraphQL types, that aren't specific to any part of the API or
//! any particular data type

use crate::graphql::{Cursor, PageInfoFields, RequestContext};
use juniper::Executor;

// TODO make these dedicated graphql scalar types, so we can get the type
// safety in the API and UI
pub type SpotifyId = String;
pub type SpotifyUri = String;
// TODO add a scalar type for Tag as well

impl Cursor {
    /// Parse the cursor into an offset value. A cursor is just an obfuscated
    /// number that indicates the element's offset into the collection. These
    /// offsets can be used with Spotify or Mongo to find the element.
    ///
    /// TODO make this return a result and Err if the value isn't a valid usize
    pub fn offset(&self) -> usize {
        // TODO cursor validation and remove unwrap
        self.0.parse::<usize>().unwrap() + 1
    }

    /// Get a cursor for an edge based on the offset of the page that if came
    /// from and the index of the edge _within that page_. These two values
    /// together tell us the total offset of the edge, which is used to
    /// generate a cursor.
    pub fn from_offset_index(offset: usize, index: usize) -> Self {
        // For now, cursors are just the stringified numbers
        // TODO make cursors more complex to seem more official
        // https://relay.dev/graphql/connections.htm#sec-Cursor
        Self((offset + index).to_string())
    }
}

/// GQL type to display information about a page of data. See the Relay
/// Connections spec: https://facebook.github.io/relay/graphql/connections.htm#sec-undefined.PageInfo
#[derive(Clone, Debug, PartialEq)]
pub struct PageInfo {
    pub offset: usize,
    pub page_len: usize,
    pub has_previous_page: bool,
    pub has_next_page: bool,
}

impl PageInfoFields for PageInfo {
    /// The spec says that the start and end cursors must be non-null, but that
    /// doesn't make sense because if the page is empty, then there is no
    /// possible value for either. So those fields should only be `None` iff
    /// the page is empty.
    fn field_start_cursor(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> Option<Cursor> {
        if self.page_len > 0 {
            Some(Cursor::from_offset_index(self.offset, 0))
        } else {
            None
        }
    }

    /// See start_cursor resolver above for why this is an option
    fn field_end_cursor(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> Option<Cursor> {
        if self.page_len > 0 {
            Some(Cursor::from_offset_index(self.offset, self.page_len - 1))
        } else {
            None
        }
    }

    fn field_has_previous_page(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> bool {
        self.has_previous_page
    }

    fn field_has_next_page(
        &self,
        _executor: &Executor<'_, '_, RequestContext>,
    ) -> bool {
        self.has_next_page
    }
}
