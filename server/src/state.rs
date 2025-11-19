use axum_extra::extract::cookie::Key;
use leptos::prelude::LeptosOptions;
use crate::auth::oauth::AuthState;

#[derive(Clone, axum::extract::FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub key: Key,
    pub auth_state: AuthState,
}

