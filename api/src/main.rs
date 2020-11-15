mod config;

use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Postgres};
use tide::Request;

use crate::config::LauludConfig;

pub type Pool = sqlx::Pool<Postgres>;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[derive(Clone)]
struct State {
    pool: Pool,
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    let config = LauludConfig::load().unwrap();

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    let mut app = tide::with_state(State { pool });
    app.at("/api").get(|_| async { Ok("Hello, world!") });
    app.listen(config.server_host.as_str()).await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(
        format!("Hello, {}! I've put in an order for {} shoes", name, legs)
            .into(),
    )
}
