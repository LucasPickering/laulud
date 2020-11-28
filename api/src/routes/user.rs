use crate::{error::ApiResult, schema::CurrentUser, spotify::Spotify};
use rocket::get;
use rocket_contrib::json::Json;

#[get("/users/current")]
pub async fn route_get_current_user(
    mut spotify: Spotify,
) -> ApiResult<Json<CurrentUser>> {
    Ok(Json(spotify.get_current_user().await?))
}
