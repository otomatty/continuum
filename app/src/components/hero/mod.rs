/**
 * Hero Component
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
 *   ├─ Spec: ./hero.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[component]
pub fn Hero(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let hero_class = if class.is_empty() {
        "hero".to_string()
    } else {
        format!("hero {}", class)
    };

    view! {
        <div class=hero_class>
            {children()}
        </div>
    }
}

#[component]
pub fn HeroContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let content_class = if class.is_empty() {
        "hero-content".to_string()
    } else {
        format!("hero-content {}", class)
    };

    view! {
        <div class=content_class>
            {children()}
        </div>
    }
}

#[component]
pub fn HeroOverlay(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let overlay_class = if class.is_empty() {
        "hero-overlay".to_string()
    } else {
        format!("hero-overlay {}", class)
    };

    view! {
        <div class=overlay_class>
            {children()}
        </div>
    }
}
