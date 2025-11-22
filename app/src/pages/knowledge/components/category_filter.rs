use crate::components::select::{Select, SelectOption, SelectVariant};
use leptos::ev::Event;
use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

/**
 * CategoryFilter Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/knowledge/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/components/select.rs
 */
#[derive(Clone, Copy, PartialEq)]
pub enum KnowledgeCategory {
    All,
    Announcements,
    QAndA,
    ShowAndTell,
    Ideas,
    General,
}

impl KnowledgeCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            KnowledgeCategory::All => "All",
            KnowledgeCategory::Announcements => "Announcements",
            KnowledgeCategory::QAndA => "Q&A",
            KnowledgeCategory::ShowAndTell => "Show and Tell",
            KnowledgeCategory::Ideas => "Ideas",
            KnowledgeCategory::General => "General",
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s {
            "Announcements" => KnowledgeCategory::Announcements,
            "Q&A" => KnowledgeCategory::QAndA,
            "Show and Tell" => KnowledgeCategory::ShowAndTell,
            "Ideas" => KnowledgeCategory::Ideas,
            "General" => KnowledgeCategory::General,
            _ => KnowledgeCategory::All,
        }
    }
}

#[component]
pub fn CategoryFilter(
    #[prop(optional)] on_change: Option<Callback<KnowledgeCategory>>,
) -> impl IntoView {
    let (selected_category, set_selected_category) = signal(KnowledgeCategory::All);

    let (category_value, set_category_value) = signal(selected_category.get().as_str().to_string());

    view! {
        <Select
            variant=SelectVariant::Bordered
            value=category_value
            on_change=Callback::new(move |ev: Event| {
                let value = ev
                    .target()
                    .unwrap()
                    .dyn_into::<leptos::web_sys::HtmlSelectElement>()
                    .unwrap()
                    .value();
                let category = KnowledgeCategory::from_str(&value);
                set_selected_category.set(category);
                set_category_value.set(category.as_str().to_string());
                if let Some(callback) = on_change {
                    callback.run(category);
                }
            })
            class="w-full md:w-48".to_string()
        >
            <SelectOption value="All".to_string() selected=true>"All Categories"</SelectOption>
            <SelectOption value="Announcements".to_string()>"Announcements"</SelectOption>
            <SelectOption value="Q&A".to_string()>"Q&A"</SelectOption>
            <SelectOption value="Show and Tell".to_string()>"Show and Tell"</SelectOption>
            <SelectOption value="Ideas".to_string()>"Ideas"</SelectOption>
            <SelectOption value="General".to_string()>"General"</SelectOption>
        </Select>
    }
}
