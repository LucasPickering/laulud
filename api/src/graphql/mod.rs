//! The GraphQL server implementation. This module holds all types that are
//! exported from the API, and all implementation of the server logic.
//!
//! Generally in these implementations, the approach we take is to defer as much
//! work as possible down the resolver tree, so that we only do stuff when it's
//! really really necessary. This includes simple stuff like wrapper structs,
//! meaning we generally store the simplest version of data possible and derive
//! new structs from that at resolution time. Similarly, Spotify API requests
//! and DB queries are deferred as much as possible.
//!
//! This is not always ideal because it leads to N+1s, so at some point we
//! should use [juniper-eager-loading](https://github.com/davidpdrsn/juniper-eager-loading)
//! to prefetch data when we can, but that's a problem for another day.

mod core;
mod internal;
mod item;
mod mutation;
mod query;
mod schema;
mod spotify;
mod tag;

pub use crate::graphql::{
    core::*, internal::*, item::*, mutation::*, query::*, schema::*,
    spotify::*, tag::*,
};

use crate::{db::DbHandler, error::ApiError, spotify::Spotify, util::UserId};
use juniper::EmptySubscription;
use mongodb::bson::doc;
use std::sync::Arc;

// This file holds GraphQL setup/implementation details, but no external GraphQL
// types

pub struct RequestContext {
    pub db_handler: Arc<DbHandler>,
    pub spotify: Spotify,
    pub user_id: UserId,
}
impl juniper::Context for RequestContext {}

pub type GraphQLSchema = juniper::RootNode<
    'static,
    Query,
    Mutation,
    EmptySubscription<RequestContext>,
>;

pub fn create_graphql_schema() -> GraphQLSchema {
    GraphQLSchema::new(Query, Mutation, EmptySubscription::new())
}
