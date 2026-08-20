//! Test-support vocabulary: fixtures this crate's tests build on,
//! kept in one place so no suite mints its own copy.
//!
//! # The gate (stated once for this crate)
//!
//! The module declaration in `lib.rs` carries
//! `#[cfg(any(test, feature = "test-support"))]`. Both arms are
//! consumers that must be able to name these items:
//!
//! - **`test`** — the in-crate `mod tests` pins. (Cargo unifies the
//!   self dev-dependency's features into that build too, so this arm is
//!   belt-and-braces: it keeps the gate true of any in-crate test build
//!   without depending on how features resolve.) The sites several of
//!   them cover are private to their modules, so those pins cannot
//!   live in `tests/`; hosting the fixtures here rather than inside
//!   one of the test modules that uses them keeps neither module the
//!   owner of the other's fixture.
//! - **`feature = "test-support"`** — this crate's `tests/` binaries.
//!   A `tests/` file is a **separate crate** that links the library as
//!   an ordinary dependency, so it can name neither a `#[cfg(test)]`
//!   item (that cfg is off when the library is built as a dependency)
//!   nor a `pub(crate)` one. `cfg(test)` alone therefore cannot serve
//!   as this module's gate: it is exactly what made six integration
//!   suites each declare their own `cube` (S52). The feature is off by
//!   default and turned on only by the **self dev-dependency** in this
//!   crate's manifest (`sweep = { path = ".", features =
//!   ["test-support"] }`), so it is on exactly when this crate's own
//!   tests compile the library and off for every other build,
//!   including every downstream dependent.
//!
//! Existence and visibility coincide here, so one gate states both:
//! nothing in this module has a non-test consumer, unlike `topo`'s
//! `test_support_impl`, whose `ArenaCounts` the debug postcondition
//! also needs and which is therefore compiled wider than it is exported.
//!
//! Neither arm is satisfied by `cargo build [--release]`, so this
//! module is absent from every shipped build. `cargo test --release`
//! satisfies `test` — which is why `cfg(debug_assertions)` cannot
//! serve as the gate either.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom_core::{Band, Point2, Tolerance};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::{Body, EdgeKey};

use crate::fillet::battery::{FilletRequest, Link, run_battery};
use crate::{Extrusion, extrude};

/// The cube side the in-crate pins build on, meters.
pub const L: f64 = 1.0;
/// The blend radius, meters.
pub const R: f64 = 0.1;

/// An axis-aligned cube of side `l` with a corner at the origin:
/// eight trivalent corners, every one of them geometrically CONVEX.
pub fn cube(l: f64) -> Body<f64> {
    let lp = ProfileLoop::new(
        [(0.0, 0.0), (l, 0.0), (l, l), (0.0, l)]
            .into_iter()
            .map(|(x, y)| ProfileVertex::new(Point2::new(x, y), 0.0))
            .collect(),
    );
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tolerance::get())
        .unwrap();
    extrude(&profile, Extrusion::Distance(l)).unwrap().body
}

/// Every edge of `body` resolved by the fillet battery, in edge
/// order.
pub fn all_links(body: &Body<f64>) -> Vec<Link<f64>> {
    let tol = Tolerance::get();
    let edges: Vec<EdgeKey> = body.edges().map(|(k, _)| k).collect();
    let verdict = run_battery(
        &FilletRequest {
            body,
            edges,
            radius: R,
        },
        Band::new(tol.eps, tol.k * tol.eps).unwrap(),
    )
    .expect("the battery resolves every edge of a cube");
    let mut links: Vec<Link<f64>> = verdict
        .chains
        .iter()
        .flat_map(|c| c.links().cloned())
        .collect();
    links.sort_by_key(|l| l.edge);
    links
}
