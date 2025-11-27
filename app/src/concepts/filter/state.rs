use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// フィルター状態
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct FilterState {
    /// 選択された言語
    pub languages: HashSet<String>,
    /// 選択されたステータス
    pub statuses: HashSet<String>,
    /// 選択されたカテゴリ
    pub categories: HashSet<String>,
    /// ソート順
    pub sort_by: SortOption,
    /// ソート方向
    pub sort_direction: SortDirection,
}

/// ソートオプション
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum SortOption {
    Name,
    #[default]
    UpdatedAt,
    Stars,
    Contributions,
    CreatedAt,
}

/// ソート方向
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum SortDirection {
    Ascending,
    #[default]
    Descending,
}
