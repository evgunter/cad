//! Python bindings for the `pncad` authoring façade.
//!
//! User-facing documentation lives outside this crate's rustdoc: the
//! README (`crates/pncad-py/README.md`) covers installation and the
//! shape of the surface, `docs/GUIDE.md` §1.3/§2.8 is the quickstart
//! and the canonical journey, and `docs/guide/north-star-audit.md`
//! tracks what is not bound yet. What follows is for readers of *this
//! crate's source*.
//!
//! # What this crate binds
//!
//! The layer is fixed: **Python speaks
//! `Doc`/`DocEdit`/`evaluate`/persist, never an arena key.** There is
//! no parallel direct-at-kernel binding surface — the one-shot user
//! ("build a bracket, export STEP") is served by a small document, not
//! by a second API. This crate therefore wraps exactly the curated
//! document surface `pncad` re-exports, plus the typed quantities
//! at the boundary.
//!
//! # Build shape
//!
//! The `pyo3` dependency is optional and lives behind the non-default
//! `python` feature; see the header comment in `Cargo.toml` for why.
//! The consequence for readers of this file: everything that mentions
//! PyO3 sits under `#[cfg(feature = "python")]`, and the modules above
//! that line are ordinary Rust that the default workspace build
//! compiles and tests.

// One check finding's evidence as Python reads its attributes off
// it: exhaustive over the kernel enum, Python-independent so the
// default build compiles the drift alarm and can construct the arms
// no authoring door reaches.
pub mod check_payload;
pub mod edit_payload;
pub mod errors;
// One escalated predicate as the two doors that carry one publish it:
// the margin's own fork written once, Python-independent so both
// doors reach it under every feature.
pub mod escalation;
pub mod identity;
// One mate refusal's payload as Python reads its attributes off it:
// exhaustive over the kernel enum, Python-independent so the default
// build compiles the drift alarm and tests the projection.
pub mod mate_payload;
pub mod node_kind;
// The pick index's three numbers as Python reads them off a pick
// refusal: exhaustive over both kernel enums, Python-independent so
// the default build compiles the drift alarm and can construct the
// one arm no authoring door reaches.
pub mod pick_payload;
// The gathered product memoized on an evaluation: the behaviour
// behind four bound doors, Python-independent so the default build
// tests it.
pub mod product_memo;
// The slot alphabet read INWARD — the word a refusal answers with,
// back to the slot a door addresses at. Python-independent, so the
// default build compiles it and pins it against the forward map's own
// committed inventory.
pub mod slot_word;
pub mod tags;
// One validator finding as Python reads it: the words `tags` mints,
// assembled into the sequence the validate doors raise. Python-
// independent, so the default build tests it.
pub mod validation;

#[cfg(feature = "python")]
mod py;

#[cfg(test)]
mod prose_census;
#[cfg(test)]
mod surface_census;
#[cfg(test)]
mod tests;
