use crate::{
    error::ApiResult,
    spotify::Spotify,
    util::{AuthenticationToken, IdentityState, OAuthHandler, UserId},
};
use oauth2::{basic::BasicClient, AuthorizationCode, CsrfToken};
use rocket::{get, http::CookieJar, post, response::Redirect, State};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::RwLock;

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
    identity_state: IdentityState,
    code: Option<String>,
    state: Option<String>,
) -> ApiResult<Redirect> {
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

    // Fetch the auth user's unique ID from the Spotify API, which we'll store
    // in the cookie
    let token: AuthenticationToken = token_response.into();
    let spotify = Spotify::new(OAuthHandler {
        client: oauth_client.inner().clone(),
        token: RwLock::new(token),
    });
    let user_id: UserId = spotify.get_current_user().await?.id.into();

    // Replace the auth state cookie with one for permanenet auth. We use
    // the UserProvider ID so that this works even if the User
    // object hasn't been created yet.
    let new_identity_state = IdentityState::PostAuth {
        // We don't need the Spotify client anymore, so take the token back
        token: spotify.into_oauth_handler().into_token(),
        user_id,
    };
    cookies.add_private(new_identity_state.to_cookie());

    // Redirect to the path specified in the state cookie
    Ok(Redirect::found(identity_state.next().to_owned()))
}

#[post("/logout")]
pub async fn route_logout(cookies: &CookieJar<'_>) {
    IdentityState::remove_cookie(cookies);
}
