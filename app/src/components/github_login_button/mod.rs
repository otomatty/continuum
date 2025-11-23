use crate::components::button::{Button, ButtonVariant};
use crate::hooks::use_auth;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

#[cfg(test)]
mod tests;

mod utils;
pub use utils::{get_dashboard_url, get_login_url, is_authenticated};

/**
 * GitHubLoginButton Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   ├─ app/src/components/header/public_header.rs
 *   ├─ app/src/pages/home/components/cta_section.rs
 *   └─ app/src/pages/home/components/final_cta.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/button/mod.rs
 *
 * Related Documentation:
 *   └─ Module: ../mod.rs
 */
#[component]
pub fn GitHubLoginButton(
    #[prop(optional, into, default = "GitHub でログイン".to_string())] text: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let auth = use_auth();

    // Handler for authenticated users - redirect to dashboard
    let dashboard_handler = move |_ev: MouseEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use web_sys::window;

            if let Some(window) = window() {
                let _ = window.location().set_href(utils::get_dashboard_url());
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // SSR時は何もしない（クライアント側でのみリダイレクト）
        }
    };

    // デフォルトスタイルを設定（classが空の場合はデフォルトを使用）
    let default_class = "flex items-center gap-2".to_string();
    let combined_class = if class.is_empty() {
        default_class
    } else {
        format!("{} {}", default_class, class)
    };

    // Create login callback
    let login_callback = Callback::new(move |_ev: MouseEvent| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use web_sys::window;

            if let Some(window) = window() {
                let _ = window.location().set_href(utils::get_login_url());
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // SSR時は何もしない（クライアント側でのみリダイレクト）
        }
    });

    let authenticated_callback = Callback::new(dashboard_handler);

    // Clone auth status before using in closure to ensure Fn instead of FnOnce
    let auth_status = auth.status;

    // Create a closure for the when condition - defined outside view! macro
    // ReadSignal is Copy, so we can use move safely
    // This closure takes no arguments as required by Show component's when prop
    let is_authenticated_closure = move || utils::is_authenticated(auth_status.get());

    // Clone values for fallback - clone before move closure to ensure Fn instead of FnOnce
    let fallback_class = combined_class.clone();
    let fallback_text = text.clone();

    view! {
        <Show
            when=is_authenticated_closure
            fallback=move || {
                let class = fallback_class.clone();
                let btn_text = fallback_text.clone();
                view! {
                    <Button
                        variant=ButtonVariant::Primary
                        class=class
                        on_click=login_callback
                    >
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            viewBox="0 0 24 24"
                            fill="currentColor"
                            class="w-5 h-5"
                        >
                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                        </svg>
                        {btn_text}
                    </Button>
                }
            }
        >
            <Button
                variant=ButtonVariant::Primary
                class=combined_class.clone()
                on_click=authenticated_callback
            >
                "ダッシュボードへ"
            </Button>
        </Show>
    }
}
