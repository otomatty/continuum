use leptos::prelude::*;

#[component]
pub fn Card(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let card_class = if class.is_empty() {
        "card".to_string()
    } else {
        format!("card {}", class)
    };
    
    view! {
        <div class=card_class>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    let title_class = if class.is_empty() {
        "text-2xl font-bold mb-4".to_string()
    } else {
        format!("text-2xl font-bold mb-4 {}", class)
    };
    
    view! {
        <h2 class=title_class>
            {children()}
        </h2>
    }
}

#[component]
pub fn CardBody(
    #[prop(optional)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class>
            {children()}
        </div>
    }
}

