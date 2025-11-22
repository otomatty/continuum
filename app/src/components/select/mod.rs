use leptos::ev::Event;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum SelectVariant {
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
pub fn Select(
    #[prop(optional)] variant: SelectVariant,
    #[prop(optional, into)] value: Option<ReadSignal<String>>,
    #[prop(optional, into)] on_change: Option<Callback<Event>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        SelectVariant::Primary => "select-primary",
        SelectVariant::Secondary => "select-secondary",
        SelectVariant::Accent => "select-accent",
        SelectVariant::Error => "select-error",
        SelectVariant::Success => "select-success",
        SelectVariant::Warning => "select-warning",
        SelectVariant::Info => "select-info",
        SelectVariant::Ghost => "select-ghost",
        SelectVariant::Bordered => "select-bordered",
    };

    let base_class = if variant == SelectVariant::Bordered {
        "select select-bordered".to_string()
    } else {
        format!("select {}", variant_class)
    };

    let combined_class = if class.is_empty() {
        base_class
    } else {
        format!("{} {}", base_class, class)
    };

    let handle_change = move |ev: leptos::web_sys::Event| {
        if let Some(cb) = on_change {
            cb.run(ev);
        }
    };

    view! {
        <select
            class=combined_class
            prop:value=move || value.map(|v| v.get()).unwrap_or_default()
            on:change=handle_change
        >
            {children()}
        </select>
    }
}

#[component]
pub fn SelectOption(
    #[prop(optional)] disabled: bool,
    #[prop(optional)] selected: bool,
    #[prop(optional, into)] value: String,
    children: Children,
) -> impl IntoView {
    view! {
        <option
            disabled=if disabled { Some("") } else { None }
            selected=if selected { Some("") } else { None }
            value=value
        >
            {children()}
        </option>
    }
}
