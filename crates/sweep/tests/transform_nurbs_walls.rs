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
//! The map is chosen with exact binary entries (a quarter turn about
//! z, then a translation of dyadic components) so the residual under
//! test is the evaluator's own summation rounding and nothing else.

// Panicking is a test's failure mechanism (workspace lint policy).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::quad;
use geom::{Curve3, Surface};
use geom_core::Tol;
use geom_core::{Affine3, Mat3, Point3, Vec3};
use sweep::{Section, loft_body};

/// Squares at z = 0 and z = 2 with a trapezoid between: the middle
/// section is not an affine image of the ends, so the four walls are
/// genuinely curved degree-2 nets rather than ruled strips.
fn walled_sections() -> (Vec<Section>, Vec<Affine3<f64>>) {
    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let sections = vec![quad(square), quad(trapezoid), quad(square)];
    let places = vec![
        Affine3::identity(),
        Affine3::translation(Vec3::new(0.0, 0.0, 1.0)),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    (sections, places)
}

/// A quarter turn about `+z` followed by a dyadic translation. Every
/// entry is exactly representable, so the rigidity door's margins are
/// exactly zero and the only rounding in the comparison below is the
/// evaluator's own.
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

/// The agreement floor. The body spans a few metres, one f64 ulp there
/// is ~1e-15, and the two sides differ only in the order the mapped
/// terms are summed. A re-fit — the thing this row exists to exclude —
/// misses by fit tolerance, which is twelve orders of magnitude wider.
const IMAGE_EPS: f64 = 1e-12;

fn far(a: Point3<f64>, b: Point3<f64>) -> f64 {
    ((a.x - b.x).abs())
        .max((a.y - b.y).abs())
        .max((a.z - b.z).abs())
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
    let mut walls = 0usize;
    for (key, before) in body.surfaces() {
        let (Surface::Nurbs(b), Some(Surface::Nurbs(a))) = (before, moved.get_surface(key)) else {
            continue;
        };
        walls += 1;
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
                    far(want, got) <= IMAGE_EPS,
                    "the mapped wall is not the image of the original at ({u}, {v}): \
                     want {want:?}, got {got:?}"
                );
            }
        }
    }
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
                far(want, got) <= IMAGE_EPS,
                "the mapped carrier is not the image of the original at {t}"
            );
        }
    }
    assert!(
        carriers > 0,
        "the loft grew no described NURBS carriers to map"
    );
}
