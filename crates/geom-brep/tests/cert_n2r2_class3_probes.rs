//! CERT-N2 R2 reviewer probes: class 3 (chart stretch sup/inf) on the
//! S99 masquerade at f64, Dual and Interval. Probe file — not for merge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use std::sync::Arc;

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Dual, Point3, Real};

fn knots3() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

/// A bowed 3x3 quadratic patch, then corrupted by `f`.
fn patch<T: Real>(f: impl Fn(usize, Point3<f64>) -> Point3<T>) -> NurbsSurface<T> {
    let mut control = Vec::new();
    for iu in 0..3 {
        for iv in 0..3 {
            let (u, v) = (iu as f64, iv as f64);
            control.push(f(iu * 3 + iv, Point3::new(u, v, 0.3 * u * v)));
        }
    }
    NurbsSurface::new(knots3(), knots3(), control, vec![1.0; 9]).unwrap()
}

#[test]
fn n2r2_class3_f64() {
    let clean = Surface::Nurbs(Arc::new(patch(|_, p| p)));
    let x = Surface::Nurbs(Arc::new(patch(|_, p| Point3::new(f64::NAN, p.y, p.z))));
    let one = Surface::Nurbs(Arc::new(patch(|i, p| {
        if i == 4 {
            Point3::new(f64::NAN, p.y, p.z)
        } else {
            p
        }
    })));
    for (name, s) in [
        ("clean", &clean),
        ("x-poison", &x),
        ("one point x-poison", &one),
    ] {
        eprintln!(
            "[class 3 f64 {name}] sup={:?} inf={:?}",
            geom_brep::chart_stretch_sup(s),
            geom_brep::chart_stretch_inf(s)
        );
    }
    let ph = Surface::<f64>::nurbs_placeholder();
    eprintln!(
        "[class 3 f64 placeholder] sup={:?} inf={:?}",
        geom_brep::chart_stretch_sup(&ph),
        geom_brep::chart_stretch_inf(&ph)
    );
}

#[test]
fn n2r2_class3_dual_poisoned_derivative_only() {
    let d: Surface<Dual<f64>> = Surface::Nurbs(Arc::new(patch(|_, p| {
        Point3::new(
            Dual::new(p.x, f64::NAN),
            Dual::constant(p.y),
            Dual::constant(p.z),
        )
    })));
    eprintln!(
        "[class 3 Dual value-finite/deriv-NaN] sup={:?}",
        geom_brep::chart_stretch_sup(&d)
    );
}

#[cfg(feature = "interval")]
#[test]
fn n2r2_class3_interval() {
    use geom_core::{Bounds, Interval};
    let show = |x: Interval| format!("[{}, {}] poison={}", x.lo(), x.hi(), x.is_poison());
    let nai = || Interval::from_f64(f64::NAN);
    let empty = || Interval::from_bounds(-2.0, -1.0).sqrt();
    let entire = || Interval::from_bounds(f64::NEG_INFINITY, f64::INFINITY);
    let cases: Vec<(&str, Surface<Interval>)> = vec![
        (
            "clean",
            Surface::Nurbs(Arc::new(patch(|_, p| p.map(Interval::from_f64)))),
        ),
        (
            "x NaI",
            Surface::Nurbs(Arc::new(patch(|_, p| {
                Point3::new(nai(), Interval::from_f64(p.y), Interval::from_f64(p.z))
            }))),
        ),
        (
            "x empty",
            Surface::Nurbs(Arc::new(patch(|_, p| {
                Point3::new(empty(), Interval::from_f64(p.y), Interval::from_f64(p.z))
            }))),
        ),
        (
            "x entire",
            Surface::Nurbs(Arc::new(patch(|_, p| {
                Point3::new(entire(), Interval::from_f64(p.y), Interval::from_f64(p.z))
            }))),
        ),
        ("placeholder", Surface::<Interval>::nurbs_placeholder()),
    ];
    for (name, s) in &cases {
        let (su, sv) = geom_brep::chart_stretch_sup(s);
        let inf = geom_brep::chart_stretch_inf(s);
        eprintln!(
            "[class 3 Interval {name}] sup_u={} sup_v={} | inf_u={} inf_v={} area_inf={}",
            show(su),
            show(sv),
            show(inf.inf_u),
            show(inf.inf_v),
            show(inf.area_inf)
        );
        // Sanity: which state does the predicate report?
        if let Surface::Nurbs(p) = s {
            eprintln!("    is_placeholder={}", p.is_placeholder());
        }
    }
}
