use leptos::prelude::*;
use leptos::ev::Event;

#[derive(Clone, Copy, PartialEq)]
pub enum CheckboxVariant {
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    Info,
}

impl Default for CheckboxVariant {
    fn default() -> Self {
        CheckboxVariant::Primary
    }
}

#[component]
pub fn Checkbox(
    #[prop(optional)] variant: CheckboxVariant,
    #[prop(optional, into)] checked: Option<ReadSignal<bool>>,
    #[prop(optional, into)] on_change: Option<Callback<Event>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match variant {
        CheckboxVariant::Primary => "checkbox-primary",
        CheckboxVariant::Secondary => "checkbox-secondary",
        CheckboxVariant::Accent => "checkbox-accent",
        CheckboxVariant::Success => "checkbox-success",
        CheckboxVariant::Warning => "checkbox-warning",
        CheckboxVariant::Error => "checkbox-error",
        CheckboxVariant::Info => "checkbox-info",
    };

    let combined_class = if class.is_empty() {
        format!("checkbox {}", variant_class)
    } else {
        format!("checkbox {} {}", variant_class, class)
    };

    let handle_change = move |ev: web_sys::Event| {
        if let Some(cb) = on_change.clone() {
            (cb)(ev);
        }
    };

    view! {
        <input
            type="checkbox"
            class=combined_class
            checked=move || checked.map(|c| c.get()).unwrap_or(false)
            on:change=handle_change
        />
    }
}

