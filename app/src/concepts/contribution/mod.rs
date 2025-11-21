/**
 * Contribution Concept
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Concept):
 *   ├─ src/concepts/mod.rs
 *   └─ src/pages/portfolio/mod.rs
 *
 * Dependencies (External files that this Concept imports):
 *   ├─ ./state.rs
 *   └─ ./actions.rs
 *
 * Related Documentation:
 *   ├─ Spec: ./contribution.spec.md
 *   ├─ Tests: ./tests.rs
 *   └─ Plan: docs/03_plans/continuum/legible-architecture-refactoring.md
 */

pub mod state;
pub mod actions;
#[cfg(test)]
mod tests;

pub use state::*;
pub use actions::*;

