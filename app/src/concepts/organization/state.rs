/**
 * Organization Concept - State
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this file):
 *   ├─ src/concepts/organization/mod.rs
 *   ├─ src/concepts/organization/actions.rs
 *   └─ src/concepts/organization/tests.rs
 *
 * Dependencies (External files that this file imports):
 *   └─ (なし - 独立したConcept)
 *
 * Related Documentation:
 *   ├─ Spec: ./organization.spec.md
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Period {
    Weekly,
    Monthly,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationStats {
    pub total_contributors: u32,
    pub total_repositories: u32,
    pub external_prs_count: u32,
    pub total_commits: u32,
    pub period: Period,
}

#[derive(Debug, Clone, Default)]
pub struct OrganizationState {
    pub stats: Vec<OrganizationStats>,
}

