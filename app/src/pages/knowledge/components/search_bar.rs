use crate::components::input::{Input, InputVariant};
use leptos::ev::InputEvent;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/**
 * SearchBar Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/knowledge/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/input.rs
 */
#[component]
pub fn SearchBar(
    #[prop(optional, into)] placeholder: String,
    #[prop(optional)] on_search: Option<Callback<String>>,
) -> impl IntoView {
    let (search_value, set_search_value) = signal(String::new());

    let handle_input = move |ev: InputEvent| {
        let value = ev
            .target()
            .unwrap()
            .dyn_into::<leptos::web_sys::HtmlInputElement>()
            .unwrap()
            .value();
        set_search_value.set(value.clone());
        if let Some(callback) = on_search {
            callback.run(value);
        }
    };

    let placeholder_text = if placeholder.is_empty() {
        "検索..."
    } else {
        placeholder.as_str()
    };

    view! {
        <div class="relative">
            <Input
                variant=InputVariant::Bordered
                placeholder=placeholder_text.to_string()
                value=search_value
                on_input=Callback::new(handle_input)
                class="w-full pl-10".to_string()
            />
            <div class="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400">
                "🔍"
            </div>
        </div>
    }
}
