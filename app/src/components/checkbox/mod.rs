use leptos::ev::Event;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum CheckboxVariant {
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
pub fn Checkbox(
    #[prop(optional)] variant: CheckboxVariant,
    /// checkedプロパティはSignal<bool>を受け入れます。
    /// これにより、ReadSignal, RwSignal, Memo などを柔軟に渡せます。
    #[prop(optional, into)]
    checked: Signal<bool>,
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

    let handle_change = move |ev: leptos::web_sys::Event| {
        if let Some(cb) = on_change {
            cb.run(ev);
        }
    };

    view! {
        <input
            type="checkbox"
            class=combined_class
            checked=move || checked.get()
            on:change=handle_change
        />
    }
}
