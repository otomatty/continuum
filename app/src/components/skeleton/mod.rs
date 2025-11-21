/**
 * Skeleton Component
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
 *   ├─ Spec: ./skeleton.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[component]
pub fn Skeleton(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] text: Option<String>,
) -> impl IntoView {
    let skeleton_class = if class.is_empty() {
        if text.is_some() {
            "skeleton skeleton-text".to_string()
        } else {
            "skeleton".to_string()
        }
    } else if text.is_some() {
        format!("skeleton skeleton-text {}", class)
    } else {
        format!("skeleton {}", class)
    };

    if let Some(text_content) = text {
        view! {
            <span class=skeleton_class>{text_content}</span>
        }
        .into_any()
    } else {
        view! {
            <div class=skeleton_class></div>
        }
        .into_any()
    }
}
