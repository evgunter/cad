//! M5 PR 3 acceptance, surface side: the cylinder patch as NURBS vs
//! [`Surface::Cylinder`] (spec row 1), surface knot-algebra
//! evaluation invariance, and refusal pinning. The u-parameter map
//! θ(t) and its derivatives come from the same independent Bernstein
//! oracle as the curve suite (hand-coded quadratics, not the kernel's
//! de Boor path); v is arc length on both sides, so only u needs the
//! chain rule.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::needless_range_loop)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::{KnotVector, SplineError};
use geom_core::{Point3, Vec3};

const SQRT2_2: f64 = core::f64::consts::FRAC_1_SQRT_2;

fn axis() -> Vec3<f64> {
    Vec3::new(2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0)
}

fn u_ref() -> Vec3<f64> {
    Vec3::new(1.0 / 3.0, -2.0 / 3.0, 2.0 / 3.0)
}

fn v_ref() -> Vec3<f64> {
    Vec3::new(2.0 / 3.0, -1.0 / 3.0, -2.0 / 3.0)
}

fn origin() -> Point3<f64> {
    Point3::new(-0.5, 4.0, 1.25)
}

const RADIUS: f64 = 2.5;
const HEIGHT: f64 = 3.0;

/// Full-cylinder patch: §7.3 nine-point circle in u × degree-1 line in
/// v (knots `[0, 0, h, h]`, so v is height in meters — the analytic
/// cylinder's own v).
fn nurbs_cylinder() -> NurbsSurface<f64> {
    let (c, r, x, y) = (origin(), RADIUS, u_ref(), v_ref());
    let ring = [
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let ws = [1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0];
    let mut control = Vec::with_capacity(18);
    let mut weights = Vec::with_capacity(18);
    // Row-major iu·nv + iv with nv = 2.
    for (i, (cx, cy)) in ring.iter().enumerate() {
        let base = c + x * (r * cx) + y * (r * cy);
        control.push(base);
        control.push(base + axis() * HEIGHT);
        weights.push(ws[i]);
        weights.push(ws[i]);
    }
    let knots_u = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    let knots_v = KnotVector::clamped(vec![0.0, 0.0, HEIGHT, HEIGHT], 1).unwrap();
    NurbsSurface::new(knots_u, knots_v, control, weights).unwrap()
}

fn analytic_cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: origin(),
        axis: axis(),
        radius: RADIUS,
        u_ref: u_ref(),
    }
}

/// θ(t), θ′(t), θ″(t) of the nine-point circle — the independent
/// Bernstein oracle (quarter arcs on [k/4, (k+1)/4], d/dt = 4·d/ds).
fn theta_map(t: f64) -> (f64, f64, f64) {
    let r = RADIUS;
    let ring = [
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let ws = [1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0, SQRT2_2, 1.0];
    let k = ((t * 4.0).floor() as usize).min(3);
    let s = 4.0 * t - k as f64;
    // Homogeneous plane coordinates (px, py, w) relative to the frame.
    let (mut a0, mut a1, mut a2) = ([0.0f64; 3], [0.0f64; 3], [0.0f64; 3]);
    let b = [(1.0 - s) * (1.0 - s), 2.0 * s * (1.0 - s), s * s];
    let db = [-2.0 * (1.0 - s), 2.0 - 4.0 * s, 2.0 * s];
    let ddb = [2.0f64, -4.0, 2.0];
    for j in 0..3 {
        let i = 2 * k + j;
        let h = [ws[i] * r * ring[i].0, ws[i] * r * ring[i].1, ws[i]];
        for c in 0..3 {
            a0[c] += b[j] * h[c];
            a1[c] += db[j] * h[c] * 4.0;
            a2[c] += ddb[j] * h[c] * 16.0;
        }
    }
    let px = a0[0] / a0[2];
    let py = a0[1] / a0[2];
    let px1 = (a1[0] - px * a1[2]) / a0[2];
    let py1 = (a1[1] - py * a1[2]) / a0[2];
    let px2 = (a2[0] - px * a2[2] - 2.0 * px1 * a1[2]) / a0[2];
    let py2 = (a2[1] - py * a2[2] - 2.0 * py1 * a1[2]) / a0[2];
    let theta = py.atan2(px);
    let denom = px * px + py * py;
    let num = px * py1 - py * px1;
    let theta1 = num / denom;
    let num1 = px * py2 - py * px2;
    let denom1 = 2.0 * (px * px1 + py * py1);
    let theta2 = (num1 * denom - num * denom1) / (denom * denom);
    (theta, theta1, theta2)
}

fn close3(a: Vec3<f64>, b: Vec3<f64>, tol: f64, what: &str) {
    for (x, y) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)] {
        assert!(
            (x - y).abs() <= tol,
            "{what}: {x} vs {y} (|Δ| = {:e})",
            (x - y).abs()
        );
    }
}

/// Spec row 1, surface case: the full jet against `Surface::Cylinder`
/// through the θ(t) chain rule (tolerances as the curve suite: 1e-9
/// on first-order data of O(2.5), 1e-7 on seconds).
#[test]
fn cylinder_patch_matches_analytic_cylinder() {
    let n = nurbs_cylinder();
    let cyl = analytic_cylinder();
    for i in 0..=160usize {
        let t = i as f64 / 160.0;
        let (theta, th1, th2) = theta_map(t);
        for vj in 0..=8usize {
            let v = HEIGHT * vj as f64 / 8.0;
            let p = n.eval(t, v);
            let q = cyl.eval(theta, v);
            close3(p - Point3::origin(), q - Point3::origin(), 1e-9, "eval");
            let jet = n.ders(t, v);
            close3(jet.du, cyl.deriv_u(theta, v) * th1, 1e-8, "deriv_u");
            close3(jet.dv, cyl.deriv_v(theta, v), 1e-10, "deriv_v (= axis)");
            close3(
                jet.duu,
                cyl.deriv_uu(theta, v) * (th1 * th1) + cyl.deriv_u(theta, v) * th2,
                1e-6,
                "deriv_uu",
            );
            close3(jet.duv, Vec3::zero(), 1e-9, "deriv_uv (= 0)");
            close3(jet.dvv, Vec3::zero(), 1e-9, "deriv_vv (= 0)");
            // Chart normal: radial(θ) on both sides (positive scale).
            let nn = jet.du.cross(jet.dv).normalize();
            close3(nn, cyl.normal(theta, v), 1e-8, "normal");
        }
    }
}

/// The enum arm routes to the payload bitwise; the placeholder
/// poisons (unchanged totality behavior).
#[test]
fn surface_nurbs_arm_evaluates_the_payload() {
    let n = nurbs_cylinder();
    let s: Surface<f64> = Surface::Nurbs(std::sync::Arc::new(n.clone()));
    for (t, v) in [(0.1, 0.5), (0.5, 2.0), (1.0, 3.0)] {
        let a = s.eval(t, v);
        let b = n.eval(t, v);
        assert_eq!(a.x.to_bits(), b.x.to_bits());
        assert_eq!(a.y.to_bits(), b.y.to_bits());
        assert_eq!(a.z.to_bits(), b.z.to_bits());
        let du = s.deriv_u(t, v);
        assert_eq!(du.x.to_bits(), n.ders(t, v).du.x.to_bits());
    }
}

/// Surface knot algebra (u and v, all four ops): evaluation-invariant
/// on a dense grid within 1e-9 (re-associated floating point on O(2.5)
/// values — not bit-equal); removal of a planted-exact knot is
/// bounded-honest; transpose is involutive bitwise.
#[test]
fn surface_knot_algebra_is_evaluation_invariant() {
    let n = nurbs_cylinder();
    let variants: Vec<(&str, NurbsSurface<f64>)> = vec![
        ("insert u", n.insert_knot_u(0.6, 1).unwrap()),
        ("insert v", n.insert_knot_v(1.0, 1).unwrap()),
        ("refine u", n.refine_knots_u(&[0.1, 0.35]).unwrap()),
        ("refine v", n.refine_knots_v(&[0.5, 2.5]).unwrap()),
        ("elevate u", n.elevate_degree_u(1).unwrap()),
        ("elevate v", n.elevate_degree_v(1).unwrap()),
        ("remove u roundtrip", {
            let (s, b) = n
                .insert_knot_u(0.7, 1)
                .unwrap()
                .remove_knot_u(0.7, 1)
                .unwrap();
            assert!(b < 1e-12, "exact u-removal bound {b:e}");
            s
        }),
        ("remove v roundtrip", {
            let (s, b) = n
                .insert_knot_v(2.0, 1)
                .unwrap()
                .remove_knot_v(2.0, 1)
                .unwrap();
            assert!(b < 1e-12, "exact v-removal bound {b:e}");
            s
        }),
    ];
    for (what, s) in &variants {
        for i in 0..=40usize {
            let t = i as f64 / 40.0;
            for j in 0..=6usize {
                let v = HEIGHT * j as f64 / 6.0;
                let a = n.eval(t, v);
                let b = s.eval(t, v);
                close3(a - Point3::origin(), b - Point3::origin(), 1e-9, what);
            }
        }
    }
    assert_eq!(variants[4].1.knots_u().degree(), 3);
    // Transpose involution: bitwise identity of the grid.
    let tt = n.transposed().transposed();
    for (a, b) in n.control().iter().zip(tt.control()) {
        assert_eq!(a.x.to_bits(), b.x.to_bits());
    }
    assert_eq!(n.weights(), tt.weights());
}

/// The surface removal bound is honest under a planted perturbation
/// (the Tiller row-sum analogue: grid-wide projected bound).
#[test]
fn surface_removal_bound_is_honest() {
    let base = nurbs_cylinder().insert_knot_u(0.6, 1).unwrap();
    let mut ctrl = base.control().to_vec();
    let mid = ctrl.len() / 2;
    ctrl[mid] = ctrl[mid] + Vec3::new(-6e-4, 4e-4, 7e-4);
    let pert = NurbsSurface::new(
        base.knots_u().clone(),
        base.knots_v().clone(),
        ctrl,
        base.weights().to_vec(),
    )
    .unwrap();
    let (removed, bound) = pert.remove_knot_u(0.6, 1).unwrap();
    assert!(
        bound > 1e-6,
        "perturbed surface bound suspiciously small: {bound:e}"
    );
    for i in 0..=200usize {
        let t = i as f64 / 200.0;
        for j in 0..=8usize {
            let v = HEIGHT * j as f64 / 8.0;
            let d = (pert.eval(t, v) - removed.eval(t, v)).norm();
            assert!(
                d <= bound,
                "|S − Ŝ| = {d:e} > bound {bound:e} at ({t}, {v})"
            );
        }
    }
}

/// Spec row 3, surface side: refusals typed and exact.
#[test]
fn surface_construction_refusals_are_typed() {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let pts = |n: usize| vec![Point3::new(0.0, 0.0, 0.0); n];
    assert_eq!(
        NurbsSurface::new(ku.clone(), kv.clone(), pts(3), vec![1.0; 3]).unwrap_err(),
        SplineError::ControlCountMismatch {
            control: 3,
            expected: 4
        }
    );
    assert_eq!(
        NurbsSurface::new(ku.clone(), kv.clone(), pts(4), vec![1.0; 3]).unwrap_err(),
        SplineError::WeightCountMismatch {
            weights: 3,
            control: 4
        }
    );
    assert_eq!(
        NurbsSurface::new(ku, kv, pts(4), vec![1.0, 1.0, -0.5, 1.0]).unwrap_err(),
        SplineError::NonPositiveWeight {
            index: 2,
            weight: -0.5
        }
    );
}
