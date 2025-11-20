/**
 * Activity Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/activity/mod.rs
 *   ├─ src/concepts/activity/actions.rs
 *   └─ src/concepts/activity/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   ├─ chrono::DateTime, chrono::Utc
 *   └─ crate::concepts::{user::state::User, repository::state::Repository} (型定義のみ)
 *
 * Related Documentation:
 *   ├─ Spec: ./activity.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */

use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

// Note: User型とRepository型は他のConceptから参照するが、
// Conceptの独立性を保つため、このファイルでは型定義のみを行い、ロジックは持たない
use crate::concepts::user::state::User;
use crate::concepts::repository::state::Repository;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActivityType {
    Commit,
    PullRequest,
    Review,
    Issue,
    Discussion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub id: String,
    pub activity_type: ActivityType,
    pub user: User,
    pub repository: Repository,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub url: String,
}

#[derive(Debug, Clone, Default)]
pub struct ActivityState {
    pub activities: Vec<Activity>,
}

