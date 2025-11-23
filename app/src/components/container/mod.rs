/**
 * Container Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Component):
 *   ├─ app/src/pages/home/mod.rs
 *   ├─ app/src/pages/home/components/feature_showcase.rs
 *   ├─ app/src/pages/home/components/final_cta.rs
 *   ├─ app/src/pages/home/components/home_footer.rs
 *   ├─ app/src/pages/home/components/social_proof.rs
 *   ├─ app/src/components/header/authenticated_header.rs
 *   └─ app/src/components/header/public_header.rs
 *
 * Dependencies (External files that this Component imports):
 *   └─ leptos::prelude
 *
 * Related Documentation:
 *   ├─ Spec: ./container.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[component]
pub fn Container(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let container_class = if class.is_empty() {
        "container max-w-5xl mx-auto px-4 md:px-6 lg:px-8".to_string()
    } else {
        format!("container max-w-5xl mx-auto px-4 md:px-6 lg:px-8 {}", class)
    };

    view! {
        <div class=container_class>
            {children()}
        </div>
    }
}
