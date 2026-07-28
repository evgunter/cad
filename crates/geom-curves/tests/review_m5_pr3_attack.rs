//! ADVERSARIAL review battery for M5 PR 3 (reviewer-authored, scratch).
//! F2 Dual kink convention, F3 rational derivatives vs finite
//! differences, F4 removal-bound honesty on reviewer-planted lossy
//! removals, F5 seeded knot-algebra invariance fuzz (SEED = 0x5EED_CAD5),
//! F6 placeholder poison bit identity, F7 validation attacks.
//!
//! Tolerance derivations (stated per F5's assignment):
//! - Central FD of eval, h = 1e-5: truncation ~ h^2|C'''|/6 ~ 1e-10·|C'''|,
//!   rounding ~ eps·|C|/h ~ 1e-11·|C|; coords are O(10), so 1e-4 abs
//!   catches any O(1)-relative formula error with 1e5x headroom.
//! - Dual-vs-ders agreement: two exact algorithms, expect ~1e-12
//!   relative; assert 1e-9·(1+|d|).
//! - Knot-algebra invariance: every op is a chain of convex (alpha in
//!   (0,1)) combinations of depth <= ~50, so fp drift <= ~1e2·eps·scale
//!   ~ 1e-13 for coords O(10); assert 1e-9 abs — 1e4x above noise,
//!   1e9x below any real coefficient bug (which is O(coordinate)).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(missing_docs)]

use geom_core::spline::{KnotVector, KnotVectorIssue, SplineError};
// Promotion adaptation (mechanical): dropped an unused Real import.
use geom_core::{Dual64, Point3};
use geom_curves::{Curve3, NurbsCurve3};

// ---------- deterministic RNG (report: SEED below) ----------
const SEED: u64 = 0x5EED_CAD5;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        // xorshift64*
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn f(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.f()
    }
}

/// Random clamped curve: degree p, `interior` distinct interior knots
/// (multiplicity 1), coords in [-10, 10], weights in [wlo, whi].
fn random_curve(rng: &mut Rng, p: usize, interior: usize, wlo: f64, whi: f64) -> NurbsCurve3<f64> {
    let mut ks: Vec<f64> = vec![0.0; p + 1];
    let mut mids: Vec<f64> = (0..interior).map(|_| rng.range(0.05, 0.95)).collect();
    mids.sort_by(f64::total_cmp);
    mids.dedup();
    ks.extend_from_slice(&mids);
    ks.extend(std::iter::repeat_n(1.0, p + 1));
    let kv = KnotVector::clamped(ks, p).unwrap();
    let n = kv.control_count();
    let control = (0..n)
        .map(|_| {
            Point3::new(
                rng.range(-10.0, 10.0),
                rng.range(-10.0, 10.0),
                rng.range(-10.0, 10.0),
            )
        })
        .collect();
    let weights = (0..n).map(|_| rng.range(wlo, whi)).collect();
    NurbsCurve3::new(kv, control, weights).unwrap()
}

fn sup3(a: geom_core::Vec3<f64>, b: geom_core::Vec3<f64>) -> f64 {
    (a.x - b.x)
        .abs()
        .max((a.y - b.y).abs())
        .max((a.z - b.z).abs())
}

/// Constant-lift a curve's control points to Dual64 (structure shared).
fn lift_dual(c: &NurbsCurve3<f64>) -> NurbsCurve3<Dual64> {
    let ctrl = c
        .control()
        .iter()
        .map(|p| {
            Point3::new(
                Dual64::constant(p.x),
                Dual64::constant(p.y),
                Dual64::constant(p.z),
            )
        })
        .collect();
    NurbsCurve3::new(c.knots().clone(), ctrl, c.weights().to_vec()).unwrap()
}

fn supp(a: Point3<f64>, b: Point3<f64>) -> f64 {
    (a.x - b.x)
        .abs()
        .max((a.y - b.y).abs())
        .max((a.z - b.z).abs())
}

/// A degree-2 curve with a genuine C0 kink: interior knot 0.5 at
/// multiplicity p = 2 and control points that bend hard there.
fn kink_curve() -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, 0.0, 1.0), // the kink vertex (interpolated at t = 0.5)
        Point3::new(3.0, -2.0, 0.0),
        Point3::new(4.0, 0.0, -1.0),
    ];
    let weights = vec![1.0, 0.7, 1.3, 0.9, 1.0];
    NurbsCurve3::new(kv, control, weights).unwrap()
}

// ---------- F2: Dual kink convention ----------

#[test]
fn f2_dual_at_c0_kink_takes_the_selected_span_derivative() {
    let c = kink_curve();
    let cd = lift_dual(&c);
    // f64 tie-break at 0.5: the span STARTING there (right side).
    let d = cd.eval(Dual64::variable(0.5));
    let p = c.eval(0.5);
    assert_eq!(
        d.x.value.to_bits(),
        p.x.to_bits(),
        "value channel == f64 eval, bitwise"
    );
    assert_eq!(d.y.value.to_bits(), p.y.to_bits());
    assert_eq!(d.z.value.to_bits(), p.z.to_bits());
    // One-sided derivatives by FD.
    let h = 1e-7;
    let right = Point3::new(
        (c.eval(0.5 + h).x - p.x) / h,
        (c.eval(0.5 + h).y - p.y) / h,
        (c.eval(0.5 + h).z - p.z) / h,
    );
    let left = Point3::new(
        (p.x - c.eval(0.5 - h).x) / h,
        (p.y - c.eval(0.5 - h).y) / h,
        (p.z - c.eval(0.5 - h).z) / h,
    );
    // Genuine kink: the sides must disagree strongly...
    assert!(
        supp(left, right) > 1.0,
        "fixture must actually kink: {left:?} vs {right:?}"
    );
    // ...and Dual's derivative channel must be the RIGHT (selected-span)
    // side, matching deriv() at the same t.
    let dr = c.deriv(0.5);
    assert!((d.x.deriv - right.x).abs() < 1e-5 && (d.y.deriv - right.y).abs() < 1e-5);
    assert!((d.x.deriv - dr.x).abs() < 1e-9 && (d.y.deriv - dr.y).abs() < 1e-9);
}

// ---------- F3: rational derivatives vs central FD ----------

#[test]
fn f3_deriv_and_deriv2_match_central_differences_fuzzed() {
    let mut rng = Rng(SEED);
    for case in 0..40 {
        let p = 1 + (rng.next() as usize) % 5;
        let interior = (rng.next() as usize) % 4;
        let c = random_curve(&mut rng, p, interior, 0.2, 5.0);
        let cd = lift_dual(&c);
        let h = 1e-5;
        for _ in 0..12 {
            let t = rng.range(2.0 * h, 1.0 - 2.0 * h);
            // Skip parameters too close to a knot (FD would straddle a
            // continuity drop — not a derivative-correctness question).
            if c.knots().knots().iter().any(|k| (k - t).abs() < 10.0 * h) {
                continue;
            }
            let d = c.deriv(t);
            let fd = (c.eval(t + h) - c.eval(t - h)) / (2.0 * h);
            assert!(
                sup3(d, fd) < 1e-4 * (1.0 + d.norm()),
                "case {case}: deriv vs FD at t={t}: {d:?} vs {fd:?}"
            );
            let d2 = c.deriv2(t);
            let fd2 = (c.deriv(t + h) - c.deriv(t - h)) / (2.0 * h);
            assert!(
                sup3(d2, fd2) < 1e-4 * (1.0 + d2.norm()),
                "case {case}: deriv2 vs FD at t={t}: {d2:?} vs {fd2:?}"
            );
            // Dual channel agrees tightly with ders (both exact).
            let du = cd.eval(Dual64::variable(t));
            assert!((du.x.deriv - d.x).abs() < 1e-9 * (1.0 + d.x.abs()));
            assert!((du.y.deriv - d.y).abs() < 1e-9 * (1.0 + d.y.abs()));
            assert!((du.z.deriv - d.z).abs() < 1e-9 * (1.0 + d.z.abs()));
        }
    }
}

// ---------- F4: removal bound honesty on reviewer-planted removals ----------

fn removal_bound_case(c: &NurbsCurve3<f64>, u: f64, times: usize) {
    let (hat, bound) = c.remove_knot(u, times).unwrap();
    let mut worst = 0.0f64;
    for i in 0..=2000 {
        let t = i as f64 / 2000.0;
        let e = supp(c.eval(t), hat.eval(t));
        // sup-norm on coordinates <= euclidean norm, so coordinate sup
        // is the conservative side of |C - Chat|.
        if e > worst {
            worst = e;
        }
        assert!(
            e <= bound + 1e-12,
            "bound violated at t={t}: |C-Chat|={e:e} > bound={bound:e}"
        );
    }
    // Not vacuous: for a genuinely lossy removal the realized error is
    // nonzero; print-style diagnostics via assert message on failure.
    assert!(bound.is_finite() && bound >= 0.0);
    assert!(worst <= bound + 1e-12);
}

#[test]
fn f4_removal_bound_contains_realized_error_lossy_and_adversarial_weights() {
    let mut rng = Rng(SEED ^ 0x0F4);
    // (a) plain lossy removal, mult-1 interior knot, random cubic.
    for _ in 0..8 {
        let c = random_curve(&mut rng, 3, 3, 0.5, 2.0);
        let u = *c
            .knots()
            .knots()
            .iter()
            .find(|k| **k > 0.0 && **k < 1.0)
            .unwrap();
        removal_bound_case(&c, u, 1);
    }
    // (b) adversarial weights spanning [1e-3, 1e3].
    for _ in 0..8 {
        let p = 2 + (rng.next() as usize) % 3;
        let c0 = random_curve(&mut rng, p, 2, 1.0, 1.0);
        let n = c0.control().len();
        let weights: Vec<f64> = (0..n)
            .map(|i| {
                if i % 2 == 0 {
                    rng.range(1e-3, 1e-2)
                } else {
                    rng.range(1e2, 1e3)
                }
            })
            .collect();
        let c = NurbsCurve3::new(c0.knots().clone(), c0.control().to_vec(), weights).unwrap();
        let u = *c
            .knots()
            .knots()
            .iter()
            .find(|k| **k > 0.0 && **k < 1.0)
            .unwrap();
        match c.remove_knot(u, 1) {
            Ok(_) => removal_bound_case(&c, u, 1),
            Err(e) => {
                // WeightCollapse is an acceptable honest refusal here.
                assert!(
                    matches!(
                        e,
                        geom_core::spline::KnotAlgebraError::WeightCollapse { .. }
                    ),
                    "unexpected: {e:?}"
                );
            }
        }
    }
    // (c) multi-pass: multiplicity-2 knot removed twice — bounds must add.
    for _ in 0..6 {
        let base = random_curve(&mut rng, 3, 2, 0.5, 2.0);
        let u = 0.437;
        let c = base.insert_knot(u, 2).unwrap();
        // Perturb one control point so the double removal is genuinely lossy.
        let mut ctrl = c.control().to_vec();
        let mid = ctrl.len() / 2;
        ctrl[mid] = ctrl[mid] + geom_core::Vec3::new(0.05, -0.03, 0.02);
        let c = NurbsCurve3::new(c.knots().clone(), ctrl, c.weights().to_vec()).unwrap();
        removal_bound_case(&c, u, 2);
    }
}

// ---------- F5: knot-algebra invariance fuzz ----------

#[test]
fn f5_insert_refine_elevate_are_evaluation_invariant_fuzzed() {
    let mut rng = Rng(SEED ^ 0x0F5);
    let tol = 1e-9;
    for case in 0..30 {
        let p = 1 + (rng.next() as usize) % 5;
        let interior = (rng.next() as usize) % 3;
        let c = random_curve(&mut rng, p, interior, 0.2, 5.0);
        let mut variants: Vec<(String, NurbsCurve3<f64>)> = Vec::new();
        // insertion, single and to-multiplicity (budget = p).
        let u = rng.range(0.1, 0.9);
        if c.knots().multiplicity_of(u).is_none() {
            variants.push(("insert1".into(), c.insert_knot(u, 1).unwrap()));
            if p >= 2 {
                variants.push(("insert_to_mult_p".into(), c.insert_knot(u, p).unwrap()));
            }
        }
        // refinement, including a tie pair (ties only when the interior
        // multiplicity budget p allows 2).
        let r1 = rng.range(0.05, 0.45);
        let r2 = rng.range(0.55, 0.95);
        let adds: &[f64] = if p >= 2 { &[r2, r1, r1] } else { &[r2, r1] };
        variants.push(("refine".into(), c.refine_knots(adds).unwrap()));
        // elevation by 1 and 2.
        let e1 = c.elevate_degree(1).unwrap();
        assert_eq!(e1.degree(), p + 1, "degree must actually increase");
        variants.push(("elevate1".into(), e1));
        let e2 = c.elevate_degree(2).unwrap();
        assert_eq!(e2.degree(), p + 2);
        variants.push(("elevate2".into(), e2));
        for (name, v) in &variants {
            // End conditions stay clamped/interpolating.
            assert!(
                supp(v.eval(0.0), c.eval(0.0)) < tol,
                "case {case} {name}: start point moved"
            );
            assert!(
                supp(v.eval(1.0), c.eval(1.0)) < tol,
                "case {case} {name}: end point moved"
            );
            for i in 0..=200 {
                let t = i as f64 / 200.0;
                let d = supp(c.eval(t), v.eval(t));
                assert!(
                    d < tol,
                    "case {case} {name}: invariance broke at t={t}: {d:e}"
                );
            }
        }
        // Elevation continuity at every interior knot: value and (where
        // continuity order permits, i.e. mult < p) first derivative agree
        // across the knot on the ELEVATED curve.
        let e1 = &variants.iter().find(|(n, _)| n == "elevate1").unwrap().1;
        let interior_vals: Vec<f64> = {
            let ks = c.knots().knots();
            let mut v: Vec<f64> = ks
                .iter()
                .copied()
                .filter(|k| *k > 0.0 && *k < 1.0)
                .collect();
            v.dedup();
            v
        };
        for kv in interior_vals {
            let h = 1e-9;
            assert!(
                supp(e1.eval(kv - h), e1.eval(kv + h)) < 1e-6,
                "case {case}: elevated curve discontinuous at {kv}"
            );
            if c.knots().multiplicity_of(kv).map(|(m, _)| m).unwrap_or(0) < p {
                let dl = e1.deriv(kv - h);
                let dr = e1.deriv(kv + h);
                assert!(
                    sup3(dl, dr) < 1e-4 * (1.0 + dl.norm()),
                    "case {case}: C1 lost at {kv}"
                );
            }
        }
    }
}

// ---------- F6: placeholder poison bit identity ----------

#[test]
fn f6_placeholder_evaluates_to_quiet_nan_bits() {
    let c: Curve3<f64> = Curve3::nurbs_placeholder();
    let p = c.eval(0.5);
    // Old unit-variant behavior: from_f64(NAN) coordinates. Bit-identical
    // quiet NaN expected.
    assert_eq!(
        p.x.to_bits(),
        f64::NAN.to_bits(),
        "poison bits changed: {:#x}",
        p.x.to_bits()
    );
    assert_eq!(p.y.to_bits(), f64::NAN.to_bits());
    assert_eq!(p.z.to_bits(), f64::NAN.to_bits());
    let d = c.deriv(0.5);
    assert_eq!(d.x.to_bits(), f64::NAN.to_bits());
    let d2 = c.deriv2(f64::NAN); // poison parameter on poison payload
    assert!(d2.x.is_nan() && d2.y.is_nan() && d2.z.is_nan());
}

// ---------- F7: validation and totality attacks ----------

#[test]
fn f7_construction_attacks_all_typed_never_panic() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let pts = |n: usize| {
        (0..n)
            .map(|i| Point3::new(i as f64, 0.0, 0.0))
            .collect::<Vec<_>>()
    };
    // weight attacks
    for (w, expect_nonpos) in [
        (0.0, true),
        (-1.0, true),
        (f64::NAN, true),
        (f64::NEG_INFINITY, true),
        (f64::INFINITY, false), // > 0 but non-finite
    ] {
        let err = NurbsCurve3::new(kv.clone(), pts(3), vec![1.0, w, 1.0]).unwrap_err();
        match (expect_nonpos, &err) {
            (true, SplineError::NonPositiveWeight { index: 1, .. }) => {}
            (false, SplineError::NonFiniteWeight { index: 1, .. }) => {}
            _ => panic!("wrong refusal for w={w}: {err:?}"),
        }
    }
    // subnormal weight: positive and finite — the documented contract accepts it.
    assert!(NurbsCurve3::<f64>::new(kv.clone(), pts(3), vec![1.0, 5e-324, 1.0]).is_ok());
    // count mismatches, empty control
    assert!(matches!(
        NurbsCurve3::<f64>::new(kv.clone(), pts(4), vec![1.0; 4]).unwrap_err(),
        SplineError::ControlCountMismatch {
            control: 4,
            expected: 3
        }
    ));
    assert!(matches!(
        NurbsCurve3::<f64>::new(kv.clone(), Vec::new(), Vec::new()).unwrap_err(),
        SplineError::ControlCountMismatch {
            control: 0,
            expected: 3
        }
    ));
    assert!(matches!(
        NurbsCurve3::<f64>::new(kv.clone(), pts(3), vec![1.0; 2]).unwrap_err(),
        SplineError::WeightCountMismatch {
            weights: 2,
            control: 3
        }
    ));
    // knot vector attacks (typed, exact variants)
    let bad = |ks: Vec<f64>, p: usize| KnotVector::clamped(ks, p).unwrap_err();
    assert!(matches!(
        bad(vec![0.0, 0.0, 1.0, 1.0], 0),
        SplineError::KnotVectorInvalid {
            reason: KnotVectorIssue::DegreeZero
        }
    ));
    assert!(matches!(
        bad(vec![0.0, 0.0, 0.0, 0.4, 0.4, 0.4, 1.0, 1.0, 1.0], 2),
        SplineError::KnotVectorInvalid {
            reason: KnotVectorIssue::InteriorMultiplicityTooHigh { .. }
        }
    ));
    assert!(matches!(
        bad(vec![0.0, 0.0, 0.0, f64::INFINITY, 1.0, 1.0, 1.0], 2),
        SplineError::KnotVectorInvalid {
            reason: KnotVectorIssue::NonFinite { index: 3 }
        }
    ));
    assert!(matches!(
        bad(vec![0.0, 0.0, 0.0, 0.6, 0.4, 1.0, 1.0, 1.0], 2),
        SplineError::KnotVectorInvalid {
            reason: KnotVectorIssue::Decreasing { index: 4 }
        }
    ));
    assert!(matches!(
        bad(vec![0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2),
        SplineError::KnotVectorInvalid {
            reason: KnotVectorIssue::StartNotClamped
        }
    ));
}

#[test]
fn f7_find_span_fuzz_totality() {
    let mut rng = Rng(SEED ^ 0x0F7);
    for _ in 0..20 {
        let p = 1 + (rng.next() as usize) % 5;
        let c = random_curve(&mut rng, p, 3, 0.5, 2.0);
        let kv = c.knots();
        for t in [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            -1e300,
            1e300,
            -0.0,
            1.0 + 1e-16,
            f64::MIN_POSITIVE,
        ] {
            let s = kv.find_span(t);
            assert!(s >= kv.first_span() && s <= kv.last_span());
            assert!(
                kv.span_is_nonempty(s),
                "find_span returned an empty span for t={t}"
            );
        }
        for _ in 0..50 {
            let t = rng.range(-2.0, 3.0);
            let s = kv.find_span(t);
            assert!(kv.span_is_nonempty(s));
            // Promotion adaptation (mechanical): clippy's
            // manual_range_contains form.
            if (0.0..1.0).contains(&t) {
                let ks = kv.knots();
                assert!(
                    ks[s] <= t && t < ks[s + 1],
                    "half-open span contract broke at t={t}"
                );
            }
        }
        // Evaluation totality at garbage parameters: never panics.
        let _ = c.eval(f64::NAN);
        let _ = c.eval(f64::INFINITY);
        let _ = c.deriv2(-5.0);
    }
    // Degenerate single-span surface-ish curve: eval at both ends.
    let kv = KnotVector::unit_segment(1);
    let c = NurbsCurve3::new(
        kv,
        vec![Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 0.0, 0.0)],
        vec![1.0, 1.0],
    )
    .unwrap();
    assert!(supp(c.eval(0.0), Point3::new(0.0, 0.0, 0.0)) < 1e-15);
    assert!(supp(c.eval(1.0), Point3::new(1.0, 0.0, 0.0)) < 1e-15);
}

// ---------- F7b: knot-algebra parameter attacks ----------

#[test]
fn f7_algebra_refusals_typed() {
    use geom_core::spline::KnotAlgebraError as E;
    let mut rng = Rng(SEED ^ 0x1F7);
    let c = random_curve(&mut rng, 3, 1, 0.5, 2.0);
    for u in [0.0, 1.0, -0.5, 1.5, f64::NAN, f64::INFINITY] {
        assert!(
            matches!(
                c.insert_knot(u, 1).unwrap_err(),
                E::ParameterOutsideDomain { .. }
            ),
            "u={u}"
        );
    }
    // over-budget insertion (mult would exceed p)
    let u = 0.3;
    assert!(matches!(
        c.insert_knot(u, 4).unwrap_err(),
        E::MultiplicityOverflow { .. }
    ));
    // removal attacks
    assert!(matches!(
        c.remove_knot(0.123456, 1).unwrap_err(),
        E::KnotNotPresent { .. }
    ));
    assert!(matches!(
        c.remove_knot(0.0, 1).unwrap_err(),
        E::KnotNotPresent { .. }
    ));
    let interior = *c
        .knots()
        .knots()
        .iter()
        .find(|k| **k > 0.0 && **k < 1.0)
        .unwrap();
    assert!(matches!(
        c.remove_knot(interior, 2).unwrap_err(),
        E::RemovalExceedsMultiplicity { .. }
    ));
    // times = 0 insertion is a no-op clone
    let same = c.insert_knot(0.3, 0).unwrap();
    assert_eq!(same.knots(), c.knots());
}
