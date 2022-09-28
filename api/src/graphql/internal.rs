//! Types that are internal to the API (but not necessarily to this module).
//! These types are used for the API implementation but shouldn't be exposed in
//! the external GraphQL API.
//!
//! This also holds implementations (both plain and trait implementations).

use crate::{
    auth::UserId,
    error::{ApiResult, InputValidationError, ParseError},
    graphql::{Cursor, TagNode, TaggedItemNode},
};
use async_graphql::Interface;
use derive_more::Display;
use std::convert::TryInto;
use strum::{EnumDiscriminants, EnumString};

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
                        value: first,
                    })?;
                Some(limit)
            }
            None => None,
        };

        // Parse `after` as a cursor then convert to a number
        let offset: Option<usize> = after.map(|cursor| cursor.offset() + 1);

        Ok(Self { limit, offset })
    }

    pub fn limit(&self) -> Option<usize> {
        self.limit
    }

    pub fn offset(&self) -> Option<usize> {
        self.offset
    }
}

#[derive(Clone, Debug, EnumDiscriminants, Interface)]
#[graphql(field(name = "id", type = "async_graphql::ID"))]
#[strum_discriminants(name(NodeType))]
#[strum_discriminants(derive(Display, EnumString))] // Add FromStr impl for discriminants
#[allow(clippy::large_enum_variant)] // tough shit clippy, get over it
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
    ///
    /// Underscored name is needed to disambiguate from the equivalent GraphQL
    /// resolver on the interface.
    pub fn id_(&self, user_id: &UserId) -> async_graphql::ID {
        let value_id: &str = match self {
            Self::TaggedItemNode(node) => node.item.uri_().id(),
            Self::TagNode(node) => node.tag.tag(),
        };

        async_graphql::ID(format!(
            "{}_{}_{}",
            self.node_type(),
            value_id,
            user_id
        ))
    }

    /// Get the static type of a node
    pub fn node_type(&self) -> NodeType {
        self.into()
    }

    /// Parse a GraphQL node ID into its components. See [Self::id_] for a
    /// description of the ID format.
    pub fn parse_id(
        id: &async_graphql::ID,
    ) -> ApiResult<(NodeType, String, UserId)> {
        match id.split('_').collect::<Vec<&str>>().as_slice() {
            [node_type, value_id, user_id] => Ok((
                node_type.parse().map_err(|_| ParseError {
                    message: "Invalid GraphQL node type".into(),
                    value: node_type.to_string(),
                })?,
                (*value_id).to_owned(),
                UserId((*user_id).to_owned()),
            )),
            _ => Err(ParseError {
                message: "Invalid GraphQL node ID".into(),
                value: id.to_string(),
            }
            .into()),
        }
    }
}

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
                cursor: Cursor::from_offset_index(offset, index),
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
            cursor: Cursor::from_offset_index(0, 0),
        }
    }
}
