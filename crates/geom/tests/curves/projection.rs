//! Point projection acceptance (M5 PR 4 spec §b, C2.1): closed-form
//! foot points on known loci, the D9 determinism pin, the typed
//! refusal, and the REQUIRED planted fixture — a mis-seeded Newton
//! that converges to the wrong branch of a closed curve, its carried
//! distance FAILING the consumer's band, which is exactly how a bad
//! projection cannot launder a bad cache.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::num::NonZeroUsize;
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
        // An interior foot converged on the cosine condition.
        let (_, d1, _) = curve.ders(proj.t);
        let orthogonality = d1.dot(proj.foot - p).abs();
        assert!(
            orthogonality <= 1e-11 * d1.norm() * proj.distance.max(1e-30),
            "orthogonality {orthogonality} at t = {}",
            proj.t
        );
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
}

#[test]
fn projection_is_deterministic_bit_for_bit() {
    let curve = nurbs_circle();
    let p = Point3::new(1.7, 2.9, 1.25);
    let a = curve.project(p).unwrap();
    let b = curve.project(p).unwrap();
    assert_eq!(a.t.to_bits(), b.t.to_bits());
    assert_eq!(a.distance.to_bits(), b.distance.to_bits());
    assert_eq!(a.iterations, b.iterations);
}

/// THE planted wrong-branch fixture (spec §b, required): a
/// deliberately mis-seeded Newton on the closed circle converges to
/// the ANTIPODAL stationary point — orthogonality is satisfied there
/// (it is a genuine stationary point of the distance), so the value
/// that fails the consumer's band is the carried DISTANCE. The honest
/// default seeding finds the near branch; the foot parameter alone,
/// stripped of its distance, could not tell the two apart — carrying
/// it is C2.1's whole point.
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
    let curve = NurbsCurve3::new(
        KnotVector::unit_segment(NonZeroUsize::MIN),
        control,
        vec![1.0, 1.0],
    )
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

/// The foot is a function of the query point only through the selected
/// parameter, which a numerical search produced: at `Sym<f64>`, with
/// the query point's `x` a parameter, the foot and its distance carry
/// that parameter as an unknown and so re-value to no claim at all,
/// never to the foot of the stale parameter. The interior query takes
/// the cosine exit and the one past the end the stagnation exit.
///
/// The carrier is built inside the session: `revalue` answers `None`
/// for a node the session never interned, so a carrier built outside it
/// would pass this row on any code. The control (a point of the same
/// carrier at a constant parameter re-values to a number) holds that.
#[test]
fn a_selected_parameter_enters_the_foot_as_an_unknown() {
    use geom_core::sym::revalue::revalue;
    use geom_core::sym::with_session;
    use geom_core::{ParamSymbol, Real, Sym, SymBudget};
    type S = Sym<f64>;
    let c = center();
    let shift = ParamSymbol::new(1);
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    for (label, (bx, by)) in [
        ("interior foot", (3.0 * SQRT2_2, 3.0 * SQRT2_2)),
        ("clamped end", (-3.0, 3.0)),
    ] {
        let ((control, revalued), _) = with_session(budget, || {
            let arc = quarter_arc().map_scalar(S::from_f64);
            let p = Point3::new(
                S::from_f64(c.x + bx) + S::param(shift, 0.0),
                S::from_f64(c.y + by),
                S::from_f64(c.z),
            );
            let proj = arc.project(p).unwrap();
            let at = |_: ParamSymbol| Some(0.25);
            (
                revalue(arc.eval(S::from_f64(0.5)).x.node(), &at),
                (
                    revalue(proj.foot.x.node(), &at),
                    revalue(proj.distance.node(), &at),
                ),
            )
        });
        assert!(
            control.is_some(),
            "{label}: the carrier's own nodes re-value, so a None below is the parameter's"
        );
        assert_eq!(
            revalued,
            (None, None),
            "{label}: the foot and its distance re-value through an unknown, not a constant parameter"
        );
    }
}

/// The `Opaque` unknowns the profiled sessions minted.
fn opaques(profile: &geom_core::sym::profile::SymProfile) -> u64 {
    profile.opaque_ids.iter().map(|s| s.len() as u64).sum()
}

/// The seeding sweep's samples feed only its `mid` comparisons, so no
/// payload carries them; each still enters `T` as its own unknown, one
/// atom per sample, which the session profile counts.
#[test]
fn the_seed_sweep_mints_one_unknown_per_sample() {
    use geom::PROJECT_SEEDS_PER_SPAN;
    use geom_core::sym::profile::{start_profile, take_profile};
    use geom_core::sym::with_session;
    use geom_core::{Real, Sym, SymBudget};
    type S = Sym<f64>;
    let c = center();
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    start_profile();
    let _ = with_session(budget, || {
        let arc = quarter_arc().map_scalar(S::from_f64);
        arc.project_seed(Point3::new(
            S::from_f64(c.x + 3.0),
            S::from_f64(c.y + 3.0),
            S::from_f64(c.z),
        ))
    });
    let profile = take_profile();
    assert_eq!(
        opaques(&profile),
        PROJECT_SEEDS_PER_SPAN as u64,
        "the quarter arc is one span: one unknown per sweep sample"
    );
}
