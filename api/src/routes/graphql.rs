use std::sync::Arc;

use juniper::http::GraphQLRequest;
use rocket::{
    http::{ContentType, Status},
    response::content::Html,
    Response, State,
};
use rocket_contrib::json::Json;

use crate::{
    db::DbHandler,
    error::ApiResult,
    graphql::{GraphQLSchema, RequestContext},
    spotify::Spotify,
    util::UserId,
};

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
    db_handler: State<'_, Arc<DbHandler>>,
    spotify: Spotify,
    user_id: UserId,
    graphql_schema: State<'_, Arc<GraphQLSchema>>,
    graphql_request: Json<GraphQLRequest>,
) -> ApiResult<Response<'static>> {
    let context = RequestContext {
        db_handler: Arc::clone(db_handler.inner()),
        spotify,
        user_id,
    };

    let graphql_response =
        graphql_request.execute(&graphql_schema, &context).await;
    // TODO no unwrap
    let body = serde_json::to_string(&graphql_response).unwrap();
    Ok(Response::build()
        .status(Status::Ok)
        .header(ContentType::JSON)
        .sized_body(None, std::io::Cursor::new(body))
        .finalize())
}

#[rocket::get("/graphiql")]
pub async fn route_graphiql() -> Html<String> {
    let html = juniper::http::graphiql::graphiql_source("/api/graphql", None);
    Html(html)
}
