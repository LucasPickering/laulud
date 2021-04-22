//! Types that are internal to the `graphql` module. These types are used for
//! the API implementation but shouldn't be shared outside the module or exposed
//! in the GraphQL API.
//!
//! This also holds implementations (both plain and trait implementations) on
//! GraphQL types, because the implementations are only used within this module.

use crate::{
    error::{ApiError, ApiResult, InputValidationError},
    graphql::{Cursor, Item, SpotifyUri},
    util::UserId,
};
use derive_more::Display;
use std::{backtrace::Backtrace, convert::TryInto, str::FromStr};

// region: Pagination

/// A vaildated version of [Cursor]. A cursor is just some offset value
/// converted into a string, so this struct represents the unconverted version.
/// This makes it easy to pass around the raw offset internally while also
/// forcing us to validate user-provided cursors before using them. Best
/// practice is to immediately convert any user-provided [Cursor] to a
/// [ValidCursor], then pass around the [ValidCursor] internally as needed.
/// To convert back to a [Cursor], use the [From] implementation.
#[derive(Clone, Debug)]
pub struct ValidCursor {
    offset: usize,
}

impl ValidCursor {
    /// Get the pre-parsed offset for this cursor. A cursor is just an
    /// obfuscated number that indicates the element's offset into the
    /// collection. These offsets can be used with Spotify or Mongo to find
    /// the element.
    pub fn offset(&self) -> usize {
        self.offset
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

impl Validate for Cursor {
    type Output = ValidCursor;

    fn validate(
        self,
        field: &str,
    ) -> Result<Self::Output, InputValidationError> {
        // Parse the string as a plain number
        let offset =
            self.0.parse::<usize>().map_err(|_| InputValidationError {
                // That's the big NG
                field: field.into(),
                message: "Invalid pagination cursor".into(),
                value: self.0.into(),
            })?;
        Ok(ValidCursor { offset })
    }
}

// Convert this internal-only cursor into a stringified cursor that can be
// returned to the user
impl From<ValidCursor> for Cursor {
    fn from(valid_cursor: ValidCursor) -> Self {
        // Right now we just stringify the offset
        // TODO use a fancy encoding here like hex or base64 to look cool
        Self(valid_cursor.offset.to_string())
    }
}

/// A parsed version of user pagination params, to make it easy to paginate
/// through Mongo or Spotify data. This struct is guaranteed to hold valid
/// values, so it can be passed around internally. To map from user pagination
/// input into this struct, use [Self::try_from_first_after].
#[derive(Clone, Debug)]
pub struct LimitOffset {
    limit: Option<usize>,
    offset: Option<usize>,
}

impl LimitOffset {
    /// Map from a user's `first` and `after` pagination params to limit/offset
    /// values that we can use internally with Spotify and Mongo. If either of
    /// the input values are invalid, an error will be returned here so it can
    /// be propagated to the user.
    ///
    /// See the [GraphQL spec](https://relay.dev/graphql/connections.htm) for
    /// more info on first/after.
    pub fn try_from_first_after(
        first: Option<i32>,
        after: Option<Cursor>,
    ) -> Result<Self, InputValidationError> {
        // Convert `first` to a usize
        let limit: Option<usize> = match first {
            Some(first) => {
                let limit: usize =
                    first.try_into().map_err(|_| InputValidationError {
                        field: "first".into(),
                        message:
                            "Invalid quantity, must be non-negative integer"
                                .into(),
                        value: first.into(),
                    })?;
                Some(limit)
            }
            None => None,
        };

        // Parse `after` as a cursor then convert to a number
        let offset: Option<usize> = match after {
            Some(cursor) => {
                let cursor = cursor.validate("after")?;
                // We want to include the first element _after_ the cursor, so
                // add 1 to the offset. E.g. if we request `after: "cursor-0"`,
                // then the first element we want to show is #1, so offset
                // should be 1
                Some(cursor.offset() + 1)
            }
            None => None,
        };

        Ok(Self { limit, offset })
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit
    }

    pub fn offset(&self) -> Option<usize> {
        self.offset
    }
}

// endregion

// region: Node

// For some reason, rust-analyzer throws an error when you try to import `Node`,
// so we use the qualified path here for dev QoL
impl crate::graphql::Node {
    /// Get a unique ID for this node. Because Relay requires a top-level
    /// query field `node` that can take in a node ID of _any_ type and
    /// return the corresponding node, each ID has to transparently indicate
    /// which type it maps to, so that we know how to retrieve the node just
    /// via the ID. Different nodes come from different data sources, so we
    /// have to include the type name in the ID.
    ///
    /// IDs use the format `<node_type>_<value_id>_<user_id>`, where `value_id`
    /// is some string that unique indentifies the node **within its type**.
    /// For example, for an item node it could be the URI.
    pub fn id(&self, user_id: &UserId) -> juniper::ID {
        let node_type = self.node_type();
        let value_id = match self {
            Self::TaggedItemNode(node) => node.item.uri(),
            Self::TagNode(node) => &node.tag,
        };

        juniper::ID::new(format!("{}_{}_{}", node_type, value_id, user_id))
    }

    /// Map this node to its simplified [NodeType]. [NodeType] has all the same
    /// variants as the `Node` enum, but doesn't hold any values.
    pub fn node_type(&self) -> NodeType {
        match self {
            Self::TaggedItemNode(_) => NodeType::TaggedItemNode,
            Self::TagNode(_) => NodeType::TagNode,
        }
    }
}

/// A discriminants-only version of the [Node] enum that's generated by
/// `juniper-from-schema`. This contains one variant for every node type defined
/// in the API, and is used to generate and parse node IDs.
///
/// TODO replace with strum's `EnumDiscriminants` derive after https://github.com/davidpdrsn/juniper-from-schema/issues/139
#[derive(Copy, Clone, Debug, Display)]
pub enum NodeType {
    TaggedItemNode,
    TagNode,
}

impl NodeType {
    /// Parse a GraphQL node ID into its components. See
    /// [Node::id](crate::graphql::Node::id) for a description of the ID format.
    pub fn parse_id(id: &juniper::ID) -> ApiResult<(Self, String, UserId)> {
        match id.split('_').collect::<Vec<&str>>().as_slice() {
            [node_type, value_id, user_id] => Ok((
                node_type.parse()?,
                (*value_id).to_owned(),
                UserId((*user_id).to_owned()),
            )),
            _ => Err(ApiError::ParseError {
                message: format!("Invalid GraphQL node ID: {}", id),
                backtrace: Backtrace::capture(),
            }),
        }
    }
}

impl FromStr for NodeType {
    type Err = ApiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TaggedItemNode" => Ok(NodeType::TaggedItemNode),
            "TagNode" => Ok(NodeType::TagNode),
            _ => Err(ApiError::ParseError {
                message: format!("Unknown GraphQL node type: {}", s),
                backtrace: Backtrace::capture(),
            }),
        }
    }
}
// endregion

// region: Item
impl Item {
    /// Get the URI for this item
    pub fn uri(&self) -> &SpotifyUri {
        match self {
            Self::Track(track) => &track.uri,
            Self::AlbumSimplified(album) => &album.uri,
            Self::Artist(artist) => &artist.uri,
        }
    }
}

// TODO replace with derive after https://github.com/davidpdrsn/juniper-from-schema/issues/139
impl Clone for Item {
    fn clone(&self) -> Self {
        match self {
            Self::Track(track) => Self::Track(track.clone()),
            Self::AlbumSimplified(album) => {
                Self::AlbumSimplified(album.clone())
            }
            Self::Artist(artist) => Self::Artist(artist.clone()),
        }
    }
}
// endregion

// region: Edge

/// Helper type to handle GQL edge types. Edges consist of a cursor, to locate
/// the edge within a Connection, and an associated node.
pub struct GenericEdge<N> {
    node: N,
    // If we end up needing to use this value as an offset, we can replace this
    // with `ValidCursor`, but using `Cursor` here seems to be simpler for now
    cursor: Cursor,
}

impl<N> GenericEdge<N> {
    pub fn node(&self) -> &N {
        &self.node
    }

    pub fn cursor(&self) -> &Cursor {
        &self.cursor
    }

    /// Convert a list of nodes into edges. The edges will keep the same
    /// ordering, and each edge will be generated a cursor based on the given
    /// offset plus that edge's location in the list.
    pub fn from_nodes(
        rows: impl Iterator<Item = N>,
        offset: usize,
    ) -> Vec<Self> {
        rows.enumerate()
            .map(|(index, node)| Self {
                node,
                cursor: ValidCursor::from_offset_index(offset, index).into(),
            })
            .collect()
    }
}
// endregion

// region: Validation

/// A trait that denotes that this type represents some user input that needs
/// to be validated, and in the process of validation will be converted to
/// some other output type. Typically, any type that implements this trait is
/// **always** considered invalid, and should generally be validated as soon as
/// possible in the API chain. Then, the validated version of the type can be
/// used internally and we know for sure that the value is valid.
///
/// Valid structs should _only_ be constructable via this trait implementation,
/// to prevent sidestepping validation by directly creating the "valid" value.
pub trait Validate: Sized {
    type Output;

    /// Validate the user-provided value. If it's valid, return the validated
    /// form of it. If not, return a validation error.
    ///
    /// `field` represents the GraphQL field that is being validated. This is
    /// used in the event of an error, to tell the user which field was invalid.
    fn validate(
        self,
        field: &str,
    ) -> Result<Self::Output, InputValidationError>;
}

// endregion
