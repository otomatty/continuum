/**
 * Breadcrumbs Component
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
 *   ├─ Spec: ./breadcrumbs.spec.md
 *   └─ Module: ../mod.rs
 */

use leptos::prelude::*;

#[component]
pub fn Breadcrumbs(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let breadcrumbs_class = if class.is_empty() {
        "breadcrumbs".to_string()
    } else {
        format!("breadcrumbs {}", class)
    };

    view! {
        <nav class=breadcrumbs_class>
            <ul>
                {children()}
            </ul>
        </nav>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let item_class = if class.is_empty() {
        "".to_string()
    } else {
        class
    };

    view! {
        <li class=item_class>
            {move || {
                if let Some(link) = href {
                    view! {
                        <a href=link>
                            {children()}
                        </a>
                    }.into_view()
                } else {
                    view! {
                        <span>
                            {children()}
                        </span>
                    }.into_view()
                }
            }}
        </li>
    }
}

