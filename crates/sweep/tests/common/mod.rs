//! Shared vocabulary for the sweep test corpus: section authoring
//! below, the orientation checking in [`orient`], the vented-cavity
//! fixtures in [`cavity`] and the closed forms in [`oracles`].
//!
//! **Routing rule**, so a further home does not appear without one.
//! These are the places a `sweep` suite can share from, and an item
//! lives at the narrowest one all of its consumers can reach:
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
//! - [`cavity`] — the vented-cavity fixture vocabulary (body
//!   authoring, same routing);
//! - [`oracles`] — closed-form volumes, which are neither: a truth
//!   derived without the kernel, so its own doc carries the rule for
//!   which per-suite spellings may come here at all;
//! - `revolve_common` — the revolve suites' own, and the place `p2`
//!   and `eps` presently live despite belonging to no verb.
//!
//! A helper one suite uses stays in that suite.
//!
//! **Two rules bind every module here, and both are checkable by
//! reading**: each module says which of its neighbours it deliberately
//! did NOT absorb, as a list that claims to be the whole of it; and
//! every suite that keeps its own copy of something this tree holds
//! says why AT the copy.
//!
//! The second rule has a MARKER so it can be compared rather than
//! sampled: every such copy's note carries the literal
//! ``NOT `common::`` naming the item it is not, on ONE line, so
//!
//! ```text
//! grep -rn 'NOT `common::' crates/sweep/tests
//! ```
//!
//! returns exactly the kept copies inside this crate and nothing else.
//! Its hits and the two module lists below name the same set; a hit
//! missing from a list, or a list entry with no hit, is the rule
//! broken. (Copies OUTSIDE `crates/sweep` are out of the recipe's
//! scope by construction — [`oracles`]'s list names the ones it knows
//! of, and they are tracked as their own item.)
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

/// The vented-cavity fixture vocabulary the concave blend suites
/// carve — brick, rod, prism, the vented cavity itself and the
/// find-an-edge-by-its-endpoints traversal. Body authoring, so it
/// routes here.
pub mod cavity;

/// The intersecting equal-radius cylinder pair — the germ lane's
/// fixture and the parameter-identity channel's, one authoring for
/// the one door both read. Body authoring, so it routes here.
pub mod germ_pair;

/// The closed-form volumes those suites meter against. Not a fixture
/// and not a check of a body, but a truth derived WITHOUT the kernel;
/// its module doc carries the rule for which per-suite spellings come
/// here and which are second derivations that must not.
pub mod oracles;

use geom::NurbsCurve3;
use geom_core::{Affine3, Mat3, Point2, Point3, Vec3};
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

/// **The arc section**: a square of half-width `s` with a
/// quarter-circle bulge on the `+x` side — the arc-bearing profile
/// whose lofted wall is RATIONAL (weights `1, cos 22.5°, 1` over two
/// 45° sub-arcs), and so the cheapest profile in this corpus that puts
/// a body's enclosure on the QUADRATURE lane rather than a closed
/// form.
///
/// One copy for the crate. It was four — `m8_3_rational_volume`,
/// `cert5_offgrid_knot_rational` and the certificate suite each held a
/// byte-identical spelling of it, and each restated the same weights
/// comment. `step-import`'s copies (`nurbs_import`, `rw2_probes`,
/// `review_probes_m7_3`, `recognize_pins`) are NOT folded in here:
/// cross-crate constant deduplication is LIB-U6's territory, which
/// this module's routing rule says is deliberately not built here.
pub fn arc_section(s: f64) -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    vec![ProfileLoop::new(vec![
        v(-s, -s, 0.0),
        // tan(π/8): a quarter-circle bulge-out.
        v(s, -s, 0.4142135623730951),
        v(s, s, 0.0),
        v(-s, s, 0.0),
    ])]
}

/// The **sup-norm distance** between two points — the largest
/// coordinate disagreement, which is the honest meter for "these two
/// constructions produced the same point": it bounds every coordinate
/// at once and never averages a bad axis away, as a Euclidean norm
/// would. Lives here because a per-coordinate comparison is what any
/// exactness row in this tree wants, and a fourth hand-rolled copy is
/// how a suite ends up with a subtly different one.
pub fn sup_dist(a: Point3<f64>, b: Point3<f64>) -> f64 {
    (a.x - b.x)
        .abs()
        .max((a.y - b.y).abs())
        .max((a.z - b.z).abs())
}

/// Loft placements: the given heights, each scaled by `s`, as pure
/// `+z` translations — the stacking that makes a loft of identical
/// sections reproduce the EXTRUSION of that section exactly.
pub fn stacked(z: &[f64], s: f64) -> Vec<Affine3<f64>> {
    z.iter()
        .map(|h| Affine3::translation(Vec3::new(0.0, 0.0, h * s)))
        .collect()
}
