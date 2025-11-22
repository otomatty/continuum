use leptos::ev::InputEvent;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum InputVariant {
    Primary,
    Secondary,
    Accent,
    Error,
    Success,
    Warning,
    Info,
    Ghost,
    #[default]
    Bordered,
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

    let handle_input = move |ev: leptos::web_sys::Event| {
        if let Some(cb) = on_input {
            use leptos::wasm_bindgen::JsCast;
            if let Ok(input_ev) = ev.dyn_into::<leptos::web_sys::InputEvent>() {
                let leptos_input_ev = input_ev;
                cb.run(leptos_input_ev);
            }
        }
    };

    view! {
        <input
            type="text"
            class=combined_class
            placeholder=placeholder
            prop:value=move || value.map(|v| v.get()).unwrap_or_default()
            on:input=handle_input
        />
    }
}
