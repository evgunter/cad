//! Shared vocabulary for the sweep test corpus: section authoring
//! below, and the orientation checking in [`orient`].
//!
//! **Routing rule**, so a fourth home does not appear without one.
//! `sweep` has four places a suite can share from, and an item lives at
//! the narrowest one all of its consumers can reach:
//!
//! - `sweep::test_support` — fixtures the LIBRARY can build, reachable
//!   from in-crate tests, from here, and (behind the same dev-only
//!   feature) from another crate's suites, which is where a fixture
//!   with consumers OUTSIDE this crate has to live: the swept elbow
//!   the `mesh` and `step-export` suites meter is there for that
//!   reason;
//! - this module — section authoring, the profile vocabulary a suite
//!   builds a body FROM;
//! - [`orient`] — what a suite CHECKS of a body it built;
//! - [`approx`] — the `Surface::Approx` surgery vocabulary (body
//!   authoring, so it routes to this module rather than to a suite);
//! - `revolve_common` — the revolve suites' own, and the place `p2`
//!   and `eps` presently live despite belonging to no verb.
//!
//! A helper one suite uses stays in that suite.
//!
//! Section authoring (LIB-U3): loft/sweep sections in the profile
//! vocabulary, one copy per crate. Cross-crate constant deduplication
//! (the 1/16-offset table relation) is LIB-U6's territory,
//! deliberately not built here.

#![allow(dead_code)]
// one instance per binary; no single consumer uses all of it
// Why a helper tree allows these: `crates/editor-core/tests/fixture/mod.rs`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(unreachable_pub)] // why: root Cargo.toml, the `unreachable_pub` stanza

/// Orientation checking: the face-facing probe and the two level-set
/// indexes it decides against. Split out because it is a different kind
/// of shared thing from the section authoring below — not a fixture,
/// but the check several suites make of a body they built.
pub mod orient;

/// The `Surface::Approx` surgery vocabulary — the pulled-back base,
/// the fixtures the OFF-C rows convert, and the surface + carrier +
/// pcurve surgery itself. Body authoring, so it routes here.
pub mod approx;

use geom::NurbsCurve3;
use geom_core::{Affine3, Mat3, Point2, Vec3};
use profile::RawLoop;
use sweep::{ProfileLoop, ProfileVertex, Section};

/// The placement a path sweep starts from: the plane through the
/// path's start point whose normal is the start TANGENT, with the
/// in-plane axes built off whichever world axis is least parallel to
/// it. `sweep::sweep_places` carries this frame along the path by
/// minimal rotation, so a section placed here stays normal to the
/// path — the recipe every path-swept fixture in this corpus starts
/// from, and the one the tour's sweep cells narrate.
pub fn normal_start_place(path: &NurbsCurve3<f64>) -> Affine3<f64> {
    let (lo, _) = path.domain();
    let d = path.deriv(lo);
    let n = d / d.norm();
    let helper = if n.z.abs() < 0.9 {
        Vec3::new(0.0, 0.0, 1.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };
    let u = helper.cross(n);
    let u = u / u.norm();
    let v = n.cross(u);
    let p = path.eval(lo);
    Affine3::from_parts(Mat3::from_cols(u, v, n), Vec3::new(p.x, p.y, p.z))
}

/// A closed four-line quad section (one loop, four vertices) — the
/// plainest INTEGRAL profile: unit weights, no arc anywhere.
pub fn quad(pts: [(f64, f64); 4]) -> Section {
    vec![ProfileLoop::polygon(
        pts.iter().map(|&(x, y)| Point2::new(x, y)),
    )]
}

/// The M5 PR 10 review section: a square-with-an-arc loop scaled by
/// `s` — three lines and one bulge-0.25 arc, so the skin exercises
/// the rational lane.
pub fn chain(s: f64) -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x * s, y * s), bulge);
    vec![ProfileLoop::new(vec![
        v(0.0, 0.0, 0.0),
        v(2.0, 0.0, 0.25),
        v(2.0, 1.0, 0.0),
        v(0.0, 1.0, 0.0),
    ])]
}
