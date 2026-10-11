//! A NURBS carrier's weights and knots are held in its scalar (C6;
//! `work/intent/nurbs-weights-and-knots-are-the-scalar.md`, unit 2).
//!
//! Each row holds a weight or a knot as a `Dual` VARIABLE and reads its
//! tangent through the carrier: through evaluation (against a central
//! difference of the `f64` carrier with that value moved), and through
//! every knot operation, which is evaluation-invariant in ℝ and so must
//! leave the tangent of the point where it was. On a carrier that held
//! its weights and knots as `f64`, none of these could be built: a
//! weight or a knot had no tangent to carry.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::num::NonZeroUsize;

use geom::{NurbsCurve3, NurbsSurface};
use geom_core::spline::{KnotVector, Knots};
use geom_core::{Dual64, Point3};

const H: f64 = 1e-6;

fn m(n: usize) -> NonZeroUsize {
    NonZeroUsize::new(n).unwrap()
}

fn net() -> Vec<Point3<f64>> {
    vec![
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.4, 1.1, 0.2),
        Point3::new(1.3, 1.6, -0.3),
        Point3::new(2.2, 0.7, 0.4),
        Point3::new(3.0, 0.1, 0.0),
    ]
}

const WEIGHTS: [f64; 5] = [1.0, 0.7, 1.8, 0.9, 1.0];

/// The cubic over `[0, 2]` with one interior breakpoint `k`, at `f64`.
fn curve_f64(k: f64, w: [f64; 5]) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, k, 2.0, 2.0, 2.0, 2.0], 3).unwrap();
    NurbsCurve3::new(kv, net(), w.to_vec()).unwrap()
}

/// The same cubic at `Dual64`, its breakpoint `k` and its weight `w₂`
/// held as given (a variable where the row asks for one).
fn curve_dual(k: Dual64, w2: Dual64) -> NurbsCurve3<Dual64> {
    let c = Dual64::constant;
    let knots = Knots::new(vec![(c(0.0), m(4)), (k, m(1)), (c(2.0), m(4))], 3).unwrap();
    let mut w: Vec<Dual64> = WEIGHTS.iter().map(|x| c(*x)).collect();
    w[2] = w2;
    NurbsCurve3::new(knots, net().iter().map(|p| p.map(c)).collect(), w).unwrap()
}

fn close(a: f64, b: f64, tol: f64, what: &str) {
    assert!((a - b).abs() <= tol * (1.0 + b.abs()), "{what}: {a} vs {b}");
}

fn params() -> impl Iterator<Item = f64> {
    (0..=40).map(|i| 2.0 * f64::from(i) / 40.0)
}

/// A held weight moves the point by its derivative, and the value
/// channel is the `f64` carrier's, bit for bit.
#[test]
fn a_held_weight_moves_the_point_by_its_derivative() {
    let held = curve_dual(Dual64::constant(0.8), Dual64::variable(WEIGHTS[2]));
    let at = |dw: f64| {
        let mut w = WEIGHTS;
        w[2] += dw;
        curve_f64(0.8, w)
    };
    let (base, up, down) = (at(0.0), at(H), at(-H));
    for t in params() {
        let p = held.eval(Dual64::constant(t));
        let q = base.eval(t);
        let (u, d) = (up.eval(t), down.eval(t));
        for (name, got, value, fd) in [
            ("x", p.x, q.x, (u.x - d.x) / (2.0 * H)),
            ("y", p.y, q.y, (u.y - d.y) / (2.0 * H)),
            ("z", p.z, q.z, (u.z - d.z) / (2.0 * H)),
        ] {
            assert_eq!(got.value.to_bits(), value.to_bits(), "t={t} {name}: value");
            close(got.deriv, fd, 1e-6, &format!("t={t} {name}: ∂/∂w₂"));
        }
    }
}

/// A held breakpoint moves the point by its derivative.
#[test]
fn a_held_knot_moves_the_point_by_its_derivative() {
    let held = curve_dual(Dual64::variable(0.8), Dual64::constant(WEIGHTS[2]));
    let (up, down) = (curve_f64(0.8 + H, WEIGHTS), curve_f64(0.8 - H, WEIGHTS));
    // Away from the breakpoint itself, where the moving knot changes
    // which span a parameter lies in.
    for t in params().filter(|t| (t - 0.8).abs() > 0.05) {
        let p = held.eval(Dual64::constant(t));
        let (u, d) = (up.eval(t), down.eval(t));
        close(
            p.x.deriv,
            (u.x - d.x) / (2.0 * H),
            1e-6,
            &format!("t={t} x: ∂/∂k"),
        );
        close(
            p.y.deriv,
            (u.y - d.y) / (2.0 * H),
            1e-6,
            &format!("t={t} y: ∂/∂k"),
        );
        close(
            p.z.deriv,
            (u.z - d.z) / (2.0 * H),
            1e-6,
            &format!("t={t} z: ∂/∂k"),
        );
    }
}

/// Both channels of `after` at every parameter are `before`'s, to the
/// rounding of the operation.
fn same_curve(before: &NurbsCurve3<Dual64>, after: &NurbsCurve3<Dual64>, what: &str) {
    for t in params() {
        let (p, q) = (
            before.eval(Dual64::constant(t)),
            after.eval(Dual64::constant(t)),
        );
        for (name, a, b) in [("x", p.x, q.x), ("y", p.y, q.y), ("z", p.z, q.z)] {
            close(
                b.value,
                a.value,
                1e-12,
                &format!("{what} t={t} {name}: value"),
            );
            close(
                b.deriv,
                a.deriv,
                1e-9,
                &format!("{what} t={t} {name}: tangent"),
            );
        }
    }
}

/// Every knot operation is evaluation-invariant in ℝ, so a held weight's
/// and a held breakpoint's tangent on the point is where it was after
/// insertion, refinement, elevation and removal; and the breakpoint
/// stays one quantity (its run carries the variable), while a knot the
/// operation brought in is a constant.
#[test]
fn the_knot_algebra_carries_held_values_and_their_tangents() {
    for (what, held) in [
        (
            "weight",
            curve_dual(Dual64::constant(0.8), Dual64::variable(WEIGHTS[2])),
        ),
        (
            "knot",
            curve_dual(Dual64::variable(0.8), Dual64::constant(WEIGHTS[2])),
        ),
    ] {
        let refined = held.refine_knots(&[0.3, 0.8, 1.4, 1.4]).unwrap();
        same_curve(&held, &refined, &format!("{what}: refine"));
        let runs: Vec<(f64, f64, usize)> = refined
            .knot_values()
            .runs()
            .map(|(v, m)| (v.value, v.deriv, m))
            .collect();
        let knot_tangent = if what == "knot" { 1.0 } else { 0.0 };
        assert_eq!(
            runs,
            vec![
                (0.0, 0.0, 4),
                (0.3, 0.0, 1),
                (0.8, knot_tangent, 2),
                (1.4, 0.0, 2),
                (2.0, 0.0, 4),
            ],
            "{what}: the refined breakpoints"
        );
        let elevated = held.elevate_degree(1).unwrap();
        same_curve(&held, &elevated, &format!("{what}: elevate"));
        let inserted = held.insert_knot(1.4, 2).unwrap();
        let (removed, bound) = inserted.remove_knot(1.4, 2).unwrap();
        same_curve(&held, &removed, &format!("{what}: insert then remove"));
        assert!(
            bound.value.abs() < 1e-9,
            "{what}: a round trip's bound {bound:?}"
        );
        let (left, right) = held.split_at(1.1).unwrap();
        for t in params() {
            let piece = if t <= 1.1 { &left } else { &right };
            let (p, q) = (
                held.eval(Dual64::constant(t)),
                piece.eval(Dual64::constant(t)),
            );
            close(q.y.deriv, p.y.deriv, 1e-9, &format!("{what}: split t={t}"));
        }
    }
}

/// `on_domain` re-expresses the held knots in `T`: a held breakpoint's
/// image carries its tangent through the affine map.
#[test]
fn a_held_knot_is_re_expressed_in_the_scalar() {
    let held = curve_dual(Dual64::variable(0.8), Dual64::constant(WEIGHTS[2]));
    let moved = held.on_domain(-1.0, 3.0).unwrap();
    let interior: Vec<(f64, f64)> = moved
        .knot_values()
        .runs()
        .map(|(v, _)| (v.value, v.deriv))
        .collect();
    assert_eq!(interior.len(), 3);
    for ((value, tangent), (want, want_tangent)) in
        interior.iter().zip([(-1.0, 0.0), (0.6, 2.0), (3.0, 0.0)])
    {
        close(*value, want, 1e-15, "a re-expressed breakpoint");
        assert_eq!(*tangent, want_tangent, "its tangent through the affine map");
    }
}

/// A surface holds its weights the same way: a variable weight carries
/// its tangent through evaluation and through a refinement in each
/// direction.
#[test]
fn a_surface_carries_a_held_weight_through_its_knot_algebra() {
    let c = Dual64::constant;
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<Point3<Dual64>> = [
        (0.0, 0.0, 0.0),
        (0.0, 1.0, 0.2),
        (1.0, 0.0, 0.5),
        (1.0, 1.0, 0.9),
        (2.0, 0.0, 0.1),
        (2.0, 1.0, 0.3),
    ]
    .iter()
    .map(|&(x, y, z)| Point3::new(c(x), c(y), c(z)))
    .collect();
    let weights = vec![
        c(1.0),
        c(1.0),
        Dual64::variable(0.6),
        c(0.6),
        c(1.0),
        c(1.0),
    ];
    let held =
        NurbsSurface::new(Knots::constant(ku), Knots::constant(kv), control, weights).unwrap();
    let refined = held
        .refine_knots_u(&[0.25, 0.5])
        .unwrap()
        .refine_knots_v(&[0.5])
        .unwrap();
    let mut moved = false;
    for i in 0..=8 {
        for j in 0..=4 {
            let (u, v) = (f64::from(i) / 8.0, f64::from(j) / 4.0);
            let (p, q) = (held.eval(c(u), c(v)), refined.eval(c(u), c(v)));
            for (a, b) in [(p.x, q.x), (p.y, q.y), (p.z, q.z)] {
                close(b.value, a.value, 1e-12, &format!("({u}, {v}) value"));
                close(b.deriv, a.deriv, 1e-9, &format!("({u}, {v}) tangent"));
                moved |= a.deriv != 0.0;
            }
        }
    }
    assert!(moved, "the held weight reached no point");
}
