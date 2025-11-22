use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub access_token: String,
    pub expires_at: u64,
}

impl Session {
    pub fn new(user_id: String, access_token: String, duration_secs: u64) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let expires_at = since_the_epoch.as_secs() + duration_secs;

        Self {
            user_id,
            access_token,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs() > self.expires_at
    }

    pub fn to_cookie_value(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_cookie_value(value: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(value)
    }
}

pub fn create_session_cookie(
    session: &Session,
    secure: bool,
    duration_secs: i64,
) -> Result<Cookie<'static>, serde_json::Error> {
    let value = session.to_cookie_value()?;
    Ok(Cookie::build(("session", value))
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Lax)
        .max_age(Duration::seconds(duration_secs))
        .build())
}

pub fn create_logout_cookie(secure: bool) -> Cookie<'static> {
    Cookie::build(("session", ""))
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Lax)
        .max_age(Duration::seconds(0))
        .build()
}

pub fn create_csrf_cookie(csrf_token: String, secure: bool) -> Cookie<'static> {
    Cookie::build(("oauth_csrf_state", csrf_token))
        .path("/auth/callback")
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Lax)
        .max_age(Duration::minutes(5))
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration as StdDuration;

    #[test]
    fn test_session_creation() {
        let session = Session::new("user123".to_string(), "token123".to_string(), 3600);
        assert_eq!(session.user_id, "user123");
        assert_eq!(session.access_token, "token123");
        assert!(!session.is_expired());
    }

    #[test]
    fn test_session_expiration() {
        // Create a session that expires in 1 second
        let session = Session::new("user123".to_string(), "token123".to_string(), 1);
        assert!(!session.is_expired());

        // Wait for 2 seconds
        thread::sleep(StdDuration::from_secs(2));
        assert!(session.is_expired());
    }

    #[test]
    fn test_session_serialization() {
        let session = Session::new("user123".to_string(), "token123".to_string(), 3600);
        let cookie_value = session.to_cookie_value().unwrap();

        let deserialized_session = Session::from_cookie_value(&cookie_value).unwrap();
        assert_eq!(deserialized_session.user_id, session.user_id);
        assert_eq!(deserialized_session.access_token, session.access_token);
        assert_eq!(deserialized_session.expires_at, session.expires_at);
    }

    #[test]
    fn test_create_session_cookie() {
        let session = Session::new("user123".to_string(), "token123".to_string(), 3600);
        let cookie = create_session_cookie(&session, true, 3600).unwrap();

        assert_eq!(cookie.name(), "session");
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.secure(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
    }

    #[test]
    fn test_create_logout_cookie() {
        let cookie = create_logout_cookie(true);

        assert_eq!(cookie.name(), "session");
        assert_eq!(cookie.value(), "");
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(cookie.max_age(), Some(Duration::seconds(0)));
    }
}
