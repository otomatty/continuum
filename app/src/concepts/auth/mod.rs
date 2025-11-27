/**
 * Auth Concept
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this Concept):
 *   ├─ app/src/concepts/mod.rs
 *   ├─ app/src/hooks/use_auth.rs
 *   └─ app/src/pages/auth_error/mod.rs
 *
 * Related Documentation:
 *   ├─ Spec: ./auth.spec.md
 *   ├─ Tests: ./tests.rs
 *   └─ Implementation Roadmap: docs/03_plans/continuum/20251121_implementation-roadmap.md
 */
pub mod actions;
pub mod state;

#[cfg(test)]
mod tests;

// Re-export commonly used types
pub use actions::*;
pub use state::*;
