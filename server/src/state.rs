use crate::auth::oauth::AuthState;
use crate::config::Config;
use axum_extra::extract::cookie::Key;
use leptos::prelude::LeptosOptions;

#[derive(Clone, axum::extract::FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub key: Key,
    pub auth_state: AuthState,
    pub config: Config,
    pub http_client: reqwest::Client,
}

impl AppState {
    pub fn new(
        leptos_options: LeptosOptions,
        key: Key,
        auth_state: AuthState,
        config: Config,
    ) -> Self {
        Self {
            leptos_options,
            key,
            auth_state,
            config,
            http_client: reqwest::Client::new(),
        }
    }
}
