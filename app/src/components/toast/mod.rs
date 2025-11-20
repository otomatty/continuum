/**
 * Toast Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Component):
 *   └─ (To be added when used)
 *
 * Dependencies (External files that this Component imports):
 *   └─ leptos::prelude
 *
 * Related Documentation:
 *   ├─ Spec: ./toast.spec.md
 *   └─ Module: ../mod.rs
 */

use leptos::prelude::*;
use leptos::ev::MouseEvent;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;
use web_sys::window;

#[derive(Clone, Copy, PartialEq)]
pub enum ToastVariant {
    Info,
    Success,
    Warning,
    Error,
}

impl Default for ToastVariant {
    fn default() -> Self {
        ToastVariant::Info
    }
}

#[component]
pub fn Toast(
    message: String,
    #[prop(optional)] variant: ToastVariant,
    #[prop(optional)] duration: Option<u32>,
    #[prop(optional, into)] on_close: Option<Callback<()>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match variant {
        ToastVariant::Info => "alert-info",
        ToastVariant::Success => "alert-success",
        ToastVariant::Warning => "alert-warning",
        ToastVariant::Error => "alert-error",
    };

    let toast_class = if class.is_empty() {
        format!("toast alert {}", variant_class)
    } else {
        format!("toast alert {} {}", variant_class, class)
    };

    let handle_close = move |_| {
        if let Some(callback) = on_close {
            callback.call(());
        }
    };

    if let Some(dur) = duration {
        let callback = on_close.clone();
        spawn_local(async move {
            let dur_ms = dur * 1000;
            let promise = Promise::new(&mut |resolve, _| {
                if let Some(window) = window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        &resolve,
                        dur_ms as i32,
                    );
                }
            });
            let _ = JsFuture::from(promise).await;
            if let Some(cb) = callback {
                cb.call(());
            }
        });
    }

    view! {
        <div class=toast_class role="alert">
            <span>{message}</span>
            {if on_close.is_some() {
                view! {
                    <button class="btn btn-sm btn-circle" on:click=handle_close>
                        "✕"
                    </button>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
        </div>
    }
}

