/**
 * Steps Component
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
 *   ├─ Spec: ./steps.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum StepStatus {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Error,
}

#[component]
pub fn Steps(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let steps_class = if class.is_empty() {
        "steps".to_string()
    } else {
        format!("steps {}", class)
    };

    view! {
        <ul class=steps_class>
            {children()}
        </ul>
    }
}

#[component]
pub fn StepItem(
    #[prop(optional)] status: StepStatus,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let status_class = match status {
        StepStatus::Default => "",
        StepStatus::Primary => "step-primary",
        StepStatus::Success => "step-success",
        StepStatus::Warning => "step-warning",
        StepStatus::Error => "step-error",
    };

    let step_class = if status_class.is_empty() {
        if class.is_empty() {
            "step".to_string()
        } else {
            format!("step {}", class)
        }
    } else if class.is_empty() {
        format!("step {}", status_class)
    } else {
        format!("step {} {}", status_class, class)
    };

    view! {
        <li class=step_class>
            {children()}
        </li>
    }
}
