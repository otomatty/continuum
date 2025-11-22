/**
 * Loading Concept
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Concept):
 *   └─ (To be added when used)
 *
 * Dependencies (External files that this Concept imports):
 *   └─ leptos::prelude
 *
 * Related Documentation:
 *   ├─ Spec: ./loading.spec.md
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum LoadingVariant {
    #[default]
    Spinner,
    Dots,
    Ring,
    Ball,
    Bars,
    Infinity,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum LoadingSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
}

#[component]
pub fn Loading(
    #[prop(optional)] variant: LoadingVariant,
    #[prop(optional)] size: LoadingSize,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let variant_class = match variant {
        LoadingVariant::Spinner => "loading-spinner",
        LoadingVariant::Dots => "loading-dots",
        LoadingVariant::Ring => "loading-ring",
        LoadingVariant::Ball => "loading-ball",
        LoadingVariant::Bars => "loading-bars",
        LoadingVariant::Infinity => "loading-infinity",
    };

    let size_class = match size {
        LoadingSize::Xs => "loading-xs",
        LoadingSize::Sm => "loading-sm",
        LoadingSize::Md => "loading-md",
        LoadingSize::Lg => "loading-lg",
    };

    let combined_class = if class.is_empty() {
        format!("loading {} {}", variant_class, size_class)
    } else {
        format!("loading {} {} {}", variant_class, size_class, class)
    };

    view! {
        <span class=combined_class></span>
    }
}
