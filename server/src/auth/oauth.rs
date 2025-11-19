use axum::{
    extract::{Query, State},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::cookie::PrivateCookieJar;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use crate::config::Config;
use crate::auth::session::{Session, create_session_cookie, create_logout_cookie, create_csrf_cookie};
use crate::state::AppState;
use leptos::logging::log;
use axum_extra::extract::cookie::Cookie;

#[derive(Clone)]
pub struct AuthState {
    pub client: BasicClient,
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/auth/login", get(login))
        .route("/auth/callback", get(callback))
        .route("/auth/logout", get(logout))
}

pub fn create_auth_state(config: &Config) -> AuthState {
    let client_id = ClientId::new(config.github.client_id.clone());
    let client_secret = ClientSecret::new(config.github.client_secret.clone());
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    let client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(
        RedirectUrl::new(config.github.callback_url.clone()).expect("Invalid redirect URL"),
    );

    AuthState { client }
}

async fn login(State(state): State<AppState>, jar: PrivateCookieJar) -> impl IntoResponse {
    let (auth_url, csrf_token) = state
        .auth_state
        .client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("read:org".to_string()))
        .add_scope(Scope::new("repo".to_string()))
        .url();

    let secure = state.config.server.env != "DEV";
    let csrf_cookie = create_csrf_cookie(csrf_token.secret().to_string(), secure);
    
    (jar.add(csrf_cookie), Redirect::to(auth_url.as_str()))
}

async fn callback(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Query(query): Query<AuthRequest>,
) -> impl IntoResponse {
    // CSRF protection: verify state parameter matches cookie
    let csrf_state_from_cookie = jar.get("oauth_csrf_state")
        .map(|c| c.value().to_string());
    
    // Remove CSRF cookie after reading (one-time use)
    let jar = jar.remove(Cookie::from("oauth_csrf_state"));
    
    if csrf_state_from_cookie.as_deref() != Some(&query.state) {
        log!("CSRF token mismatch in OAuth callback");
        return (jar, Redirect::to("/?error=csrf_mismatch")).into_response();
    }

    let token_result = state
        .auth_state
        .client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret();
            
            // Fetch user info to get user ID/login using shared HTTP client
            let user_res = state.http_client
                .get("https://api.github.com/user")
                .header("Authorization", format!("Bearer {}", access_token))
                .header("User-Agent", "continuum")
                .send()
                .await;

            match user_res {
                Ok(res) => {
                    if let Ok(user_data) = res.json::<serde_json::Value>().await {
                        if let Some(login) = user_data.get("login").and_then(|v| v.as_str()) {
                            let secure = state.config.server.env != "DEV";
                            let duration_secs = state.config.session.duration_secs as i64;
                            
                            let session = Session::new(
                                login.to_string(),
                                access_token.clone(),
                                state.config.session.duration_secs,
                            );
                            
                            match create_session_cookie(&session, secure, duration_secs) {
                                Ok(cookie) => {
                                    return (jar.add(cookie), Redirect::to("/")).into_response();
                                }
                                Err(e) => {
                                    log!("Failed to create session cookie: {}", e);
                                    return (jar, Redirect::to("/?error=session_creation_failed")).into_response();
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    log!("Failed to fetch user from GitHub: {}", e);
                }
            }
            
            (jar, Redirect::to("/?error=user_fetch_failed")).into_response()
        }
        Err(e) => {
            log!("Failed to exchange OAuth code for token: {}", e);
            (jar, Redirect::to("/?error=token_exchange_failed")).into_response()
        }
    }
}

async fn logout(State(state): State<AppState>, jar: PrivateCookieJar) -> impl IntoResponse {
    let secure = state.config.server.env != "DEV";
    (jar.add(create_logout_cookie(secure)), Redirect::to("/"))
}
