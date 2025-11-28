/**
 * Contribution Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/contribution/mod.rs
 *   ├─ src/concepts/contribution/actions.rs
 *   └─ src/concepts/contribution/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ chrono::NaiveDate
 *
 * Related Documentation:
 *   ├─ Spec: ./contribution.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 *
 * Note: Legible Architecture の「状態の単一所有」原則に従い、
 * User/Repository/Period は ID 参照のみ保持し、実際のオブジェクトは
 * Synchronization 層または UI 層で結合する
 */
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// Period enumを Contribution Concept 内で定義（独立性の維持）
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContributionPeriod {
    Weekly,
    Monthly,
    All,
}

impl Default for ContributionPeriod {
    fn default() -> Self {
        Self::Monthly
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContributionDay {
    pub date: NaiveDate,
    pub commits: u32,
    pub prs: u32,
    pub reviews: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContributionGraph {
    /// ユーザーの参照（ID のみ保持、Concept の独立性を維持）
    pub user_id: String,
    pub data: Vec<ContributionDay>,
    pub period: ContributionPeriod,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RepositoryContribution {
    /// リポジトリの参照（ID のみ保持、Concept の独立性を維持）
    pub repository_id: String,
    pub commits: u32,
    pub prs: u32,
    pub reviews: u32,
    pub lines_added: u32,
    pub lines_deleted: u32,
    pub percentage: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ContributionState {
    pub graphs: Vec<ContributionGraph>,
    pub repository_contributions: Vec<RepositoryContribution>,
}
