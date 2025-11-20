/**
 * Footer Component
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
 *   ├─ Spec: ./footer.spec.md
 *   └─ Module: ../mod.rs
 */

use leptos::prelude::*;

#[component]
pub fn Footer(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let footer_class = if class.is_empty() {
        "footer".to_string()
    } else {
        format!("footer {}", class)
    };

    view! {
        <footer class=footer_class>
            {children()}
        </footer>
    }
}

