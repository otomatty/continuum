use serde::{Deserialize, Serialize};

/// Authentication status structure shared between client and server
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthStatus {
    pub authenticated: bool,
    pub user_id: Option<String>,
}

/// Session structure matching server::auth::session::Session
/// This is a duplicate definition to avoid cross-crate dependencies
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub access_token: String,
    pub expires_at: u64,
}

#[cfg(feature = "ssr")]
impl Session {
    pub fn is_expired(&self) -> bool {
        use std::time::{SystemTime, UNIX_EPOCH};
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs() > self.expires_at
    }

    pub fn from_cookie_value(value: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(value)
    }
}

#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;
#[cfg(feature = "ssr")]
use leptos::*;

// Helper function to extract auth status from cookie value (for use in shell function)
#[cfg(feature = "ssr")]
pub fn extract_auth_status_from_cookie(cookie_value: Option<&str>) -> AuthStatus {
    if let Some(value) = cookie_value {
        if let Ok(session) = Session::from_cookie_value(value) {
            if !session.is_expired() {
                return AuthStatus {
                    authenticated: true,
                    user_id: Some(session.user_id),
                };
            }
        }
    }

    AuthStatus {
        authenticated: false,
        user_id: None,
    }
}

#[server(GetAuthStatus, "/api/auth/status")]
pub async fn get_auth_status() -> Result<AuthStatus, ServerFnError> {
    // NOTE: Leptos Server Functions cannot directly access PrivateCookieJar
    // as a parameter because it's not serializable. This is a limitation
    // of the Server Function architecture.
    //
    // To work around this limitation, we reuse the existing /api/auth/me endpoint
    // which already handles cookie extraction efficiently using PrivateCookieJar.
    // While this creates an internal HTTP request, it's the most practical
    // solution given the current architecture constraints.
    //
    // Alternative approaches considered:
    // 1. Using RequestParts: Not supported in leptos_axum Server Functions
    // 2. Sharing cookie extraction logic: Requires refactoring to extract
    //    cookie parsing logic into a shared module accessible from both
    //    Server Functions and route handlers
    //
    // Future improvement: Consider refactoring to share cookie extraction
    // logic between Server Functions and route handlers if Leptos adds
    // support for accessing request context directly in Server Functions.

    use reqwest::Client;
    let client = Client::new();

    let base_url =
        std::env::var("LEPTOS_SITE_ADDR").unwrap_or_else(|_| "http://localhost:3000".to_string());

    let url = format!("{}/api/auth/me", base_url);
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to fetch auth status: {}", e)))?;

    let auth_status: AuthStatus = response
        .json()
        .await
        .map_err(|e| ServerFnError::new(format!("Failed to parse auth status: {}", e)))?;

    Ok(auth_status)
}

use leptos::prelude::*;

/// Authentication context for sharing auth status across the application
#[derive(Clone)]
pub struct AuthContext {
    pub status: ReadSignal<Option<AuthStatus>>,
    pub set_status: WriteSignal<Option<AuthStatus>>,
    #[cfg(target_arch = "wasm32")]
    pub cached_status: RwSignal<Option<AuthStatus>>,
    #[cfg(target_arch = "wasm32")]
    pub cache_timestamp: RwSignal<Option<u64>>,
}

/// Hook to get the authentication context
/// Panics if AuthContext is not provided (use provide_auth_context first)
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>()
        .expect("AuthContext should be provided. Call provide_auth_context() first.")
}

/// Provide authentication context to the component tree (SSR version)
/// Returns the AuthContext that was created
#[cfg(not(target_arch = "wasm32"))]
pub fn provide_auth_context(initial_status: Option<AuthStatus>) -> AuthContext {
    let (status, set_status) = signal(initial_status.clone());
    let context = AuthContext { status, set_status };
    provide_context(context.clone());
    context
}
