use leptos::ev::MouseEvent;
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
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
            cb.run(ev);
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
