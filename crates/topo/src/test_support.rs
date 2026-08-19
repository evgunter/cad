//! Test-support vocabulary: items this crate's own test binaries must
//! be able to name, kept in one place so no suite mints its own copy.
//!
//! # The gate (stated once for this crate)
//!
//! The module declaration in `lib.rs` carries
//! `#[cfg(any(debug_assertions, test, feature = "test-support"))]`.
//! Each arm is a consumer that must be able to name these items:
//!
//! - **`test`** — the in-crate `mod tests` oracles, which compile with
//!   the library. Cargo unifies the self dev-dependency's features into
//!   that build too, so this arm is belt-and-braces: it keeps the gate
//!   true of any in-crate test build without depending on how features
//!   resolve.
//! - **`feature = "test-support"`** — this crate's `tests/` binaries.
//!   A `tests/` binary is a **separate crate** that links the library
//!   as an ordinary dependency, so it can name neither a
//!   `#[cfg(test)]` item (that cfg is off when the library is built
//!   as a dependency) nor a `pub(crate)` one — which is why an
//!   in-crate helper is invisible from `tests/` and every integration
//!   suite otherwise declares its own copy. The feature is
//!   off by default and turned on only by the **self
//!   dev-dependency** in this crate's manifest
//!   (`topo = { path = ".", features = ["test-support"] }`), so it is
//!   on exactly when this crate's own tests compile the library and
//!   off for every other build, including every downstream dependent.
//! - **`debug_assertions`** — [`ArenaCounts`] is also the vehicle of
//!   the D1 postcondition assert in [`crate::euler`], which is
//!   debug-only. One gate for one item: the counts exist wherever any
//!   of their three consumers do.
//!
//! **Not public API.** The `debug_assertions` arm makes this module
//! nameable from a downstream *debug* build; nothing outside this crate
//! may rely on that, since the same reference does not resolve in
//! release.
//!
//! `cargo build --release` satisfies none of the three arms (no
//! `debug_assertions`, no `test`, and the feature is off by default
//! and reachable only through a dev-dependency), so this module is
//! absent from release builds. `cargo test --release` satisfies
//! `test` — which is why `debug_assertions` alone cannot serve as the
//! gate.

use geom_core::Real;

use crate::body::Body;

/// The seven topology-arena lengths.
///
/// Captured for the debug postcondition's Euler-vector check in
/// [`crate::euler`], and compared directly by the in-crate and
/// integration test oracles.
/// A different quantity from the six-component Euler vector
/// `(v, e, f, h, r, s)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArenaCounts {
    /// Solids in the body.
    pub solids: usize,
    /// Shells across every solid.
    pub shells: usize,
    /// Faces across every shell.
    pub faces: usize,
    /// Loops across every face.
    pub loops: usize,
    /// Half-edges across every loop.
    pub half_edges: usize,
    /// Edges in the body.
    pub edges: usize,
    /// Vertices in the body.
    pub vertices: usize,
}

impl<T: Real> Body<T> {
    /// Captures the topology-arena lengths.
    pub fn arena_counts(&self) -> ArenaCounts {
        ArenaCounts {
            solids: self.solids.len(),
            shells: self.shells.len(),
            faces: self.faces.len(),
            loops: self.loops.len(),
            half_edges: self.half_edges.len(),
            edges: self.edges.len(),
            vertices: self.vertices.len(),
        }
    }
}
