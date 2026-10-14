//! Review probe for PR 5050 (not for merge).
use core::f64::consts::TAU;
use super::harmonic_sup_rows::{band, circle};
use super::*;

fn frame(skew: bool) -> (Vec3<f64>, Vec3<f64>) {
    if skew {
        let k = Vec3::new(0.3, -0.5, 0.81).normalize();
        let d = k.cross(Vec3::new(0.2, 0.9, -0.1)).normalize();
        (k, d)
    } else {
        (Vec3::unit_z(), Vec3::unit_x())
    }
}

fn row(skew: bool, l: f64, minor: f64, push: f64) {
    let major = 2.0;
    let (k, d) = frame(skew);
    let dir = Vec3::new(0.6, 0.48, 0.64).normalize();
    let centre = Point3::origin() + dir * l;
    let torus = Surface::Torus { center: centre, axis: k, major_radius: major, minor_radius: minor, u_ref: d };
    let e = k.cross(d);
    let tilt = (minor / major).asin();
    let n = k * tilt.cos() - e * tilt.sin();
    // `push` moves the stored centre tangentially (along e) by an absolute amount
    let c = centre + d * minor + e * push;
    let carrier = circle(c, n, major, d);
    let img = chart_pcurve(&carrier, &torus, band());
    let Ok(img @ Pcurve::FocalSection(_)) = img else { println!("skew {skew} L {l:e} r {minor:e} push {push:e}: {img:?}"); return };
    let (mut miss, mut off) = (0.0f64, 0.0f64);
    for i in 0..4096 {
        let t = TAU * f64::from(i) / 4096.0;
        let q = img.eval(t);
        let p = carrier.eval(t);
        miss = miss.max(torus.eval(q.x, q.y).distance(p));
        off = off.max(chart_distance(&torus, p));
    }
    let cert = PcurveCache::certify(img.clone(), 0.0, TAU, &carrier, &torus, band())
        .map(|c| format!("ok {:e}", c.certificate().envelope))
        .unwrap_or_else(|e| format!("{e:?}"));
    println!("skew {skew} L {l:e} r {minor:e} push {push:e}: miss {miss:.3e} off {off:.2e} cert {cert}");
}

#[test]
fn review_5050_villarceau_table() {
    for skew in [false, true] {
        for l in [0.0, 1e3, 1e6] {
            for r in [0.7, 1e-2, 1e-4] {
                row(skew, l, r, 0.0);
            }
        }
    }
    // a deliberate tangential centre offset δ: miss ~ δ, or ~ R·δ/r?
    for r in [0.7, 1e-2, 1e-4] {
        for push in [1e-12, 1e-11] {
            row(true, 0.0, r, push);
        }
    }
}
