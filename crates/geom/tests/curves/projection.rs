//! Point projection acceptance (M5 PR 4 spec §b, C2.1): closed-form
//! foot points on known loci, the D9 determinism pin, the typed
//! refusal, and the REQUIRED planted fixtures — a mis-seeded Newton
//! that converges to the wrong branch of a closed curve, and a
//! domain-end clamp — both showing the carried residuals FAILING the
//! consumer's band, which is exactly how a bad projection cannot
//! launder a bad cache.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsCurve2, NurbsCurve3};
use geom_core::spline::KnotVector;
use geom_core::{Point2, Point3};

const SQRT2_2: f64 = core::f64::consts::FRAC_1_SQRT_2;
const RADIUS: f64 = 2.5;

fn center() -> Point3<f64> {
    Point3::new(-0.5, 4.0, 1.25)
}

/// The §7.3 full circle in the xy-plane about `center()` (radius 2.5):
/// the wrong-branch fixture wants a closed curve with an unambiguous
/// antipode, so the frame is axis-aligned on purpose.
fn nurbs_circle() -> NurbsCurve3<f64> {
    let c = center();
    let p = |cx: f64, cy: f64| Point3::new(c.x + RADIUS * cx, c.y + RADIUS * cy, c.z);
    let control = vec![
        p(1.0, 0.0),
        p(1.0, 1.0),
        p(0.0, 1.0),
        p(-1.0, 1.0),
        p(-1.0, 0.0),
        p(-1.0, -1.0),
        p(0.0, -1.0),
        p(1.0, -1.0),
        p(1.0, 0.0),
    ];
    let weights = vec![1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0];
    let knots = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    NurbsCurve3::new(knots, control, weights).unwrap()
}

/// A quarter arc (first 90° of the circle): the open-curve fixture for
/// the domain-end clamp.
fn quarter_arc() -> NurbsCurve3<f64> {
    let c = center();
    let p = |cx: f64, cy: f64| Point3::new(c.x + RADIUS * cx, c.y + RADIUS * cy, c.z);
    let control = vec![p(1.0, 0.0), p(1.0, 1.0), p(0.0, 1.0)];
    let weights = vec![1.0, SQRT2_2, 1.0];
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    NurbsCurve3::new(knots, control, weights).unwrap()
}

#[test]
fn projection_onto_a_circle_matches_the_closed_form() {
    let curve = nurbs_circle();
    let c = center();
    // Points inside and outside, at a few angles away from seams.
    for (dx, dy, off) in [
        (1.0f64, 0.5, 1.5),
        (-0.7, 0.9, 0.75),
        (0.3, -1.1, 3.0),
        (-1.0, -0.2, 0.4),
    ] {
        let n = (dx * dx + dy * dy).sqrt();
        let (ux, uy) = (dx / n, dy / n);
        let p = Point3::new(c.x + ux * off, c.y + uy * off, c.z);
        let proj = curve.project(p).unwrap();
        // Closed form: distance = |off − R|, foot = c + û·R.
        assert!(
            (proj.distance - (off - RADIUS).abs()).abs() < 1e-9,
            "distance {} vs {}",
            proj.distance,
            (off - RADIUS).abs()
        );
        let foot_expect = Point3::new(c.x + ux * RADIUS, c.y + uy * RADIUS, c.z);
        assert!(proj.foot.distance(foot_expect) < 1e-8);
        // The carried orthogonality residual is tiny for an interior
        // foot (the acceptance is the cosine condition).
        let speed = curve.deriv(proj.t).norm();
        assert!(proj.orthogonality <= 1e-11 * speed * proj.distance.max(1e-30));
    }
}

#[test]
fn projection_onto_a_line_is_exact_and_2d_rides_the_same_machinery() {
    // Degree-1 line x ∈ [0, 4] on y = 1: foot of (1.5, 3) is (1.5, 1).
    let knots = KnotVector::clamped(vec![0.0, 0.0, 4.0, 4.0], 1).unwrap();
    let control = vec![Point2::new(0.0, 1.0), Point2::new(4.0, 1.0)];
    let curve = NurbsCurve2::new(knots, control, vec![1.0, 1.0]).unwrap();
    let proj = curve.project(Point2::new(1.5, 3.0)).unwrap();
    assert!((proj.distance - 2.0).abs() < 1e-12);
    assert!((proj.foot.x - 1.5).abs() < 1e-12 && (proj.foot.y - 1.0).abs() < 1e-12);
    assert!(proj.orthogonality < 1e-12);
}

#[test]
fn projection_is_deterministic_bit_for_bit() {
    let curve = nurbs_circle();
    let p = Point3::new(1.7, 2.9, 1.25);
    let a = curve.project(p).unwrap();
    let b = curve.project(p).unwrap();
    assert_eq!(a.t.to_bits(), b.t.to_bits());
    assert_eq!(a.distance.to_bits(), b.distance.to_bits());
    assert_eq!(a.orthogonality.to_bits(), b.orthogonality.to_bits());
    assert_eq!(a.iterations, b.iterations);
}

/// THE planted wrong-branch fixture (spec §b, required): a
/// deliberately mis-seeded Newton on the closed circle converges to
/// the ANTIPODAL stationary point — orthogonality is satisfied there
/// (it is a genuine stationary point of the distance), so the value
/// that fails the consumer's band is the carried DISTANCE. The honest
/// default seeding finds the near branch; the projection result alone,
/// stripped of its residuals, could not tell the two apart — carrying
/// them is C2.1's whole point.
#[test]
fn planted_wrong_branch_fixture_fails_the_band_through_the_carried_distance() {
    let curve = nurbs_circle();
    let c = center();
    // P just outside the circle near θ = 0 (t ≈ 0): true distance 0.5.
    let p = Point3::new(c.x + RADIUS + 0.5, c.y, c.z);
    let honest = curve.project(p).unwrap();
    assert!((honest.distance - 0.5).abs() < 1e-9);

    // Mis-seed at the antipode (t = 0.5, θ = π): Newton stays on the
    // far branch — a stationary point, so it CONVERGES, with a tiny
    // orthogonality residual and a distance of 2R + 0.5.
    let wrong = curve.project_from_seed(p, 0.5).unwrap();
    assert!(
        (wrong.distance - (2.0 * RADIUS + 0.5)).abs() < 1e-9,
        "wrong-branch distance {}",
        wrong.distance
    );

    // The consumer's band (say ε = 1e-3 on the nearness residual vs
    // the expected 0.5 offset): the honest projection passes, the
    // wrong-branch one FAILS — the bad projection cannot launder.
    let band = 1e-3;
    assert!((honest.distance - 0.5).abs() <= band);
    assert!((wrong.distance - 0.5).abs() > band);
}

/// The open-curve companion fixture: a point past the arc's end
/// projects to the clamped domain end, and the projection carries the
/// honest, LARGE orthogonality residual of that boundary foot — the
/// band on orthogonality refuses it (a foot-point consumer that needs
/// interior orthogonality sees the violation immediately).
#[test]
fn domain_end_clamp_carries_an_honest_failing_orthogonality_residual() {
    let arc = quarter_arc();
    let c = center();
    // Past the θ = π/2 end, outside, at a bearing the arc cannot
    // reach — chosen so its antipode (the far-side stationary point)
    // is ALSO outside the arc: bearing 135°, antipode 315°, both
    // outside [0°, 90°], so no interior zero of g exists and the
    // clamp is the only place Newton can stop.
    let p = Point3::new(c.x - 3.0, c.y + 3.0, c.z);
    let proj = arc.project(p).unwrap();
    // Clamped at a domain end.
    let (lo, hi) = arc.domain();
    assert!(proj.t == lo || proj.t == hi, "t = {}", proj.t);
    // The orthogonality residual is honest and large: normalized as a
    // cosine it is O(1), nowhere near the acceptance threshold.
    let speed = arc.deriv(proj.t).norm();
    let cosine = proj.orthogonality / (speed * proj.distance);
    assert!(cosine > 0.1, "boundary foot cosine {cosine}");
}

#[test]
fn poisoned_input_is_a_typed_refusal_never_a_foot_point() {
    let curve = nurbs_circle();
    let p = Point3::new(f64::NAN, 0.0, 0.0);
    match curve.project(p) {
        Err(e) => {
            assert!(e.last_orthogonality.is_nan() || e.last_distance.is_nan());
        }
        Ok(pr) => panic!("NaN point projected to t = {}", pr.t),
    }
}

/// An **overflowing residual refuses**, and the inputs that produce
/// one are all finite.
///
/// `NurbsCurve3::new` validates counts and weight positivity, never
/// coordinate magnitude, so a control net at 1e200 is a legal curve
/// reachable through the public door. Its squared distance to a point
/// at the origin overflows to `+∞`, and the Newton loop reads that
/// residual through `crate::projection_policy::mid` — `x + ½(x − x)`, which
/// is NaN at `±∞`. NaN loses every acceptance comparison, so the
/// iteration falls out to the typed refusal instead of reporting a
/// converged foot at infinite distance.
///
/// That is the intended posture (an overflowed residual is not an
/// honest answer) and it is a **behaviour change** against the
/// `f64`-only form this lift replaced, which returned
/// `Ok { distance: inf }`. This row exists so the change cannot
/// silently revert: `mid` is what makes it happen, and nothing else in
/// the suite reaches an overflowing residual.
#[test]
fn an_overflowing_residual_refuses_rather_than_reporting_an_infinite_foot() {
    let huge = 1.0e200;
    let control = vec![
        Point3::new(huge, huge, huge),
        Point3::new(2.0 * huge, huge, huge),
    ];
    let curve = NurbsCurve3::new(KnotVector::unit_segment(1), control, vec![1.0, 1.0])
        .expect("a degree-1 segment with unit weights is a valid curve");

    // Finite inputs throughout: the curve's own coordinates and the
    // query point are all representable.
    assert!(
        curve
            .control()
            .iter()
            .all(|p| p.x.is_finite() && p.y.is_finite() && p.z.is_finite())
    );
    let p = Point3::new(0.0, 0.0, 0.0);
    assert!(p.x.is_finite() && p.y.is_finite() && p.z.is_finite());

    // The residual itself is what overflows.
    let d = curve.eval(0.0) - p;
    assert!(d.dot(d).is_infinite(), "the fixture must overflow");

    match curve.project(p) {
        Err(e) => {
            assert!(
                e.last_distance.is_nan() && e.last_orthogonality.is_nan(),
                "the refusal carries the poisoned state honestly: {e:?}"
            );
        }
        Ok(pr) => panic!("an overflowed residual was reported as a foot point: {pr:?}"),
    }
}
