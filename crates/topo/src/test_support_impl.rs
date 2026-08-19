//! Test-support vocabulary: items this crate's own test binaries must
//! be able to name, kept in one place so no suite mints its own copy.
//!
//! # The gate (stated once for this crate)
//!
//! Existence and visibility are separate questions and `lib.rs` gates
//! them separately.
//!
//! **Existence** — `#[cfg(any(debug_assertions, test, feature =
//! "test-support"))]` on the `mod` declaration. Each arm is a consumer
//! that must be able to name these items:
//!
//! - **`debug_assertions`** — [`ArenaCounts`] is the vehicle of the D1
//!   postcondition assert in [`crate::euler`], which is debug-only.
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
//!
//! **Visibility** — `#[cfg(any(test, feature = "test-support"))]` on
//! the `pub use ... as test_support` re-export. The only reason to
//! export any of this is a test naming it from another crate, so the
//! public door opens on the test arms alone: in a plain build, debug or
//! release, the module is private and `topo::test_support` does not
//! resolve. **Not public API, in any profile.**
//!
//! `cargo build --release` satisfies no arm of either gate, so nothing
//! here is compiled at all. `cargo test --release` satisfies `test` —
//! which is why `debug_assertions` alone cannot serve as the existence
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
    ///
    /// `pub(crate)`, never `pub`: an inherent method's reach follows
    /// its own visibility and its type's, not its module's, so a `pub`
    /// one here would be public API on [`Body`] whatever this module's
    /// door does. The cross-crate reader is the free `arena_counts` in
    /// the `test_support` facade.
    pub(crate) fn arena_counts(&self) -> ArenaCounts {
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
