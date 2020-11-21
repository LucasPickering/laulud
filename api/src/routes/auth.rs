use crate::{
    error::ApiResult,
    util::{IdentityState, OAUTH_COOKIE_NAME},
};
use oauth2::{basic::BasicClient, AuthorizationCode, CsrfToken};
use rocket::{
    get,
    http::{Cookie, CookieJar, Status},
    post,
    response::Redirect,
    State,
};
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::sync::Arc;

/// The frontend will redirect to this before being sent off to the
/// actual openid provider
#[get("/oauth/redirect?<next>")]
pub async fn route_auth_redirect(
    oauth_client: State<'_, Arc<BasicClient>>,
    cookies: &CookieJar<'_>,
    next: Option<String>,
) -> ApiResult<Redirect> {
    let (auth_url, csrf_token) =
        oauth_client.authorize_url(CsrfToken::new_random).url();

    // Encode the CSRF token and some extra data, then store that in a
    // signed+encrypted cookie. We'll read the CSRF token from there in the
    // callback and compare to what we get from the URL. Since the cookie is
    // signed, this prevents CSRF attacks.
    let state = IdentityState::DuringAuth { next, csrf_token };
    cookies.add_private(state.to_cookie());

    Ok(Redirect::found(auth_url.to_string()))
}

#[derive(Deserialize, Debug)]
pub struct LoginQuery {
    code: String,
    state: Option<String>,
    nonce: Option<String>,
}

/// Provider redirects back to this route after the login
#[get("/oauth/callback?<code>&<state>")]
pub async fn route_auth_callback(
    oauth_client: State<'_, Arc<BasicClient>>,
    cookies: &CookieJar<'_>,
    code: Option<String>,
    state: Option<String>,
) -> ApiResult<Redirect> {
    // Read identity/state data that stored in an encrypted+signed cookie.
    // We know this data is safe, we wrote it and it hasn't been
    // modified.
    let identity_state = IdentityState::from_cookies(cookies)?;

    // VERY IMPORTANT - read the CSRF token from the state param, and
    // compare it to the token we stored in the cookie. The cookie
    // is encrypted+signed, Parse the state param and validate the
    // CSRF token in there
    identity_state.verify_csrf(state.as_deref().unwrap_or(""))?;

    // Exchange the temp code for a token
    let token_response = oauth_client
        .exchange_code(AuthorizationCode::new(code.unwrap_or_else(String::new)))
        .request_async(oauth2::reqwest::async_http_client)
        .await?;

    // Replace the auth state cookie with one for permanenet auth. We use
    // the UserProvider ID so that this works even if the User
    // object hasn't been created yet.
    let new_identity_state = IdentityState::PostAuth(token_response.into());
    cookies.add_private(new_identity_state.to_cookie());

    // Redirect to the path specified in the state cookie
    Ok(Redirect::found(identity_state.next().to_owned()))
}

#[post("/logout")]
pub async fn route_logout(cookies: &CookieJar<'_>) -> Json<()> {
    cookies.remove_private(Cookie::named(OAUTH_COOKIE_NAME));
    Json(())
}
