/**
 * Search Concept
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Concept):
 *   ├─ app/src/concepts/mod.rs
 *   ├─ app/src/pages/contributors/mod.rs (予定)
 *   └─ app/src/pages/repositories/mod.rs (予定)
 *
 * Related Documentation:
 *   ├─ Spec: ./search.spec.md
 *   ├─ Tests: ./tests.rs
 *   ├─ Plan: docs/03_plans/continuum/tasks/task-06-search-filter-concepts.md
 *   └─ Issue: (To be created if needed)
 */
pub mod actions;
pub mod state;

#[cfg(test)]
mod tests;

pub use actions::*;
pub use state::*;
