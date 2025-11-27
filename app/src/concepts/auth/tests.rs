/**
 * Auth Concept - Tests
 *
 * DEPENDENCY MAP:
 *
 * Parents: None (test module)
 *
 * Dependencies:
 *   ├─ super::state::{AuthState, AuthUser, AuthError}
 *   └─ super::actions::*
 *
 * Related Documentation:
 *   └─ Spec: ./auth.spec.md
 */

#[cfg(test)]
mod tests {
    use crate::concepts::auth::actions::*;
    use crate::concepts::auth::state::*;

    // TC-001: create_authenticated_state
    #[test]
    fn test_create_authenticated_state() {
        let user = AuthUser::new(
            "alice-dev".to_string(),
            "Alice Developer".to_string(),
            Some("https://example.com/avatar.png".to_string()),
        );

        let state = create_authenticated_state(user.clone());

        assert!(state.is_authenticated);
        assert_eq!(state.user, Some(user));
    }

    // TC-002: create_unauthenticated_state
    #[test]
    fn test_create_unauthenticated_state() {
        let state = create_unauthenticated_state();

        assert!(!state.is_authenticated);
        assert_eq!(state.user, None);
    }

    // TC-003: is_authenticated - 認証済み
    #[test]
    fn test_is_authenticated_when_authenticated() {
        let user = AuthUser::from_id("alice-dev".to_string());
        let state = create_authenticated_state(user);

        assert!(is_authenticated(&state));
    }

    // TC-004: is_authenticated - 未認証
    #[test]
    fn test_is_authenticated_when_unauthenticated() {
        let state = create_unauthenticated_state();

        assert!(!is_authenticated(&state));
    }

    // TC-005: get_user - 認証済み
    #[test]
    fn test_get_user_when_authenticated() {
        let user = AuthUser::new("alice-dev".to_string(), "Alice Developer".to_string(), None);
        let state = create_authenticated_state(user.clone());

        let result = get_user(&state);

        assert!(result.is_some());
        assert_eq!(result.unwrap().id, "alice-dev");
        assert_eq!(result.unwrap().display_name, "Alice Developer");
    }

    // TC-006: get_user - 未認証
    #[test]
    fn test_get_user_when_unauthenticated() {
        let state = create_unauthenticated_state();

        let result = get_user(&state);

        assert!(result.is_none());
    }

    // TC-007: parse_auth_error - csrf_mismatch
    #[test]
    fn test_parse_auth_error_csrf_mismatch() {
        let error = parse_auth_error("csrf_mismatch");

        assert_eq!(error, AuthError::CsrfMismatch);
        assert_eq!(error.code(), "csrf_mismatch");
    }

    // TC-008: parse_auth_error - token_exchange_failed
    #[test]
    fn test_parse_auth_error_token_exchange_failed() {
        let error = parse_auth_error("token_exchange_failed");

        assert_eq!(error, AuthError::TokenExchangeFailed);
        assert_eq!(error.code(), "token_exchange_failed");
    }

    // TC-009: parse_auth_error - unknown error
    #[test]
    fn test_parse_auth_error_unknown() {
        let error = parse_auth_error("some_unknown_error");

        assert_eq!(error, AuthError::Unknown("some_unknown_error".to_string()));
        assert_eq!(error.code(), "some_unknown_error");
    }

    // Additional tests for other error types
    #[test]
    fn test_parse_auth_error_user_fetch_failed() {
        let error = parse_auth_error("user_fetch_failed");

        assert_eq!(error, AuthError::UserFetchFailed);
        assert_eq!(error.code(), "user_fetch_failed");
    }

    #[test]
    fn test_parse_auth_error_session_creation_failed() {
        let error = parse_auth_error("session_creation_failed");

        assert_eq!(error, AuthError::SessionCreationFailed);
    }

    #[test]
    fn test_parse_auth_error_session_expired() {
        let error = parse_auth_error("session_expired");

        assert_eq!(error, AuthError::SessionExpired);
    }

    // Test login action
    #[test]
    fn test_login() {
        let state = create_unauthenticated_state();
        let user = AuthUser::from_id("bob-dev".to_string());

        let new_state = login(state, user.clone());

        assert!(new_state.is_authenticated);
        assert_eq!(new_state.user, Some(user));
    }

    // Test logout action
    #[test]
    fn test_logout() {
        let user = AuthUser::from_id("alice-dev".to_string());
        let state = create_authenticated_state(user);

        let new_state = logout(state);

        assert!(!new_state.is_authenticated);
        assert_eq!(new_state.user, None);
    }

    // Test AuthUser creation methods
    #[test]
    fn test_auth_user_new() {
        let user = AuthUser::new(
            "user123".to_string(),
            "User Name".to_string(),
            Some("https://avatar.url".to_string()),
        );

        assert_eq!(user.id, "user123");
        assert_eq!(user.display_name, "User Name");
        assert_eq!(user.avatar_url, Some("https://avatar.url".to_string()));
    }

    #[test]
    fn test_auth_user_from_id() {
        let user = AuthUser::from_id("user123".to_string());

        assert_eq!(user.id, "user123");
        assert_eq!(user.display_name, "user123"); // display_name should equal id
        assert_eq!(user.avatar_url, None);
    }

    // Test AuthError messages
    #[test]
    fn test_auth_error_messages() {
        assert!(!AuthError::CsrfMismatch.message().is_empty());
        assert!(!AuthError::TokenExchangeFailed.message().is_empty());
        assert!(!AuthError::UserFetchFailed.message().is_empty());
        assert!(!AuthError::SessionCreationFailed.message().is_empty());
        assert!(!AuthError::SessionExpired.message().is_empty());
        assert!(!AuthError::Unknown("test".to_string()).message().is_empty());
    }

    // Test AuthState default
    #[test]
    fn test_auth_state_default() {
        let state = AuthState::default();

        assert!(!state.is_authenticated);
        assert!(state.user.is_none());
    }
}
