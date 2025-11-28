use leptos::ev::Event;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum RadioVariant {
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
pub fn Radio(
    name: String,
    value: String,
    #[prop(optional)] variant: RadioVariant,
    /// checkedプロパティはSignal<bool>を受け入れます。
    /// これにより、ReadSignal, RwSignal, Memo などを柔軟に渡せます。
    #[prop(optional, into)]
    checked: Signal<bool>,
    #[prop(optional, into)] on_change: Option<Callback<Event>>,
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

    let handle_change = move |ev: leptos::web_sys::Event| {
        if let Some(cb) = on_change {
            cb.run(ev);
        }
    };

    view! {
        <input
            type="radio"
            name=name
            value=value
            class=combined_class
            checked=move || checked.get()
            on:change=handle_change
        />
    }
}
