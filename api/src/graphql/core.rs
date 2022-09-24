//! Basic, generic GraphQL types, that aren't specific to any part of the API or
//! any particular data type

use crate::graphql::{
    internal::Cursor, Cursor, PageInfoFields, RequestContext,
};

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
            Some(Cursor::from_offset_index(self.offset, 0).into())
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
            Some(
                Cursor::from_offset_index(self.offset, self.page_len - 1)
                    .into(),
            )
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
