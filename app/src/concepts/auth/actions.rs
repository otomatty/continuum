/**
 * Auth Concept - Actions
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   └─ app/src/concepts/auth/mod.rs
 *
 * Dependencies:
 *   └─ super::state::{AuthState, AuthUser, AuthError}
 *
 * Related Documentation:
 *   ├─ Spec: ./auth.spec.md
 *   └─ Tests: ./tests.rs
 */
use super::state::{AuthError, AuthState, AuthUser};

/// 認証済み状態を作成
///
/// # Arguments
/// * `user` - 認証済みユーザー情報
///
/// # Returns
/// 認証済みのAuthState
pub fn create_authenticated_state(user: AuthUser) -> AuthState {
    AuthState {
        is_authenticated: true,
        user: Some(user),
    }
}

/// 未認証状態を作成
///
/// # Returns
/// 未認証のAuthState
pub fn create_unauthenticated_state() -> AuthState {
    AuthState::default()
}

/// 認証状態を判定
///
/// # Arguments
/// * `state` - 認証状態
///
/// # Returns
/// 認証済みの場合true
pub fn is_authenticated(state: &AuthState) -> bool {
    state.is_authenticated
}

/// ユーザー情報を取得
///
/// # Arguments
/// * `state` - 認証状態
///
/// # Returns
/// 認証済みの場合はユーザー情報、未認証の場合はNone
pub fn get_user(state: &AuthState) -> Option<&AuthUser> {
    state.user.as_ref()
}

/// エラーコードからAuthErrorを解析
///
/// # Arguments
/// * `error_code` - URLクエリパラメータから取得したエラーコード
///
/// # Returns
/// 対応するAuthError
pub fn parse_auth_error(error_code: &str) -> AuthError {
    match error_code {
        "csrf_mismatch" => AuthError::CsrfMismatch,
        "token_exchange_failed" => AuthError::TokenExchangeFailed,
        "user_fetch_failed" => AuthError::UserFetchFailed,
        "session_creation_failed" => AuthError::SessionCreationFailed,
        "session_expired" => AuthError::SessionExpired,
        _ => AuthError::Unknown(error_code.to_string()),
    }
}

/// AuthStateをログイン状態に更新
///
/// # Arguments
/// * `state` - 現在の認証状態
/// * `user` - ログインするユーザー
///
/// # Returns
/// 更新されたAuthState
pub fn login(state: AuthState, user: AuthUser) -> AuthState {
    // 既存の状態は無視して新しい認証済み状態を作成
    let _ = state;
    create_authenticated_state(user)
}

/// AuthStateをログアウト状態に更新
///
/// # Arguments
/// * `state` - 現在の認証状態
///
/// # Returns
/// 更新されたAuthState（未認証）
pub fn logout(state: AuthState) -> AuthState {
    // 既存の状態は無視して未認証状態を作成
    let _ = state;
    create_unauthenticated_state()
}
