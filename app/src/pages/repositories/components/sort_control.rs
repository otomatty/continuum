use crate::concepts::filter::{SortDirection, SortOption};
use leptos::prelude::*;

/**
 * SortControl Component
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this component):
 *   └─ app/src/pages/repositories/components/mod.rs
 *
 * Dependencies (External files that this component imports):
 *   └─ app/src/concepts/filter/mod.rs
 */
#[component]
pub fn SortControl(
    sort_by: SortOption,
    direction: SortDirection,
    on_change: Callback<(SortOption, SortDirection)>,
) -> impl IntoView {
    // このコンポーネントでサポートするソートオプション
    // SortOptionには他にContributions, CreatedAtがあるが、
    // リポジトリ一覧ではこの3つのみを使用
    let options = vec![
        (SortOption::UpdatedAt, "更新日"),
        (SortOption::Stars, "Star数"),
        (SortOption::Name, "名前"),
    ];

    // SortOption, SortDirectionはCopyを実装しているため、明示的なclone()は不要
    // ただし、所有権の移動を明確にするため変数を分けて定義
    let sort_by_for_options = sort_by;
    let sort_by_for_button = sort_by;
    let direction_for_select = direction;
    let direction_for_button = direction;

    view! {
        <div class="flex items-center gap-2">
            <select
                class="select select-bordered select-sm"
                on:change=move |ev| {
                    let value = event_target_value(&ev);
                    // サポートするオプションのみを明示的にマッチ
                    let new_sort = match value.as_str() {
                        "updated" => SortOption::UpdatedAt,
                        "stars" => SortOption::Stars,
                        "name" => SortOption::Name,
                        // 上記のオプション以外は来ないはずだが、
                        // デフォルト値を設定（防御的プログラミング）
                        _ => SortOption::UpdatedAt,
                    };
                    on_change.run((new_sort, direction_for_select));
                }
            >
                {options
                    .into_iter()
                    .map(|(opt, label)| {
                        // サポートするオプションのみを明示的にマッチ
                        let value = match opt {
                            SortOption::UpdatedAt => "updated",
                            SortOption::Stars => "stars",
                            SortOption::Name => "name",
                            // 上記以外のバリアント（Contributions, CreatedAt）は
                            // optionsに含まれないため、このブランチは到達しない
                            SortOption::Contributions | SortOption::CreatedAt => "updated",
                        };
                        // PartialEqを実装しているため、直接==で比較可能
                        let is_selected = sort_by_for_options == opt;
                        view! {
                            <option value=value selected=is_selected>
                                {label}
                            </option>
                        }
                    })
                    .collect_view()}
            </select>

            <button
                class="btn btn-sm btn-ghost"
                on:click=move |_| {
                    let new_direction = match direction {
                        SortDirection::Ascending => SortDirection::Descending,
                        SortDirection::Descending => SortDirection::Ascending,
                    };
                    on_change.run((sort_by_for_button, new_direction));
                }
            >
                {move || {
                    match direction_for_button {
                        SortDirection::Descending => "↓",
                        SortDirection::Ascending => "↑",
                    }
                }}
            </button>
        </div>
    }
}
