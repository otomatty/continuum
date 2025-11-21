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
 *   ├─ chrono::NaiveDate
 *   ├─ crate::concepts::user::state::User (型定義のみ)
 *   ├─ crate::concepts::repository::state::Repository (型定義のみ)
 *   └─ crate::concepts::organization::state::Period (型定義のみ)
 *
 * Related Documentation:
 *   ├─ Spec: ./contribution.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */

use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

// Note: User型、Repository型、Period型は他のConceptから参照するが、
// Conceptの独立性を保つため、このファイルでは型定義のみを行い、ロジックは持たない
use crate::concepts::user::state::User;
use crate::concepts::repository::state::Repository;
use crate::concepts::organization::state::Period;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionDay {
    pub date: NaiveDate,
    pub commits: u32,
    pub prs: u32,
    pub reviews: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributionGraph {
    pub user: User,
    pub data: Vec<ContributionDay>,
    pub period: Period,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryContribution {
    pub repository: Repository,
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

