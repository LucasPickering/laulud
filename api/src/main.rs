mod config;
mod routes;

use crate::{config::LauludConfig, routes::*};
use mongodb::{options::ClientOptions, Client};
use serde::Deserialize;
use tide::Request;

struct UserTrackTags {
    user_id: String,
    track_id: String,
    tags: Vec<String>,
}

#[derive(Clone)]
pub struct State {
    db_client: Client,
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    let config = LauludConfig::load().unwrap();

    let db_options = ClientOptions::parse(&config.database_url).await.unwrap();
    let db_client = Client::with_options(db_options).unwrap();

    let mut app = tide::with_state(State { db_client });

    // Routes
    app.at("/api/tracks/:track_id/tags").post(route_create_tag);

    app.listen(config.server_host.as_str()).await?;
    Ok(())
}
