//! Review probes for PR 4190 (lane r1): `max_principal_curvature` and
//! the at-rest arm. Output lines start `R1UNIT`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::dihedral::max_principal_curvature;
use geom::{NurbsSurface, SurfaceJet};
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Interval, Point3, Real, Vec3};

fn kv2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

/// `fixture::sphere_band`, with the longitude weights optionally
/// Möbius-rescaled by `c` (row `i` times `c^i`).
fn sphere_band(r: f64, lat0: f64, lat1: f64, c: f64) -> NurbsSurface<f64> {
    let theta = 0.5 * (lat1 - lat0);
    let wm = theta.cos();
    let a = (r * lat0.cos(), r * lat0.sin());
    let b = (r * lat1.cos(), r * lat1.sin());
    let mid = (a.0 + b.0, a.1 + b.1);
    let mlen = (mid.0 * mid.0 + mid.1 * mid.1).sqrt();
    let m = (mid.0 / mlen * r / wm, mid.1 / mlen * r / wm);
    let meridian = [(a.0, a.1, 1.0), (m.0, m.1, wm), (b.0, b.1, 1.0)];
    let wr = core::f64::consts::FRAC_1_SQRT_2;
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for iu in 0..3 {
        for (x, z, w) in meridian {
            control.push(match iu {
                0 => Point3::new(x, 0.0, z),
                1 => Point3::new(x, x, z),
                _ => Point3::new(0.0, x, z),
            });
            let base = if iu == 1 { w * wr } else { w };
            weights.push(base * c.powi(iu as i32));
        }
    }
    NurbsSurface::new(kv2(), kv2(), control, weights).unwrap()
}

#[test]
fn r1_sphere_band_reads_one_over_r_under_two_charts() {
    let r = 0.3;
    for (lat1, c) in [(1.0, 1.0), (1.0, 3.0), (1.0, 0.01), (1.5, 1.0), (1.5707, 1.0)] {
        let s = sphere_band(r, 0.1, lat1, c);
        let mut worst: f64 = 0.0;
        for i in 0..=20 {
            for j in 0..=20 {
                let (u, v) = (f64::from(i) / 20.0, f64::from(j) / 20.0);
                let k = max_principal_curvature(&s.ders(u, v));
                worst = worst.max((k * r - 1.0).abs());
            }
        }
        println!("R1UNIT sphere lat1={lat1} mobius c={c}: max |kR-1| = {worst:e}");
    }
    // The pole: lat1 = π/2 puts a control row on the axis; S_u = 0 at v = 1.
    let s = sphere_band(r, 0.1, core::f64::consts::FRAC_PI_2, 1.0);
    for v in [1.0, 1.0 - 1e-4, 1.0 - 1e-6, 1.0 - 1e-8, 1.0 - 1e-10, 1.0 - 1e-12] {
        let j = s.ders(0.37, v);
        let k = max_principal_curvature(&j);
        println!(
            "R1UNIT sphere pole v=1-{:e}: |Su x Sv| = {:e}, kR = {:e}",
            1.0 - v,
            j.du.cross(j.dv).norm(),
            k * r
        );
    }
}

#[test]
fn r1_saddle_reads_its_analytic_curvature() {
    // z = x·y over [−1, 1]²: bilinear patch.
    let k1 = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let ctl = vec![
        Point3::new(-1.0, -1.0, 1.0),
        Point3::new(-1.0, 1.0, -1.0),
        Point3::new(1.0, -1.0, -1.0),
        Point3::new(1.0, 1.0, 1.0),
    ];
    let s = NurbsSurface::new(k1.clone(), k1, ctl, vec![1.0; 4]).unwrap();
    let mut worst: f64 = 0.0;
    for i in 0..=10 {
        for j in 0..=10 {
            let (u, v) = (f64::from(i) / 10.0, f64::from(j) / 10.0);
            let (x, y) = (2.0 * u - 1.0, 2.0 * v - 1.0);
            let q = 1.0 + x * x + y * y;
            let kk = -1.0 / (q * q);
            let h = -x * y / q.powf(1.5);
            let truth = h.abs() + (h * h - kk).sqrt();
            let got = max_principal_curvature(&s.ders(u, v));
            worst = worst.max((got / truth - 1.0).abs());
        }
    }
    println!("R1UNIT saddle z=xy: max rel err = {worst:e}");
}

fn ijet(j: &SurfaceJet<f64>, pad: f64) -> SurfaceJet<Interval> {
    let iv = |v: Vec3<f64>| {
        Vec3::new(
            Interval::from_bounds(v.x - pad, v.x + pad),
            Interval::from_bounds(v.y - pad, v.y + pad),
            Interval::from_bounds(v.z - pad, v.z + pad),
        )
    };
    SurfaceJet {
        point: Point3::new(
            Interval::from_bounds(j.point.x, j.point.x),
            Interval::from_bounds(j.point.y, j.point.y),
            Interval::from_bounds(j.point.z, j.point.z),
        ),
        du: iv(j.du),
        dv: iv(j.dv),
        duu: iv(j.duu),
        duv: iv(j.duv),
        dvv: iv(j.dvv),
    }
}

#[test]
fn r1_interval_and_poison() {
    let r = 0.3;
    let s = sphere_band(r, 0.1, 1.0, 1.0);
    let j = s.ders(0.4, 0.6);
    for pad in [0.0, 1e-12, 1e-6, 1e-2, 0.3] {
        let k = max_principal_curvature(&ijet(&j, pad));
        println!(
            "R1UNIT interval pad={pad:e}: kR in [{:e}, {:e}] certified={}",
            k.lo() * r,
            k.hi() * r,
            k.is_certified()
        );
        // The at-rest arm as edge_nurbs spells it.
        let e = Interval::from_bounds(1.0, 1.0);
        let arm = e / Interval::one().max(k * e);
        println!(
            "R1UNIT interval pad={pad:e}: arm in [{:e}, {:e}] certified={}",
            arm.lo(),
            arm.hi(),
            arm.is_certified()
        );
    }
    // A singular chart: S_v = 0.
    let mut d = j;
    d.dv = Vec3::new(0.0, 0.0, 0.0);
    println!("R1UNIT singular f64: {:e}", max_principal_curvature(&d));
    let ki = max_principal_curvature(&ijet(&d, 0.0));
    println!(
        "R1UNIT singular interval: [{:e}, {:e}] certified={}",
        ki.lo(),
        ki.hi(),
        ki.is_certified()
    );
    // Poisoned second derivative only.
    let mut p = j;
    p.duu = Vec3::new(f64::NAN, 0.0, 0.0);
    println!("R1UNIT duu NaN: {:e}", max_principal_curvature(&p));
    // An infinite second derivative: L = inf and LN − M² may be NaN.
    let mut q = j;
    q.duu = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    let kq = max_principal_curvature(&q);
    let e = 1.0_f64;
    println!(
        "R1UNIT duu inf: kappa {kq:e}, at-rest arm {:e}",
        e / Real::max(1.0_f64, kq * e)
    );
    for kappa in [0.0, f64::NAN, f64::INFINITY, 1e-300, 1e300] {
        let arm = e / Real::max(1.0_f64, kappa * e);
        println!("R1UNIT at-rest arm kappa={kappa:e}: {arm:e}");
    }
    // Umbilic cancellation: H² − K on a sphere at 1e-3 and 1e3 m.
    for rr in [1e-3, 1.0, 1e3] {
        let s = sphere_band(rr, 0.1, 1.0, 1.0);
        let mut worst: f64 = 0.0;
        for i in 0..=50 {
            let k = max_principal_curvature(&s.ders(f64::from(i) / 50.0, 0.33));
            worst = worst.max((k * rr - 1.0).abs());
        }
        println!("R1UNIT umbilic R={rr:e}: max |kR-1| = {worst:e}");
    }
}
