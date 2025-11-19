use axum::{Router, middleware};
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use app::*;
use leptos::logging::log;
use axum_extra::extract::cookie::Key;

mod config;
mod auth;
mod state;
use config::Config;
use auth::oauth::{auth_routes, create_auth_state};
use auth::middleware::auth_middleware;
use state::AppState;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    log!("Configuration loaded: env={}", config.server.env);

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    // Create cookie key from session secret
    // Key::from expects exactly 64 bytes, so we pad or truncate the secret
    let mut key_bytes = config.session.secret.as_bytes().to_vec();
    if key_bytes.len() < 64 {
        // Pad with zeros if shorter than 64 bytes
        key_bytes.resize(64, 0);
    } else if key_bytes.len() > 64 {
        // Truncate if longer than 64 bytes
        key_bytes.truncate(64);
    }
    let key = Key::from(&key_bytes);
    
    // Create auth state
    let auth_state = create_auth_state(&config);

    let app_state = AppState::new(
        leptos_options.clone(),
        key.clone(),
        auth_state,
        config.clone(),
    );

    let app: Router<AppState> = Router::default();
    let app = app
        .leptos_routes(&app_state, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .merge(auth_routes())
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware::<AppState>))
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .with_state(app_state);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
