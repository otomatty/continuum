pub mod actions;
/**
 * Organization Concept
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Concept):
 *   ├─ src/concepts/mod.rs
 *   ├─ src/pages/dashboard/mod.rs
 *   └─ src/synchronizations/ranking_sync.rs (Period enumを使用)
 *
 * Dependencies (External files that this Concept imports):
 *   ├─ ./state.rs
 *   └─ ./actions.rs
 *
 * Related Documentation:
 *   ├─ Spec: ./organization.spec.md
 *   ├─ Tests: ./tests.rs
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */
pub mod state;
#[cfg(test)]
mod tests;

pub use actions::*;
pub use state::*;
