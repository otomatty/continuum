/**
 * Stats Component
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
 *   ├─ Spec: ./stats.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[component]
pub fn Stats(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let stats_class = if class.is_empty() {
        "stats shadow".to_string()
    } else {
        format!("stats shadow {}", class)
    };

    view! {
        <div class=stats_class>
            {children()}
        </div>
    }
}

#[component]
pub fn StatItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let stat_class = if class.is_empty() {
        "stat".to_string()
    } else {
        format!("stat {}", class)
    };

    view! {
        <div class=stat_class>
            {children()}
        </div>
    }
}

#[component]
pub fn StatTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let title_class = if class.is_empty() {
        "stat-title".to_string()
    } else {
        format!("stat-title {}", class)
    };

    view! {
        <div class=title_class>
            {children()}
        </div>
    }
}

#[component]
pub fn StatValue(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let value_class = if class.is_empty() {
        "stat-value".to_string()
    } else {
        format!("stat-value {}", class)
    };

    view! {
        <div class=value_class>
            {children()}
        </div>
    }
}

#[component]
pub fn StatDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let desc_class = if class.is_empty() {
        "stat-desc".to_string()
    } else {
        format!("stat-desc {}", class)
    };

    view! {
        <div class=desc_class>
            {children()}
        </div>
    }
}
