use leptos::prelude::*;
use leptos::ev::ChangeEvent;

#[derive(Clone, Copy, PartialEq)]
pub enum ToggleVariant {
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    Info,
}

impl Default for ToggleVariant {
    fn default() -> Self {
        ToggleVariant::Primary
    }
}

#[component]
pub fn Toggle(
    #[prop(optional)] variant: ToggleVariant,
    #[prop(optional, into)] checked: Option<ReadSignal<bool>>,
    #[prop(optional, into)] on_change: Option<Callback<ChangeEvent>>,
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

    view! {
        <input
            type="checkbox"
            class=combined_class
            checked=move || checked.map(|c| c.get()).unwrap_or(false)
            on:change=on_change
        />
    }
}

