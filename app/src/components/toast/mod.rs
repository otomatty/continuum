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

#[cfg(feature = "hydrate")]
use js_sys::Promise;
#[cfg(feature = "hydrate")]
use leptos::web_sys;
#[cfg(feature = "hydrate")]
use wasm_bindgen_futures::spawn_local;
#[cfg(feature = "hydrate")]
use wasm_bindgen_futures::JsFuture;

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
    #[prop(optional)]
    #[allow(unused_variables)]
    duration: Option<u32>,
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
        if let Some(callback) = on_close.clone() {
            callback.run(());
        }
    };

    #[cfg(feature = "hydrate")]
    if let Some(dur) = duration {
        let callback = on_close.clone();
        spawn_local(async move {
            let dur_ms = dur * 1000;
            let promise = Promise::new(&mut |resolve, _| {
                if let Some(window) = web_sys::window() {
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        &resolve,
                        dur_ms as i32,
                    );
                }
            });
            let _ = JsFuture::from(promise).await;
            if let Some(cb) = callback {
                cb.run(());
            }
        });
    }

    view! {
        <div class=toast_class role="alert">
            <span>{message}</span>
            {move || {
                if on_close.is_some() {
                    view! {
                        <button class="btn btn-sm btn-circle" on:click=handle_close>
                            "✕"
                        </button>
                    }.into_view().into_any()
                } else {
                    view! { <span></span> }.into_view().into_any()
                }
            }}
        </div>
    }
}
