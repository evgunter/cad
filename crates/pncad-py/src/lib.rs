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

pub mod errors;
pub mod identity;
pub mod tags;

#[cfg(feature = "python")]
mod py;

#[cfg(test)]
mod tests;
