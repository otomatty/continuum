/**
 * Alert Concept
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
 *   ├─ Spec: ./alert.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}


#[component]
pub fn Alert(
    #[prop(optional)] variant: AlertVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        AlertVariant::Info => "alert-info",
        AlertVariant::Success => "alert-success",
        AlertVariant::Warning => "alert-warning",
        AlertVariant::Error => "alert-error",
    };

    let combined_class = if class.is_empty() {
        format!("alert {}", variant_class)
    } else {
        format!("alert {} {}", variant_class, class)
    };

    view! {
        <div role="alert" class=combined_class>
            {children()}
        </div>
    }
}

#[component]
pub fn AlertTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let title_class = if class.is_empty() {
        "font-bold".to_string()
    } else {
        format!("font-bold {}", class)
    };

    view! {
        <h3 class=title_class>
            {children()}
        </h3>
    }
}

#[component]
pub fn AlertDescription(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let desc_class = if class.is_empty() {
        "text-sm".to_string()
    } else {
        format!("text-sm {}", class)
    };

    view! {
        <div class=desc_class>
            {children()}
        </div>
    }
}
