use crate::hooks::AuthStatus;

/**
 * GitHubLoginButton Utilities
 *
 * DEPENDENCY MAP:
 *
 * Dependencies (External files that this module imports):
 *   └─ app/src/hooks/use_auth.rs (AuthStatus)
 *
 * Related Documentation:
 *   ├─ Spec: docs/01_issues/open/2025_11/20251123_01_authentication-state-management.md
 *   └─ Issue: https://github.com/otomatty/continuum/issues/13
 */

/// Check if the user is authenticated based on auth status
///
/// # Arguments
/// * `auth_status` - Optional authentication status
///
/// # Returns
/// * `true` if authenticated, `false` otherwise
///
/// # Examples
/// ```
/// use crate::hooks::AuthStatus;
/// use crate::components::github_login_button::utils::is_authenticated;
///
/// let authenticated = AuthStatus {
///     authenticated: true,
///     user_id: Some("user123".to_string()),
/// };
/// assert!(is_authenticated(Some(authenticated)));
///
/// let unauthenticated = AuthStatus {
///     authenticated: false,
///     user_id: None,
/// };
/// assert!(!is_authenticated(Some(unauthenticated)));
/// assert!(!is_authenticated(None));
/// ```
pub fn is_authenticated(auth_status: Option<AuthStatus>) -> bool {
    auth_status.map(|s| s.authenticated).unwrap_or(false)
}

/// Get the dashboard URL for authenticated users
pub const fn get_dashboard_url() -> &'static str {
    "/dashboard"
}

/// Get the login URL for unauthenticated users
pub const fn get_login_url() -> &'static str {
    "/auth/login"
}
