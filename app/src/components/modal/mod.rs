use leptos::ev::MouseEvent;
/**
 * Modal Component
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
 *   ├─ Spec: ./modal.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[component]
pub fn Modal(
    open: ReadSignal<bool>,
    #[prop(optional, into)] on_close: Option<Callback<()>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let modal_class = move || {
        let base = if open.get() {
            "modal modal-open"
        } else {
            "modal"
        };
        if class.is_empty() {
            base.to_string()
        } else {
            format!("{} {}", base, class)
        }
    };

    let handle_backdrop_click = move |_| {
        if let Some(callback) = on_close.clone() {
            callback.run(());
        }
    };

    view! {
        <div class=modal_class>
            <div class="modal-backdrop" on:click=handle_backdrop_click></div>
            {children()}
        </div>
    }
}

#[component]
pub fn ModalBox(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let box_class = if class.is_empty() {
        "modal-box".to_string()
    } else {
        format!("modal-box {}", class)
    };

    view! {
        <div class=box_class>
            {children()}
        </div>
    }
}

#[component]
pub fn ModalHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let header_class = if class.is_empty() {
        "font-bold text-lg mb-4".to_string()
    } else {
        format!("font-bold text-lg mb-4 {}", class)
    };

    view! {
        <h3 class=header_class>
            {children()}
        </h3>
    }
}

#[component]
pub fn ModalBody(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let body_class = if class.is_empty() {
        "py-4".to_string()
    } else {
        format!("py-4 {}", class)
    };

    view! {
        <div class=body_class>
            {children()}
        </div>
    }
}

#[component]
pub fn ModalFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let footer_class = if class.is_empty() {
        "modal-action".to_string()
    } else {
        format!("modal-action {}", class)
    };

    view! {
        <div class=footer_class>
            {children()}
        </div>
    }
}

#[component]
pub fn ModalAction(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let action_class = if class.is_empty() {
        "btn".to_string()
    } else {
        format!("btn {}", class)
    };

    let handle_click = move |ev: MouseEvent| {
        if let Some(cb) = on_click.clone() {
            cb.run(ev);
        }
    };

    view! {
        <button class=action_class on:click=handle_click>
            {children()}
        </button>
    }
}
