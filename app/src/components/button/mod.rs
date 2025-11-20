use leptos::prelude::*;
use leptos::ev::MouseEvent;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Ghost,
}

impl Default for ButtonVariant {
    fn default() -> Self {
        ButtonVariant::Primary
    }
}

#[component]
pub fn Button(
    #[prop(optional)] variant: ButtonVariant,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        ButtonVariant::Primary => "btn-primary",
        ButtonVariant::Secondary => "btn-secondary",
        ButtonVariant::Ghost => "btn-ghost",
    };

    let combined_class = if class.is_empty() {
        format!("btn {}", variant_class)
    } else {
        format!("btn {} {}", variant_class, class)
    };

    let handle_click = move |ev: MouseEvent| {
        if let Some(cb) = on_click {
            (cb)(ev);
        }
    };

    view! {
        <button 
            class=combined_class
            on:click=handle_click
        >
            {children()}
        </button>
    }
}

