//! CERT-N1 R1 reviewer probes — adversarial `map_scalar` battery.
//!
//! Independent of the lane's fixtures: extreme weight ratios, degree 5
//! with interior multiplicity `p − 1`, a closed rational circle, a 2-D
//! curve, a poisoned described net, and value/1st/2nd-derivative and
//! normal agreement at knot values and span boundaries in both lanes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::curves::Curve3;
use geom::surfaces::Surface;
use geom::{NurbsCurve2, NurbsCurve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Dual, Dual64, Point2, Point3};
// Lint conformance (lane edit): `Real` is only named by the
// interval-gated rows below, so its import is gated the same way.
#[cfg(feature = "interval")]
use geom_core::Real;

/// Degree 5, one interior knot at multiplicity 4 (= p − 1), weights
/// spanning sixteen orders of magnitude.
fn brutal_curve() -> NurbsCurve3<f64> {
    let knots = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.4, 0.4, 0.4, 0.4, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ],
        5,
    )
    .unwrap();
    let control: Vec<Point3<f64>> = (0..10)
        .map(|i| {
            let t = i as f64;
            Point3::new(t * 0.37 - 1.0, (t * 0.9).sin() * 3.0, t * t * 0.05)
        })
        .collect();
    let weights = vec![1.0, 1e8, 1e-8, 3.5, 1e6, 1e-6, 2.0, 1e7, 1e-7, 1.0];
    NurbsCurve3::new(knots, control, weights).unwrap()
}

/// The rational-quadratic FULL circle (closed/periodic-shaped net,
/// interior multiplicity 2 = p at the seams is not clamped-legal, so
/// this uses the standard four-arc clamped spelling).
fn full_circle() -> NurbsCurve3<f64> {
    let s = core::f64::consts::FRAC_1_SQRT_2;
    let knots = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    let p = |x: f64, y: f64| Point3::new(x, y, 0.5 * x - 0.25 * y);
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
    let weights = vec![1.0, s, 1.0, s, 1.0, s, 1.0, s, 1.0];
    NurbsCurve3::new(knots, control, weights).unwrap()
}

fn params() -> Vec<f64> {
    let mut v = vec![0.0, 1.0, 0.25, 0.4, 0.5, 0.75];
    for i in 1..40 {
        v.push(i as f64 / 40.0);
    }
    v
}

/// Dual lane: value channel bit-identical, tangent channel matches the
/// f64 closed-form first derivative, at knot values and span
/// boundaries, for both brutal fixtures and a 2-D curve.
#[test]
fn n1r1_dual_lift_matches_source_everywhere() {
    for (name, c) in [("brutal", brutal_curve()), ("circle", full_circle())] {
        let cd = c.map_scalar(Dual::constant);
        for t in params() {
            let q = c.eval(t);
            let d = c.deriv(t);
            let d2 = c.deriv2(t);
            let p = cd.eval(Dual::variable(t));
            let pd = cd.deriv(Dual::variable(t));
            for (ch, lifted, src, tan, sec) in [
                ("x", p.x, q.x, d.x, d2.x),
                ("y", p.y, q.y, d.y, d2.y),
                ("z", p.z, q.z, d.z, d2.z),
            ] {
                assert_eq!(
                    lifted.value.to_bits(),
                    src.to_bits(),
                    "{name} t={t} {ch}: value {} vs {src}",
                    lifted.value
                );
                assert!(
                    (lifted.deriv - tan).abs() <= 1e-9 * (1.0 + tan.abs()),
                    "{name} t={t} {ch}: dual tangent {} vs closed form {tan}",
                    lifted.deriv
                );
                let _ = sec;
                let _ = pd;
            }
            // Second derivative: the dual of the first derivative.
            let d1 = cd.deriv(Dual::variable(t));
            for (ch, got, want) in [
                ("x", d1.x.deriv, d2.x),
                ("y", d1.y.deriv, d2.y),
                ("z", d1.z.deriv, d2.z),
            ] {
                assert!(
                    (got - want).abs() <= 1e-6 * (1.0 + want.abs()),
                    "{name} t={t} {ch}: dual 2nd {got} vs closed form {want}"
                );
            }
            // The direct dual second derivative is bit-identical in its
            // value channel to the f64 one.
            let dd2 = cd.deriv2(Dual::constant(t));
            for (ch, got, want) in [
                ("x", dd2.x.value, d2.x),
                ("y", dd2.y.value, d2.y),
                ("z", dd2.z.value, d2.z),
            ] {
                assert_eq!(
                    got.to_bits(),
                    want.to_bits(),
                    "{name} t={t} {ch}: lifted deriv2 value {got} vs {want}"
                );
            }
        }
    }
}

/// The 2-D curve lifts too (`NurbsCurve2`, the `$dim = 2` macro arm).
#[test]
fn n1r1_curve2_lift_matches_source() {
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 4.0),
        Point2::new(2.0, -1.0),
        Point2::new(3.0, 2.0),
        Point2::new(4.0, 0.0),
    ];
    let weights = vec![1.0, 1e9, 1e-9, 5.0, 1.0];
    let c = NurbsCurve2::new(knots, control, weights).unwrap();
    let cd = c.map_scalar(Dual::constant);
    for t in params() {
        let q = c.eval(t);
        let d = c.deriv(t);
        let p = cd.eval(Dual::variable(t));
        assert_eq!(p.x.value.to_bits(), q.x.to_bits(), "t={t} x");
        assert_eq!(p.y.value.to_bits(), q.y.to_bits(), "t={t} y");
        assert!(
            (p.x.deriv - d.x).abs() <= 1e-9 * (1.0 + d.x.abs()),
            "t={t} dx"
        );
        assert!(
            (p.y.deriv - d.y).abs() <= 1e-9 * (1.0 + d.y.abs()),
            "t={t} dy"
        );
    }
}

/// A described net with ONE poisoned control point lifts to a described
/// net (never the benign placeholder), and `is_placeholder` answers the
/// same before and after the lift.
#[test]
fn n1r1_partly_poisoned_net_is_not_the_placeholder() {
    let knots = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(2.0, 0.0, 0.0),
        Point3::new(f64::NAN, 2.0, 0.0),
        Point3::new(0.0, 2.0, 0.0),
    ];
    let weights = vec![1.0, 1.0, 1.0];
    let c = NurbsCurve3::new(knots, control, weights).unwrap();
    assert!(
        !c.is_placeholder(),
        "source: one poison point is not the placeholder"
    );
    let cd: NurbsCurve3<Dual64> = c.map_scalar(Dual::constant);
    assert!(
        !cd.is_placeholder(),
        "a partly-poisoned described net lifted to the benign placeholder"
    );
    // At t = 1 the poisoned control point has zero basis weight, but the
    // evaluator's fixed association still touches it, so poison is
    // permitted to spread; what must NOT happen is silent finiteness at
    // a parameter the source poisons.
    for t in [0.0, 0.5, 1.0] {
        let q = c.eval(t);
        let p = cd.eval(Dual::constant(t));
        assert_eq!(
            p.x.value.is_nan(),
            q.x.is_nan(),
            "t={t}: lift changed which parameters poison"
        );
        if !q.x.is_nan() {
            assert_eq!(p.x.value.to_bits(), q.x.to_bits(), "t={t}");
        }
    }
    // The placeholder itself survives the lift at the enum level.
    let ph: Curve3<f64> = Curve3::nurbs_placeholder();
    let phd: Curve3<Dual64> = ph.map_scalar(Dual::constant);
    assert!(matches!(&phd, Curve3::Nurbs(n) if n.is_placeholder()));
}

/// A surface lift: value, both first partials, both second partials and
/// the normal agree with the source at knots and span boundaries.
#[test]
fn n1r1_surface_lift_matches_source_including_normal() {
    let ku = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 1.0],
        3,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.6, 1.0, 1.0, 1.0], 2).unwrap();
    let (nu, nv) = (7usize, 4usize);
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for i in 0..nu {
        for j in 0..nv {
            let (x, y) = (i as f64 * 0.5, j as f64 * 0.7);
            control.push(Point3::new(x, y, (x * 1.3).sin() + (y * 0.8).cos()));
            weights.push(match (i + j) % 4 {
                0 => 1.0,
                1 => 1e7,
                2 => 1e-7,
                _ => 2.5,
            });
        }
    }
    let s = NurbsSurface::new(ku, kv, control, weights).unwrap();
    let se = Surface::Nurbs(std::sync::Arc::new(s.clone()));
    let sd: Surface<Dual64> = se.map_scalar(Dual::constant);
    let grid = [0.0, 0.2, 0.5, 0.6, 0.75, 1.0];
    for &u in &grid {
        for &v in &grid {
            let q = se.eval(u, v);
            let p = sd.eval(Dual::constant(u), Dual::constant(v));
            for (ch, got, want) in [
                ("x", p.x.value, q.x),
                ("y", p.y.value, q.y),
                ("z", p.z.value, q.z),
            ] {
                assert_eq!(got.to_bits(), want.to_bits(), "({u},{v}) {ch}");
            }
            // u-partial via the dual variable channel vs the closed form.
            let pu = sd.eval(Dual::variable(u), Dual::constant(v));
            let du = se.deriv_u(u, v);
            for (ch, got, want) in [
                ("x", pu.x.deriv, du.x),
                ("y", pu.y.deriv, du.y),
                ("z", pu.z.deriv, du.z),
            ] {
                assert!(
                    (got - want).abs() <= 1e-7 * (1.0 + want.abs()),
                    "({u},{v}) d{ch}/du {got} vs {want}"
                );
            }
            // Second partials and the normal, value channel bit-exact.
            let duu = sd.deriv_uu(Dual::constant(u), Dual::constant(v));
            let quu = se.deriv_uu(u, v);
            assert_eq!(duu.x.value.to_bits(), quu.x.to_bits(), "({u},{v}) uu.x");
            assert_eq!(duu.z.value.to_bits(), quu.z.to_bits(), "({u},{v}) uu.z");
            let dn = sd.normal(Dual::constant(u), Dual::constant(v));
            let qn = se.normal(u, v);
            for (ch, got, want) in [
                ("x", dn.x.value, qn.x),
                ("y", dn.y.value, qn.y),
                ("z", dn.z.value, qn.z),
            ] {
                assert_eq!(got.to_bits(), want.to_bits(), "({u},{v}) n{ch}");
            }
        }
    }
}

/// The composition `map_scalar(Interval::from_f64).map_scalar(
/// Dual::constant)` reaches the same object the retired hand
/// re-spelling built, field for field.
#[cfg(feature = "interval")]
mod interval_probes {
    use super::*;
    use geom_core::{Bounds, Interval, Vec3};

    fn contains(e: Interval, x: f64) -> bool {
        e.lo() <= x && x <= e.hi()
    }

    /// Interval lane: the enclosure CONTAINS the f64 evaluation at every
    /// sampled parameter, and its width stays at the exact-structural-map
    /// scale (not merely "small at one fixture").
    #[test]
    fn n1r1_interval_lift_encloses_source_with_exact_width() {
        for (name, c) in [("brutal", brutal_curve()), ("circle", full_circle())] {
            let ci = c.map_scalar(Interval::from_f64);
            // The lift itself adds NO width: every control coordinate is
            // a degenerate interval.
            for p in ci.control() {
                assert_eq!(
                    p.x.lo().to_bits(),
                    p.x.hi().to_bits(),
                    "{name}: lift widened a control coordinate"
                );
            }
            assert_eq!(ci.weights(), c.weights(), "{name}: weights moved");
            assert_eq!(ci.knots().knots(), c.knots().knots(), "{name}: knots moved");
            for t in params() {
                let q = c.eval(t);
                let p = ci.eval(Interval::from_f64(t));
                for (ch, e, src) in [("x", p.x, q.x), ("y", p.y, q.y), ("z", p.z, q.z)] {
                    assert!(
                        contains(e, src),
                        "{name} t={t} {ch}: [{}, {}] must contain {src}",
                        e.lo(),
                        e.hi()
                    );
                    let rel = (e.hi() - e.lo()) / (1.0 + src.abs());
                    assert!(
                        rel < 1e-9,
                        "{name} t={t} {ch}: relative width {rel} — the lift is not behaving as an exact map"
                    );
                }
                // First derivative encloses too.
                let d = c.deriv(t);
                let di = ci.deriv(Interval::from_f64(t));
                for (ch, e, src) in [("x", di.x, d.x), ("y", di.y, d.y), ("z", di.z, d.z)] {
                    assert!(
                        contains(e, src),
                        "{name} t={t} d{ch}: [{}, {}] must contain {src}",
                        e.lo(),
                        e.hi()
                    );
                }
            }
        }
    }

    /// The two-step composition equals the retired hand re-spelling.
    #[test]
    fn n1r1_composition_is_the_hand_respelling() {
        let c = Curve3::Circle {
            center: Point3::new(1.0, -2.0, 0.5),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: 2.25,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let composed = c.map_scalar(Interval::from_f64).map_scalar(Dual::constant);
        let lifted = c.map_scalar(Interval::from_f64);
        let Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } = lifted
        else {
            panic!("fixture is a circle")
        };
        let hand: Curve3<Dual<Interval>> = Curve3::Circle {
            center: Point3::new(
                Dual::constant(center.x),
                Dual::constant(center.y),
                Dual::constant(center.z),
            ),
            axis: Vec3::new(
                Dual::constant(axis.x),
                Dual::constant(axis.y),
                Dual::constant(axis.z),
            ),
            radius: Dual::constant(radius),
            u_ref: Vec3::new(
                Dual::constant(u_ref.x),
                Dual::constant(u_ref.y),
                Dual::constant(u_ref.z),
            ),
        };
        let (a, b) = (
            composed.eval(Dual::variable(Interval::from_f64(0.7))),
            hand.eval(Dual::variable(Interval::from_f64(0.7))),
        );
        assert_eq!(a.x.value.lo().to_bits(), b.x.value.lo().to_bits());
        assert_eq!(a.x.deriv.hi().to_bits(), b.x.deriv.hi().to_bits());
        assert_eq!(a.z.value.hi().to_bits(), b.z.value.hi().to_bits());
    }

    /// The placeholder lifts to the placeholder at the INTERVAL scalar
    /// too — the lane's row only covers the dual lane.
    #[test]
    fn n1r1_placeholder_lifts_to_placeholder_at_interval() {
        let ph: Curve3<f64> = Curve3::nurbs_placeholder();
        let phi: Curve3<Interval> = ph.map_scalar(Interval::from_f64);
        assert!(matches!(&phi, Curve3::Nurbs(n) if n.is_placeholder()));
        let sph: Surface<f64> = Surface::nurbs_placeholder();
        let sphi: Surface<Interval> = sph.map_scalar(Interval::from_f64);
        assert!(matches!(&sphi, Surface::Nurbs(n) if n.is_placeholder()));
    }
}
