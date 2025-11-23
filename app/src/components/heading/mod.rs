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
macro_rules! make_heading {
    ($name:ident, $tag:ident, $default_class:literal) => {
        #[component]
        pub fn $name(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
            let combined_class = if class.is_empty() {
                $default_class.to_string()
            } else {
                format!("{} {}", $default_class, class)
            };

            view! {
                <$tag class=combined_class>
                    {children()}
                </$tag>
            }
        }
    };
}

make_heading!(Heading1, h1, "text-4xl md:text-5xl lg:text-6xl font-bold");
make_heading!(Heading2, h2, "text-3xl font-bold");
make_heading!(Heading3, h3, "text-2xl font-bold");
make_heading!(Heading4, h4, "text-xl font-semibold");
make_heading!(Heading5, h5, "text-lg font-semibold");
make_heading!(Heading6, h6, "text-base font-semibold");
