mod section_title;

use leptos::prelude::*;

pub use section_title::SectionTitle;

/**
 * Heading Components
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import these components):
 *   └─ (To be updated as components are used)
 *
 * Dependencies (External files that these components import):
 *   └─ leptos::prelude
 *
 * Related Documentation:
 *   └─ Spec: ./heading.spec.md
 */

#[component]
pub fn Heading1(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let default_class = "text-4xl md:text-5xl lg:text-6xl font-bold";
    let combined_class = if class.is_empty() {
        default_class.to_string()
    } else {
        format!("{} {}", default_class, class)
    };

    view! {
        <h1 class=combined_class>
            {children()}
        </h1>
    }
}

#[component]
pub fn Heading2(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let default_class = "text-3xl font-bold";
    let combined_class = if class.is_empty() {
        default_class.to_string()
    } else {
        format!("{} {}", default_class, class)
    };

    view! {
        <h2 class=combined_class>
            {children()}
        </h2>
    }
}

#[component]
pub fn Heading3(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let default_class = "text-2xl font-bold";
    let combined_class = if class.is_empty() {
        default_class.to_string()
    } else {
        format!("{} {}", default_class, class)
    };

    view! {
        <h3 class=combined_class>
            {children()}
        </h3>
    }
}

#[component]
pub fn Heading4(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let default_class = "text-xl font-semibold";
    let combined_class = if class.is_empty() {
        default_class.to_string()
    } else {
        format!("{} {}", default_class, class)
    };

    view! {
        <h4 class=combined_class>
            {children()}
        </h4>
    }
}

#[component]
pub fn Heading5(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let default_class = "text-lg font-semibold";
    let combined_class = if class.is_empty() {
        default_class.to_string()
    } else {
        format!("{} {}", default_class, class)
    };

    view! {
        <h5 class=combined_class>
            {children()}
        </h5>
    }
}

#[component]
pub fn Heading6(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let default_class = "text-base font-semibold";
    let combined_class = if class.is_empty() {
        default_class.to_string()
    } else {
        format!("{} {}", default_class, class)
    };

    view! {
        <h6 class=combined_class>
            {children()}
        </h6>
    }
}
