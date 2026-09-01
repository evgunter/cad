//! M5 PR 3 acceptance, interval lane (spec row 4): Interval
//! containment for NURBS curve evaluation — single-span boxes and
//! knot-straddling boxes — plus poison propagation and the
//! `Dual<Interval>` instantiation. Runs under `--features interval`.
//!
//! Containment logic: every interval ring op encloses all exact
//! results over contained inputs and rounds outward, and f64's
//! nearest-rounded step result stays inside the outward-rounded
//! enclosure of a contained intermediate — so an f64 run of the same
//! fixed-association recipe is contained inductively, and asserting
//! `eval(Interval) ∋ eval(f64)` is legitimate here (pure ring
//! arithmetic; contrast the transcendental caveat in `Bounds`' docs).

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::needless_range_loop)]

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Dual, Interval, Point3, Real};

const SQRT2_2: f64 = core::f64::consts::FRAC_1_SQRT_2;

/// The §7.3 rational-quadratic full circle in a tilted exact frame
/// (the differential suite's fixture, duplicated — integration tests
/// are separate crates).
fn nurbs_circle() -> NurbsCurve3<f64> {
    let c = Point3::new(-0.5, 4.0, 1.25);
    let x = geom_core::Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0);
    let y = geom_core::Vec3::new(2.0 / 3.0, -1.0 / 3.0, -2.0 / 3.0);
    let r = 2.5;
    let p = |cx: f64, cy: f64| c + x * (r * cx) + y * (r * cy);
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

fn contains(e: Interval, v: f64) -> bool {
    e.lo() <= v && v <= e.hi()
}

/// Single-span boxes: `eval(t: Interval)` contains `eval(t: f64)` for
/// a dense grid of f64 parameters inside the box.
#[test]
fn interval_eval_contains_f64_eval_single_span() {
    let n = nurbs_circle();
    let ni = n.map_scalar(Interval::from_f64);
    for (lo, hi) in [(0.02, 0.2), (0.26, 0.49), (0.51, 0.74), (0.9, 0.999)] {
        let t = Interval::from_bounds(lo, hi);
        let p = ni.eval(t);
        for i in 0..=60usize {
            let tf = lo + (hi - lo) * i as f64 / 60.0;
            let pf = n.eval(tf);
            assert!(
                contains(p.x, pf.x) && contains(p.y, pf.y) && contains(p.z, pf.z),
                "eval containment fails on [{lo}, {hi}] at t = {tf}"
            );
        }
        // Derivatives too.
        let d = ni.deriv(t);
        let d2 = ni.deriv2(t);
        for i in 0..=20usize {
            let tf = lo + (hi - lo) * i as f64 / 20.0;
            let df = n.deriv(tf);
            let d2f = n.deriv2(tf);
            assert!(contains(d.x, df.x) && contains(d.y, df.y) && contains(d.z, df.z));
            assert!(contains(d2.x, d2f.x) && contains(d2.y, d2f.y) && contains(d2.z, d2f.z));
        }
    }
}

/// Knot-straddling boxes: the hull-of-spans result still contains
/// every pointwise value (continuity across the knot).
#[test]
fn interval_eval_contains_f64_across_knots() {
    let n = nurbs_circle();
    let ni = n.map_scalar(Interval::from_f64);
    for (lo, hi) in [(0.2, 0.3), (0.45, 0.8), (0.0, 1.0)] {
        let t = Interval::from_bounds(lo, hi);
        let p = ni.eval(t);
        let d = ni.deriv(t);
        for i in 0..=120usize {
            let tf = lo + (hi - lo) * i as f64 / 120.0;
            let pf = n.eval(tf);
            let df = n.deriv(tf);
            assert!(
                contains(p.x, pf.x) && contains(p.y, pf.y) && contains(p.z, pf.z),
                "straddle containment fails on [{lo}, {hi}] at t = {tf}"
            );
            assert!(contains(d.x, df.x) && contains(d.y, df.y) && contains(d.z, df.z));
        }
    }
}

/// Poison propagation (spec row 4): poisoned parameter or control
/// point yields a poisoned result — a value, never a decision, never
/// a panic.
#[test]
fn poison_propagates_through_values() {
    let n = nurbs_circle();
    let ni = n.map_scalar(Interval::from_f64);
    // Poisoned parameter (NaI).
    let p = ni.eval(Interval::from_f64(f64::NAN));
    assert!(p.x.lo().is_nan() && p.x.hi().is_nan());
    // Poisoned control point at f64.
    let mut ctrl = n.control().to_vec();
    ctrl[3] = Point3::new(f64::NAN, 0.0, 0.0);
    let np = NurbsCurve3::new(n.knots().clone(), ctrl, n.weights().to_vec()).unwrap();
    // A parameter whose span touches control index 3 poisons x.
    assert!(np.eval(0.3).x.is_nan());
    // A far-away span does not (locality of the basis).
    assert!(!np.eval(0.9).x.is_nan());
}

/// `Dual<Interval>` instantiates and both channels contain the
/// pointwise `Dual<f64>` runs (value and derivative channels hulled
/// independently across spans — the seam's convention).
#[test]
fn dual_interval_channels_contain_pointwise_duals() {
    let n = nurbs_circle();
    let ndi: NurbsCurve3<Dual<Interval>> = n.map_scalar(|x| Dual::constant(Interval::from_f64(x)));
    for (lo, hi) in [(0.1, 0.2), (0.45, 0.55)] {
        let t = Dual::variable(Interval::from_bounds(lo, hi));
        let p = ndi.eval(t);
        for i in 0..=40usize {
            let tf = lo + (hi - lo) * i as f64 / 40.0;
            let pf = n.eval(tf);
            let df = n.deriv(tf);
            assert!(
                contains(p.x.value, pf.x),
                "value channel at t = {tf} in [{lo}, {hi}]"
            );
            assert!(
                contains(p.x.deriv, df.x),
                "deriv channel at t = {tf} in [{lo}, {hi}]"
            );
            assert!(contains(p.y.deriv, df.y) && contains(p.z.deriv, df.z));
        }
    }
}

/// Knot-algebra results at Interval contain the f64 results on shared
/// samples (the spec's per-op containment obligation).
#[test]
fn knot_algebra_interval_contains_f64() {
    let n = nurbs_circle();
    let ni = n.map_scalar(Interval::from_f64);
    let cases_f64 = [
        n.insert_knot(0.6, 1).unwrap(),
        n.refine_knots(&[0.1, 0.9]).unwrap(),
        n.elevate_degree(1).unwrap(),
    ];
    let cases_iv = [
        ni.insert_knot(0.6, 1).unwrap(),
        ni.refine_knots(&[0.1, 0.9]).unwrap(),
        ni.elevate_degree(1).unwrap(),
    ];
    for (cf, ci) in cases_f64.iter().zip(cases_iv.iter()) {
        assert_eq!(cf.knots(), ci.knots());
        for i in 0..=50usize {
            let t = i as f64 / 50.0;
            let pf = cf.eval(t);
            let pi = ci.eval(Interval::from_f64(t));
            assert!(
                contains(pi.x, pf.x) && contains(pi.y, pf.y) && contains(pi.z, pf.z),
                "knot-algebra containment at t = {t}"
            );
        }
    }
    // Removal: the interval bound is an enclosure; its f64 counterpart
    // lies inside it.
    let with = n.insert_knot(0.6, 1).unwrap();
    let with_i = ni.insert_knot(0.6, 1).unwrap();
    let (_, bf) = with.remove_knot(0.6, 1).unwrap();
    let (_, bi) = with_i.remove_knot(0.6, 1).unwrap();
    assert!(
        bi.lo() <= bf && bf <= bi.hi(),
        "bound enclosure: {bf:e} in [{}, {}]",
        bi.lo(),
        bi.hi()
    );
}
