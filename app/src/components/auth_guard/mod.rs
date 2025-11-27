/**
 * Auth Guard Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   ├─ app/src/components/mod.rs
 *   └─ app/src/pages/{dashboard, portfolio, knowledge, settings}/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ leptos::prelude
 *   ├─ crate::hooks::use_auth
 *   └─ crate::components::loading::Loading
 *
 * Related Documentation:
 *   ├─ Auth Concept: app/src/concepts/auth/auth.spec.md
 *   └─ Implementation Roadmap: docs/03_plans/continuum/20251121_implementation-roadmap.md
 */
use crate::components::loading::Loading;
use crate::hooks::use_auth;
use leptos::prelude::*;

/// 認証ガードコンポーネント
/// 認証済みの場合のみ子コンポーネントを表示し、
/// 未認証の場合はログインページにリダイレクトする
#[component]
pub fn AuthGuard(children: ChildrenFn) -> impl IntoView {
    let auth = use_auth();

    // 認証状態を取得
    let is_authenticated = move || {
        auth.status
            .get()
            .map(|status| status.authenticated)
            .unwrap_or(false)
    };

    // 認証チェック中かどうか（まだstatusがNoneの場合）
    let is_loading = move || auth.status.get().is_none();

    // クライアントサイドでのリダイレクト処理
    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            // 認証チェック完了後、未認証の場合はリダイレクト
            if !is_loading() && !is_authenticated() {
                if let Some(window) = web_sys::window() {
                    let _ = window.location().set_href("/auth/login");
                }
            }
        });
    }

    // childrenをStoredValueに保存して複数回使用可能にする
    let children = StoredValue::new(children);

    view! {
        <Show
            when=move || !is_loading()
            fallback=move || view! {
                <div class="min-h-[60vh] flex items-center justify-center">
                    <Loading />
                </div>
            }
        >
            <Show
                when=is_authenticated
                fallback=move || view! {
                    <div class="min-h-[60vh] flex items-center justify-center">
                        <div class="text-center">
                            <p class="text-lg mb-4">"ログインが必要です"</p>
                            <a href="/auth/login" class="btn btn-primary">
                                "ログイン"
                            </a>
                        </div>
                    </div>
                }
            >
                {children.with_value(|c| c())}
            </Show>
        </Show>
    }
}

/// 認証が必要なページをラップするためのコンポーネント
/// SSR時には認証チェックを行わず、クライアントサイドでチェックを行う
#[component]
pub fn RequireAuth(children: ChildrenFn) -> impl IntoView {
    view! {
        <AuthGuard>
            {children()}
        </AuthGuard>
    }
}
