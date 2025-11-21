use crate::auth::session::Session;
use axum::{
    body::Body,
    extract::{FromRef, Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::cookie::{Key, PrivateCookieJar};

pub async fn auth_middleware<S>(
    State(_state): State<S>,
    jar: PrivateCookieJar,
    request: Request<Body>,
    next: Next,
) -> impl IntoResponse
where
    S: Clone + Send + Sync + 'static,
    Key: FromRef<S>,
{
    let path = request.uri().path();

    // Public routes that don't require authentication
    if path == "/"
        || path.starts_with("/auth/")
        || path.starts_with("/pkg/")
        || path.starts_with("/api/public/")
        || path.starts_with("/favicon.ico")
        || path == "/components"
        || path == "/dashboard"
        || path.starts_with("/portfolio/")
    {
        return next.run(request).await;
    }

    // Check for session cookie
    if let Some(cookie) = jar.get("session") {
        if let Ok(session) = Session::from_cookie_value(cookie.value()) {
            if !session.is_expired() {
                return next.run(request).await;
            }
        }
    }

    // Redirect to login if not authenticated
    Redirect::to("/auth/login").into_response()
}
