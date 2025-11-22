use leptos::ev::Event;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum ToggleVariant {
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
pub fn Toggle(
    #[prop(optional)] variant: ToggleVariant,
    #[prop(optional, into)] checked: Option<ReadSignal<bool>>,
    #[prop(optional, into)] on_change: Option<Callback<Event>>,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match variant {
        ToggleVariant::Primary => "toggle-primary",
        ToggleVariant::Secondary => "toggle-secondary",
        ToggleVariant::Accent => "toggle-accent",
        ToggleVariant::Success => "toggle-success",
        ToggleVariant::Warning => "toggle-warning",
        ToggleVariant::Error => "toggle-error",
        ToggleVariant::Info => "toggle-info",
    };

    let combined_class = if class.is_empty() {
        format!("toggle {}", variant_class)
    } else {
        format!("toggle {} {}", variant_class, class)
    };

    let handle_change = move |ev: leptos::web_sys::Event| {
        if let Some(cb) = on_change {
            cb.run(ev);
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
