mod config;
mod routes;

use crate::config::LauludConfig;
use mongodb::{options::ClientOptions, Client};

#[rocket::main]
async fn main() {
    let config = LauludConfig::load().unwrap();

    let db_options = ClientOptions::parse(&config.database_url).await.unwrap();
    let db_client = Client::with_options(db_options).unwrap();

    rocket::ignite()
        .mount("/api", routes::all_routes())
        .manage(db_client)
        .launch()
        .await
        .unwrap();
}
