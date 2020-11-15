mod db;
mod error;
mod routes;

use crate::db::DbHandler;
use serde::Deserialize;

/// App-wide configuration settings
#[derive(Debug, Deserialize)]
pub struct LauludConfig {
    /// The URL of the DB that we connect to, as a Mongo URI.
    /// https://docs.mongodb.com/manual/reference/connection-string/
    pub database_url: String,
}

#[rocket::main]
async fn main() {
    let rocket = rocket::ignite();

    // Load custom config and set up the DB connection
    let config: LauludConfig = rocket.figment().extract().unwrap();
    let db_handler = DbHandler::connect(&config).await.unwrap();

    rocket
        .mount("/api", routes::all_routes())
        .manage(db_handler)
        .launch()
        .await
        .unwrap();
}
