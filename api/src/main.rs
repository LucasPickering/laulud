mod config;
mod db;
mod routes;

use crate::{config::LauludConfig, db::DbHandler};

#[rocket::main]
async fn main() {
    let config = LauludConfig::load().unwrap();

    let db_handler = DbHandler::connect(&config).await.unwrap();

    rocket::ignite()
        .mount("/api", routes::all_routes())
        .manage(db_handler)
        .launch()
        .await
        .unwrap();
}
