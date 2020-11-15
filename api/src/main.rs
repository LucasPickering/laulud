mod db;
mod error;
mod routes;
mod util;

use crate::db::DbHandler;
use rspotify::{client::Spotify, oauth2::SpotifyClientCredentials};
use serde::Deserialize;

/// App-wide configuration settings
#[derive(Debug, Deserialize)]
pub struct LauludConfig {
    /// The URL of the DB that we connect to, as a Mongo URI.
    /// https://docs.mongodb.com/manual/reference/connection-string/
    pub database_url: String,

    /// ID for our Spotify app
    pub spotify_client_id: String,
    /// Secret for our Spotify app
    pub spotify_client_secret: String,
}

#[rocket::main]
async fn main() {
    env_logger::init();
    let rocket = rocket::ignite();

    // Load custom config and set up the DB connection
    let config: LauludConfig = rocket.figment().extract().unwrap();
    let db_handler = DbHandler::connect(&config).await.unwrap();
    let spotify_creds = SpotifyClientCredentials::default()
        .client_id(&config.spotify_client_id)
        .client_secret(&config.spotify_client_secret)
        .build();
    let spotify = Spotify::default()
        .client_credentials_manager(spotify_creds)
        .build();

    rocket
        .mount("/api", routes::all_routes())
        .manage(db_handler)
        .manage(spotify)
        .launch()
        .await
        .unwrap();
}
