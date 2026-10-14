//! **A loft whose corner path sways sideways builds**: four 2 × 2
//! squares at z = 0, 1, 2, 3 with x offsets alternating `−a, a, −a, a`.
//! Each corner edge is a smooth curve some 3 m long that never stalls,
//! and certification meters its span through
//! `NurbsCurve3::speed_lower_bound`, which must answer positive on it.

// Panicking is a test's failure mechanism (workspace lint policy).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use geom_core::{Bounds, Interval, Point2, Real, Tol};
use profile::SketchPlane;
use profile::{ProfileLoop, RawLoop};
use sweep::{Section, loft_body};

fn swaying(a: f64) -> (Vec<Section>, Vec<SketchPlane<f64>>) {
    let square = || {
        vec![ProfileLoop::polygon(
            [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                .iter()
                .map(|&(x, y)| Point2::new(x, y)),
        )]
    };
    let places = [-a, a, -a, a]
        .into_iter()
        .zip([0.0, 1.0, 2.0, 3.0])
        .map(|(dx, z)| SketchPlane::xy_at(Point3::new(dx, 0.0, z)))
        .collect();
    (vec![square(), square(), square(), square()], places)
}

/// The least enclosure ceiling of `‖C′‖` over a dense grid: at least
/// the true minimum speed, so a meter above it is unsound with
/// certainty.
fn min_speed_ceiling(c: &geom::NurbsCurve3<f64>) -> f64 {
    let ci = c.map_scalar(<Interval as Real>::from_f64);
    let (lo, hi) = c.domain();
    (0..=4000)
        .map(|i| {
            let t = lo + (hi - lo) * f64::from(i) / 4000.0;
            ci.deriv(Interval::from_f64(t)).norm().hi()
        })
        .fold(f64::INFINITY, f64::min)
}

/// The family the meter used to refuse: `a ≥ 0.75` at v-degree 2, and
/// `a = 0.5` at v-degree 3. At both scalars each body builds and
/// certifies (the build runs certification), validates and is closed,
/// and each corner's meter is positive and under its speed — at the
/// interval scalar, its certified floor.
#[test]
fn a_swaying_loft_builds_closed() {
    for (v_degree, a) in [(2, 0.75), (2, 1.0), (2, 1.5), (2, 2.0), (3, 0.5)] {
        let (sections, places) = swaying(a);
        let at = format!("v-degree {v_degree}, a = {a}");
        let lofted = loft_body::<f64>(&sections, &places, v_degree, Tol::witness())
            .unwrap_or_else(|e| panic!("{at}: {e:?}"));
        let lofted_i = loft_body::<Interval>(
            &sweep::test_support::lift_sections::<Interval>(&sections),
            &sweep::test_support::lift_places::<Interval>(&places, geom_core::Tol::witness()),
            v_degree,
            Tol::witness(),
        )
        .unwrap_or_else(|e| panic!("{at}: at the interval scalar: {e:?}"));
        let f64_meters = corner_meters(&lofted.body, &at, |c| c.speed_lower_bound().get());
        let interval_meters =
            corner_meters(&lofted_i.body, &at, |c| c.speed_lower_bound().get().lo());
        for (scalar, meters) in [("f64", f64_meters), ("interval", interval_meters)] {
            for (m, reference) in meters {
                assert!(
                    m > 0.0 && m <= reference,
                    "{at}, {scalar}: corner meter {m}, speed ceiling {reference}"
                );
            }
        }
    }
}

/// Validates `body` as closed and reads `meter` off each of its four
/// NURBS corner carriers, beside that carrier's speed ceiling (read on
/// its `f64` midpoint net).
fn corner_meters<T: geom_core::Decide + Bounds>(
    body: &topo::Body<T>,
    at: &str,
    meter: impl Fn(&geom::NurbsCurve3<T>) -> f64,
) -> Vec<(f64, f64)> {
    assert_eq!(topo::validate(body), Ok(()), "{at}: tier 1");
    assert_eq!(topo::validate_closed(body), Ok(()), "{at}: closed");
    let mut out = Vec::new();
    for (_, edge) in body.edges() {
        let topo::CurveGeom::Certified(curve) =
            body.get_curve_geom(edge.curve).expect("curve key resolves")
        else {
            panic!("a finished body has no null scaffolding");
        };
        if let geom::Curve3::Nurbs(c) = curve.carrier() {
            let mid = c.map_scalar(|x| 0.5 * (x.lo() + x.hi()));
            out.push((meter(c), min_speed_ceiling(&mid)));
        }
    }
    assert_eq!(out.len(), 4, "{at}: four corner edges");
    out
}
