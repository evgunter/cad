//! M5 PR 7: the **certified speed meter** a rung-3 carrier needs at
//! `split_edge` (C12.3).
//!
//! `Body::split_edge` states its interiority margin in metres by
//! multiplying a parameter distance by a per-carrier meter: `1` for a
//! line, the radius for a circle, the **minor** semi-axis for an
//! ellipse — each a conservative lower bound on `‖dP/dt‖`. A fitted SSI
//! carrier had no such meter and poisoned; this is it, and the rows
//! below pin the three things a consumer relies on: it is a genuine
//! lower bound, it is honest (non-positive, not optimistic) when the
//! curve gives it nothing to stand on, and it is TOTAL — a structural
//! violation poisons rather than inventing a scale.
//!
//! **M7: rational carriers now state a real bound.** The original
//! third row asserted that a rational carrier poisons. It no longer
//! does: `speed_lower_bound` grew a second arm, a quotient-rule
//! assembly over the homogeneous control net (its derivation lives on
//! the method). The rows below re-derive that claim from scratch
//! rather than deleting it — real *and* sound on ordinary and
//! adversarial rational carriers, non-positive where the speed truly
//! collapses, poison where the structure is illegal, and one row that
//! states the conservative frontier out loud instead of hiding it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
// `!(m > 0.0)` is deliberate everywhere below: the refusal rows accept
// EITHER a non-positive bound or poison, and `m <= 0.0` would let a NaN
// through as a pass. Same NaN-catching idiom as the kernel's own weight
// validation.
#![allow(clippy::neg_cmp_op_on_partial_ord)]

use geom::NurbsCurve3;
use geom_core::Point3;
use geom_core::spline::KnotVector;

#[test]
fn the_meter_lower_bounds_the_real_speed() {
    // A gently curved cubic advancing steadily in +x.
    let pts: Vec<Point3<f64>> = (0..=40)
        .map(|i| {
            let t = f64::from(i) / 40.0;
            Point3::new(t, 0.15 * (t * 6.0).sin(), 0.05 * t * t)
        })
        .collect();
    let c = NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap();
    let m = c.speed_lower_bound();
    assert!(
        m > 0.0,
        "a monotone carrier must have a positive meter: {m}"
    );
    // Sampled speeds must never fall below the certified meter.
    let mut lo = f64::INFINITY;
    let mut hi = 0.0f64;
    for i in 0..=200 {
        let t = f64::from(i) / 200.0;
        let s = c.deriv(t).norm();
        assert!(
            s >= m - 1e-12,
            "meter {m} exceeds the real speed {s} at t = {t}"
        );
        lo = lo.min(s);
        hi = hi.max(s);
    }
    // Anti-vacuity: this carrier crosses a full unit in x over the unit
    // domain, so its speed cannot sit near zero. A fixture edit that
    // flattened it would leave `m > 0` and the domination above
    // satisfiable by an arbitrarily small meter.
    assert!(
        lo > 1.0,
        "this carrier advances in +x, so its true minimum speed cannot be {lo}"
    );
    // The meter is a LOWER bound, so its loose direction is downward
    // and `s >= m` alone never sees it — a meter of 1e-300 satisfies
    // that forever. This carrier's true speed varies by 0.10% end to
    // end ({lo} … {hi}), so a per-span hull that is working gives away
    // about as much; it gives away 0.61% here. The guard admits 10%,
    // a hundred times the curve's own variation, and goes red long
    // before the assembly is hulling as coarsely as it does on the
    // adversarial nets below (32% on the degree-5 alternating-weight
    // one).
    assert!(
        m >= 0.9 * lo,
        "the meter gave away more than a tenth of the true speed: {m} against {lo}"
    );
}

#[test]
fn a_carrier_that_doubles_back_now_meters_its_speed() {
    // FLIPPED (M8-14, #222): this row used to assert a non-positive
    // meter, because "no single direction bounds this curve's velocity
    // from below" — which was true of the ARM (one global chord), not
    // of the curve: the meter's contract is SPEED, never injectivity,
    // and this loop's speed never drops. The integral arm now runs a
    // per-span scan joined with the global chord (derivation on
    // `speed_lower_bound`), so the honest answer is positive — and
    // still a genuine lower bound. The refusal this row really
    // guarded — a carrier whose speed COLLAPSES — is pinned in
    // `m8_14_long_turn_meter::a_genuine_cusp_still_refuses`.
    let pts: Vec<Point3<f64>> = (0..=40)
        .map(|i| {
            let t = f64::from(i) / 40.0;
            let a = std::f64::consts::TAU * t;
            Point3::new(a.cos(), a.sin(), 0.0)
        })
        .collect();
    let c = NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap();
    let m = c.speed_lower_bound();
    assert!(
        m > 0.0,
        "a closed loop's speed never drops, so the per-span meter must be \
         positive, got {m}"
    );
    for i in 0..=400 {
        let t = f64::from(i) / 400.0;
        let s = c.deriv(t).norm();
        assert!(
            s >= m - 1e-12,
            "meter {m} exceeds the real speed {s} at t = {t}"
        );
    }
}

/// Both halves of the contract at once: the bound is REAL (strictly
/// positive) and SOUND (never above the densely sampled true minimum
/// speed). Returns `bound / true_min` — how much the conservative
/// assembly gives away.
fn assert_real_and_sound(name: &str, c: &NurbsCurve3<f64>) -> f64 {
    let m = c.speed_lower_bound();
    assert!(m > 0.0, "{name}: the rational meter must be real, got {m}");
    let mut lo = f64::INFINITY;
    for i in 0..=4000 {
        let t = f64::from(i) / 4000.0;
        let s = c.deriv(t).norm();
        assert!(
            s >= m - 1e-12,
            "{name}: meter {m} exceeds the real speed {s} at t = {t}"
        );
        if s < lo {
            lo = s;
        }
    }
    m / lo
}

/// An exact quarter circle of radius `r` — the plainest rational
/// carrier the kernel actually mints (an arc's weights are the arc,
/// not an artifact to be removed).
fn rational_arc(r: f64) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let w = 0.5f64.sqrt();
    NurbsCurve3::new(
        kv,
        vec![
            Point3::new(r, 0.0, 0.0),
            Point3::new(r, r, 0.0),
            Point3::new(0.0, r, 0.0),
        ],
        vec![1.0, w, 1.0],
    )
    .unwrap()
}

/// A shallow quadratic net whose middle weight is the dial.
fn weighted_net(w1: f64) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    NurbsCurve3::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.5, 0.3, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![1.0, w1, 1.0],
    )
    .unwrap()
}

/// A degree-5 single-span net; the weights are the dial.
fn quintic_net(weights: Vec<f64>) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        5,
    )
    .unwrap();
    NurbsCurve3::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.2, 0.1, 0.0),
            Point3::new(0.4, -0.1, 0.05),
            Point3::new(0.6, 0.1, -0.05),
            Point3::new(0.8, -0.1, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
        weights,
    )
    .unwrap()
}

#[test]
fn a_rational_carrier_states_a_real_lower_bound() {
    // FLIPPED from `a_rational_carrier_poisons_rather_than_claiming_a_
    // bound` (M5 PR 7 → M7). The convex-hull argument is still about
    // the control net of a NON-rational derivative, and a rational
    // curve's derivative is still not a convex combination of any net
    // — so the bound is no longer read off one. It is the quotient-rule
    // assembly documented at `speed_lower_bound`'s rational arm:
    // `C′ = (Ã′ − (C − c)·w′)/w` with `Ã = A − c·w`, every ingredient a
    // hull of HOMOGENEOUS coefficients over the span, divided by the
    // weight hull's MAXIMUM (a lower bound wants the largest
    // denominator, not the min-weight floor an upper bound wants).
    // The claim is the same one the integral arm makes, and it is
    // checked the same way: real, and never above the true speed.
    //
    // The exact curve the retired poison row used, first.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ];
    let c = NurbsCurve3::<f64>::new(kv, control, vec![1.0, 0.5, 1.0]).unwrap();
    assert_real_and_sound("the retired poison row", &c);

    // And the carrier the kernel actually mints, at two scales — the
    // meter is a rate, so the ratio must not move with the radius.
    let big = assert_real_and_sound("quarter circle r = 1", &rational_arc(1.0));
    let small = assert_real_and_sound("quarter circle r = 0.05", &rational_arc(0.05));
    assert!(
        (big - small).abs() < 1e-9,
        "the bound must be scale-covariant: {big} vs {small}"
    );
}

#[test]
fn the_rational_meter_survives_adversarial_weights() {
    // Extreme ratios (0.01 … 100 against unit end weights), a weight a
    // hair off the non-positive boundary, and higher degrees. Every
    // row states a real, sound bound.
    for w1 in [0.01, 0.1, 10.0, 100.0, 1e-6] {
        assert_real_and_sound(&format!("w1 = {w1}"), &weighted_net(w1));
    }

    // Degree 3, two spans, weights spanning 0.5 … 3.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let c = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.25, 0.2, 0.1),
            Point3::new(0.5, -0.1, 0.0),
            Point3::new(0.75, 0.2, -0.1),
            Point3::new(1.0, 0.0, 0.0),
        ],
        vec![1.0, 2.0, 0.5, 3.0, 1.0],
    )
    .unwrap();
    assert_real_and_sound("degree 3, two spans", &c);

    assert_real_and_sound("degree 5", &quintic_net(vec![1.0, 0.5, 2.0, 0.5, 2.0, 1.0]));
}

#[test]
fn a_rational_carrier_whose_speed_collapses_still_refuses() {
    // The honest half of the flip — the rational analogue of
    // `a_carrier_that_doubles_back_reports_a_non_positive_meter`. Both
    // fixtures below have a GENUINE stationary point, and each proves
    // it in the row rather than asserting it in a comment: the sampled
    // minimum speed is required to collapse before the meter is asked
    // anything. No assembly may manufacture a positive
    // metre-per-parameter for a curve that stops, and the refusal
    // machinery (`nurbs_span_meter`, `split_edge_param_interior`)
    // reads exactly this and escalates.
    //
    // 1. A cubic whose two legs are exactly ANTI-PARALLEL across a
    //    repeated interior control point. For a cubic the homogeneous
    //    velocity is `(1−t)²·A + t²·B` in the legs `A = P₁ − P₀` and
    //    `B = P₃ − P₂`, which vanishes exactly when `B ∝ −A` — here
    //    `B = −A/2`, so the curve genuinely stops (and the endpoints
    //    still differ, so this is not the turn-around case below).
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let stationary = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(1.0, 1.0, 0.0),
            Point3::new(0.5, 0.5, 0.0),
        ],
        vec![1.0, 0.6, 2.0, 1.0],
    )
    .unwrap();

    // 2. The exact turn-around: a net whose ends coincide really does
    //    stop and reverse.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let turn_around = NurbsCurve3::<f64>::new(
        kv,
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.5, 1.0, 0.0),
            Point3::new(0.0, 0.0, 0.0),
        ],
        vec![1.0, 0.5, 1.0],
    )
    .unwrap();

    for (name, c) in [
        ("anti-parallel legs", &stationary),
        ("turn-around", &turn_around),
    ] {
        // The fixture must EARN the name: sample densely and require
        // the true minimum speed to have collapsed. A fixture that
        // merely turns sharply would refuse from hull conservatism,
        // which is a different property and has its own row.
        let mut lo = f64::INFINITY;
        for i in 0..=40_000 {
            let t = f64::from(i) / 40_000.0;
            let s = c.deriv(t).norm();
            if s < lo {
                lo = s;
            }
        }
        assert!(
            lo < 1e-4,
            "{name}: this row pins a GENUINE speed collapse, but the fixture's true \
             minimum speed is {lo} — re-derive the fixture or move it to the \
             conservatism row"
        );
        let m = c.speed_lower_bound();
        assert!(
            !(m > 0.0),
            "{name}: a carrier that stops has no positive speed bound, got {m}"
        );
    }
}

#[test]
fn the_conservative_frontier_is_stated_not_hidden() {
    // Where the assembly gives up while the CURVE IS FINE. Both rows
    // below refuse with a comfortably positive true minimum speed, so
    // what they pin is the schedule's conservatism, not a property of
    // the geometry — stated out loud so the frontier moves visibly if
    // `RATIONAL_METER_SPLITS` is ever revisited. Refusing is always
    // sound; it is only ever a usability cost.
    let sharp_turn = {
        // A near-cusp: a repeated interior control point whose legs are
        // NOT anti-parallel, so the curve turns hard but never stops.
        // Opening the corner further moves the meter positive only once
        // the turn is quite gentle — the conservatism is broad, and this
        // row is where that is admitted.
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
        NurbsCurve3::<f64>::new(
            kv,
            vec![
                Point3::new(0.0, 0.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(1.0, 1.0, 0.0),
                Point3::new(0.0, 0.2, 0.0),
            ],
            vec![1.0, 0.6, 2.0, 1.0],
        )
        .unwrap()
    };
    // A high degree AND alternating extreme weights leave the span
    // hulls too coarse for a positive answer even after refinement.
    let steep_weights = quintic_net(vec![1.0, 0.01, 100.0, 0.01, 100.0, 1.0]);

    for (name, c, floor) in [
        ("near-cusp", &sharp_turn, 1e-2),
        (
            "degree 5, alternating 0.01/100 weights",
            &steep_weights,
            1e-3,
        ),
    ] {
        let m = c.speed_lower_bound();
        assert!(
            !(m > 0.0),
            "the stated frontier moved — {name} now meters at {m}. Good news, but \
             re-derive this row (and the schedule constant it pins) rather than \
             deleting it."
        );
        // Sound even where it refuses, and the truth is positive — which
        // is exactly what makes this conservatism rather than collapse.
        let mut lo = f64::INFINITY;
        for i in 0..=4000 {
            let t = f64::from(i) / 4000.0;
            let s = c.deriv(t).norm();
            assert!(
                s >= m - 1e-12,
                "{name}: meter {m} exceeds the real speed {s}"
            );
            if s < lo {
                lo = s;
            }
        }
        assert!(
            lo > floor,
            "{name}: this row pins CONSERVATISM, so the curve must not actually stop \
             — true minimum speed {lo}"
        );
    }
}

/// The certified lane's own row: the `Interval` instantiation's meter
/// must BRACKET the `f64` one for the same carrier. The `f64` lane
/// computes the assembly with nearest rounding, so its answer is a
/// bound up to ~ulp; the interval lane is the one that carries the
/// enclosure, and containment is the property that makes the two
/// readings compatible rather than merely similar.
///
/// Adopted from R1's review probes for PR #306
/// (`kernel/span-review-probes`, `review_span_probe.rs`) — the review
/// noticed the rational arm had no interval-bracket row and wrote one.
#[cfg(feature = "interval")]
#[test]
fn the_interval_meter_brackets_the_f64_meter() {
    use geom_core::{Bounds, Interval};
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let weights = vec![1.0, 0.05, 1.0];
    let cf = NurbsCurve3::<f64>::new(
        kv.clone(),
        vec![
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(0.5, 0.3, 0.0),
            Point3::new(1.0, 0.0, 0.0),
        ],
        weights.clone(),
    )
    .unwrap();
    let mf = cf.speed_lower_bound();
    assert!(mf > 0.0, "the spot carrier meters positively: {mf}");

    let pt = |x: f64, y: f64, z: f64| {
        Point3::new(
            Interval::from_bounds(x, x),
            Interval::from_bounds(y, y),
            Interval::from_bounds(z, z),
        )
    };
    let ci = NurbsCurve3::<Interval>::new(
        kv,
        vec![pt(0.0, 0.0, 0.0), pt(0.5, 0.3, 0.0), pt(1.0, 0.0, 0.0)],
        weights,
    )
    .unwrap();
    let mi = ci.speed_lower_bound();
    assert!(
        mi.lo() <= mf && mf <= mi.hi(),
        "the interval meter [{}, {}] does not bracket the f64 meter {mf}",
        mi.lo(),
        mi.hi()
    );
}

#[test]
fn an_illegal_rational_carrier_is_still_poison() {
    // A non-positive weight breaks the convex-combination licence the
    // whole assembly stands on — every hull in it would be a lie. The
    // constructor refuses one outright, so the illegal net cannot even
    // be built, and the meter re-checks the same structure itself
    // rather than trusting that.
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.5, 0.3, 0.0),
        Point3::new(1.0, 0.0, 0.0),
    ];
    assert!(NurbsCurve3::<f64>::new(kv, control, vec![1.0, -0.5, 1.0]).is_err());
}
