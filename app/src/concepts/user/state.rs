/**
 * User Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/user/mod.rs
 *   ├─ src/concepts/user/actions.rs
 *   └─ src/concepts/user/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ chrono::DateTime, chrono::Utc
 *
 * Related Documentation:
 *   ├─ Spec: ./user.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserRole {
    CurrentEmployee,
    Alumni,
    ExternalContributor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub username: String,
    pub display_name: String,
    pub avatar_url: String,
    pub github_url: String,
    pub role: UserRole,
    pub joined_at: Option<DateTime<Utc>>,
    pub left_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Default)]
pub struct UserState {
    pub users: Vec<User>,
    pub current_user_id: Option<String>,
}
