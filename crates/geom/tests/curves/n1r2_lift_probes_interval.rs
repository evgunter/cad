//! CERT-N1 R2 probes, interval lane: the interval lift of adversarial
//! described NURBS encloses the f64 evaluation (value, first and second
//! derivative, normal) with widths consistent with an exact structural
//! map; the placeholder and poison posture at Interval and
//! Dual<Interval>; the composition `map_scalar(from_f64).map_scalar
//! (Dual::constant)` is the hand re-spelling, bit for bit.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod n1r2_fixtures;

use std::sync::Arc;

use geom::{Curve3, NurbsCurve3, Surface};
use geom_core::{Bounds, Dual, Interval, Point3, Real};
use n1r2_fixtures::{curves2, curves3, params, surfaces};

fn contains(e: Interval, x: f64) -> bool {
    e.lo() <= x && x <= e.hi()
}

fn width_ok(e: Interval, x: f64, rel: f64) -> bool {
    (e.hi() - e.lo()) <= rel * (1.0 + x.abs())
}

#[test]
fn n1r2_curve_interval_lift_encloses_source_on_adversarial_nets() {
    let mut worst = 0.0f64;
    for (name, c) in curves3() {
        let e = Curve3::Nurbs(Arc::new(c.clone()));
        let ei: Curve3<Interval> = e.map_scalar(Interval::from_f64);
        assert!(
            matches!(&ei, Curve3::Nurbs(n) if !n.is_placeholder()),
            "{name}"
        );
        for t in params(c.knots()) {
            let ti = Interval::from_f64(t);
            for (label, li, s) in [
                ("p", ei.eval(ti), e.eval(t).into_vec()),
                ("d1", ei.deriv(ti).into_vec_i(), e.deriv(t)),
                ("d2", ei.deriv2(ti).into_vec_i(), e.deriv2(t)),
            ] {
                for (l, r) in [(li.x, s.x), (li.y, s.y), (li.z, s.z)] {
                    assert!(
                        contains(l, r),
                        "{name} t={t} {label}: [{}, {}] must contain {r}",
                        l.lo(),
                        l.hi()
                    );
                    let w = (l.hi() - l.lo()) / (1.0 + r.abs());
                    worst = worst.max(w);
                    assert!(
                        width_ok(l, r, 1e-6),
                        "{name} t={t} {label}: width {w:e} too wide"
                    );
                }
            }
        }
    }
    println!("n1r2: worst relative interval width {worst:e}");
}

trait IntoVec {
    fn into_vec(self) -> geom_core::Vec3<f64>;
}
impl IntoVec for Point3<f64> {
    fn into_vec(self) -> geom_core::Vec3<f64> {
        geom_core::Vec3::new(self.x, self.y, self.z)
    }
}
trait IntoVecI {
    fn into_vec_i(self) -> Point3<Interval>;
}
impl IntoVecI for geom_core::Vec3<Interval> {
    fn into_vec_i(self) -> Point3<Interval> {
        Point3::new(self.x, self.y, self.z)
    }
}

#[test]
fn n1r2_curve2_interval_lift_encloses_source() {
    for (name, c) in curves2() {
        let ci = c.map_scalar(Interval::from_f64);
        for t in params(c.knots()) {
            let p = ci.eval(Interval::from_f64(t));
            let q = c.eval(t);
            assert!(contains(p.x, q.x) && contains(p.y, q.y), "{name} t={t}");
            assert!(
                width_ok(p.x, q.x, 1e-9) && width_ok(p.y, q.y, 1e-9),
                "{name} t={t}"
            );
        }
    }
}

#[test]
fn n1r2_surface_interval_lift_encloses_source_on_adversarial_nets() {
    for (name, s) in surfaces() {
        let e = Surface::Nurbs(Arc::new(s.clone()));
        let ei: Surface<Interval> = e.map_scalar(Interval::from_f64);
        assert!(
            matches!(&ei, Surface::Nurbs(n) if !n.is_placeholder()),
            "{name}"
        );
        for u in params(s.knots_u()) {
            for v in params(s.knots_v()) {
                let (ui, vi) = (Interval::from_f64(u), Interval::from_f64(v));
                let p = ei.eval(ui, vi);
                let q = e.eval(u, v);
                for (l, r) in [(p.x, q.x), (p.y, q.y), (p.z, q.z)] {
                    assert!(contains(l, r), "{name} ({u},{v}) eval containment");
                    assert!(
                        width_ok(l, r, 1e-6),
                        "{name} ({u},{v}) eval width {:e}",
                        l.hi() - l.lo()
                    );
                }
                let du = ei.deriv_u(ui, vi);
                let dv = ei.deriv_v(ui, vi);
                let fu = e.deriv_u(u, v);
                let fv = e.deriv_v(u, v);
                for (l, r) in [
                    (du.x, fu.x),
                    (du.y, fu.y),
                    (du.z, fu.z),
                    (dv.x, fv.x),
                    (dv.y, fv.y),
                    (dv.z, fv.z),
                ] {
                    assert!(
                        contains(l, r),
                        "{name} ({u},{v}) deriv containment [{}, {}] vs {r}",
                        l.lo(),
                        l.hi()
                    );
                    assert!(
                        width_ok(l, r, 1e-6),
                        "{name} ({u},{v}) deriv width {:e}",
                        l.hi() - l.lo()
                    );
                }
                let n = e.normal(u, v);
                let ni = ei.normal(ui, vi);
                for (l, r) in [(ni.x, n.x), (ni.y, n.y), (ni.z, n.z)] {
                    // The normal normalises (a sqrt), so only containment.
                    assert!(
                        contains(l, r) || (r.is_nan() && l.is_poison()),
                        "{name} ({u},{v}) normal"
                    );
                }
            }
        }
    }
}

#[test]
fn n1r2_placeholder_and_poison_at_interval_and_dual_interval() {
    let c: Curve3<f64> = Curve3::nurbs_placeholder();
    let ci = c.map_scalar(Interval::from_f64);
    assert!(matches!(&ci, Curve3::Nurbs(n) if n.is_placeholder()));
    let cdi = ci.map_scalar(Dual::constant);
    assert!(matches!(&cdi, Curve3::Nurbs(n) if n.is_placeholder()));
    assert!(
        cdi.eval(Dual::variable(Interval::from_f64(0.5)))
            .x
            .value
            .is_poison()
    );
    let s: Surface<f64> = Surface::nurbs_placeholder();
    let si = s.map_scalar(Interval::from_f64);
    assert!(matches!(&si, Surface::Nurbs(n) if n.is_placeholder()));

    let knots = geom_core::spline::KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0],
        2,
    )
    .unwrap();
    let mut control: Vec<Point3<f64>> = (0..5)
        .map(|i| Point3::new(i as f64, 1.0, -(i as f64)))
        .collect();
    control[4] = Point3::new(f64::NAN, f64::NAN, f64::NAN);
    let c = NurbsCurve3::new(knots, control, vec![1.0, 2.0, 0.5, 1.0, 1.0]).unwrap();
    let ci = c.map_scalar(Interval::from_f64);
    assert!(!ci.is_placeholder(), "lift widened into the placeholder");
    let p = ci.eval(Interval::from_f64(0.1));
    let q = c.eval(0.1);
    assert!(contains(p.x, q.x) && width_ok(p.x, q.x, 1e-12));
    assert!(ci.eval(Interval::from_f64(0.9)).x.is_poison());
}

#[test]
fn n1r2_composed_lift_is_the_hand_re_spelling_bit_for_bit() {
    for (name, c) in curves3() {
        let composed: NurbsCurve3<Dual<Interval>> =
            c.map_scalar(Interval::from_f64).map_scalar(Dual::constant);
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
        let hand: NurbsCurve3<Dual<Interval>> =
            NurbsCurve3::new(c.knots().clone(), ctrl, c.weights().to_vec()).unwrap();
        for t in params(c.knots()) {
            let tt = Dual::variable(Interval::from_f64(t));
            let a = composed.eval(tt);
            let b = hand.eval(tt);
            for (l, r) in [(a.x, b.x), (a.y, b.y), (a.z, b.z)] {
                assert_eq!(
                    l.value.lo().to_bits(),
                    r.value.lo().to_bits(),
                    "{name} t={t}"
                );
                assert_eq!(
                    l.value.hi().to_bits(),
                    r.value.hi().to_bits(),
                    "{name} t={t}"
                );
                assert_eq!(
                    l.deriv.lo().to_bits(),
                    r.deriv.lo().to_bits(),
                    "{name} t={t}"
                );
                assert_eq!(
                    l.deriv.hi().to_bits(),
                    r.deriv.hi().to_bits(),
                    "{name} t={t}"
                );
            }
        }
        // And the enum composition equals the direct payload composition.
        let e = Curve3::Nurbs(Arc::new(c.clone()));
        let ec: Curve3<Dual<Interval>> =
            e.map_scalar(Interval::from_f64).map_scalar(Dual::constant);
        let t = Dual::variable(Interval::from_f64(0.31));
        let a = ec.eval(t);
        let b = composed.eval(t);
        assert_eq!(a.x.value.lo().to_bits(), b.x.value.lo().to_bits(), "{name}");
    }
}
