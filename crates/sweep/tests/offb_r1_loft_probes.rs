//! VERBS-OFF-B r1 reviewer probe — the E2E consumer row the offset
//! fit exists for: a genuinely non-analytic SKINNED loft from the
//! existing `sweep::skin` machinery, offset through
//! `geom_brep::offset_fit::fit_offset`, its certificate checked
//! against a dense independent sample of the exact offset locus.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::curves::NurbsCurve3;
use geom_brep::offset_fit::fit_offset;
use geom_brep::offset_fit::offset_point;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol};
use sweep::skin::skin;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A cubic section: a planar wave at height `z`, amplitude `a`,
/// phase `phi` — no analytic kind, honest loft input.
fn section(z: f64, a: f64, phi: f64) -> NurbsCurve3<f64> {
    let n = 8;
    let control: Vec<Point3<f64>> = (0..n)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f64 / (n - 1) as f64;
            Point3::new(
                2.0 * t,
                a * (3.0 * t + phi).sin(),
                z + 0.15 * (2.0 * t + phi).cos(),
            )
        })
        .collect();
    let knots = KnotVector::clamped(
        vec![0.0, 0.0, 0.0, 0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.0, 1.0, 1.0],
        3,
    )
    .unwrap();
    NurbsCurve3::new(knots, control, vec![1.0; n]).unwrap()
}

/// Skin four wavy sections, offset both ways, and require the
/// certificate to contain a dense independent sample of the exact
/// offset locus — the red direction (a bound that under-reports).
#[test]
fn r1_skinned_loft_offset_certificate_contains_dense_sample() {
    let sections = vec![
        section(0.0, 0.30, 0.0),
        section(0.5, 0.42, 0.35),
        section(1.0, 0.28, 0.7),
        section(1.5, 0.38, 1.1),
    ];
    let base = skin(&sections, 3).unwrap();
    let tol = 5e-4;
    for d in [0.06_f64, -0.05] {
        let (fit, cert) = fit_offset(&base, d, tol, band())
            .unwrap_or_else(|e| panic!("fit_offset refused the skinned loft at d = {d}: {e}"));
        assert!(cert.hull_sup <= tol);
        let (nu, nv) = (43usize, 41usize);
        let (ulo, uhi) = base.knots_u().domain();
        let (vlo, vhi) = base.knots_v().domain();
        let mut worst = 0.0f64;
        for i in 0..nu {
            for j in 0..nv {
                #[allow(clippy::cast_precision_loss)]
                let u = ulo + (uhi - ulo) * (i as f64 / (nu - 1) as f64);
                #[allow(clippy::cast_precision_loss)]
                let v = vlo + (vhi - vlo) * (j as f64 / (nv - 1) as f64);
                let target = offset_point(&base, d, u, v).unwrap();
                worst = worst.max((fit.eval(u, v) - target).norm());
            }
        }
        assert!(
            worst <= cert.hull_sup,
            "skinned loft d = {d}: certified sup {} UNDER-reports the dense sampled max {worst}",
            cert.hull_sup
        );
        eprintln!(
            "loft d={d}: cells={} rounds={} on_locus={:.3e} hull_sup={:.3e} sampled={worst:.3e} \
             floor={:.4} reach={}",
            cert.cells,
            cert.rounds,
            cert.on_locus_max,
            cert.hull_sup,
            cert.normal_floor,
            cert.curvature_reach,
        );
    }
}
