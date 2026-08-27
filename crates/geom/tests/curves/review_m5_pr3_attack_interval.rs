//! ADVERSARIAL review battery for M5 PR 3, interval lane
//! (reviewer-authored, scratch): F1 empty-span-skip containment attack
//! at C0 kinks (degenerate/straddling/adjacent boxes), F2 Dual<Interval>
//! straddle enclosing BOTH one-sided tangents, F4 interval bound
//! enclosure, F5 post-op enclosures contain pre-op f64 samples.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(missing_docs)]

use geom::NurbsCurve3;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Dual, Interval, Point3, Real};

fn kink_curve() -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, 0.0, 1.0),
        Point3::new(3.0, -2.0, 0.0),
        Point3::new(4.0, 0.0, -1.0),
    ];
    let weights = vec![1.0, 0.7, 1.3, 0.9, 1.0];
    NurbsCurve3::new(kv, control, weights).unwrap()
}

fn lift(c: &NurbsCurve3<f64>) -> NurbsCurve3<Interval> {
    let ctrl = c
        .control()
        .iter()
        .map(|p| {
            Point3::new(
                Interval::from_f64(p.x),
                Interval::from_f64(p.y),
                Interval::from_f64(p.z),
            )
        })
        .collect();
    NurbsCurve3::new(c.knots().clone(), ctrl, c.weights().to_vec()).unwrap()
}

fn contains(e: Interval, v: f64) -> bool {
    e.lo() <= v && v <= e.hi()
}

fn assert_box_contains_samples(
    ci: &NurbsCurve3<Interval>,
    cf: &NurbsCurve3<f64>,
    lo: f64,
    hi: f64,
    extra: &[f64],
) {
    let t = if lo == hi {
        Interval::from_f64(lo)
    } else {
        Interval::from_bounds(lo, hi)
    };
    let p = ci.eval(t);
    let mut samples: Vec<f64> = (0..=400)
        .map(|i| lo + (hi - lo) * i as f64 / 400.0)
        .collect();
    samples.extend_from_slice(extra);
    for tf in samples {
        if tf < lo || tf > hi {
            continue;
        }
        let pf = cf.eval(tf);
        assert!(
            contains(p.x, pf.x) && contains(p.y, pf.y) && contains(p.z, pf.z),
            "F1 containment FAILS on [{lo},{hi}] at t={tf}: box=({:?},{:?},{:?}) point=({},{},{})",
            p.x,
            p.y,
            p.z,
            pf.x,
            pf.y,
            pf.z
        );
    }
}

/// F1: boxes degenerate AT the multiplicity-p knot, straddling it, and
/// adjacent on both sides — containment of the f64 curve everywhere,
/// including exactly at the kink and one ulp to each side.
#[test]
fn f1_kink_boxes_contain_f64_everywhere() {
    let c = kink_curve();
    let ci = lift(&c);
    let u = 0.5f64;
    let below = f64::from_bits(u.to_bits() - 1);
    let above = f64::from_bits(u.to_bits() + 1);
    let extra = [u, below, above];
    // Degenerate box exactly at the kink.
    assert_box_contains_samples(&ci, &c, u, u, &extra);
    // Tight straddles.
    assert_box_contains_samples(&ci, &c, u - 1e-9, u + 1e-9, &extra);
    assert_box_contains_samples(&ci, &c, u - 1e-3, u + 1e-3, &extra);
    // One-sided boxes ending exactly at the kink.
    assert_box_contains_samples(&ci, &c, 0.4, u, &extra);
    assert_box_contains_samples(&ci, &c, u, 0.6, &extra);
    // Wide straddle and full domain.
    assert_box_contains_samples(&ci, &c, 0.3, 0.7, &extra);
    assert_box_contains_samples(&ci, &c, 0.0, 1.0, &extra);
}

/// F1b: full-multiplicity insertion creates the empty spans in vivo —
/// a smooth cubic with 0.3 inserted to multiplicity 3 must still give
/// containment for straddling boxes on the refined curve.
#[test]
fn f1_full_multiplicity_insertion_straddle_containment() {
    let kv =
        KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.3, 0.7, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 3.0, -1.0),
        Point3::new(2.0, -1.0, 2.0),
        Point3::new(3.0, 2.0, 0.5),
        Point3::new(4.0, 0.0, -2.0),
        Point3::new(5.0, 1.0, 1.0),
    ];
    let weights = vec![1.0, 0.6, 1.4, 0.8, 1.2, 1.0];
    let c = NurbsCurve3::new(kv, control, weights).unwrap();
    // 0.3 already has multiplicity 1; two more copies reach the full
    // interior budget p = 3 (C0 kink, two EMPTY spans at 0.3).
    let refined = c.insert_knot(0.3, 2).unwrap();
    // The refined knot vector now has runs 0.3 x3 (and original 0.7):
    // straddling boxes cross empty spans.
    let ri = lift(&refined);
    let u = 0.3f64;
    let extra = [
        u,
        f64::from_bits(u.to_bits() - 1),
        f64::from_bits(u.to_bits() + 1),
        0.7,
    ];
    assert_box_contains_samples(&ri, &refined, u, u, &extra);
    assert_box_contains_samples(&ri, &refined, u - 1e-6, u + 1e-6, &extra);
    assert_box_contains_samples(&ri, &refined, 0.25, 0.35, &extra);
    assert_box_contains_samples(&ri, &refined, 0.1, 0.9, &extra);
    // The refined interval curve must also contain the ORIGINAL f64
    // curve's values (evaluation invariance + containment composed).
    assert_box_contains_samples(&ri, &c, 0.2, 0.8, &extra);
}

/// F2b: Dual<Interval> straddling the C0 kink — derivative channel must
/// enclose BOTH one-sided derivatives.
#[test]
fn f2_dual_interval_straddle_encloses_both_tangents() {
    let c = kink_curve();
    let ctrl = c
        .control()
        .iter()
        .map(|p| {
            Point3::new(
                Dual::constant(Interval::from_f64(p.x)),
                Dual::constant(Interval::from_f64(p.y)),
                Dual::constant(Interval::from_f64(p.z)),
            )
        })
        .collect();
    let cdi: NurbsCurve3<Dual<Interval>> =
        NurbsCurve3::new(c.knots().clone(), ctrl, c.weights().to_vec()).unwrap();
    let t = Dual::variable(Interval::from_bounds(0.5 - 1e-9, 0.5 + 1e-9));
    let p = cdi.eval(t);
    // One-sided tangents from the f64 lane.
    let dl = c.deriv(0.5 - 1e-9);
    let dr = c.deriv(0.5);
    // Both sides differ strongly (genuine kink) and BOTH must be inside
    // the derivative channel.
    assert!((dl.y - dr.y).abs() > 1.0, "fixture must kink");
    for (side, d) in [("left", dl), ("right", dr)] {
        assert!(
            contains(p.x.deriv, d.x) && contains(p.y.deriv, d.y) && contains(p.z.deriv, d.z),
            "F2 {side} tangent dropped: channel=({:?},{:?},{:?}) tangent=({},{},{})",
            p.x.deriv,
            p.y.deriv,
            p.z.deriv,
            d.x,
            d.y,
            d.z
        );
    }
    // Value channel still contains the pointwise values near the kink.
    for tf in [0.5 - 1e-9, 0.5, 0.5 + 1e-9] {
        let pf = c.eval(tf);
        assert!(
            contains(p.x.value, pf.x) && contains(p.y.value, pf.y) && contains(p.z.value, pf.z)
        );
    }
}

/// F4b: the Interval instantiation of the removal bound encloses the
/// f64 bound on a reviewer-planted LOSSY removal.
#[test]
fn f4_interval_removal_bound_encloses_f64_bound() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 0.4, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 4.0, -2.0),
        Point3::new(2.0, -3.0, 1.0),
        Point3::new(3.0, 2.0, 3.0),
        Point3::new(4.0, 0.0, 0.0),
    ];
    let weights = vec![1.0, 0.8, 1.3, 0.9, 1.0];
    // Near-removable planted case: perturb one control point so the
    // removal is genuinely lossy but the weight chain stays positive.
    let c = NurbsCurve3::new(kv, control, weights).unwrap();
    let mut ctrl = c.control().to_vec();
    ctrl[2] = ctrl[2] + geom_core::Vec3::new(0.02, -0.015, 0.01);
    let c = NurbsCurve3::new(c.knots().clone(), ctrl, c.weights().to_vec()).unwrap();
    let ci = lift(&c);
    let (hat_f, bf) = c.remove_knot(0.4, 1).unwrap();
    let (_hat_i, bi) = ci.remove_knot(0.4, 1).unwrap();
    assert!(
        bf > 0.0,
        "removal must be genuinely lossy, got bound {bf:e}"
    );
    assert!(
        bi.lo() <= bf && bf <= bi.hi(),
        "interval bound [{},{}] does not enclose f64 bound {bf}",
        bi.lo(),
        bi.hi()
    );
    // And the f64 bound is honest on a dense sample.
    for i in 0..=1000 {
        let t = i as f64 / 1000.0;
        let (p, q) = (c.eval(t), hat_f.eval(t));
        let e = (p.x - q.x)
            .abs()
            .max((p.y - q.y).abs())
            .max((p.z - q.z).abs());
        assert!(e <= bf + 1e-12, "bound violated at t={t}");
    }
}

/// F5b: post-op Interval enclosures contain pre-op f64 samples for
/// insertion-to-multiplicity and elevation on the KINK curve (the
/// hostile fixture, not the smooth circle).
#[test]
fn f5_post_op_enclosures_contain_pre_op_f64_on_kink_curve() {
    let c = kink_curve();
    let ci = lift(&c);
    let cases: Vec<(&str, NurbsCurve3<Interval>)> = vec![
        ("insert_to_mult", ci.insert_knot(0.25, 2).unwrap()),
        ("refine_ties", ci.refine_knots(&[0.75, 0.75, 0.1]).unwrap()),
        ("elevate", ci.elevate_degree(1).unwrap()),
    ];
    for (name, post) in &cases {
        for i in 0..=300 {
            let t = i as f64 / 300.0;
            let pf = c.eval(t);
            let pi = post.eval(Interval::from_f64(t));
            assert!(
                contains(pi.x, pf.x) && contains(pi.y, pf.y) && contains(pi.z, pf.z),
                "{name}: post-op enclosure lost the pre-op value at t={t}"
            );
        }
    }
}
