use crate::State;
use serde::{Deserialize, Serialize};
use tide::{Body, Request};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Track {
    track_id: String,
    tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateTagBody {
    tags: Vec<String>,
}

pub async fn route_create_tag(mut req: Request<State>) -> tide::Result<Body> {
    let CreateTagBody { tags } = req.body_json().await?;
    let track = Track {
        track_id: req.param("track_id")?.into(),
        tags,
    };
    Ok(Body::from_json(&track)?)
}
