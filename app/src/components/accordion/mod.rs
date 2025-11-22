/**
 * Accordion Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Component):
 *   └─ app/src/pages/components/mod.rs
 *
 * Dependencies (External files that this Component imports):
 *   └─ leptos::prelude
 *
 * Related Documentation:
 *   ├─ Spec: ./accordion.spec.md
 *   ├─ Tests: ./tests.rs
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum AccordionVariant {
    Arrow,
    Plus,
    #[default]
    None,
}

#[component]
pub fn Accordion(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let accordion_class = if class.is_empty() {
        "".to_string()
    } else {
        class
    };

    view! {
        <div class=accordion_class>
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionItem(
    #[prop(optional)] variant: AccordionVariant,
    #[prop(optional, into)] open: Option<ReadSignal<bool>>,
    #[prop(optional, into)] set_open: Option<WriteSignal<bool>>,
    #[prop(optional, into)] on_toggle: Option<Callback<bool>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    // 内部状態または外部状態を使用
    let internal_open = signal(false);
    let (is_open, set_is_open) =
        if let (Some(open_signal), Some(set_open_signal)) = (open, set_open) {
            (open_signal, set_open_signal)
        } else {
            internal_open
        };

    // checkboxの変更イベントハンドラ
    let handle_change = move |ev: leptos::ev::Event| {
        let checked = event_target_checked(&ev);

        // 外部状態が提供されている場合のみ更新
        if open.is_none() {
            set_is_open.set(checked);
        }

        // コールバックを呼び出す
        if let Some(callback) = on_toggle {
            callback.run(checked);
        }
    };

    // variantに応じたクラスを追加
    let variant_class = match variant {
        AccordionVariant::Arrow => "collapse-arrow",
        AccordionVariant::Plus => "collapse-plus",
        AccordionVariant::None => "",
    };

    let collapse_class = move || {
        let base = if variant_class.is_empty() {
            "collapse".to_string()
        } else {
            format!("collapse {}", variant_class)
        };

        if class.is_empty() {
            base
        } else {
            format!("{} {}", base, class)
        }
    };

    view! {
        <div class=collapse_class>
            <input
                type="checkbox"
                checked=move || is_open.get()
                on:change=handle_change
            />
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let title_class = if class.is_empty() {
        "collapse-title".to_string()
    } else {
        format!("collapse-title {}", class)
    };

    view! {
        <div class=title_class>
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionContent(
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let content_class = if class.is_empty() {
        "collapse-content".to_string()
    } else {
        format!("collapse-content {}", class)
    };

    view! {
        <div class=content_class>
            {children()}
        </div>
    }
}
