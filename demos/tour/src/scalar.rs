//! The tour's scalar abstraction (M4 PR 8b, D3): every scene builds
//! its geometry generically over [`Scalar`] so the SAME construction
//! code runs both the f64 tour and the K-telemetry Probe sweep
//! (`crate::probe`) — no duplicated scene data to drift.
//!
//! `f()` is the narration escape hatch: demo code asserts volume
//! oracles and formats captions in plain f64, which generic kernel
//! code must never do (the evaluation-code discipline routes every
//! decision through `Decide`). Demos are narration, not evaluation
//! code — and for both implementors the extraction is EXACT (`Probe`
//! is a transparent f64 wrapper; its arithmetic delegates verbatim),
//! so every oracle asserted at f64 holds bit-identically at Probe.

use pncad::document::ContentBits;
use pncad::geom_core::{Bounds, Decide};
// Only the `Scalar for Probe` impl below names it; the recording scalar
// rides the `probe` feature (see this crate's manifest).
#[cfg(feature = "probe")]
use pncad::geom_core::Probe;

/// A scalar the tour can build scenes at: kernel-decidable, document-
/// evaluable (the heat-sink recipe), and exactly f64-extractable for
/// narration.
pub trait Scalar:
    Decide + Bounds + ContentBits + pncad::topo::AtRestPolicy + Send + Sync + Copy + 'static
{
    /// The exact f64 value (narration/oracles only — never decisions).
    fn f(self) -> f64;
}

impl Scalar for f64 {
    fn f(self) -> f64 {
        self
    }
}

#[cfg(feature = "probe")]
impl Scalar for Probe {
    fn f(self) -> f64 {
        self.0
    }
}
