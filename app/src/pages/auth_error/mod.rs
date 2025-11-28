/**
 * Auth Error Page
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this page):
 *   └─ app/src/pages/mod.rs
 *
 * Dependencies (External files that this page imports):
 *   ├─ leptos::prelude
 *   ├─ leptos_router::hooks::use_query_map
 *   ├─ crate::concepts::auth::{parse_auth_error, AuthError}
 *   └─ crate::components::container::Container
 *
 * Related Documentation:
 *   ├─ Auth Concept: app/src/concepts/auth/auth.spec.md
 *   └─ Implementation Roadmap: docs/03_plans/continuum/20251121_implementation-roadmap.md
 */
use crate::components::container::Container;
use crate::concepts::auth::{parse_auth_error, AuthError};
use leptos::prelude::*;
use leptos_router::hooks::use_query_map;

/// 認証エラーページ
/// URLクエリパラメータからエラーコードを取得し、適切なエラーメッセージを表示
#[component]
pub fn AuthErrorPage() -> impl IntoView {
    let query_map = use_query_map();

    let error = move || {
        let map = query_map.get();
        let error_code = map.get("error").map(|s| s.to_string()).unwrap_or_default();
        if error_code.is_empty() {
            AuthError::Unknown("unknown".to_string())
        } else {
            parse_auth_error(&error_code)
        }
    };

    let error_title = move || match error() {
        AuthError::CsrfMismatch => "セキュリティエラー",
        AuthError::TokenExchangeFailed => "認証エラー",
        AuthError::UserFetchFailed => "ユーザー情報取得エラー",
        AuthError::SessionCreationFailed => "セッションエラー",
        AuthError::SessionExpired => "セッション期限切れ",
        AuthError::Unknown(_) => "エラーが発生しました",
    };

    let error_message = move || error().message().to_string();

    let error_icon = move || {
        match error() {
        AuthError::CsrfMismatch => view! {
            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-error" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
            </svg>
        }.into_any(),
        AuthError::SessionExpired => view! {
            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-warning" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
        }.into_any(),
        _ => view! {
            <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 text-error" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
        }.into_any(),
    }
    };

    view! {
        <Container>
            <div class="min-h-[60vh] flex items-center justify-center">
                <div class="card bg-base-200 shadow-xl max-w-md w-full">
                    <div class="card-body items-center text-center">
                        // Error icon
                        <div class="mb-4">
                            {error_icon}
                        </div>

                        // Error title
                        <h1 class="card-title text-2xl mb-2">
                            {error_title}
                        </h1>

                        // Error message
                        <p class="text-base-content/70 mb-6">
                            {error_message}
                        </p>

                        // Action buttons
                        <div class="card-actions flex-col gap-2 w-full">
                            <a href="/auth/login" class="btn btn-primary w-full">
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2" viewBox="0 0 24 24" fill="currentColor">
                                    <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd" />
                                </svg>
                                "GitHubでログインし直す"
                            </a>
                            <a href="/" class="btn btn-ghost w-full">
                                "ホームに戻る"
                            </a>
                        </div>

                        // Help text
                        <div class="text-sm text-base-content/50 mt-4">
                            "問題が解決しない場合は、しばらく時間をおいてから再度お試しください。"
                        </div>
                    </div>
                </div>
            </div>
        </Container>
    }
}
