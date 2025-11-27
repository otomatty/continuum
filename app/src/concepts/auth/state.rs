/**
 * Auth Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ app/src/concepts/auth/mod.rs
 *   └─ app/src/concepts/auth/actions.rs
 *
 * Dependencies:
 *   └─ serde (Serialize, Deserialize)
 *
 * Related Documentation:
 *   ├─ Spec: ./auth.spec.md
 *   └─ Tests: ./tests.rs
 */
use serde::{Deserialize, Serialize};

/// 認証済みユーザー情報
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthUser {
    /// GitHub user login (ユーザーID)
    pub id: String,
    /// 表示名
    pub display_name: String,
    /// アバターURL
    pub avatar_url: Option<String>,
}

impl AuthUser {
    /// 新しいAuthUserを作成
    pub fn new(id: String, display_name: String, avatar_url: Option<String>) -> Self {
        Self {
            id,
            display_name,
            avatar_url,
        }
    }

    /// IDのみで作成（display_nameはIDと同じ）
    pub fn from_id(id: String) -> Self {
        Self {
            display_name: id.clone(),
            id,
            avatar_url: None,
        }
    }
}

/// 認証状態
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthState {
    /// 認証済みかどうか
    pub is_authenticated: bool,
    /// 認証済みユーザー情報
    pub user: Option<AuthUser>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            user: None,
        }
    }
}

/// 認証エラーの種類
#[derive(Debug, Clone, PartialEq)]
pub enum AuthError {
    /// CSRFトークン不一致
    CsrfMismatch,
    /// トークン交換失敗
    TokenExchangeFailed,
    /// ユーザー情報取得失敗
    UserFetchFailed,
    /// セッション作成失敗
    SessionCreationFailed,
    /// セッション期限切れ
    SessionExpired,
    /// その他のエラー
    Unknown(String),
}

impl AuthError {
    /// エラーメッセージを取得
    pub fn message(&self) -> &str {
        match self {
            AuthError::CsrfMismatch => {
                "セキュリティトークンが一致しません。もう一度ログインしてください。"
            }
            AuthError::TokenExchangeFailed => "認証に失敗しました。もう一度お試しください。",
            AuthError::UserFetchFailed => "ユーザー情報の取得に失敗しました。",
            AuthError::SessionCreationFailed => "セッションの作成に失敗しました。",
            AuthError::SessionExpired => "セッションが期限切れです。もう一度ログインしてください。",
            AuthError::Unknown(_) => "予期しないエラーが発生しました。",
        }
    }

    /// エラーコードを取得
    pub fn code(&self) -> &str {
        match self {
            AuthError::CsrfMismatch => "csrf_mismatch",
            AuthError::TokenExchangeFailed => "token_exchange_failed",
            AuthError::UserFetchFailed => "user_fetch_failed",
            AuthError::SessionCreationFailed => "session_creation_failed",
            AuthError::SessionExpired => "session_expired",
            AuthError::Unknown(code) => code,
        }
    }
}
