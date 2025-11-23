use leptos::prelude::*;

/**
 * SectionTitle Component
 *
 * A refined section title component with optional subtitle and decorative elements.
 * Uses custom styling for the heading element.
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ (To be updated as components are used)
 *
 * Dependencies (External files that this component imports):
 *   └─ leptos::prelude
 */
#[component]
pub fn SectionTitle(
    title: String,
    #[prop(optional, into)] subtitle: Option<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] title_class: String,
    #[prop(optional, into)] subtitle_class: String,
    #[prop(optional, into)] margin_bottom: Option<String>,
    #[prop(optional, default = true)] show_decoration: bool,
) -> impl IntoView {
    let mb_class = margin_bottom.unwrap_or_else(|| "mb-16".to_string());
    let container_class = if class.is_empty() {
        format!("text-center {}", mb_class)
    } else {
        format!("text-center {} {}", mb_class, class)
    };

    let title_default_class =
        "text-4xl md:text-5xl lg:text-6xl font-semibold tracking-tight leading-tight";
    let title_combined_class = if title_class.is_empty() {
        title_default_class.to_string()
    } else {
        format!("{} {}", title_default_class, title_class)
    };

    let subtitle_default_class = "text-xl md:text-2xl text-gray-600 mt-6";
    let subtitle_combined_class = if subtitle_class.is_empty() {
        subtitle_default_class.to_string()
    } else {
        format!("{} {}", subtitle_default_class, subtitle_class)
    };

    let decoration = if show_decoration {
        Some(view! {
            <div class="flex justify-center mt-6">
                <div class="w-16 h-1 bg-gradient-to-r from-transparent via-primary to-transparent rounded-full"></div>
            </div>
        })
    } else {
        None
    };

    view! {
        <div class=container_class>
            <h2 class=title_combined_class>
                {title}
            </h2>
            {subtitle.map(|sub| {
                view! {
                    <p class=subtitle_combined_class>
                        {sub}
                    </p>
                }
            })}
            {decoration}
        </div>
    }
}
