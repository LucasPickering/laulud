use crate::{
    db::DbHandler,
    graphql::{GraphQLSchema, RequestContext},
    spotify::Spotify,
    util::UserId,
};
use async_graphql::http::GraphiQLSource;
use async_graphql_rocket::{GraphQLRequest, GraphQLResponse};
use rocket::{response::content::RawHtml, State};
use std::sync::Arc;

/// Route for all GraphQL requests. **All GraphQL requests require the user
/// to be logged in.** Pretty much ever API request will require accessing the
/// Spotify API, which requires login, so we just gate the entire API on login.
/// In the future, we could have a portion of the API that's public but for
/// now that's not happening.
#[rocket::post(
    "/graphql",
    format = "application/json",
    data = "<graphql_request>"
)]
pub async fn route_graphql(
    db_handler: &State<Arc<DbHandler>>,
    spotify: Spotify,
    user_id: UserId,
    graphql_schema: &State<Arc<GraphQLSchema>>,
    graphql_request: GraphQLRequest,
) -> GraphQLResponse {
    graphql_schema
        .execute(
            graphql_request
                // Use the wrapping RequestContext instead of passing a bunch of
                // little contexts, so we can get the benefits of static typing
                .data(RequestContext {
                    db_handler: Arc::clone(db_handler.inner()),
                    spotify,
                    user_id,
                }),
        )
        .await
}

#[rocket::get("/graphiql")]
pub async fn route_graphiql() -> RawHtml<String> {
    RawHtml(GraphiQLSource::build().endpoint("/api/graphql").finish())
}
