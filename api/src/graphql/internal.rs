//! Types that are internal to the API (but not necessarily to this module).
//! These types are used for the API implementation but shouldn't be exposed in
//! the external GraphQL API.
//!
//! This also holds implementations (both plain and trait implementations).

use crate::{
    auth::UserId,
    error::{ApiResult, InputValidationError, ParseError},
    graphql::{RequestContext, TagNode, TaggedItemNode},
};
use async_graphql::{scalar, Context, Interface};
use derive_more::Display;
use mongodb::bson::Bson;
use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};

// region: Pagination

/// A vaildated version of [Cursor]. A cursor is just some offset value
/// converted into a string, so this struct represents the unconverted version.
/// This makes it easy to pass around the raw offset internally while also
/// forcing us to validate user-provided cursors before using them. Best
/// practice is to immediately convert any user-provided [Cursor] to a
/// [ValidCursor], then pass around the [ValidCursor] internally as needed.
/// To convert back to a [Cursor], use the [From] implementation.
#[derive(Copy, Clone, Debug, Display, Serialize, Deserialize)]
// TODO use a fancy encoding here like hex or base64 to look cool
#[display(fmt="{}", self.offset)]
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
    ///
    /// TODO replace this with a custom validator
    /// https://async-graphql.github.io/async-graphql/en/input_value_validators.html#custom-validator
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

// region: URI

// endregion

// region: Node

#[derive(Clone, Debug, Interface)]
#[graphql(field(name = "id", type = "async_graphql::ID"))]
pub enum Node {
    TaggedItemNode(TaggedItemNode),
    TagNode(TagNode),
}

impl Node {
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
    pub async fn get_id(
        &self,
        context: &Context<'_>,
    ) -> ApiResult<async_graphql::ID> {
        let node_type = self.node_type();
        let value_id = match self {
            Self::TaggedItemNode(node) => {
                node.item.uri(context).await?.to_string()
            }
            Self::TagNode(node) => node.tag.tag().to_string(),
        };

        let user_id = &context.data::<RequestContext>()?.user_id;
        Ok(async_graphql::ID(format!(
            "{}_{}_{}",
            node_type, value_id, user_id
        )))
    }

    /// Map this node to its simplified [NodeType]. [NodeType] has all the same
    /// variants as the `Node` enum, but doesn't hold any values.
    /// TODO use a generated enum from strum instead
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
    /// Parse a GraphQL node ID into its components. See [Node::id] for a
    /// description of the ID format.
    pub fn parse_id(
        id: &async_graphql::ID,
    ) -> Result<(Self, String, UserId), ParseError> {
        match id.split('_').collect::<Vec<&str>>().as_slice() {
            [node_type, value_id, user_id] => Ok((
                node_type.parse()?,
                (*value_id).to_owned(),
                UserId((*user_id).to_owned()),
            )),
            _ => Err(ParseError {
                message: "Invalid GraphQL node ID".into(),
                value: id.to_string(),
            }),
        }
    }
}

impl FromStr for NodeType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "TaggedItemNode" => Ok(NodeType::TaggedItemNode),
            "TagNode" => Ok(NodeType::TagNode),
            _ => Err(ParseError {
                message: "Unknown GraphQL node type".into(),
                value: s.into(),
            }),
        }
    }
}
// endregion

// region: Edge

/// Helper type to handle GQL edge types. Edges consist of a cursor, to locate
/// the edge within a Connection, and an associated node.
#[derive(Clone, Debug)]
pub struct GenericEdge<N> {
    pub node: N,
    pub cursor: Cursor,
}

impl<N> GenericEdge<N> {
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
                cursor: Cursor::from_offset_index(offset, index).into(),
            })
            .collect()
    }
}

// A conversion for mapping single nodes into edges. This is useful in mutations
// which usually return edges
impl<N> From<N> for GenericEdge<N> {
    fn from(node: N) -> Self {
        Self {
            node,
            // Use a bullshit cursor, this seems to work so ¯\_(ツ)_/¯
            cursor: Cursor::from_offset_index(0, 0).into(),
        }
    }
}
// endregion

/// A validated version of [Tag]. This can only be constructed via the
/// [Validate] trait, so any instance of this struct is guaranteed to be valid
#[derive(Clone, Debug, Display, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Tag(String);

scalar!(Tag);

impl Tag {
    pub fn new(tag: String) -> Self {
        Self(tag)
    }

    pub fn tag(&self) -> &str {
        &self.0
    }
}

impl FromStr for Tag {
    type Err = ParseError;

    /// Make sure the tag is non-empty
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            Err(ParseError {
                message: "Tag cannot be empty".into(),
                value: value.into(),
            })
        } else {
            Ok(Tag::new(value.into()))
        }
    }
}

// For DB interactions
impl From<&Tag> for Bson {
    fn from(uri: &Tag) -> Self {
        uri.to_string().into()
    }
}

// These two impls needed for serde
impl From<Tag> for String {
    fn from(tag: Tag) -> Self {
        tag.to_string()
    }
}
impl TryFrom<String> for Tag {
    type Error = <Tag as FromStr>::Err;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
