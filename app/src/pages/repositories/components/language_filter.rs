use crate::components::{
    card::{Card, CardBody, CardTitle},
    checkbox::Checkbox,
};
use leptos::prelude::*;

/**
 * LanguageFilter Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/repositories/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   ├─ app/src/components/card.rs
 *   └─ app/src/components/checkbox.rs
 */
#[component]
pub fn LanguageFilter(
    selected_languages: RwSignal<Vec<String>>,
    available_languages: Vec<String>,
    on_change: Callback<Vec<String>>,
) -> impl IntoView {
    let handle_toggle = move |lang: String| {
        let mut current = selected_languages.get();
        if current.contains(&lang) {
            current.retain(|l| l != &lang);
        } else {
            current.push(lang);
        }
        selected_languages.set(current.clone());
        on_change(current);
    };

    view! {
        <Card>
            <CardTitle>"Filter by Language"</CardTitle>
            <CardBody>
                <div class="flex flex-col gap-2 max-h-64 overflow-y-auto">
                    {available_languages.into_iter().map(|lang| {
                        let lang_clone = lang.clone();
                        let is_checked = move || selected_languages.get().contains(&lang_clone);
                        view! {
                            <label class="flex items-center gap-2 cursor-pointer">
                                <Checkbox
                                    checked=is_checked()
                                    on_change=Callback::new(move |_| handle_toggle(lang_clone.clone()))
                                />
                                <span class="text-sm">{lang_clone.clone()}</span>
                            </label>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </CardBody>
        </Card>
    }
}
