/**
 * Ranking Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/ranking/mod.rs
 *   ├─ src/concepts/ranking/actions.rs
 *   └─ src/concepts/ranking/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ crate::concepts::user::state::User (型定義のみ)
 *
 * Related Documentation:
 *   ├─ Spec: ./ranking.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */

use serde::{Serialize, Deserialize};

// Note: User型はuser conceptから参照するが、Conceptの独立性を保つため、
// このファイルでは型定義のみを行い、ロジックは持たない
use crate::concepts::user::state::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingEntry {
    pub user: User,
    pub commits: u32,
    pub prs: u32,
    pub reviews: u32,
    pub score: u32,
    pub rank: u32,
}

#[derive(Debug, Clone, Default)]
pub struct RankingState {
    pub entries: Vec<RankingEntry>,
}

