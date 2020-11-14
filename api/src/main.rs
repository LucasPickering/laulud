use tide::{prelude::*, Request};

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/api").get(|_| async { Ok("Hello, world!") });
    app.listen("api:8000").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(
        format!("Hello, {}! I've put in an order for {} shoes", name, legs)
            .into(),
    )
}
