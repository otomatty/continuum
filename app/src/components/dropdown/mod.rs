/**
 * Dropdown Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Component):
 *   └─ (To be added when used)
 *
 * Dependencies (External files that this Component imports):
 *   └─ leptos::prelude
 *
 * Related Documentation:
 *   ├─ Spec: ./dropdown.spec.md
 *   └─ Module: ../mod.rs
 */

use leptos::prelude::*;
use leptos::ev::MouseEvent;

#[derive(Clone, Copy, PartialEq)]
pub enum DropdownVariant {
    Hover,
    Click,
}

impl Default for DropdownVariant {
    fn default() -> Self {
        DropdownVariant::Click
    }
}

#[component]
pub fn Dropdown(
    open: ReadSignal<bool>,
    #[prop(optional)] variant: DropdownVariant,
    #[prop(optional, into)] on_toggle: Option<Callback<()>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        DropdownVariant::Hover => "dropdown dropdown-hover",
        DropdownVariant::Click => "dropdown",
    };

    let dropdown_class = move || {
        let base = if open.get() {
            format!("{} dropdown-open", variant_class)
        } else {
            variant_class.to_string()
        };
        if class.is_empty() {
            base
        } else {
            format!("{} {}", base, class)
        }
    };

    provide_context(open);
    provide_context(on_toggle);

    view! {
        <div class=dropdown_class>
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownButton(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<ReadSignal<bool>>().expect("DropdownButton must be used within Dropdown");
    let on_toggle = use_context::<Option<Callback<()>>>().expect("DropdownButton must be used within Dropdown");

    let handle_click = move |ev: MouseEvent| {
        if let Some(callback) = on_toggle.clone() {
            (callback)(());
        }
        if let Some(click_callback) = on_click.clone() {
            (click_callback)(ev);
        }
    };

    let button_class = if class.is_empty() {
        "btn".to_string()
    } else {
        format!("btn {}", class)
    };

    view! {
        <label tabindex="0" class=button_class on:click=handle_click>
            {children()}
        </label>
    }
}

#[component]
pub fn DropdownMenu(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let menu_class = if class.is_empty() {
        "dropdown-content menu bg-base-200 rounded-box z-[1] w-52 p-2 shadow".to_string()
    } else {
        format!("dropdown-content menu bg-base-200 rounded-box z-[1] w-52 p-2 shadow {}", class)
    };

    view! {
        <ul tabindex="0" class=menu_class>
            {children()}
        </ul>
    }
}

#[component]
pub fn DropdownItem(
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let item_class = if class.is_empty() {
        "".to_string()
    } else {
        class
    };

    let handle_click = move |ev: MouseEvent| {
        if let Some(cb) = on_click.clone() {
            (cb)(ev);
        }
    };

    view! {
        <li class=item_class>
            {move || {
                if let Some(link) = href {
                    view! {
                        <a href=link on:click=handle_click>
                            {children()}
                        </a>
                    }.into_view()
                } else {
                    view! {
                        <a on:click=handle_click>
                            {children()}
                        </a>
                    }.into_view()
                }
            }}
        </li>
    }
}

