use leptos::ev::MouseEvent;
/**
 * Tabs Component
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
 *   ├─ Spec: ./tabs.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum TabsVariant {
    Bordered,
    Lifted,
}

impl Default for TabsVariant {
    fn default() -> Self {
        TabsVariant::Bordered
    }
}

#[component]
pub fn Tabs(
    #[prop(optional)] variant: TabsVariant,
    #[prop(optional, into)] default_active: Option<usize>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let (active_index, set_active_index) = signal(default_active.unwrap_or(0));

    let variant_class = match variant {
        TabsVariant::Bordered => "tabs tabs-bordered",
        TabsVariant::Lifted => "tabs tabs-lifted",
    };

    let combined_class = if class.is_empty() {
        variant_class.to_string()
    } else {
        format!("{} {}", variant_class, class)
    };

    provide_context(active_index);
    provide_context(set_active_index);

    view! {
        <div class=combined_class>
            {children()}
        </div>
    }
}

#[component]
pub fn TabList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let list_class = if class.is_empty() {
        "".to_string()
    } else {
        class
    };

    view! {
        <div class=list_class>
            {children()}
        </div>
    }
}

#[component]
pub fn Tab(
    index: usize,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    let active_index = use_context::<ReadSignal<usize>>().expect("Tab must be used within Tabs");
    let set_active_index =
        use_context::<WriteSignal<usize>>().expect("Tab must be used within Tabs");

    let is_active = move || active_index.get() == index;

    let tab_class = move || {
        let base = if is_active() { "tab tab-active" } else { "tab" };
        if class.is_empty() {
            base.to_string()
        } else {
            format!("{} {}", base, class)
        }
    };

    let handle_click = move |ev: MouseEvent| {
        set_active_index.set(index);
        if let Some(callback) = on_click.clone() {
            callback.run(ev);
        }
    };

    view! {
        <a
            class=tab_class
            on:click=handle_click
            role="tab"
            aria-selected=move || is_active().to_string()
        >
            {children()}
        </a>
    }
}

#[component]
pub fn TabPanel(
    index: usize,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let active_index =
        use_context::<ReadSignal<usize>>().expect("TabPanel must be used within Tabs");

    let is_active = move || active_index.get() == index;

    let panel_class = if class.is_empty() {
        "".to_string()
    } else {
        class
    };

    view! {
        <div
            class=panel_class
            role="tabpanel"
            hidden=move || !is_active()
        >
            {children()}
        </div>
    }
}
