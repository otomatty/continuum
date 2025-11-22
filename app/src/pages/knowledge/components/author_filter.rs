use crate::components::select::{Select, SelectOption, SelectVariant};
use leptos::ev::Event;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/**
 * AuthorFilter Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/knowledge/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/select.rs
 */
#[component]
pub fn AuthorFilter(
    authors: Vec<String>,
    #[prop(optional)] on_change: Option<Callback<String>>,
) -> impl IntoView {
    let (selected_author, set_selected_author) = signal(String::new());

    let handle_change = move |ev: Event| {
        let value = ev
            .target()
            .unwrap()
            .dyn_into::<leptos::web_sys::HtmlSelectElement>()
            .unwrap()
            .value();
        set_selected_author.set(value.clone());
        if let Some(callback) = on_change {
            callback.run(value);
        }
    };

    view! {
        <Select
            variant=SelectVariant::Bordered
            value=selected_author
            on_change=Callback::new(handle_change)
            class="w-full md:w-48".to_string()
        >
            <SelectOption value="".to_string() selected=true>"All Authors"</SelectOption>
            {authors.iter().map(|author| {
                let author_clone = author.clone();
                view! {
                    <SelectOption value=author_clone.clone()>{author_clone}</SelectOption>
                }
            }).collect_view()}
        </Select>
    }
}
