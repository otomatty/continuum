/**
 * Skeleton Concept
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
 *   ├─ Spec: ./skeleton.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[component]
pub fn Skeleton(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let skeleton_class = if class.is_empty() {
        "skeleton".to_string()
    } else {
        format!("skeleton {}", class)
    };

    view! {
        <div class=skeleton_class>
            {children.map(|c| c().into_view().into_any()).unwrap_or_else(|| view! { <span></span> }.into_view().into_any())}
        </div>
    }
}
