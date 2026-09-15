//! R2 probe for SENSE-DOORS: prints bit-faithful Debug of every door's
//! verdict/margin on fixtures where the two senses classify
//! differently, at f64 and (feature `interval`) at `Interval`. Run at
//! the merge base (with the `T` adapter) and at the head (with the
//! `bool` adapter) and diff the stdout.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, curved_face};
use geom_brep::{classify_material_pairing, material_kappa_rel};
use geom_core::{Band, Decide, Point3, Real, Tol, Vec3};

// HEAD adapter: the door takes the bit.
fn sa<T: Real>(b: bool) -> bool {
    b
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn p3<T: Real>(x: f64, y: f64, z: f64) -> Point3<T> {
    Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}
fn v3<T: Real>(x: f64, y: f64, z: f64) -> Vec3<T> {
    Vec3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z))
}
fn plane<T: Real>(n: Vec3<T>, u: Vec3<T>) -> Surface<T> {
    Surface::Plane { origin: p3(0.0, 0.0, 0.0), normal: n, u_ref: u }
}
fn edge<T: Real>(carrier: Curve3<T>, a: f64, b: f64, s: u32, e: u32) -> LoopEdge<T> {
    let (t0, t1, fwd) = if a < b { (a, b, true) } else { (b, a, false) };
    LoopEdge::hand_built(carrier, T::from_f64(t0), T::from_f64(t1), fwd, s, e)
}
fn sphere<T: Real>(r: f64) -> Surface<T> {
    Surface::Sphere { center: p3(0.0, 0.0, 0.0), radius: T::from_f64(r), axis: v3(0.0, 0.0, 1.0), u_ref: v3(1.0, 0.0, 0.0) }
}
fn srim<T: Real>(r: f64, v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<T> {
    edge(Curve3::Circle { center: p3(0.0, 0.0, r * v.sin()), axis: v3(0.0, 0.0, 1.0), radius: T::from_f64(r * v.cos()), u_ref: v3(1.0, 0.0, 0.0) }, u0, u1, a, b)
}
fn sgreat<T: Real>(r: f64, u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
    edge(Curve3::Circle { center: p3(0.0, 0.0, 0.0), axis: v3(u.sin(), -u.cos(), 0.0), radius: T::from_f64(r), u_ref: v3(u.cos(), u.sin(), 0.0) }, t0, t1, a, b)
}
fn cyl<T: Real>() -> Surface<T> {
    Surface::Cylinder { origin: p3(0.0, 0.0, 0.0), axis: v3(0.0, 0.0, 1.0), radius: T::one(), u_ref: v3(1.0, 0.0, 0.0) }
}
fn crim<T: Real>(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<T> {
    edge(Curve3::Circle { center: p3(0.0, 0.0, v), axis: v3(0.0, 0.0, 1.0), radius: T::one(), u_ref: v3(1.0, 0.0, 0.0) }, u0, u1, a, b)
}
fn cmer<T: Real>(u: f64, v0: f64, v1: f64, a: u32, b: u32) -> LoopEdge<T> {
    edge(Curve3::Line { origin: p3(u.cos(), u.sin(), 0.0), dir: v3(0.0, 0.0, 1.0) }, v0, v1, a, b)
}

fn run<T: Decide + core::fmt::Debug>(tag: &str) {
    let b = band();
    let o = p3::<T>(0.0, 0.0, 0.0);
    let one = T::one();
    // --- classify_material_pairing ---
    let s1 = plane(v3::<T>(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0));
    let s2 = plane(v3::<T>(0.0, 0.0, 1.0), v3(0.0, 1.0, 0.0));
    let s3 = plane(-v3::<T>(0.0, 0.0, 1.0), v3(1.0, 0.0, 0.0));
    let inner = Surface::Cylinder { origin: p3::<T>(0.0, 0.0, 1.0), axis: v3(0.0, 1.0, 0.0), radius: one, u_ref: v3(1.0, 0.0, 0.0) };
    let outer = Surface::Cylinder { origin: p3::<T>(0.0, 0.0, 2.0), axis: v3(0.0, 1.0, 0.0), radius: T::from_f64(2.0), u_ref: v3(1.0, 0.0, 0.0) };
    let d = 1e-10;
    let near = plane(v3::<T>(1.0, 0.0, d).normalize(), v3(0.0, 1.0, 0.0));
    let sliver = plane(v3::<T>((3e-9f64).sin(), 0.0, (3e-9f64).cos()), v3(0.0, 1.0, 0.0));
    let sph = sphere::<T>(1.0);
    let equ = p3::<T>(1.0, 0.0, 0.0);
    let cy = cyl::<T>();
    let pairs: Vec<(&str, &Surface<T>, &Surface<T>, Point3<T>, T)> = vec![
        ("coplanar", &s1, &s2, o, one),
        ("antiparallel", &s1, &s3, o, one),
        ("kissing", &inner, &outer, o, one),
        ("near-perp", &s1, &near, o, one),
        ("sliver", &s1, &sliver, o, one),
        ("sphere-cyl", &sph, &cy, equ, T::from_f64(2.0)),
    ];
    for (name, a, c, p, arm) in pairs {
        for (sp, sm) in [(true, true), (true, false), (false, true), (false, false)] {
            let r = classify_material_pairing(a, sa::<T>(sp), c, sa::<T>(sm), p, arm, b);
            println!("[{tag}] pairing {name} {sp} {sm}: {r:?}");
        }
    }
    // --- material_kappa_rel ---
    for k in [0.0, -0.0, 1.5, -2.25, 1e-300, f64::MIN_POSITIVE, 3.0e10, 0.1 + 0.2, f64::MAX] {
        for s in [true, false] {
            let r = material_kappa_rel(T::from_f64(k), sa::<T>(s));
            println!("[{tag}] kappa {k:e} {s}: {r:?}");
        }
    }
    let half = core::f64::consts::FRAC_PI_2;
    let rs = 0.010;
    let s = sphere::<T>(rs);
    let rimless = vec![sgreat::<T>(rs, 0.0, -half, half, 0, 1), sgreat(rs, 0.0, half, 3.0 * half, 1, 0)];
    let zone = vec![
        srim::<T>(rs, 0.2, 0.0, 1.0, 0, 1),
        sgreat(rs, 1.0, 0.2, 0.9, 1, 2),
        srim(rs, 0.9, 1.0, 0.0, 2, 3),
        sgreat(rs, 0.0, 0.9, 0.2, 3, 0),
    ];
    let zone_rev: Vec<LoopEdge<T>> = vec![
        sgreat::<T>(rs, 0.0, 0.2, 0.9, 0, 1),
        srim(rs, 0.9, 0.0, 1.0, 1, 2),
        sgreat(rs, 1.0, 0.9, 0.2, 2, 3),
        srim(rs, 0.2, 1.0, 0.0, 3, 0),
    ];
    let c = cyl::<T>();
    let rect = vec![crim::<T>(0.0, 0.0, 1.5, 0, 1), cmer(1.5, 0.0, 1.0, 1, 2), crim(1.0, 1.5, 0.0, 2, 3), cmer(0.0, 1.0, 0.0, 3, 0)];
    let loops: Vec<(&str, &Surface<T>, &Vec<LoopEdge<T>>)> = vec![
        ("rimless", &s, &rimless),
        ("zone", &s, &zone),
        ("zone-rev", &s, &zone_rev),
        ("cyl-rect", &c, &rect),
    ];
    for (name, surf, l) in loops {
        for sn in [true, false] {
            let r = curved_face(surf, l, sa::<T>(sn), b);
            println!("[{tag}] curved {name} {sn}: {r:?}");
        }
    }
}

#[test]
fn r2_sense_doors_probe_f64() {
    run::<f64>("f64");
}

#[cfg(feature = "interval")]
#[test]
fn r2_sense_doors_probe_interval() {
    run::<geom_core::Interval>("interval");
}
