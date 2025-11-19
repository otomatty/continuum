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
use crate::auth::session::{Session, create_session_cookie, create_logout_cookie};
use crate::state::AppState;

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

async fn login(State(state): State<AppState>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = state
        .auth_state
        .client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_string()))
        .add_scope(Scope::new("read:org".to_string()))
        .add_scope(Scope::new("repo".to_string()))
        .url();

    Redirect::to(auth_url.as_str())
}

async fn callback(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    Query(query): Query<AuthRequest>,
) -> impl IntoResponse {
    let token_result = state
        .auth_state
        .client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await;

    match token_result {
        Ok(token) => {
            let access_token = token.access_token().secret();
            
            // Fetch user info to get user ID/login
            let client = reqwest::Client::new();
            let user_res = client
                .get("https://api.github.com/user")
                .header("Authorization", format!("Bearer {}", access_token))
                .header("User-Agent", "continuum")
                .send()
                .await;

            match user_res {
                Ok(res) => {
                    if let Ok(user_data) = res.json::<serde_json::Value>().await {
                        if let Some(login) = user_data.get("login").and_then(|v| v.as_str()) {
                            let session = Session::new(
                                login.to_string(),
                                access_token.clone(),
                                24 * 60 * 60, // 1 day
                            );
                            
                            let cookie = create_session_cookie(&session);
                            return (jar.add(cookie), Redirect::to("/")).into_response();
                        }
                    }
                }
                Err(_) => {}
            }
            
            Redirect::to("/?error=user_fetch_failed").into_response()
        }
        Err(_) => Redirect::to("/?error=token_exchange_failed").into_response(),
    }
}

async fn logout(
    State(_state): State<AppState>,
    jar: PrivateCookieJar,
) -> impl IntoResponse {
    (jar.add(create_logout_cookie()), Redirect::to("/"))
}
