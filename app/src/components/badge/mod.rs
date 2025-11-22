use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BadgeVariant {
    Primary,
    Secondary,
    Accent,
    Success,
    Warning,
    Error,
    Info,
    Ghost,
}

#[component]
pub fn Badge(
    variant: BadgeVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant {
        BadgeVariant::Primary => "badge-primary",
        BadgeVariant::Secondary => "badge-secondary",
        BadgeVariant::Accent => "badge-accent",
        BadgeVariant::Success => "badge-success",
        BadgeVariant::Warning => "badge-warning",
        BadgeVariant::Error => "badge-error",
        BadgeVariant::Info => "badge-info",
        BadgeVariant::Ghost => "badge-ghost",
    };

    let badge_class = if class.is_empty() {
        format!("badge {}", variant_class)
    } else {
        format!("badge {} {}", variant_class, class)
    };

    view! {
        <span class=badge_class>
            {children()}
        </span>
    }
}
