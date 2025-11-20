/**
 * Repository Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/repository/mod.rs
 *   ├─ src/concepts/repository/actions.rs
 *   └─ src/concepts/repository/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   ├─ chrono::DateTime, chrono::Utc
 *   └─ crate::concepts::user::state::User (Note: User Conceptへの参照だが、型定義のみでロジックは持たない)
 *
 * Related Documentation:
 *   ├─ Spec: ./repository.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

// Note: User型はuser conceptから参照するが、Conceptの独立性を保つため、
// このファイルでは型定義のみを行い、ロジックは持たない
use crate::concepts::user::state::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContributorStats {
    pub user: User,
    pub commits: u32,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stars: u32,
    pub language: Option<String>,
    pub updated_at: DateTime<Utc>,
    pub contributors: Vec<ContributorStats>,
}

#[derive(Debug, Clone, Default)]
pub struct RepositoryState {
    pub repositories: Vec<Repository>,
}

