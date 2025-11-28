# Auth Concept Specification

## Related Files

- Implementation: `src/concepts/auth/mod.rs`
- State: `src/concepts/auth/state.rs`
- Actions: `src/concepts/auth/actions.rs`
- Tests: `src/concepts/auth/tests.rs`

## Related Documentation

- Server OAuth Implementation: `server/src/auth/oauth.rs`
- Server Session: `server/src/auth/session.rs`
- Hooks: `app/src/hooks/use_auth.rs`
- Implementation Roadmap: `docs/03_plans/continuum/20251121_implementation-roadmap.md`

## Requirements

### 責務
- 認証状態の管理
- ユーザー認証情報の保持
- 認証状態の判定

### 状態構造

```rust
/// 認証状態
pub struct AuthState {
    pub is_authenticated: bool,
    pub user: Option<AuthUser>,
}

/// 認証済みユーザー情報
pub struct AuthUser {
    pub id: String,           // GitHub user login
    pub display_name: String, // 表示名
    pub avatar_url: Option<String>, // アバターURL
}

/// 認証エラーの種類
pub enum AuthError {
    CsrfMismatch,          // CSRFトークン不一致
    TokenExchangeFailed,   // トークン交換失敗
    UserFetchFailed,       // ユーザー情報取得失敗
    SessionCreationFailed, // セッション作成失敗
    SessionExpired,        // セッション期限切れ
    Unknown(String),       // その他のエラー
}
```

### アクション

1. `create_authenticated_state(user: AuthUser) -> AuthState`
   - 認証済み状態を作成

2. `create_unauthenticated_state() -> AuthState`
   - 未認証状態を作成

3. `is_authenticated(state: &AuthState) -> bool`
   - 認証状態を判定

4. `get_user(state: &AuthState) -> Option<&AuthUser>`
   - ユーザー情報を取得

5. `parse_auth_error(error_code: &str) -> AuthError`
   - エラーコードからAuthErrorを解析

## Test Cases

### TC-001: create_authenticated_state
- Given: AuthUser情報
- When: create_authenticated_state(user)を実行
- Then: is_authenticated=true, user=Some(user)のAuthStateが返される

### TC-002: create_unauthenticated_state
- Given: なし
- When: create_unauthenticated_state()を実行
- Then: is_authenticated=false, user=NoneのAuthStateが返される

### TC-003: is_authenticated - 認証済み
- Given: is_authenticated=trueのAuthState
- When: is_authenticated(&state)を実行
- Then: trueが返される

### TC-004: is_authenticated - 未認証
- Given: is_authenticated=falseのAuthState
- When: is_authenticated(&state)を実行
- Then: falseが返される

### TC-005: get_user - 認証済み
- Given: user=Some(user)のAuthState
- When: get_user(&state)を実行
- Then: Some(&user)が返される

### TC-006: get_user - 未認証
- Given: user=NoneのAuthState
- When: get_user(&state)を実行
- Then: Noneが返される

### TC-007: parse_auth_error - csrf_mismatch
- Given: "csrf_mismatch"
- When: parse_auth_error(error_code)を実行
- Then: AuthError::CsrfMismatchが返される

### TC-008: parse_auth_error - token_exchange_failed
- Given: "token_exchange_failed"
- When: parse_auth_error(error_code)を実行
- Then: AuthError::TokenExchangeFailedが返される

### TC-009: parse_auth_error - unknown error
- Given: "unknown_error_code"
- When: parse_auth_error(error_code)を実行
- Then: AuthError::Unknown("unknown_error_code")が返される

