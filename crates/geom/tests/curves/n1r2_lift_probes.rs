//! CERT-N1 R2 probes, default lane: the Dual lift of adversarial
//! described NURBS evaluates to its source (value bits, first and
//! second derivative channels, normals), the placeholder and poison
//! posture, and the C24 bit identity against the retired spelling
//! reconstructed from the diff.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#![allow(clippy::needless_range_loop)]

use crate::curves::n1r2_fixtures;

use std::sync::Arc;

use geom::surfaces::{
    ApproxSurface, ApproxWindow, OffsetCertificate, SurfaceDescription, SurfaceSpec,
};
use geom::{Curve3, NurbsCurve3, Surface};
use geom_core::spline::Span;
use geom_core::spline::basis::ders_basis_funs;
use geom_core::{Dual, Dual64, Point3, Real, Vec3};
use n1r2_fixtures::{curves2, curves3, params, surfaces};

fn close(a: f64, b: f64, rel: f64) -> bool {
    (a - b).abs() <= rel * (1.0 + a.abs().max(b.abs()))
}

#[test]
fn n1r2_curve_dual_lift_evaluates_to_source_on_adversarial_nets() {
    let mut worst_d1 = 0.0f64;
    let mut worst_d2 = 0.0f64;
    for (name, c) in curves3() {
        let e = Curve3::Nurbs(Arc::new(c.clone()));
        let ed: Curve3<Dual64> = e.map_scalar(Dual::constant);
        assert!(
            matches!(&ed, Curve3::Nurbs(n) if !n.is_placeholder()),
            "{name}"
        );
        for t in params(c.knots()) {
            let p = ed.eval(Dual::variable(t));
            let q = e.eval(t);
            let d1 = e.deriv(t);
            let d2 = e.deriv2(t);
            for (l, s) in [(p.x, q.x), (p.y, q.y), (p.z, q.z)] {
                assert_eq!(l.value.to_bits(), s.to_bits(), "{name} t={t}: value bits");
            }
            for (l, s) in [(p.x.deriv, d1.x), (p.y.deriv, d1.y), (p.z.deriv, d1.z)] {
                let err = (l - s).abs() / (1.0 + s.abs());
                worst_d1 = worst_d1.max(err);
                assert!(
                    close(l, s, 1e-8),
                    "{name} t={t}: dual tangent {l} vs closed-form {s}"
                );
            }
            // Second derivative: the dual of the closed-form first
            // derivative; its value channel is the f64 tangent bit for bit.
            let dd = ed.deriv(Dual::variable(t));
            for (l, s) in [(dd.x, d1.x), (dd.y, d1.y), (dd.z, d1.z)] {
                assert_eq!(
                    l.value.to_bits(),
                    s.to_bits(),
                    "{name} t={t}: deriv value bits"
                );
            }
            for (l, s) in [(dd.x.deriv, d2.x), (dd.y.deriv, d2.y), (dd.z.deriv, d2.z)] {
                let err = (l - s).abs() / (1.0 + s.abs());
                worst_d2 = worst_d2.max(err);
                assert!(
                    close(l, s, 1e-7),
                    "{name} t={t}: dual d2 {l} vs closed-form {s}"
                );
            }
        }
        // Finite differences away from knots, as an independent oracle.
        let (lo, hi) = c.knots().domain();
        for i in 1..40 {
            let t = lo + (hi - lo) * (i as f64 + 0.37) / 41.0;
            if c.knots().knots().iter().any(|k| (k - t).abs() < 1e-3) {
                continue;
            }
            let h = 1e-6;
            let a = e.eval(t - h);
            let b = e.eval(t + h);
            let fd = Vec3::new(
                (b.x - a.x) / (2.0 * h),
                (b.y - a.y) / (2.0 * h),
                (b.z - a.z) / (2.0 * h),
            );
            let p = ed.eval(Dual::variable(t));
            for (l, s) in [(p.x.deriv, fd.x), (p.y.deriv, fd.y), (p.z.deriv, fd.z)] {
                assert!(
                    close(l, s, 1e-4),
                    "{name} t={t}: dual tangent {l} vs FD {s}"
                );
            }
        }
    }
    println!("n1r2: worst relative dual d1 error {worst_d1:e}, d2 error {worst_d2:e}");
}

#[test]
fn n1r2_curve2_dual_lift_evaluates_to_source() {
    for (name, c) in curves2() {
        let cd = c.map_scalar(Dual::constant);
        for t in params(c.knots()) {
            let p = cd.eval(Dual::variable(t));
            let q = c.eval(t);
            let d = c.deriv(t);
            assert_eq!(p.x.value.to_bits(), q.x.to_bits(), "{name} t={t}");
            assert_eq!(p.y.value.to_bits(), q.y.to_bits(), "{name} t={t}");
            assert!(
                close(p.x.deriv, d.x, 1e-8),
                "{name} t={t}: {} vs {}",
                p.x.deriv,
                d.x
            );
            assert!(
                close(p.y.deriv, d.y, 1e-8),
                "{name} t={t}: {} vs {}",
                p.y.deriv,
                d.y
            );
        }
        assert_eq!(cd.weights(), c.weights());
        assert_eq!(cd.knots().knots(), c.knots().knots());
    }
}

#[test]
fn n1r2_surface_dual_lift_evaluates_to_source_on_adversarial_nets() {
    for (name, s) in surfaces() {
        let e = Surface::Nurbs(Arc::new(s.clone()));
        let ed: Surface<Dual64> = e.map_scalar(Dual::constant);
        assert!(
            matches!(&ed, Surface::Nurbs(n) if !n.is_placeholder()),
            "{name}"
        );
        let us = params(s.knots_u());
        let vs = params(s.knots_v());
        for &u in &us {
            for &v in &vs {
                let q = e.eval(u, v);
                let pu = ed.eval(Dual::variable(u), Dual::constant(v));
                let pv = ed.eval(Dual::constant(u), Dual::variable(v));
                let du = e.deriv_u(u, v);
                let dv = e.deriv_v(u, v);
                for (l, r) in [
                    (pu.x, q.x),
                    (pu.y, q.y),
                    (pu.z, q.z),
                    (pv.x, q.x),
                    (pv.y, q.y),
                    (pv.z, q.z),
                ] {
                    assert_eq!(
                        l.value.to_bits(),
                        r.to_bits(),
                        "{name} ({u},{v}) value bits"
                    );
                }
                for (l, r) in [(pu.x.deriv, du.x), (pu.y.deriv, du.y), (pu.z.deriv, du.z)] {
                    assert!(close(l, r, 1e-8), "{name} ({u},{v}) du {l} vs {r}");
                }
                for (l, r) in [(pv.x.deriv, dv.x), (pv.y.deriv, dv.y), (pv.z.deriv, dv.z)] {
                    assert!(close(l, r, 1e-8), "{name} ({u},{v}) dv {l} vs {r}");
                }
                // Normal: value channel bit-identical.
                let n = e.normal(u, v);
                let nd = ed.normal(Dual::constant(u), Dual::constant(v));
                for (l, r) in [(nd.x, n.x), (nd.y, n.y), (nd.z, n.z)] {
                    assert_eq!(
                        l.value.to_bits(),
                        r.to_bits(),
                        "{name} ({u},{v}) normal bits"
                    );
                }
                // Second derivatives via the dual of the first.
                let duu = e.deriv_uu(u, v);
                let duv = e.deriv_uv(u, v);
                let ddu = ed.deriv_u(Dual::variable(u), Dual::constant(v));
                let ddv = ed.deriv_u(Dual::constant(u), Dual::variable(v));
                for (l, r) in [(ddu.x, du.x), (ddu.y, du.y), (ddu.z, du.z)] {
                    assert_eq!(
                        l.value.to_bits(),
                        r.to_bits(),
                        "{name} ({u},{v}) du value bits"
                    );
                }
                for (l, r) in [
                    (ddu.x.deriv, duu.x),
                    (ddu.y.deriv, duu.y),
                    (ddu.z.deriv, duu.z),
                ] {
                    assert!(close(l, r, 1e-7), "{name} ({u},{v}) duu {l} vs {r}");
                }
                for (l, r) in [
                    (ddv.x.deriv, duv.x),
                    (ddv.y.deriv, duv.y),
                    (ddv.z.deriv, duv.z),
                ] {
                    assert!(close(l, r, 1e-7), "{name} ({u},{v}) duv {l} vs {r}");
                }
            }
        }
    }
}

#[test]
fn n1r2_approx_lift_carries_record_and_evaluates_to_source() {
    let fit = Arc::new(surfaces()[0].1.clone());
    let certificate = OffsetCertificate {
        distance: 0.5,
        cells: 9,
        samples: 5,
        on_locus_max: 3e-9,
        hull_sup: 4e-9,
        normal_floor: 0.25,
        curvature_reach: 2.5,
        rounds: 7,
    };
    let spec = SurfaceSpec {
        description: SurfaceDescription::Offset {
            base: Arc::clone(&fit),
            d: 0.5,
        },
        fit: (*fit).clone(),
        window: ApproxWindow::of(&*fit),
        tolerance: 1e-7,
    };
    let approx = ApproxSurface::certify(spec, |_, _, _, _| Ok::<_, ()>(certificate)).unwrap();
    let s = Surface::Approx(Arc::new(approx));
    let sd: Surface<Dual64> = s.map_scalar(Dual::constant);
    let Surface::Approx(l) = &sd else {
        panic!("variant")
    };
    assert_eq!(format!("{:?}", l.certificate()), format!("{certificate:?}"));
    assert_eq!(l.tolerance(), 1e-7);
    let SurfaceDescription::Offset { base, d } = l.description();
    assert_eq!(d.value.to_bits(), 0.5f64.to_bits());
    assert_eq!(base.weights(), fit.weights());
    for u in params(fit.knots_u()) {
        for v in params(fit.knots_v()) {
            let p = sd.eval(Dual::constant(u), Dual::constant(v));
            let q = s.eval(u, v);
            assert_eq!(p.x.value.to_bits(), q.x.to_bits());
            assert_eq!(p.y.value.to_bits(), q.y.to_bits());
            assert_eq!(p.z.value.to_bits(), q.z.to_bits());
        }
    }
}

#[test]
fn n1r2_placeholder_lifts_to_placeholder_curve_and_surface_dual() {
    let c: Curve3<f64> = Curve3::nurbs_placeholder();
    let cd = c.map_scalar(Dual::constant);
    assert!(matches!(&cd, Curve3::Nurbs(n) if n.is_placeholder()));
    assert!(cd.eval(Dual::variable(0.5)).x.value.is_nan());
    let s: Surface<f64> = Surface::nurbs_placeholder();
    let sd = s.map_scalar(Dual::constant);
    assert!(matches!(&sd, Surface::Nurbs(n) if n.is_placeholder()));
}

#[test]
fn n1r2_one_poisoned_point_lifts_to_a_described_poisoned_net() {
    // Degree 2, knots [0,0,0,1/3,2/3,1,1,1]: 5 control points; span 0
    // (t < 1/3) reads indices 0..=2 only, span 2 reads 2..=4.
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
    assert!(!c.is_placeholder());
    let e = Curve3::Nurbs(Arc::new(c.clone()));
    let ed = e.map_scalar(Dual::constant);
    assert!(
        matches!(&ed, Curve3::Nurbs(n) if !n.is_placeholder()),
        "lift widened into the placeholder"
    );
    // Where the poisoned point is outside the window: finite and identical.
    let p = ed.eval(Dual::variable(0.1));
    let q = e.eval(0.1);
    assert!(q.x.is_finite());
    assert_eq!(p.x.value.to_bits(), q.x.to_bits());
    // Where it is inside: poison, on both sides.
    assert!(e.eval(0.9).x.is_nan());
    assert!(ed.eval(Dual::variable(0.9)).x.value.is_nan());
    // A net poisoned in ONE channel at every point, the mirror of the
    // masquerade `net_placeholder_width.rs` pins.
    let mut control2: Vec<Point3<f64>> = (0..5).map(|i| Point3::new(i as f64, 1.0, 0.0)).collect();
    for p in &mut control2 {
        p.y = f64::NAN;
    }
    let knots = geom_core::spline::KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0, 1.0, 1.0],
        2,
    )
    .unwrap();
    let c2 = NurbsCurve3::new(knots, control2, vec![1.0; 5]).unwrap();
    // Every point has a poisoned SECOND channel and a finite first one:
    // corrupt described geometry, never the benign placeholder, before
    // and after the lift identically.
    assert!(!c2.is_placeholder());
    assert!(!c2.map_scalar(Dual::constant).is_placeholder());
}

/// The retired `ders_in_span` spelling, reconstructed verbatim from the
/// removed lines of commit 7bf42740e (order-2 basis, inline corrections).
// Lint conformance (lane edit): this is the RETIRED spelling kept
// verbatim for the bit check, `x[k] = x[k] + …` included.
#[allow(clippy::assign_op_pattern)]
fn retired_ders_in_span(
    c: &NurbsCurve3<f64>,
    span: Span,
    t: f64,
) -> (Point3<f64>, Vec3<f64>, Vec3<f64>) {
    let ders = ders_basis_funs(c.knots(), span, t, 2);
    let base = span.first_control();
    let mut x = [0.0f64; 3];
    let mut y = [0.0f64; 3];
    let mut z = [0.0f64; 3];
    let mut w_hom = [0.0f64; 3];
    for (k, row) in ders.iter().enumerate() {
        for (j, nkj) in row.iter().enumerate() {
            let i = base + j;
            let cw = *nkj * f64::from_f64(c.weights()[i]);
            let pt = c.control()[i];
            x[k] = x[k] + cw * pt.x;
            y[k] = y[k] + cw * pt.y;
            z[k] = z[k] + cw * pt.z;
            w_hom[k] = w_hom[k] + cw;
        }
    }
    let two = 2.0f64;
    let corr = |hom: [f64; 3]| {
        let c0 = hom[0] / w_hom[0];
        let c1 = (hom[1] - c0 * w_hom[1]) / w_hom[0];
        let c2 = (hom[2] - c0 * w_hom[2] - c1 * w_hom[1] * two) / w_hom[0];
        (c0, c1, c2)
    };
    let (x, y, z) = (corr(x), corr(y), corr(z));
    (
        Point3::new(x.0, y.0, z.0),
        Vec3::new(x.1, y.1, z.1),
        Vec3::new(x.2, y.2, z.2),
    )
}

#[test]
fn n1r2_c24_bit_identity_against_the_retired_spelling() {
    let mut n = 0usize;
    for (name, c) in curves3() {
        let kv = c.knots();
        for t in params(kv) {
            for idx in 0..kv.knots().len() {
                let Some(span) = kv.span(idx) else { continue };
                let (op, od1, od2) = retired_ders_in_span(&c, span, t);
                let (p, d1, d2) = c.ders_in_span(span, t);
                let e1 = c.deriv_in_span(span, t);
                let e2 = c.deriv2_in_span(span, t);
                for (a, b) in [
                    (op.x, p.x),
                    (op.y, p.y),
                    (op.z, p.z),
                    (od1.x, d1.x),
                    (od1.y, d1.y),
                    (od1.z, d1.z),
                    (od2.x, d2.x),
                    (od2.y, d2.y),
                    (od2.z, d2.z),
                    (od1.x, e1.x),
                    (od1.y, e1.y),
                    (od1.z, e1.z),
                    (od2.x, e2.x),
                    (od2.y, e2.y),
                    (od2.z, e2.z),
                ] {
                    assert_eq!(
                        a.to_bits(),
                        b.to_bits(),
                        "{name} t={t} span={idx}: {a} vs {b}"
                    );
                    n += 1;
                }
            }
        }
    }
    println!("n1r2: {n} components bit-identical to the retired spelling");
}
