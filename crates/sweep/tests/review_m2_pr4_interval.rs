//! M2 PR 4 adversarial review — interval lane (feature `interval`;
//! from `review/m2-4`, promoted permanently as `review_m2_pr4_interval`).
//!
//! Companion of `review_m2_pr4.rs`: the interval scalar's extrusion
//! paths beyond the acceptance tests' axis-aligned +n cases — the
//! reversed direction, the ring path (kemr/kfmrh) on exact-dyadic
//! data, and (post-B1-fix, flipped by the M2 PR 4 fix pass from the
//! review's honesty pins) REQUIRED tier-valid builds on the diagonal
//! hole-planting bridge and on rotated (non-dyadic) placements.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Bounds, Interval, Point2, Point3, Real, Tolerance, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{validate, validate_closed, validate_geometric};
use geom_core::Tol;

fn p2(x: f64, y: f64) -> Point2<Interval> {
    Point2::new(Interval::from_f64(x), Interval::from_f64(y))
}

fn square(x0: f64, y0: f64, s: f64) -> ProfileLoop<Interval> {
    ProfileLoop::polygon([
        p2(x0, y0),
        p2(x0 + s, y0),
        p2(x0 + s, y0 + s),
        p2(x0, y0 + s),
    ])
}

/// The REVERSED direction at the interval scalar (the acceptance tests
/// only extrude +n): exact-dyadic L, Distance(−1.5), all tiers, exact
/// raised-vertex enclosures at z = −1.5.
#[test]
fn interval_reversed_l_profile_all_tiers() {
    let lp = ProfileLoop::polygon([
        p2(0.0, 0.0),
        p2(2.0, 0.0),
        p2(2.0, 1.0),
        p2(1.0, 1.0),
        p2(1.0, 2.0),
        p2(0.0, 2.0),
    ]);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(Interval::from_f64(-1.5))).unwrap();
    assert_eq!(validate(&t.body), Ok(()));
    assert_eq!(validate_closed(&t.body), Ok(()));
    assert_eq!(validate_geometric(&t.body), Ok(()));
    assert_eq!(t.body.vertices().count(), 12);
    let strut = t.strut_edges[0][0];
    let he = t.body.get_edge(strut).unwrap().he_plus;
    let top_v = t.body.half_edge_end(he).unwrap();
    let p = t
        .body
        .get_point(t.body.get_vertex(top_v).unwrap().point)
        .unwrap();
    assert_eq!(p.z.lo(), -1.5);
    assert_eq!(p.z.hi(), -1.5);
}

/// The ring path (bridge mev + kemr + chain + mef + kfmrh) at the
/// interval scalar, arranged so even the transient bridge chord is
/// axis-aligned dyadic: a staircase outer whose canonical start
/// (0, 0.5) is horizontally aligned with the hole's canonical start
/// (0.5, 0.5). Every carrier — including the hole-planting bridge —
/// then has an exact direction and dyadic length, so the whole genus-1
/// build must certify even before the B1 fix.
#[test]
fn interval_axis_aligned_bridge_ring_path_genus_one() {
    let outer = ProfileLoop::polygon([
        p2(0.0, 0.5),
        p2(0.25, 0.5),
        p2(0.25, 0.0),
        p2(1.0, 0.0),
        p2(1.0, 1.0),
        p2(0.0, 1.0),
    ]);
    let hole = square(0.5, 0.5, 0.25);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![outer, hole])
        .validate(Tol::witness())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(Interval::from_f64(1.0))).unwrap();
    assert_eq!(validate(&t.body), Ok(()));
    assert_eq!(validate_closed(&t.body), Ok(()));
    assert_eq!(validate_geometric(&t.body), Ok(()));
    let v = t.body.vertices().count() as isize;
    let e = t.body.edges().count() as isize;
    let f = t.body.faces().count() as isize;
    let r: isize = t.body.faces().map(|(_, fc)| fc.rings.len() as isize).sum();
    assert_eq!((v, e, f, r), (20, 30, 12, 2));
    assert_eq!(v - e + f - r, 0); // genus 1
    // Both caps carry the ring.
    assert_eq!(t.body.get_face(t.top).unwrap().rings.len(), 1);
    assert_eq!(t.body.get_face(t.bottom).unwrap().rings.len(), 1);
}

/// FIXED (was `finding_interval_diagonal_bridge_hits_b1_pre_fix`): a
/// fully dyadic, fully RECTILINEAR holed profile used to refuse at the
/// interval scalar, because the transient hole-planting BRIDGE chord
/// (outer canonical start → hole canonical start) is diagonal — its
/// normalized direction is non-dyadic, the endpoint-pinning residual
/// `distance(eval(t_end), endpoint)` squared a straddling-zero
/// enclosure, and B1 poisoned the decoration (`carrier_endpoint_end`
/// escalated as Invalid). The refusal was honest (typed, atomic, no
/// wrong-accept — the review's pin), and geom-core's tight-square fix
/// (PR 3 fix pass) removed the poison: the build is now REQUIRED to
/// succeed tier-valid — the diagonal-bridge holed profile is the
/// interval lane's ring-path coverage beyond axis-aligned data.
#[test]
fn fixed_interval_diagonal_bridge_builds_tier_valid() {
    let outer = square(0.0, 0.0, 1.0);
    let hole = square(0.25, 0.25, 0.5);
    let vp = Profile::new(SketchPlane::<Interval>::xy(), vec![outer, hole])
        .validate(Tol::witness())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(Interval::from_f64(1.0))).unwrap();
    assert_eq!(validate(&t.body), Ok(()));
    assert_eq!(validate_closed(&t.body), Ok(()));
    assert_eq!(validate_geometric(&t.body), Ok(()));
    // Genus 1: the ring path ran end to end at the interval scalar.
    let v = t.body.vertices().count() as isize;
    let e = t.body.edges().count() as isize;
    let f = t.body.faces().count() as isize;
    let r: isize = t.body.faces().map(|(_, fc)| fc.rings.len() as isize).sum();
    assert_eq!((v, e, f, r), (16, 24, 10, 2));
    assert_eq!(v - e + f - r, 0);
}

/// FIXED (was `interval_rotated_placement_refuses_honestly_pre_b1_fix`):
/// the frame u = (2/3, 2/3, 1/3), v = (1/3, −2/3, 2/3) is exactly
/// orthonormal in ℝ but not dyadic, so every world point is a
/// nondegenerate enclosure and `norm`/`distance` used to hit PR 3
/// review B1 (straddling-zero squares poisoned the decoration through
/// `sqrt`); the pre-fix build refused typed (the review's honesty pin —
/// no panic, no silently wrong certification). Post-fix, every residual
/// enclosure classifies definitely through clean decorations and the
/// rotated build is REQUIRED to succeed tier-valid — the interval
/// lane's coverage of genuinely inexact (non-dyadic) placements.
#[test]
fn fixed_interval_rotated_placement_builds_tier_valid() {
    let third = Interval::from_f64(1.0) / Interval::from_f64(3.0);
    let two_thirds = Interval::from_f64(2.0) / Interval::from_f64(3.0);
    let u = Vec3::new(two_thirds, two_thirds, third);
    let v = Vec3::new(third, Interval::from_f64(0.0) - two_thirds, two_thirds);
    let plane = SketchPlane::from_frame(
        Point3::new(
            Interval::from_f64(0.25),
            Interval::from_f64(-0.5),
            Interval::from_f64(1.0),
        ),
        u,
        v,
    );
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(1.0, 0.0), p2(1.0, 1.0), p2(0.0, 1.0)]);
    let vp = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let t = extrude(&vp, Extrusion::Distance(Interval::from_f64(1.0))).unwrap();
    assert_eq!(validate(&t.body), Ok(()));
    assert_eq!(validate_closed(&t.body), Ok(()));
    assert_eq!(validate_geometric(&t.body), Ok(()));
    assert_eq!(t.body.vertices().count(), 8);
}
