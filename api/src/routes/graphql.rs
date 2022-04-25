use crate::{
    db::DbHandler,
    error::{ApiError, ApiResult},
    graphql::{GraphQLSchema, RequestContext},
    spotify::Spotify,
    util::UserId,
};
use juniper::http::GraphQLRequest;
use rocket::{
    response::content::{self, Html},
    serde::json::Json,
    State,
};
use std::{backtrace::Backtrace, sync::Arc};

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
    graphql_request: Json<GraphQLRequest>,
) -> ApiResult<content::Json<String>> {
    let context = RequestContext {
        db_handler: Arc::clone(db_handler.inner()),
        spotify,
        user_id,
    };

    let graphql_response =
        graphql_request.execute(graphql_schema, &context).await;
    // Serialization should never fail
    let body = serde_json::to_string(&graphql_response).map_err(|err| {
        ApiError::Unknown {
            message: err.to_string(),
            backtrace: Backtrace::capture(),
        }
    })?;
    Ok(content::Json(body))
}

#[rocket::get("/graphiql")]
pub async fn route_graphiql() -> Html<String> {
    let html = juniper::http::graphiql::graphiql_source("/api/graphql", None);
    Html(html)
}
