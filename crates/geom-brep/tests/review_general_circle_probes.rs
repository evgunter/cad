//! Reviewer probes for PR 3733 (the sphere general-circle fitted route):
//! the image producer's fidelity BETWEEN its nodes (dense 3-D residual
//! against the certified sampled one), the pole fence's reach, and the
//! Circle arm's closed-form envelope against a circle off its sphere.
//! Printing probes: each row asserts only what the review's claims need
//! and prints the measurements.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::sync::Arc;

use geom::{Curve3, Surface};
use geom_brep::{FittedLane, Pcurve, PcurveCache};
use geom_core::{Band, Point3, Tol, Vec3};

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The section of the unit sphere by the plane `n·x = h`, `n` tilted
/// by `tilt` from +z in the xz-plane; `u_ref` points toward the
/// north pole's projection, so `t = 0` is the circle's nearest
/// approach to it. `grow` scales the radius off the sphere.
fn section(tilt: f64, h: f64, grow: f64) -> Curve3<f64> {
    let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    // pole − centre, projected into the plane: (0,0,1) − n·(n_z) ∝ (−sin·cos, 0, sin²)
    let e = Vec3::new(-tilt.cos(), 0.0, tilt.sin());
    Curve3::Circle {
        center: Point3::origin() + n * h,
        axis: n,
        radius: (1.0 - h * h).sqrt() * grow,
        u_ref: e,
    }
}

struct Measured {
    sampled: f64,
    dense: f64,
    certified: Result<f64, String>,
}

fn measure(carrier: &Curve3<f64>, t0: f64, t1: f64) -> Result<Measured, String> {
    let band = Band::linear(Tol::witness()).unwrap();
    let s = sphere();
    let image = FittedLane::<f64>::certified()
        .sphere_circle_image(carrier, t0, t1, &s, band)
        .map_err(|e| format!("image refused: {e:?}"))?;
    let pc = Pcurve::Fitted(Arc::new(image.clone()));
    let res = |t: f64| {
        let p = pc.eval(t);
        (s.eval(p.x, p.y) - carrier.eval(t)).norm()
    };
    let sampled = (0..9)
        .map(|k| res(t0 + (t1 - t0) * k as f64 / 8.0))
        .fold(0.0, f64::max);
    let dense = (0..=20000)
        .map(|k| res(t0 + (t1 - t0) * k as f64 / 20000.0))
        .fold(0.0, f64::max);
    let window = pc.chart_box(t0, t1);
    let certified = PcurveCache::certify_fitted(
        Arc::new(image),
        t0,
        t1,
        carrier,
        &s,
        None,
        window,
        band,
        FittedLane::certified(),
    )
    .map(|c| c.certificate().envelope)
    .map_err(|e| format!("{e:?}"));
    Ok(Measured {
        sampled,
        dense,
        certified,
    })
}

fn report(name: &str, carrier: &Curve3<f64>, t0: f64, t1: f64) -> Option<Measured> {
    match measure(carrier, t0, t1) {
        Ok(m) => {
            println!(
                "{name:<44} sampled {:9.2e}  dense {:9.2e}  certify {:?}",
                m.sampled, m.dense, m.certified
            );
            Some(m)
        }
        Err(e) => {
            println!("{name:<44} {e}");
            None
        }
    }
}

/// (b) The image's fidelity between its nodes across arc shapes. The
/// certificate's statement does not cover the map residual between
/// samples (`OnLocusHull`), so the dense residual is the only measure
/// of what a reader of the image gets.
#[test]
fn the_image_is_faithful_between_its_nodes() {
    let eps = Tol::witness().get().eps;
    println!("eps = {eps:e}");
    let pi = core::f64::consts::PI;
    let base = section(0.6, 0.3, 1.0);
    let mut worst_certified_dense = 0.0_f64;
    let mut rows: Vec<(String, Curve3<f64>, f64, f64)> = vec![
        (
            "moderate quarter turn".into(),
            base.clone(),
            0.3,
            0.3 + pi / 2.0,
        ),
        (
            "long arc 1.9π (cap off the pole)".into(),
            base.clone(),
            0.1,
            0.1 + 1.9 * pi,
        ),
        ("short arc 1e-3".into(), base.clone(), 0.3, 0.3 + 1e-3),
        ("short arc 1e-7".into(), base.clone(), 0.3, 0.3 + 1e-7),
        (
            "arc across the azimuth seam".into(),
            base.clone(),
            pi - 0.7,
            pi + 0.7,
        ),
        (
            "cap around the pole, 1.9π".into(),
            section(0.3, 0.8, 1.0),
            0.05,
            0.05 + 1.9 * pi,
        ),
    ];
    for delta in [1e-1, 1e-2, 1e-3, 1e-4, 1e-6] {
        // Plane distance of the pole from the plane = cos(tilt) − h.
        let tilt: f64 = 0.6;
        rows.push((
            format!("near pole, plane gap {delta:e}, arc ±0.5"),
            section(tilt, tilt.cos() - delta, 1.0),
            -0.5,
            0.5,
        ));
        rows.push((
            format!("near pole, plane gap {delta:e}, arc 0.2..0.9"),
            section(tilt, tilt.cos() - delta, 1.0),
            0.2,
            0.9,
        ));
    }
    for (name, c, t0, t1) in &rows {
        if let Some(m) = report(name, c, *t0, *t1)
            && m.certified.is_ok()
        {
            worst_certified_dense = worst_certified_dense.max(m.dense);
        }
    }
    println!("worst dense residual over CERTIFIED rows: {worst_certified_dense:e} (eps {eps:e})");
}

/// (d) The pole fence reads the circle's PLANE, not the arc: an arc of
/// a circle through the pole that stays well away from it.
#[test]
fn the_pole_fence_refuses_an_arc_that_avoids_the_pole() {
    let tilt: f64 = 0.6;
    let c = section(tilt, tilt.cos(), 1.0); // plane contains the pole; t = 0 IS the pole
    let pi = core::f64::consts::PI;
    let r = report(
        "pole-through circle, arc π-0.5..π+0.5",
        &c,
        pi - 0.5,
        pi + 0.5,
    );
    let r2 = report("pole-through circle, arc 1.0..2.0", &c, 1.0, 2.0);
    assert!(r.is_none() && r2.is_none(), "the fence is plane-wide");
}

/// (a) The closed-form envelope against a circle grown off its sphere:
/// the envelope must bound the dense true distance.
#[test]
fn the_envelope_bounds_a_circle_off_its_sphere() {
    let eps = Tol::witness().get().eps;
    for grow in [
        1.0 + eps * 0.1,
        1.0 - eps * 0.1,
        1.0 + eps * 0.5,
        1.0 - eps * 0.5,
    ] {
        let c = section(0.6, 0.3, grow);
        let true_sup = (0..=4000)
            .map(|k| {
                let t = core::f64::consts::TAU * k as f64 / 4000.0;
                ((c.eval(t) - Point3::origin()).norm() - 1.0).abs()
            })
            .fold(0.0, f64::max);
        let m = measure(&c, 0.3, 1.8).expect("image");
        println!(
            "grow {grow}: true sup {true_sup:e}  envelope {:?}",
            m.certified
        );
        if let Ok(env) = m.certified {
            assert!(
                env >= true_sup,
                "the envelope {env:e} under-bounds {true_sup:e}"
            );
        }
    }
}
