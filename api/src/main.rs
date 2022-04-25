#![feature(backtrace)]

mod db;
mod error;
mod graphql;
mod routes;
mod spotify;
mod util;

use crate::db::DbHandler;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl,
};
use rocket::routes;
use serde::Deserialize;
use std::sync::Arc;

/// App-wide configuration settings
#[derive(Debug, Deserialize)]
pub struct LauludConfig {
    /// The URL of the DB that we connect to, as a Mongo URI.
    /// https://docs.mongodb.com/manual/reference/connection-string/
    pub database_url: String,
    /// The host server, for use with the OAuth flow
    pub hostname: String,
    /// ID for our Spotify app
    pub spotify_client_id: String,
    /// Secret for our Spotify app
    pub spotify_client_secret: String,
}

/// Initialize the Spotify OAuth client. Any failures in here will cause a
/// panic, so this should only be called during server startup.
pub async fn init_spotify_client(config: &LauludConfig) -> BasicClient {
    // Create an OAuth2 client by specifying the client ID, client
    // secret, authorization URL and token URL.
    BasicClient::new(
        ClientId::new(config.spotify_client_id.to_string()),
        Some(ClientSecret::new(config.spotify_client_secret.to_string())),
        AuthUrl::new("https://accounts.spotify.com/authorize".to_string())
            .unwrap(),
        Some(
            TokenUrl::new("https://accounts.spotify.com/api/token".to_string())
                .unwrap(),
        ),
    )
    // Set the URL the user will be redirected to after the authorization
    // process.
    .set_redirect_uri(
        RedirectUrl::new(format!("{}/api/oauth/callback", config.hostname))
            .unwrap(),
    )
}

#[rocket::main]
async fn main() {
    env_logger::init();
    let rocket = rocket::build();
    let config: LauludConfig = rocket.figment().extract().unwrap();

    let db_handler = DbHandler::connect(&config).await.unwrap();
    let spotify_oauth_client = init_spotify_client(&config).await;
    let graphql_schema = graphql::create_graphql_schema();

    rocket
        .mount(
            "/api",
            routes![
                // auth
                routes::auth::route_auth_redirect,
                routes::auth::route_auth_callback,
                routes::auth::route_auth_check,
                routes::auth::route_logout,
                // graphql
                routes::graphql::route_graphql,
                routes::graphql::route_graphiql,
            ],
        )
        .manage(Arc::new(db_handler))
        .manage(Arc::new(spotify_oauth_client))
        .manage(Arc::new(graphql_schema))
        .launch()
        .await
        .unwrap();
}
