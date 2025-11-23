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

/// Provide authentication context to the component tree
/// Returns the AuthContext that was created
pub fn provide_auth_context(initial_status: Option<AuthStatus>) -> AuthContext {
    let (status, set_status) = signal(initial_status.clone());

    #[cfg(target_arch = "wasm32")]
    let cached_status = RwSignal::new(initial_status.clone());

    #[cfg(target_arch = "wasm32")]
    let cache_timestamp = RwSignal::new({
        if initial_status.is_some() {
            Some(get_current_timestamp())
        } else {
            None
        }
    });

    #[cfg(target_arch = "wasm32")]
    let context = AuthContext {
        status,
        set_status,
        cached_status,
        cache_timestamp,
    };

    #[cfg(not(target_arch = "wasm32"))]
    let context = AuthContext { status, set_status };

    provide_context(context.clone());
    context
}

/// Get current timestamp in seconds since Unix epoch
#[cfg(target_arch = "wasm32")]
fn get_current_timestamp() -> u64 {
    use js_sys::Date;

    let date = Date::new_0();
    let timestamp_ms = date.get_time();
    (timestamp_ms / 1000.0) as u64
}

/// Check if cache is still valid (within 5 minutes)
#[cfg(target_arch = "wasm32")]
fn is_cache_valid(timestamp: Option<u64>) -> bool {
    if let Some(ts) = timestamp {
        let current = get_current_timestamp();
        let cache_duration_secs = 300; // 5 minutes
        current.saturating_sub(ts) < cache_duration_secs
    } else {
        false
    }
}

/// Refresh auth status from server if cache is expired
#[cfg(target_arch = "wasm32")]
pub async fn refresh_auth_status_if_needed(context: &AuthContext) -> Result<(), ServerFnError> {
    use super::get_auth_status;
    use leptos::prelude::ServerFnError;

    let should_refresh = {
        let cached = context.cached_status.get();
        let timestamp = context.cache_timestamp.get();

        // If no cache or cache expired, refresh
        cached.is_none() || !is_cache_valid(timestamp)
    };

    if should_refresh {
        match get_auth_status().await {
            Ok(new_status) => {
                context.set_status.set(Some(new_status.clone()));
                context.cached_status.set(Some(new_status));
                context.cache_timestamp.set(Some(get_current_timestamp()));
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(())
}

/// Clear auth cache (used on logout)
#[cfg(target_arch = "wasm32")]
pub fn clear_auth_cache(context: &AuthContext) {
    context.set_status.set(None);
    context.cached_status.set(None);
    context.cache_timestamp.set(None);
}

/// Read auth status from HTML data attribute
/// This should be called on the client side after hydration
#[cfg(target_arch = "wasm32")]
pub fn read_auth_status_from_html() -> Option<AuthStatus> {
    use web_sys::window;

    if let Some(_window) = window() {
        if let Some(document) = window.document() {
            if let Some(html_element) = document.document_element() {
                if let Some(auth_status_attr) = html_element.get_attribute("data-auth-status") {
                    if let Ok(auth_status) = serde_json::from_str::<AuthStatus>(&auth_status_attr) {
                        return Some(auth_status);
                    }
                }
            }
        }
    }

    None
}
