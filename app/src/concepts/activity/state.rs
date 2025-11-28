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
 *   └─ chrono::DateTime, chrono::Utc
 *
 * Related Documentation:
 *   ├─ Spec: ./activity.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 *
 * Note: Legible Architecture の「状態の単一所有」原則に従い、
 * User/Repository は ID 参照のみ保持し、実際のオブジェクトは
 * Synchronization 層または UI 層で結合する
 */
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ActivityType {
    Commit,
    PullRequest,
    Review,
    Issue,
    Discussion,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Activity {
    pub id: String,
    pub activity_type: ActivityType,
    /// ユーザーの参照（ID のみ保持、Concept の独立性を維持）
    pub user_id: String,
    /// リポジトリの参照（ID のみ保持、Concept の独立性を維持）
    pub repository_id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub url: String,
}

#[derive(Debug, Clone, Default)]
pub struct ActivityState {
    pub activities: Vec<Activity>,
}
