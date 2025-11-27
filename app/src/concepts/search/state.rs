use serde::{Deserialize, Serialize};

/// 検索状態
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchState {
    /// 検索クエリ
    pub query: String,
    /// 検索対象のフィールド
    pub search_fields: Vec<SearchField>,
    /// 検索結果の件数
    pub result_count: Option<usize>,
}

/// 検索対象フィールド
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SearchField {
    Name,
    Description,
    Author,
    Tag,
    All,
}

impl Default for SearchField {
    fn default() -> Self {
        SearchField::All
    }
}

