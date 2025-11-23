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
use leptos::*;
#[cfg(feature = "ssr")]
use leptos::prelude::ServerFnError;

#[cfg(feature = "ssr")]
#[server(GetAuthStatus, "/api/auth/status")]
pub async fn get_auth_status() -> Result<AuthStatus, ServerFnError> {
    // Server Function implementation will be handled by leptos_axum
    // The actual cookie extraction will be done by the framework
    // For now, return unauthenticated status as placeholder
    // TODO: Implement actual cookie extraction using leptos_axum's RequestParts
    Ok(AuthStatus {
        authenticated: false,
        user_id: None,
    })
}

