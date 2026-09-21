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
//! **Which builds compile this module.** The `debug_assertions` arm is
//! not "debug builds": it tracks the flag, and this workspace's
//! `[profile.release]` sets `debug-assertions = true` (root
//! `Cargo.toml`). So `cargo build --release` DOES satisfy that arm and
//! this module is compiled — correctly, because the D1 postcondition
//! assert it exists to serve is compiled in exactly the builds where
//! that flag is on. The stanza is marked to come out before publishing;
//! when it does, release stops satisfying the arm and this module stops
//! existing there. Visibility does not ride along either way: the door's
//! gate is `test`/`test-support`, which no profile turns on, so
//! `topo::test_support` still does not resolve in a release build.
//! `cargo test --release` satisfies `test` independently — which is why
//! `debug_assertions` alone cannot serve as the existence gate.
//!
//! # Which home a new test item goes in (three under `src/`, and
//! `tests/` besides)
//!
//! Stated here because this is the module the gate's argument lives in;
//! the others point at this paragraph rather than restating it. The
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
//! - **A `tests/` binary needs it and the library's own probes may
//!   too** — `crate::test_support_fixtures`, a sibling module behind
//!   the same `test_support` door, gated on the test arms alone and so
//!   not linkable from here either. The Euler-op fixture family lives
//!   there. It is the only home the two can share: `src/fixtures.rs`
//!   cannot be named from `tests/`, and a module under `tests/` cannot
//!   be named from `src/`, so an item both must name has nowhere else
//!   to go.
//!
//! A module under `tests/` is a home too, and the cheapest one —
//! nothing in the library pays for it and no feature is involved —
//! but it is not a fourth *vocabulary* home, because `src/` cannot
//! reach it. It is where a fixture that exactly one suite family reads
//! belongs; `tests/fixture/mod.rs` is the one such module today.
//!
//! **The unit that moves is the family, not the item.** An item's
//! consumers set its floor — the narrowest home all of them can reach
//! — and where a choice remains the narrowest wins, because widening
//! the library for a `tests/`-only item buys nothing. But a builder,
//! the key bundle it returns and the helpers that read that bundle are
//! one vocabulary, and splitting them across two homes to save the
//! library a few functions costs every reader the question of which
//! half is where. So a family goes whole, to the floor of its widest
//! member. That is the rule the Euler-op family follows: three of its
//! items — `geometric_cube`, `describe_as_intersections` and
//! `face_surface_of_he` — are named from `src/`, and the rest of the
//! family sits beside them rather than in `tests/`.

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
// one that satisfies the widest arm. The narrow arm is not hypothetical:
// `cargo check -p topo --lib --release` reaches this declaration and,
// without this allow, warns on it.
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
