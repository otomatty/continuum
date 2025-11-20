/**
 * Drawer Component
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
 *   ├─ Spec: ./drawer.spec.md
 *   └─ Module: ../mod.rs
 */

use leptos::prelude::*;
use leptos::ev::MouseEvent;

#[derive(Clone, Copy, PartialEq)]
pub enum DrawerSide {
    Left,
    Right,
}

impl Default for DrawerSide {
    fn default() -> Self {
        DrawerSide::Left
    }
}

#[component]
pub fn Drawer(
    open: ReadSignal<bool>,
    #[prop(optional)] side: DrawerSide,
    #[prop(optional, into)] on_close: Option<Callback<()>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let drawer_class = move || {
        let base = if open.get() {
            "drawer drawer-open"
        } else {
            "drawer"
        };
        if class.is_empty() {
            base.to_string()
        } else {
            format!("{} {}", base, class)
        }
    };

    provide_context(open);
    provide_context(on_close);
    provide_context(side);

    view! {
        <div class=drawer_class>
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerSide(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let side = use_context::<DrawerSide>().expect("DrawerSide must be used within Drawer");
    let open = use_context::<ReadSignal<bool>>().expect("DrawerSide must be used within Drawer");
    let on_close = use_context::<Option<Callback<()>>>().expect("DrawerSide must be used within Drawer");

    let side_class = move || {
        let base = match side {
            DrawerSide::Left => "drawer-side",
            DrawerSide::Right => "drawer-side drawer-end",
        };
        if class.is_empty() {
            base.to_string()
        } else {
            format!("{} {}", base, class)
        }
    };

    let handle_backdrop_click = move |_| {
        if let Some(callback) = on_close.clone() {
            (callback)(());
        }
    };

    view! {
        <div class=side_class>
            <label class="drawer-overlay" on:click=handle_backdrop_click></label>
            <aside class="menu min-h-full w-80 bg-base-200 p-4">
                {children()}
            </aside>
        </div>
    }
}

#[component]
pub fn DrawerContent(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let content_class = if class.is_empty() {
        "drawer-content".to_string()
    } else {
        format!("drawer-content {}", class)
    };

    view! {
        <div class=content_class>
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerToggle(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let open = use_context::<ReadSignal<bool>>().expect("DrawerToggle must be used within Drawer");
    let on_close = use_context::<Option<Callback<()>>>().expect("DrawerToggle must be used within Drawer");

    let handle_click = move |ev: MouseEvent| {
        if let Some(callback) = on_close.clone() {
            (callback)(());
        }
        if let Some(click_callback) = on_click.clone() {
            (click_callback)(ev);
        }
    };

    let toggle_class = if class.is_empty() {
        "btn btn-primary drawer-button".to_string()
    } else {
        format!("btn btn-primary drawer-button {}", class)
    };

    view! {
        <label for="drawer-toggle" class=toggle_class on:click=handle_click>
            {children()}
        </label>
    }
}

