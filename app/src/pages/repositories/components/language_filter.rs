use crate::components::checkbox::Checkbox;
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
            current.push(lang.clone());
        }
        selected_languages.set(current.clone());
        on_change.run(current);
    };

    view! {
        <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn btn-sm btn-ghost gap-2">
                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z" />
                </svg>
                "言語"
                {move || {
                    let count = selected_languages.get().len();
                    if count > 0 {
                        Some(view! {
                            <span class="badge badge-sm badge-primary">{count}</span>
                        })
                    } else {
                        None
                    }
                }}
            </div>
            <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-base-200 rounded-box w-52 max-h-60 overflow-y-auto">
                {available_languages.into_iter().map(move |lang| {
                    let lang_for_check = lang.clone();
                    let lang_for_toggle = lang.clone();
                    // 派生シグナルを使用（効率的なパターン）
                    // RwSignal + Effect の代わりに Memo を使用することで、
                    // 不要なリアクティブのオーバーヘッドを削減
                    let is_selected = Memo::new(move |_| {
                        selected_languages.get().contains(&lang_for_check)
                    });
                    view! {
                        <li>
                            <label class="label cursor-pointer justify-start gap-2">
                                <Checkbox
                                    checked=is_selected
                                    on_change=Callback::new(move |_| handle_toggle(lang_for_toggle.clone()))
                                />
                                <span>{lang.clone()}</span>
                            </label>
                        </li>
                    }
                }).collect_view()}
            </ul>
        </div>
    }
}
