//! Python bindings for the `pncad` authoring façade (LIB-U9S).
//!
//! # What this crate binds
//!
//! LIBRARY-DESIGN §L3 fixes the layer: **Python speaks
//! `Doc`/`DocEdit`/`evaluate`/persist, never an arena key.** There is
//! no parallel direct-at-kernel binding surface — the one-shot user
//! ("build a bracket, export STEP") is served by a small document, not
//! by a second API. This crate therefore wraps exactly the curated
//! document surface `pncad` re-exports, plus the §L4 typed quantities
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
pub mod tags;

#[cfg(feature = "python")]
mod py;

#[cfg(test)]
mod tests;
