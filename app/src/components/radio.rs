use leptos::prelude::*;
use leptos::ev::ChangeEvent;

#[derive(Clone, Copy, PartialEq)]
pub enum RadioVariant {
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    Info,
}

impl Default for RadioVariant {
    fn default() -> Self {
        RadioVariant::Primary
    }
}

#[component]
pub fn Radio(
    name: String,
    value: String,
    #[prop(optional)] variant: RadioVariant,
    #[prop(optional, into)] checked: Option<ReadSignal<bool>>,
    #[prop(optional, into)] on_change: Option<Callback<ChangeEvent>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match variant {
        RadioVariant::Primary => "radio-primary",
        RadioVariant::Secondary => "radio-secondary",
        RadioVariant::Accent => "radio-accent",
        RadioVariant::Success => "radio-success",
        RadioVariant::Warning => "radio-warning",
        RadioVariant::Error => "radio-error",
        RadioVariant::Info => "radio-info",
    };

    let combined_class = if class.is_empty() {
        format!("radio {}", variant_class)
    } else {
        format!("radio {} {}", variant_class, class)
    };

    view! {
        <input
            type="radio"
            name=name
            value=value
            class=combined_class
            checked=move || checked.map(|c| c.get()).unwrap_or(false)
            on:change=on_change
        />
    }
}

