/**
 * Popover Component
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
 *   ├─ Spec: ./popover.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[component]
pub fn Popover(
    open: ReadSignal<bool>,
    content: String,
    #[prop(optional, into)] on_toggle: Option<Callback<()>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let popover_class = move || {
        let base = if open.get() {
            "popover popover-open"
        } else {
            "popover"
        };
        if class.is_empty() {
            base.to_string()
        } else {
            format!("{} {}", base, class)
        }
    };

    let handle_toggle = move |_| {
        if let Some(callback) = on_toggle.clone() {
            callback.run(());
        }
    };

    view! {
        <div class=popover_class>
            <div class="popover-trigger" on:click=handle_toggle>
                {children()}
            </div>
            <div class="popover-content">
                {content}
            </div>
        </div>
    }
}
