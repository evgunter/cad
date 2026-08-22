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
//!
//! # Which home a new test item goes in (this crate has three)
//!
//! Stated here because this is the module the gate's argument lives in;
//! the other two point at this paragraph rather than restating it. The
//! question that routes an item is **who needs to name it**:
//!
//! - **The library itself needs it** (a debug assert, an in-crate
//!   oracle) *and* a `tests/` binary does — **here**. That is the only
//!   case that needs the split gate, because the item must exist more
//!   widely than it is exported. [`ArenaCounts`] is the whole
//!   population today.
//! - **Only the crate's own `mod tests` needs it** — `src/fixtures.rs`,
//!   which is plain `#[cfg(test)]` (and so is not linkable from here:
//!   it does not exist in the doc build). It costs a
//!   `tests/` binary nothing because it is not compiled for one, and it
//!   needs no feature. Do not move an item here just to share it with
//!   `tests/`: a `tests/` binary cannot name it.
//! - **Only `tests/` binaries need it, and the library never does** —
//!   `tests/common/mod.rs`, which is compiled into the test binary
//!   itself. Nothing in the library pays for it and no feature is
//!   involved, so this is the cheapest home and the right default for
//!   test-only scaffolding. The geometric cube lives there for exactly
//!   this reason.
//!
//! The rule in one line: **an item lives at the narrowest of the three
//! that all of its consumers can reach.** Reaching for this module when
//! `tests/common` would do widens the library for nothing.

use geom_core::Real;

use crate::body::Body;

/// The seven topology-arena lengths.
///
/// Captured for the debug postcondition's Euler-vector check in
/// [`crate::euler`], and compared directly by the in-crate and
/// integration test oracles.
/// A different quantity from the six-component Euler vector
/// `(v, e, f, h, r, s)`.
///
/// The crate's single spelling of the topology census: the test-only
/// `ArenaSnapshot` (`crate::fixtures`, not linked because it is
/// `#[cfg(test)]` and absent from a doc build) holds one of these
/// alongside the three geometry-arena lengths rather than restating
/// the seven.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
// `pub` is load-bearing under the `test`/`test-support` arms — the
// `crate::test_support` door re-exports this type across the crate
// boundary, and a `pub use` cannot widen a `pub(crate)` item. Under the
// `debug_assertions` arm alone the door does not exist, so the same
// declaration is genuinely unreachable from outside and the lint fires.
// The two gates differ on purpose (this module's header states why), so
// the reachable spelling of this type differs with them; `pub` is the
// one that satisfies the widest arm.
#[allow(unreachable_pub)]
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
