use leptos::ev::MouseEvent;
/**
 * Accordion Component
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
 *   ├─ Spec: ./accordion.spec.md
 *   ├─ Tests: ./tests.rs
 *   └─ Module: ../mod.rs
 */
use leptos::prelude::*;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AccordionVariant {
    Arrow,
    Plus,
}

// AccordionItemのコンテキスト（toggle関数を子コンポーネントに提供）
#[derive(Clone, Copy)]
struct AccordionItemContext {
    toggle: Callback<()>,
}

impl Default for AccordionVariant {
    fn default() -> Self {
        AccordionVariant::Arrow
    }
}

#[component]
pub fn Accordion(
    #[prop(optional)] _variant: AccordionVariant,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
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
    #[prop(optional, into)] open: Option<ReadSignal<bool>>,
    #[prop(optional, into)] set_open: Option<WriteSignal<bool>>,
    #[prop(optional, into)] on_toggle: Option<Callback<()>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let internal_open = signal(false);
    let (is_open, set_is_open) =
        if let (Some(open_signal), Some(set_open_signal)) = (open, set_open) {
            (open_signal, set_open_signal)
        } else {
            internal_open
        };

    let handle_toggle = move |_: ()| {
        if let Some(callback) = on_toggle.clone() {
            callback.run(());
        }
        if open.is_none() {
            set_is_open.set(!is_open.get());
        }
    };

    // トグル関数をコンテキストとして提供
    let toggle_callback = Callback::new(handle_toggle);
    let context = AccordionItemContext {
        toggle: toggle_callback,
    };
    provide_context(context);

    let collapse_class = move || {
        let base = if is_open.get() {
            "collapse collapse-open"
        } else {
            "collapse"
        };
        if class.is_empty() {
            base.to_string()
        } else {
            format!("{} {}", base, class)
        }
    };

    view! {
        <div class=collapse_class>
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionHeader(
    #[prop(optional)] variant: AccordionVariant,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
    children: Children,
) -> impl IntoView {
    // AccordionItemから提供されるコンテキストを取得
    let context = use_context::<AccordionItemContext>();

    let variant_class = match variant {
        AccordionVariant::Arrow => "collapse-title",
        AccordionVariant::Plus => "collapse-title collapse-plus",
    };

    let header_class = if class.is_empty() {
        variant_class.to_string()
    } else {
        format!("{} {}", variant_class, class)
    };

    let handle_click = move |ev: MouseEvent| {
        // 外部のon_clickコールバックを呼び出す
        if let Some(cb) = on_click.clone() {
            cb.run(ev);
        }
        // コンテキストからtoggle関数を取得して呼び出す（AccordionItemの開閉を切り替え）
        if let Some(ctx) = context {
            ctx.toggle.run(());
        }
    };

    view! {
        <div
            class=header_class
            on:click=handle_click
        >
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
