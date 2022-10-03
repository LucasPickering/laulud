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
mod tag;

pub use crate::graphql::{
    core::*, internal::*, item::*, mutation::*, query::*, tag::*,
};
use crate::{auth::UserId, db::DbHandler, error::ApiResult, spotify::Spotify};
use async_graphql::{EmptySubscription, Schema};
use log::info;
use std::{fmt::Display, path::Path, sync::Arc};
use tokio::{fs::File, io::AsyncWriteExt};

// This file holds GraphQL setup/implementation details, but no external GraphQL
// types

/// All the external context that a resolver might need. Async-graphql supports
/// passing multiple context values so technically this isn't needed, but since
/// there's no static typing when grabbing context from async-graphql, we use
/// this wrapping type to ensure that changes to the available context will be
/// caught by static typing.
pub struct RequestContext {
    pub db_handler: Arc<DbHandler>,
    pub spotify: Spotify,
    pub user_id: UserId,
}

pub type GraphQLSchema = Schema<Query, Mutation, EmptySubscription>;

/// Create GraphQL schema object and export it to an external file
pub async fn create_graphql_schema(
    export_path: impl AsRef<Path> + Display,
) -> ApiResult<GraphQLSchema> {
    let schema =
        GraphQLSchema::build(Query, Mutation, EmptySubscription).finish();
    let mut file = File::create(export_path.as_ref()).await?;
    file.write_all(schema.sdl().as_bytes()).await?;
    info!("Wrote schema to {}", export_path);
    Ok(schema)
}
