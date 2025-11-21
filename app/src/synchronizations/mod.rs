/**
 * Synchronizations Module
 *
 * DEPENDENCY MAP:
 *
 * Parents (Files that import this module):
 *   └─ app/src/lib.rs
 *
 * Dependencies (Synchronization modules):
 *   └─ ranking_sync
 *
 * Related Documentation:
 *   └─ docs/00_prompts/legible-architecture.md
 */

pub mod ranking_sync;

#[cfg(test)]
mod ranking_sync_test;

pub use ranking_sync::*;

