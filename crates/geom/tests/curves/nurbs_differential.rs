//! M5 PR 3 acceptance: the closed-form differential suite for NURBS
//! curves (spec row 1) plus determinism (row 2), refusal pinning
//! (row 3), knot-algebra evaluation invariance and the Tiller-bound
//! honesty test.
//!
//! The circle comparison uses the §7.3 exact rational quadratic (full
//! circle from four 90° arcs, w₁ = cos 45° = √2/2) against
//! [`Curve3::Circle`]. The NURBS parameter is not the angle, so the
//! comparison goes through the parameter map θ(t) and the chain rule;
//! θ, θ′, θ″ come from an independent closed-form Bernstein oracle
//! (hand-coded quadratics — not the kernel's de Boor path).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::needless_range_loop)]

use geom::{Curve3, NurbsCurve2, NurbsCurve3};
use geom_core::spline::{KnotVector, SplineError};
use geom_core::{Dual64, Point2, Point3, Vec3};

const SQRT2_2: f64 = core::f64::consts::FRAC_1_SQRT_2;

fn axis() -> Vec3<f64> {
    Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)
}

fn u_ref() -> Vec3<f64> {
    Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0)
}

fn v_ref() -> Vec3<f64> {
    // axis × u_ref, exact: (2, −1, −2)/3.
    Vec3::new(2.0 / 3.0, -1.0 / 3.0, -2.0 / 3.0)
}

fn center() -> Point3<f64> {
    Point3::new(-0.5, 4.0, 1.25)
}

const RADIUS: f64 = 2.5;

/// The §7.3 full circle: four rational-quadratic 90° arcs, w₁ = √2/2.
fn nurbs_circle() -> NurbsCurve3<f64> {
    let (c, r, x, y) = (center(), RADIUS, u_ref(), v_ref());
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

fn analytic_circle() -> Curve3<f64> {
    Curve3::Circle {
        center: center(),
        axis: axis(),
        radius: RADIUS,
        u_ref: u_ref(),
    }
}

/// Independent Bernstein oracle: hom point and derivatives (w.r.t. the
/// GLOBAL parameter t) of the rational-quadratic circle at t.
fn oracle(curve: &NurbsCurve3<f64>, t: f64) -> ([f64; 4], [f64; 4], [f64; 4]) {
    // Segment index: quarter arcs on [k/4, (k+1)/4]; t = 1 uses the
    // last. Local s = 4t − k, d/dt = 4·d/ds.
    let k = ((t * 4.0).floor() as usize).min(3);
    let s = 4.0 * t - k as f64;
    let idx = [2 * k, 2 * k + 1, 2 * k + 2];
    let ctrl = curve.control();
    let w = curve.weights();
    let mut h = [[0.0f64; 4]; 3];
    for (j, i) in idx.iter().enumerate() {
        h[j] = [
            w[*i] * (ctrl[*i].x),
            w[*i] * (ctrl[*i].y),
            w[*i] * (ctrl[*i].z),
            w[*i],
        ];
    }
    let b = [(1.0 - s) * (1.0 - s), 2.0 * s * (1.0 - s), s * s];
    let db = [-2.0 * (1.0 - s), 2.0 - 4.0 * s, 2.0 * s];
    let ddb = [2.0, -4.0, 2.0];
    let mut a0 = [0.0f64; 4];
    let mut a1 = [0.0f64; 4];
    let mut a2 = [0.0f64; 4];
    for j in 0..3 {
        for c in 0..4 {
            a0[c] += b[j] * h[j][c];
            a1[c] += db[j] * h[j][c] * 4.0; // d/dt = 4 d/ds
            a2[c] += ddb[j] * h[j][c] * 16.0;
        }
    }
    (a0, a1, a2)
}

/// Rational point + derivatives from the hom oracle (quotient rule).
fn oracle_cd(curve: &NurbsCurve3<f64>, t: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
    let (a0, a1, a2) = oracle(curve, t);
    let mut c = [0.0f64; 3];
    let mut c1 = [0.0f64; 3];
    let mut c2 = [0.0f64; 3];
    for i in 0..3 {
        c[i] = a0[i] / a0[3];
        c1[i] = (a1[i] - c[i] * a1[3]) / a0[3];
        c2[i] = (a2[i] - c[i] * a2[3] - 2.0 * c1[i] * a1[3]) / a0[3];
    }
    (c, c1, c2)
}

fn close(a: f64, b: f64, tol: f64, what: &str) {
    assert!(
        (a - b).abs() <= tol,
        "{what}: {a} vs {b} (|Δ| = {:e})",
        (a - b).abs()
    );
}

fn close3(a: Vec3<f64>, b: Vec3<f64>, tol: f64, what: &str) {
    close(a.x, b.x, tol, what);
    close(a.y, b.y, tol, what);
    close(a.z, b.z, tol, what);
}

/// Spec row 1: eval + deriv + deriv2 vs `Curve3::Circle` across a
/// dense schedule, through the θ(t) parameter map and the chain rule.
///
/// Tolerances: coordinates are O(r = 2.5) built from ~10 rational ops;
/// θ′/θ″ compose ~20 more. 1e-9 absolute is ~10⁵ ulps of slack at this
/// scale — tight against any structural error (a wrong span, weight,
/// or association term shows up at O(1)).
#[test]
fn rational_quadratic_circle_matches_analytic_circle() {
    let n = nurbs_circle();
    let circ = analytic_circle();
    let (c, x, y) = (center(), u_ref(), v_ref());
    for i in 0..=400usize {
        let t = i as f64 / 400.0;
        // Oracle-side parameter map and its derivatives.
        let (p, p1, p2) = oracle_cd(&n, t);
        let d = Vec3::new(p[0] - c.x, p[1] - c.y, p[2] - c.z); // (P − center)
        let (px, py) = (d.dot(x), d.dot(y));
        let (vx, vy) = (
            Vec3::new(p1[0], p1[1], p1[2]).dot(x),
            Vec3::new(p1[0], p1[1], p1[2]).dot(y),
        );
        let (axx, ayy) = (
            Vec3::new(p2[0], p2[1], p2[2]).dot(x),
            Vec3::new(p2[0], p2[1], p2[2]).dot(y),
        );
        let theta = py.atan2(px);
        let denom = px * px + py * py;
        let num = px * vy - py * vx;
        let theta1 = num / denom;
        let num1 = px * ayy - py * axx;
        let denom1 = 2.0 * (px * vx + py * vy);
        let theta2 = (num1 * denom - num * denom1) / (denom * denom);

        // eval: NURBS point vs circle at θ(t).
        let np = n.eval(t);
        let cp = circ.eval(theta);
        close3(np - Point3::origin(), cp - Point3::origin(), 1e-9, "eval");
        // Locus residual: |P − center| = r.
        close((np - c).norm(), RADIUS, 1e-9, "radius residual");
        // deriv: chain rule C′(t) = circle′(θ)·θ′(t).
        close3(n.deriv(t), circ.deriv(theta) * theta1, 1e-9, "deriv");
        // deriv2: C″(t) = circle″(θ)·θ′² + circle′(θ)·θ″.
        close3(
            n.deriv2(t),
            circ.deriv2(theta) * (theta1 * theta1) + circ.deriv(theta) * theta2,
            1e-7,
            "deriv2",
        );
    }
}

/// Bézier special case (spec row 1): all-1 weights, single span — the
/// binomial closed form, 3-D and 2-D.
#[test]
fn bezier_special_case_matches_binomial_closed_form() {
    let ctrl3 = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, -1.0),
        Point3::new(3.0, -1.0, 0.5),
        Point3::new(4.0, 1.0, 2.0),
    ];
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let c3 = NurbsCurve3::new(kv.clone(), ctrl3.clone(), vec![1.0; 4]).unwrap();
    let ctrl2: Vec<Point2<f64>> = ctrl3.iter().map(|p| Point2::new(p.x, p.y)).collect();
    let c2 = NurbsCurve2::new(kv, ctrl2.clone(), vec![1.0; 4]).unwrap();
    for i in 0..=50usize {
        let t = i as f64 / 50.0;
        let b = [
            (1.0 - t).powi(3),
            3.0 * t * (1.0 - t).powi(2),
            3.0 * t * t * (1.0 - t),
            t.powi(3),
        ];
        let db = [
            -3.0 * (1.0 - t).powi(2),
            3.0 * (1.0 - t).powi(2) - 6.0 * t * (1.0 - t),
            6.0 * t * (1.0 - t) - 3.0 * t * t,
            3.0 * t * t,
        ];
        let mut e = [0.0f64; 3];
        let mut de = [0.0f64; 3];
        for j in 0..4 {
            e[0] += b[j] * ctrl3[j].x;
            e[1] += b[j] * ctrl3[j].y;
            e[2] += b[j] * ctrl3[j].z;
            de[0] += db[j] * ctrl3[j].x;
            de[1] += db[j] * ctrl3[j].y;
            de[2] += db[j] * ctrl3[j].z;
        }
        let p = c3.eval(t);
        close3(
            p - Point3::origin(),
            Vec3::new(e[0], e[1], e[2]),
            1e-13,
            "bezier eval",
        );
        close3(
            c3.deriv(t),
            Vec3::new(de[0], de[1], de[2]),
            1e-12,
            "bezier deriv",
        );
        let p2 = c2.eval(t);
        close(p2.x, e[0], 1e-13, "bezier2 x");
        close(p2.y, e[1], 1e-13, "bezier2 y");
    }
}

/// The enum arm routes to the payload (and the placeholder poisons).
#[test]
fn curve3_nurbs_arm_evaluates_the_payload() {
    let n = nurbs_circle();
    let curve: Curve3<f64> = Curve3::Nurbs(std::sync::Arc::new(n.clone()));
    for t in [0.0, 0.3, 0.5, 0.99, 1.0] {
        let a = curve.eval(t);
        let b = n.eval(t);
        assert_eq!(
            (a.x.to_bits(), a.y.to_bits(), a.z.to_bits()),
            (b.x.to_bits(), b.y.to_bits(), b.z.to_bits())
        );
    }
    let ph: Curve3<f64> = Curve3::nurbs_placeholder();
    // ALL-poison, which is the placeholder's own promise: a
    // first-channel read passes on a net poisoned only in x.
    let pp = ph.eval(0.5);
    assert!(pp.x.is_nan() && pp.y.is_nan() && pp.z.is_nan());
    for v in [ph.deriv(0.5), ph.deriv2(0.5)] {
        assert!(v.x.is_nan() && v.y.is_nan() && v.z.is_nan());
    }
}

/// Spec row 2: determinism — bit-identical outputs across repeated
/// evaluation and across knot-algebra round trips.
#[test]
fn determinism_bitwise() {
    let n = nurbs_circle();
    for i in 0..=40usize {
        let t = i as f64 / 40.0;
        let a = n.eval(t);
        let b = n.eval(t);
        assert_eq!(a.x.to_bits(), b.x.to_bits());
        assert_eq!(a.y.to_bits(), b.y.to_bits());
        assert_eq!(a.z.to_bits(), b.z.to_bits());
    }
    let r1 = n.insert_knot(0.4, 2).unwrap().remove_knot(0.4, 1).unwrap();
    let r2 = n.insert_knot(0.4, 2).unwrap().remove_knot(0.4, 1).unwrap();
    assert_eq!(r1.0.knots(), r2.0.knots());
    let bits = |v: &[f64]| v.iter().map(|f| f.to_bits()).collect::<Vec<_>>();
    assert_eq!(bits(r1.0.weights()), bits(r2.0.weights()));
    let pbits = |c: &NurbsCurve3<f64>| {
        c.control()
            .iter()
            .flat_map(|p| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
            .collect::<Vec<_>>()
    };
    assert_eq!(pbits(&r1.0), pbits(&r2.0));
    assert_eq!(r1.1.to_bits(), r2.1.to_bits());
}

/// Spec row 3: refusals pinned — typed errors, exact variants.
#[test]
fn construction_refusals_are_typed_and_exact() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let pts = |n: usize| vec![Point3::new(0.0, 0.0, 0.0); n];
    assert_eq!(
        NurbsCurve3::new(kv.clone(), pts(2), vec![1.0, 1.0]).unwrap_err(),
        SplineError::ControlCountMismatch {
            control: 2,
            expected: 3
        }
    );
    assert_eq!(
        NurbsCurve3::new(kv.clone(), pts(3), vec![1.0, 1.0]).unwrap_err(),
        SplineError::WeightCountMismatch {
            weights: 2,
            control: 3
        }
    );
    assert_eq!(
        NurbsCurve3::new(kv.clone(), pts(3), vec![1.0, 0.0, 1.0]).unwrap_err(),
        SplineError::NonPositiveWeight {
            index: 1,
            weight: 0.0
        }
    );
    assert_eq!(
        NurbsCurve3::new(kv.clone(), pts(3), vec![1.0, -2.0, 1.0]).unwrap_err(),
        SplineError::NonPositiveWeight {
            index: 1,
            weight: -2.0
        }
    );
    // NaN weight fails the positivity gate (NaN > 0 is false).
    assert!(matches!(
        NurbsCurve3::new(kv, pts(3), vec![1.0, f64::NAN, 1.0]).unwrap_err(),
        SplineError::NonPositiveWeight { index: 1, .. }
    ));
    // Knot-vector refusals are KnotVector::clamped's (pinned in
    // geom-core's unit tests); spot-check the composition surface.
    assert!(KnotVector::clamped(vec![0.0, 0.5, 1.0, 1.0], 1).is_err());
}

/// Knot-algebra evaluation invariance at the curve level: insertion,
/// refinement, elevation agree with the original on a dense sample
/// within 1e-9 (values O(2.5); re-associated floating point — not
/// bit-equal, see the module docs), and Interval containment of these
/// f64 values is exercised in the interval-lane test file.
#[test]
fn knot_algebra_is_evaluation_invariant() {
    let n = nurbs_circle();
    let variants: Vec<(&str, NurbsCurve3<f64>)> = vec![
        ("insert x1", n.insert_knot(0.6, 1).unwrap()),
        ("insert to mult 2", n.insert_knot(0.6, 2).unwrap()),
        ("refine", n.refine_knots(&[0.1, 0.35, 0.35, 0.9]).unwrap()),
        ("elevate +1", n.elevate_degree(1).unwrap()),
        ("elevate +2", n.elevate_degree(2).unwrap()),
        ("insert+remove roundtrip", {
            let (c, _) = n.insert_knot(0.7, 2).unwrap().remove_knot(0.7, 2).unwrap();
            c
        }),
    ];
    for (what, v) in &variants {
        for i in 0..=300usize {
            let t = i as f64 / 300.0;
            let a = n.eval(t);
            let b = v.eval(t);
            close3(a - Point3::origin(), b - Point3::origin(), 1e-9, what);
        }
    }
    // Structure sanity: elevation raised the degree.
    assert_eq!(variants[3].1.degree(), 3);
}

/// The Tiller-bound honesty test (spec, binding): a planted
/// perturbation makes the knot genuinely non-removable; removal still
/// returns a curve plus a bound, and |C − Ĉ| ≤ bound on a dense
/// sample. The exact-removal case pins the bound near zero.
#[test]
fn removal_bound_is_honest_under_planted_perturbation() {
    let base = nurbs_circle().insert_knot(0.6, 1).unwrap();
    // Exactly removable: the bound is fp-noise-scale and honest.
    let (back, b0) = base.remove_knot(0.6, 1).unwrap();
    assert!(b0 < 1e-12, "exact-removal bound {b0:e}");
    for i in 0..=300usize {
        let t = i as f64 / 300.0;
        let d = (base.eval(t) - back.eval(t)).norm();
        assert!(
            d <= b0 + 1e-13,
            "exact removal: |Δ| = {d:e} > bound {b0:e} at t = {t}"
        );
    }
    // Planted perturbation: nudge a control point inside the affected
    // window, then remove.
    let mut ctrl = base.control().to_vec();
    let mid = ctrl.len() / 2;
    ctrl[mid] = ctrl[mid] + Vec3::new(8e-4, -5e-4, 6e-4);
    let pert = NurbsCurve3::new(base.knots().clone(), ctrl, base.weights().to_vec()).unwrap();
    let (removed, bound) = pert.remove_knot(0.6, 1).unwrap();
    assert!(
        bound > 1e-5,
        "perturbed-removal bound suspiciously small: {bound:e}"
    );
    let mut max_err = 0.0f64;
    for i in 0..=800usize {
        let t = i as f64 / 800.0;
        let d = (pert.eval(t) - removed.eval(t)).norm();
        max_err = max_err.max(d);
        assert!(d <= bound, "|C − Ĉ| = {d:e} > bound {bound:e} at t = {t}");
    }
    // Honesty, not vacuity: the bound is within a few orders of the
    // realized error (documents the partition-of-unity conservatism).
    assert!(
        max_err > 0.0 && bound / max_err < 1e4,
        "bound/max_err = {:e}",
        bound / max_err
    );
}

/// Dual consistency: the value channel is bit-identical to the f64
/// run; the dual derivative of eval matches deriv, and the dual
/// derivative of deriv matches deriv2 (value-close — different
/// association orders).
#[test]
fn dual_channels_are_consistent_with_derivatives() {
    let n64 = nurbs_circle();
    // Constant-lift the geometry to Dual64.
    let ctrl: Vec<Point3<Dual64>> = n64
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
    let nd = NurbsCurve3::new(n64.knots().clone(), ctrl, n64.weights().to_vec()).unwrap();
    for i in 0..=100usize {
        let t = i as f64 / 100.0;
        let p = nd.eval(Dual64::variable(t));
        let p64 = n64.eval(t);
        assert_eq!(
            p.x.value.to_bits(),
            p64.x.to_bits(),
            "value channel bit-identity at {t}"
        );
        let d = n64.deriv(t);
        close(p.x.deriv, d.x, 1e-8, "dual eval′ vs deriv");
        close(p.y.deriv, d.y, 1e-8, "dual eval′ vs deriv");
        close(p.z.deriv, d.z, 1e-8, "dual eval′ vs deriv");
        let dd = nd.deriv(Dual64::variable(t));
        let d2 = n64.deriv2(t);
        close(dd.x.deriv, d2.x, 1e-7, "dual deriv′ vs deriv2");
        close(dd.y.deriv, d2.y, 1e-7, "dual deriv′ vs deriv2");
        close(dd.z.deriv, d2.z, 1e-7, "dual deriv′ vs deriv2");
    }
}

/// The Dual kink convention at a knot: the span tie-break selects the
/// span *starting* at the knot, and the dual derivative is that span's
/// polynomial derivative — the derivative of the program as evaluated
/// (the floor/copysign branch-consistency rule). The fixture is a
/// genuine C⁰ corner (interior knot at full multiplicity p = 2), so
/// left- and right-span derivatives differ and the choice is visible.
#[test]
fn dual_kink_convention_at_knot_follows_the_tie_break() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let ctrl64 = vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(1.0, 2.0, 0.0),
        Point3::new(2.0, 0.0, 0.0), // the corner: tangent y flips sign here
        Point3::new(3.0, 2.0, 1.0),
        Point3::new(4.0, 0.0, 1.0),
    ];
    let n64 = NurbsCurve3::new(kv.clone(), ctrl64.clone(), vec![1.0; 5]).unwrap();
    let ctrl: Vec<Point3<Dual64>> = ctrl64
        .iter()
        .map(|p| {
            Point3::new(
                Dual64::constant(p.x),
                Dual64::constant(p.y),
                Dual64::constant(p.z),
            )
        })
        .collect();
    let nd = NurbsCurve3::new(kv, ctrl, vec![1.0; 5]).unwrap();
    let t = 0.5;
    let span_right = n64.knots().span_at(t); // the span starting at 0.5
    // The [0, 0.5) span (mult-2 knot between).
    let span_left = n64
        .knots()
        .span(span_right.index() - 2)
        .expect("the left span is nonempty");
    let d_right = n64.deriv_in_span(span_right, t);
    let d_left = n64.deriv_in_span(span_left, t);
    // The corner is real: one-sided derivatives disagree.
    assert!(
        (d_right.y - d_left.y).abs() > 1.0,
        "fixture must be a corner"
    );
    let d_dual = nd.eval(Dual64::variable(t));
    close(
        d_dual.x.deriv,
        d_right.x,
        1e-12,
        "kink deriv = right-span deriv",
    );
    close(
        d_dual.y.deriv,
        d_right.y,
        1e-12,
        "kink deriv = right-span deriv",
    );
    close(
        d_dual.z.deriv,
        d_right.z,
        1e-12,
        "kink deriv = right-span deriv",
    );
}
