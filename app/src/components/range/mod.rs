use leptos::ev::InputEvent;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum RangeVariant {
    #[default]
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    Info,
}

#[component]
pub fn Range(
    #[prop(optional)] min: Option<f64>,
    #[prop(optional)] max: Option<f64>,
    #[prop(optional)] step: Option<f64>,
    #[prop(optional)] variant: RangeVariant,
    #[prop(optional, into)] value: Option<ReadSignal<f64>>,
    #[prop(optional, into)] on_input: Option<Callback<InputEvent>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match variant {
        RangeVariant::Primary => "range-primary",
        RangeVariant::Secondary => "range-secondary",
        RangeVariant::Accent => "range-accent",
        RangeVariant::Success => "range-success",
        RangeVariant::Warning => "range-warning",
        RangeVariant::Error => "range-error",
        RangeVariant::Info => "range-info",
    };

    let combined_class = if class.is_empty() {
        format!("range {}", variant_class)
    } else {
        format!("range {} {}", variant_class, class)
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
            type="range"
            min=min.map(|m| m.to_string())
            max=max.map(|m| m.to_string())
            step=step.map(|s| s.to_string())
            class=combined_class
            value=move || value.map(|v| v.get().to_string()).unwrap_or_default()
            on:input=handle_input
        />
    }
}
