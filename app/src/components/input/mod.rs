use leptos::prelude::*;
use leptos::ev::InputEvent;

#[derive(Clone, Copy, PartialEq)]
pub enum InputVariant {
    Primary,
    Secondary,
    Accent,
    Error,
    Success,
    Warning,
    Info,
    Ghost,
    Bordered,
}

impl Default for InputVariant {
    fn default() -> Self {
        InputVariant::Bordered
    }
}

#[component]
pub fn Input(
    #[prop(optional)] variant: InputVariant,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] value: Option<ReadSignal<String>>,
    #[prop(optional, into)] on_input: Option<Callback<InputEvent>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match variant {
        InputVariant::Primary => "input-primary",
        InputVariant::Secondary => "input-secondary",
        InputVariant::Accent => "input-accent",
        InputVariant::Error => "input-error",
        InputVariant::Success => "input-success",
        InputVariant::Warning => "input-warning",
        InputVariant::Info => "input-info",
        InputVariant::Ghost => "input-ghost",
        InputVariant::Bordered => "input-bordered",
    };

    let base_class = if variant == InputVariant::Bordered {
        "input input-bordered".to_string()
    } else {
        format!("input {}", variant_class)
    };

    let combined_class = if class.is_empty() {
        base_class
    } else {
        format!("{} {}", base_class, class)
    };

    view! {
        <input
            type="text"
            class=combined_class
            placeholder=placeholder
            value=move || value.map(|v| v.get()).unwrap_or_default()
            on:input=on_input
        />
    }
}

