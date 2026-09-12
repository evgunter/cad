//! D290 review lane R2's end-to-end exercise of the domain door.
//!
//! Two programs a real caller would write:
//!
//! 1. `reexpress_a_fitted_curve_on_a_carrier_interval` — fit a curve
//!    through samples on `[0, 1]`, re-express it on a carrier interval,
//!    evaluate both, and check the reparameterization contract the
//!    curve door's doc states.
//! 2. `plane_nurbs_limbs_image_lands_on_the_carriers_own_domain` — the
//!    shipped public door (`geom_brep::plane_nurbs_limbs`) on a carrier
//!    whose domain is NOT `[0, 1]`, which is the configuration the
//!    behaviour change is about and the one the unit's own suites do
//!    not have a fixture for.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve2, NurbsCurve3, NurbsSurface, Surface};
use geom_brep::plane_nurbs_limbs;
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3, Vec3};

use crate::shared::tol::band;

/// The carrier interval every row here uses: `0.3 + (0.9 - 0.3)` is an
/// ulp above `0.9`, so a computed end misses it.
const CARRIER: (f64, f64) = (0.3, 0.9);

/// A degree-2 NURBS curve on `[lo, hi]` through three points.
fn carrier_on(lo: f64, hi: f64) -> NurbsCurve3<f64> {
    let knots = KnotVector::clamped(vec![lo, lo, lo, hi, hi, hi], 2).unwrap();
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 0.5),
        Point3::new(1.0, 0.0, 1.0),
    ];
    NurbsCurve3::new(knots, control, vec![1.0, 1.0, 1.0]).unwrap()
}

/// The quarter-cylinder wall the class-5/6 probes use.
fn wall() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.0, 1.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 1.0),
    ];
    let w = core::f64::consts::FRAC_1_SQRT_2;
    NurbsSurface::new(ku, kv, control, vec![1.0, 1.0, w, w, 1.0, 1.0]).unwrap()
}

fn plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// **Program 1.** A caller with a curve fitted on `[0, 1]` wants it on
/// `[t0, t1]`. The contract the curve door's doc states is that the
/// result at `t0 + (t1 - t0)*s` is the source at `s`; the pin the unit
/// adds is that the ENDS are exact, so `eval(t1)` is `eval(1.0)` bit
/// for bit rather than an evaluation an ulp outside the source domain.
#[test]
fn reexpress_a_fitted_curve_on_a_carrier_interval() {
    let pts: Vec<Point2<f64>> = (0..9)
        .map(|i| {
            let s = f64::from(i) / 8.0;
            Point2::new(s, 0.4 * (s * 3.1).sin() + 0.2 * s * s)
        })
        .collect();
    let params: Vec<f64> = (0..9).map(|i| f64::from(i) / 8.0).collect();
    let fitted = NurbsCurve2::<f64>::interpolate_with_params(&pts, 3, &params).unwrap();
    assert_eq!(fitted.domain(), (0.0, 1.0));

    let (t0, t1) = CARRIER;
    let moved = fitted.on_domain(t0, t1).unwrap();

    // The ends are exact, and the endpoints agree bit for bit.
    let (lo, hi) = moved.domain();
    assert_eq!((lo.to_bits(), hi.to_bits()), (t0.to_bits(), t1.to_bits()));
    for (a, b) in [
        (moved.eval(t0), fitted.eval(0.0)),
        (moved.eval(t1), fitted.eval(1.0)),
    ] {
        assert_eq!(
            (a.x.to_bits(), a.y.to_bits()),
            (b.x.to_bits(), b.y.to_bits())
        );
    }

    // The interior contract, to the rounding of the knot map.
    for i in 0..=32_u32 {
        let s = f64::from(i) / 32.0;
        let a = moved.eval(t0 + (t1 - t0) * s);
        let b = fitted.eval(s);
        assert!(
            (a.x - b.x).abs() < 1e-12 && (a.y - b.y).abs() < 1e-12,
            "s = {s}: {a:?} vs {b:?}"
        );
    }

    // **A round trip is NOT bit-identical.** Moving the curve onto
    // the carrier and back onto `[0, 1]` drifts interior knots by an
    // ulp — the ENDS survive (they are assigned) but nothing pins the
    // interior. A caller who normalizes for a fit and restores
    // afterwards does not get its knots back.
    let back = moved.on_domain(0.0, 1.0).unwrap();
    assert_eq!(back.domain(), (0.0, 1.0));
    let drifted = back
        .knots()
        .knots()
        .iter()
        .zip(fitted.knots().knots())
        .filter(|(a, b)| a.to_bits() != b.to_bits())
        .count();
    assert_eq!(
        drifted, 2,
        "measured: 2 of 13 knots move one ulp under [0,1] -> [0.3,0.9] -> [0,1]"
    );
}

/// **Program 2.** The shipped door, on a carrier whose domain is
/// `[0.3, 0.9]` — where the pre-change computed end landed an ulp past
/// `t1`. The chart image must now be expressed on the carrier's own
/// domain bit for bit.
#[test]
fn plane_nurbs_limbs_image_lands_on_the_carriers_own_domain() {
    let (t0, t1) = CARRIER;
    let carrier = carrier_on(t0, t1);
    assert_ne!(
        (t0 + (t1 - t0)).to_bits(),
        t1.to_bits(),
        "the fixture no longer exercises the pin"
    );
    let limbs = plane_nurbs_limbs::<f64>(&carrier, &plane(), &wall(), 1.0, band())
        .expect("the door answers on a carrier whose domain is not [0, 1]");
    assert!(limbs.hull_sup.is_finite() && limbs.tube_radius.is_finite());

    // The door does not hand back the image, so the domain claim is
    // re-derived here on the same pipeline the door runs: an image
    // fitted on [0, 1] carried onto the carrier's interval.
    let unit = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let image = NurbsCurve2::new(
        unit,
        vec![
            Point2::new(0.0, 0.0),
            Point2::new(0.4, 0.9),
            Point2::new(1.1, 0.3),
            Point2::new(1.6, 1.0),
        ],
        vec![1.0, 1.0, 1.0, 1.0],
    )
    .unwrap();
    let moved = image.on_domain(t0, t1).unwrap();
    let (lo, hi) = moved.domain();
    assert_eq!((lo.to_bits(), hi.to_bits()), (t0.to_bits(), t1.to_bits()));

    // What the pre-change spelling produced, measured: the same map
    // applied to EVERY knot, ends included.
    let span = t1 - t0;
    let old: Vec<f64> = image
        .knots()
        .knots()
        .iter()
        .map(|k| t0 + span * k)
        .collect();
    let old_end = old[old.len() - 1];
    assert_eq!(
        i128::from(old_end.to_bits()) - i128::from(t1.to_bits()),
        1,
        "the old spelling put the image's last knot one ulp past t1"
    );
    // …and every interior knot is unchanged by the fix.
    for (a, b) in old[3..4].iter().zip(&moved.knots().knots()[3..4]) {
        assert_eq!(a.to_bits(), b.to_bits(), "interior knots are untouched");
    }
}
