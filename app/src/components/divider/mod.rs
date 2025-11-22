/**
 * Divider Concept
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Concept):
 *   └─ (To be added when used)
 *
 * Dependencies (External files that this Concept imports):
 *   └─ leptos::prelude
 *
 * Related Documentation:
 *   ├─ Spec: ./divider.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[component]
pub fn Divider(
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let divider_class = if class.is_empty() {
        "divider".to_string()
    } else {
        format!("divider {}", class)
    };

    view! {
        <div class=divider_class>
            {move || {
                if let Some(text) = text.as_ref() {
                    text.clone().into_view()
                } else {
                    "".to_string().into_view()
                }
            }}
        </div>
    }
}
