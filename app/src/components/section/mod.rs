use crate::components::container::Container;
use leptos::prelude::*;

/**
 * Section Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/home/components/
 *
 * Dependencies (External files that this component imports):
 *   ├─ leptos::prelude
 *   └─ app/src/components/container/mod.rs
 *
 * Related Documentation:
 *   └─ Module: ../mod.rs
 */
#[component]
pub fn Section(
    children: Children,
    #[prop(optional, into, default = "py-32".to_string())] padding: String,
    #[prop(optional, into)] background: Option<String>,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    let mut section_classes = Vec::new();

    if !padding.is_empty() {
        section_classes.push(padding);
    }

    if let Some(bg) = background {
        if !bg.is_empty() {
            section_classes.push(bg);
        }
    }

    if let Some(c) = class {
        if !c.is_empty() {
            section_classes.push(c);
        }
    }

    let section_class = if section_classes.is_empty() {
        String::new()
    } else {
        section_classes.join(" ")
    };

    view! {
        <section class=section_class>
            <Container>
                {children()}
            </Container>
        </section>
    }
}
