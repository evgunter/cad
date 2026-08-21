//! Shared test scaffolding for the whole tree — the pieces several
//! suites would otherwise each hand-roll a copy of.
//!
//! Three things today:
//!
//! - [`fuzz`], the harness every randomized falsification sweep draws
//!   its RNG, its per-run seed and its EFFORT dial from.
//! - [`source`], the shared *"is this text code?"* predicate for the
//!   guards that pin a claim about the code against the code (S117's
//!   named way out of five hand-rolled readers).
//! - [`vacuity`], the **anti-vacuity floor** — a statement of how much a
//!   sampling guard actually exercised, printed every run and asserted,
//!   so a run that exercised nothing goes red instead of green.
//!
//! # DEV-ONLY, by convention
//!
//! Nothing depends on this crate outside `[dev-dependencies]`, and
//! nothing should: a crate that no production manifest names cannot be
//! reached from a production build path.
//!
//! Being a LEAF with ZERO dependencies is the other half of the point.
//! `interval-transcendentals/` is its own workspace root and is
//! path-depended on by `geom-core` (the `interval` feature's backend),
//! so its tests could never have used a harness living in `geom-core`
//! without inverting the layering. Below everything, there is no cycle
//! to create.

pub mod fuzz;
pub mod source;
pub mod vacuity;
