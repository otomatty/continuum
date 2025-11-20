use leptos::prelude::*;
use leptos::ev::InputEvent;

#[derive(Clone, Copy, PartialEq)]
pub enum TextareaVariant {
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

impl Default for TextareaVariant {
    fn default() -> Self {
        TextareaVariant::Bordered
    }
}

#[component]
pub fn Textarea(
    #[prop(optional)] variant: TextareaVariant,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional, into)] value: Option<ReadSignal<String>>,
    #[prop(optional, into)] on_input: Option<Callback<InputEvent>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match variant {
        TextareaVariant::Primary => "textarea-primary",
        TextareaVariant::Secondary => "textarea-secondary",
        TextareaVariant::Accent => "textarea-accent",
        TextareaVariant::Error => "textarea-error",
        TextareaVariant::Success => "textarea-success",
        TextareaVariant::Warning => "textarea-warning",
        TextareaVariant::Info => "textarea-info",
        TextareaVariant::Ghost => "textarea-ghost",
        TextareaVariant::Bordered => "textarea-bordered",
    };

    let base_class = if variant == TextareaVariant::Bordered {
        "textarea textarea-bordered".to_string()
    } else {
        format!("textarea {}", variant_class)
    };

    let combined_class = if class.is_empty() {
        base_class
    } else {
        format!("{} {}", base_class, class)
    };

    let handle_input = move |ev: web_sys::Event| {
        if let Some(cb) = on_input.clone() {
            use wasm_bindgen::JsCast;
            if let Ok(input_ev) = ev.dyn_into::<web_sys::InputEvent>() {
                let leptos_input_ev = InputEvent::new(&input_ev);
                (cb)(leptos_input_ev);
            }
        }
    };

    view! {
        <textarea
            class=combined_class
            placeholder=placeholder
            rows=rows.map(|r| r.to_string())
            prop:value=move || value.map(|v| v.get()).unwrap_or_default()
            on:input=handle_input
        ></textarea>
    }
}

