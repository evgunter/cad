//! **A described-NURBS-walled body moves.** `topo::transform_rigid`
//! refuses only the NURBS *placeholder*; a body whose walls are
//! described nets — every loft, sweep and skin this kernel builds —
//! maps by its control points and comes out the EXACT IMAGE of the
//! original.
//!
//! The property asserted is the commuting square itself, not a
//! silhouette: for every mapped wall, `S'(u, v) = M(S(u, v))` at a
//! grid of parameters, with the knot vectors and the weight channel
//! bitwise unchanged. That is what separates a mapped net from a
//! re-fit — a re-fit would agree to fit tolerance and disagree in the
//! last dozen digits, and it would move the knots.
//!
//! # Why there are two bodies here, and why the second one is the row
//! # that can fail
//!
//! The whole argument for mapping the control net rests on the nets
//! being stored EUCLIDEAN, with the weights in a channel beside them
//! (`geom`'s `curves::nurbs` data model). **Under uniform weights that
//! premise is untestable**: `w = 1` everywhere makes the Euclidean and
//! the weighted/homogeneous storage numerically identical, so a body
//! whose walls are all `w = 1` would pass these rows just as happily if
//! the kernel had the storage backwards.
//!
//! The polyline-profile loft below is exactly that body — measured, all
//! four of its walls carry unit weights throughout. It is kept because
//! it is the shape every loft/sweep/skin scene in the tree actually
//! builds. The **arc-profile** loft beside it is the one that carries
//! the argument: its walls are genuinely rational — measured, weights
//! off 1 by up to 7.6e-2 — so a storage confusion would move its
//! mapped points by far more than the floor here admits. Its `rational > 0` guard is not
//! ceremony — it is what stops the fixture silently reverting to unit
//! weights and taking the evidence with it.

// Panicking is a test's failure mechanism (workspace lint policy).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::{arc_section, quad, stacked, sup_dist};
use geom::{Curve3, Surface};
use geom_core::Tol;
use geom_core::{Affine3, Mat3, Vec3};
use sweep::{Section, loft_body};

/// Squares at z = 0 and z = 2 with a trapezoid between: the middle
/// section is not an affine image of the ends, so the four walls are
/// genuinely curved degree-2 nets rather than ruled strips — and, being
/// polyline profiles, every weight is 1.
fn walled_sections() -> (Vec<Section>, Vec<Affine3<f64>>) {
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sections = vec![quad(square), quad(trapezoid), quad(square)];
    (sections, stacked(&[0.0, 1.0, 2.0], 1.0))
}

/// The arc-bearing profile at two scales: its quarter-circle side
/// skins to a RATIONAL wall, which is the storage the map's
/// correctness actually depends on.
fn rational_sections() -> (Vec<Section>, Vec<Affine3<f64>>) {
    let sections = vec![arc_section(1.0), arc_section(1.4), arc_section(1.0)];
    (sections, stacked(&[0.0, 1.0, 2.0], 1.0))
}

/// A quarter turn about `+z` followed by a dyadic translation.
fn quarter_turn_aside() -> Affine3<f64> {
    Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::new(4.0, -2.0, 0.5),
    )
}

/// An **awkward** rigid map: a rotation about a non-axis, non-dyadic
/// direction through a non-dyadic angle, with a non-dyadic
/// translation. Nothing here is exactly representable, so the mapped
/// net and the mapped evaluation genuinely disagree in the last bits —
/// which is the point: it is the map under which an exactness claim
/// can be wrong.
fn awkward() -> Affine3<f64> {
    let axis = Vec3::new(0.3, -0.7, 0.648_074_069_840_786_1).normalize();
    Affine3::from_parts(
        Mat3::rotation_about(axis, 0.937_142_1),
        Vec3::new(-1.483_902_117_4, 0.294_615_883_7, 2.061_338_492_5),
    )
}

/// The agreement floor. It is set to exclude a **re-fit**, which is the
/// alternative this suite exists to rule out and which misses by fit
/// tolerance — twelve orders wider than this.
///
/// It is deliberately NOT a claim that any row's residual is near it,
/// and the two rows below are honest about that in opposite ways.
/// Measured over every sample this suite takes:
///
/// - under [`quarter_turn_aside`] the residual is **exactly 0** — that
///   map is a signed coordinate permutation plus dyadic adds, so
///   nothing rounds. A summation-order argument would be dressing a
///   fixture property up as a numerical result;
/// - under [`awkward`], where nothing is exactly representable, the
///   worst sample is **1.8e-15** — a couple of ulp at these
///   coordinates, and three orders inside this floor.
///
/// So the floor is doing exactly one job: excluding a re-fit. It is
/// not a measurement, and it is not tight against either row.
const IMAGE_EPS: f64 = 1e-12;

/// Every mapped NURBS wall is the image of the original wall, and its
/// knots and weights are untouched. Returns `(walls, rational_walls)`
/// so a caller can assert its fixture is the one it thinks it is.
fn check_walls(
    before: &topo::Body<f64>,
    after: &topo::Body<f64>,
    map: &Affine3<f64>,
) -> (usize, usize) {
    let (mut walls, mut rational) = (0usize, 0usize);
    for (key, b) in before.surfaces() {
        let (Surface::Nurbs(b), Some(Surface::Nurbs(a))) = (b, after.get_surface(key)) else {
            continue;
        };
        walls += 1;
        if b.weights().iter().any(|w| (w - 1.0).abs() > 1e-9) {
            rational += 1;
        }
        assert_eq!(a.knots_u(), b.knots_u(), "u knots moved");
        assert_eq!(a.knots_v(), b.knots_v(), "v knots moved");
        assert_eq!(a.weights(), b.weights(), "the weight channel moved");
        let (du, dv) = (b.knots_u().domain(), b.knots_v().domain());
        for i in 0..=8 {
            for j in 0..=8 {
                let u = du.0 + (du.1 - du.0) * f64::from(i) / 8.0;
                let v = dv.0 + (dv.1 - dv.0) * f64::from(j) / 8.0;
                let want = map.transform_point(b.eval(u, v));
                let got = a.eval(u, v);
                assert!(
                    sup_dist(want, got) <= IMAGE_EPS,
                    "the mapped wall is not the image of the original at ({u}, {v}): \
                     want {want:?}, got {got:?}"
                );
            }
        }
    }
    (walls, rational)
}

#[test]
fn a_described_nurbs_walled_body_maps_to_its_exact_image() {
    let (sections, places) = walled_sections();
    let lofted = loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("the loft builds");
    let body = lofted.body;
    let map = quarter_turn_aside();

    let moved = topo::transform_rigid(&body, &map, Tol::witness())
        .expect("a described-NURBS-walled body is movable");

    // The arenas are key-stable (transform_rigid's contract), so the
    // two nets are compared entry for entry under the same key.
    let (walls, _) = check_walls(&body, &moved, &map);
    assert!(walls > 0, "the loft grew no described NURBS walls to map");

    // Tier 3 at rest on the moved body: every edge carrier was
    // re-certified against the mapped surfaces on the way here, and
    // the standing validators agree with that verdict afterwards.
    assert_eq!(topo::validate(&moved), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&moved), Ok(()), "tier 2");
    assert_eq!(
        topo::validate_geometric(&moved, Tol::witness()),
        Ok(()),
        "tier 3 at rest"
    );
}

/// The row that can actually catch a Euclidean/homogeneous storage
/// confusion, under a map with nothing exactly representable in it.
///
/// **Deliberately no tier-3 assertion here.** The arc-profile loft's
/// certified enclosure misses its quadrature budget away from the
/// origin — including when the body is AUTHORED at the offset with no
/// transform involved at all, so it is nothing this door does. That is
/// `work/exch/rational-patch-flux-quadrature-budget.md` (#390), and
/// pinning it here would attach an unrelated program's open dial to
/// this door's regression surface.
#[test]
fn a_rational_walled_body_maps_to_its_exact_image() {
    let (sections, places) = rational_sections();
    let lofted =
        loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("the arc loft builds");
    let body = lofted.body;
    let map = awkward();

    let moved = topo::transform_rigid(&body, &map, Tol::witness())
        .expect("a rational-walled body is movable");

    let (walls, rational) = check_walls(&body, &moved, &map);
    assert!(walls > 0, "the arc loft grew no described NURBS walls");
    assert!(
        rational > 0,
        "this fixture no longer carries a single non-unit weight, so it can no \
         longer tell Euclidean storage from homogeneous storage — which is the \
         only thing it was here to do"
    );
}

#[test]
fn a_nurbs_carrier_maps_to_its_exact_image() {
    let (sections, places) = walled_sections();
    let lofted = loft_body::<f64>(&sections, &places, 2, Tol::witness()).expect("the loft builds");
    let body = lofted.body;
    let map = quarter_turn_aside();
    let moved = topo::transform_rigid(&body, &map, Tol::witness())
        .expect("a described-NURBS-walled body is movable");

    let mut carriers = 0usize;
    for (key, before) in body.curves() {
        let (Some(b), Some(a)) = (
            before.certified(),
            moved
                .get_curve_geom(key)
                .and_then(topo::CurveGeom::certified),
        ) else {
            continue;
        };
        let (Curve3::Nurbs(b), Curve3::Nurbs(a)) = (b.carrier(), a.carrier()) else {
            continue;
        };
        carriers += 1;
        assert_eq!(a.knots(), b.knots(), "knots moved");
        assert_eq!(a.weights(), b.weights(), "the weight channel moved");
        let d = b.knots().domain();
        for i in 0..=16 {
            let t = d.0 + (d.1 - d.0) * f64::from(i) / 16.0;
            let want = map.transform_point(b.eval(t));
            let got = a.eval(t);
            assert!(
                sup_dist(want, got) <= IMAGE_EPS,
                "the mapped carrier is not the image of the original at {t}"
            );
        }
    }
    assert!(
        carriers > 0,
        "the loft grew no described NURBS carriers to map"
    );
}
