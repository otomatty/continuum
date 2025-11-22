/**
 * Progress Concept
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
 *   ├─ Spec: ./progress.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum ProgressVariant {
    #[default]
    Primary,
    Success,
    Warning,
    Error,
}


#[component]
pub fn Progress(
    value: u32,
    #[prop(optional)] max: Option<u32>,
    #[prop(optional)] variant: ProgressVariant,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let max_value = max.unwrap_or(100);
    let clamped_value = value.min(max_value);
    let percentage = (clamped_value as f64 / max_value as f64 * 100.0) as u32;

    let variant_class = match variant {
        ProgressVariant::Primary => "progress-primary",
        ProgressVariant::Success => "progress-success",
        ProgressVariant::Warning => "progress-warning",
        ProgressVariant::Error => "progress-error",
    };

    let combined_class = if class.is_empty() {
        format!("progress {}", variant_class)
    } else {
        format!("progress {} {}", variant_class, class)
    };

    view! {
        <progress
            class=combined_class
            value=clamped_value
            max=max_value
            style=format!("--value: {}%", percentage)
        />
    }
}
