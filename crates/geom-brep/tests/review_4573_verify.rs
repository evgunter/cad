//! Verifier probe for PR 4573's fix pass (e379f1fd): the solid-angle
//! form swept across region sizes, both senses and both storages, and
//! holed faces whose outer region is at most a steradian.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, curved_face_loops};
use geom_core::{Band, Point3, Tol, Vec3};

const TAU: f64 = std::f64::consts::TAU;
const FOUR_PI: f64 = 2.0 * TAU;

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn unit(v: Vec3<f64>) -> Vec3<f64> {
    v / v.norm()
}

fn perp(d: Vec3<f64>) -> Vec3<f64> {
    let a = if d.x.abs() < 0.6 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    unit(a - d * a.dot(d))
}

/// The loop of a cap of half-angle `theta` about `d` on the sphere
/// `(c, r)`, in `n` arcs, traversed counterclockwise about `d` when
/// `ccw`, each arc stored on the reversed carrier when `stored_rev`.
fn cap_loop(
    c: Point3<f64>,
    r: f64,
    d: Vec3<f64>,
    theta: f64,
    n: usize,
    ccw: bool,
    stored_rev: bool,
) -> Vec<LoopEdge<f64>> {
    let d = unit(d);
    let u = perp(d);
    let centre = c + d * (r * theta.cos());
    let rho = r * theta.sin();
    let carrier = |axis: Vec3<f64>| Curve3::Circle {
        center: centre,
        axis,
        radius: rho,
        u_ref: u,
    };
    let step = TAU / n as f64;
    let ks: Vec<usize> = if ccw {
        (0..n).collect()
    } else {
        (0..n).rev().collect()
    };
    ks.into_iter()
        .map(|k| {
            let (a, b) = (k as f64 * step, (k + 1) as f64 * step);
            if !stored_rev {
                LoopEdge::hand_built(carrier(d), a, b, ccw, 0, 0)
            } else {
                LoopEdge::hand_built(carrier(-d), TAU - b, TAU - a, !ccw, 0, 0)
            }
        })
        .collect()
}

fn sphere(c: Point3<f64>, r: f64) -> Surface<f64> {
    Surface::Sphere {
        center: c,
        radius: r,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn cap_solid(theta: f64) -> f64 {
    FOUR_PI * (theta / 2.0).sin().powi(2)
}

fn theta_for(solid: f64) -> f64 {
    2.0 * (solid / FOUR_PI).sqrt().asin()
}

/// Every (sense, traversal, storage) combination that states the cap
/// as the face, and the ones that state its complement.
fn measure(c: Point3<f64>, r: f64, d: Vec3<f64>, theta: f64, n: usize) -> Vec<(String, f64, f64)> {
    let s = sphere(c, r);
    let cap = cap_solid(theta) * r * r;
    let mut out = vec![];
    for (sense, ccw) in [(true, true), (false, false), (true, false), (false, true)] {
        for rev in [false, true] {
            let lp = cap_loop(c, r, d, theta, n, ccw, rev);
            let want = if sense == ccw { cap } else { FOUR_PI * r * r - cap };
            let tag = format!("sense={sense} ccw={ccw} rev={rev}");
            match curved_face_loops(&s, &[&lp[..]], sense, band()) {
                Ok(f) => out.push((tag, f.area, want)),
                Err(e) => out.push((format!("{tag} ERR {e:?}"), f64::NAN, want)),
            }
        }
    }
    out
}

#[test]
fn verify_4573_solid_angle_sweep() {
    let c = Point3::new(0.3, -1.1, 0.7);
    let d = Vec3::new(0.8, 0.5, 0.33);
    let mut sizes: Vec<f64> = vec![];
    let mut a = 1e-14;
    while a < 3.5 {
        sizes.push(a);
        a *= 3.0;
    }
    for i in -20..=20 {
        sizes.push(1.0 + i as f64 * 1e-5);
    }
    for i in -5..=5 {
        sizes.push(1.0 + i as f64 * 2.3e-16);
    }
    sizes.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut errs = std::collections::BTreeMap::<String, u32>::new();
    for &r in &[1.0, 3.0] {
        for &n in &[1usize, 3, 7] {
            for &sa in &sizes {
                let theta = theta_for(sa);
                let mut worst: f64 = 0.0;
                let mut worst_tag = String::new();
                for (tag, got, want) in measure(c, r, d, theta, n) {
                    if got.is_nan() {
                        let k = tag.split("ERR").nth(1).unwrap_or("").chars().take(60).collect::<String>();
                        let cls = if tag.contains("sense=true ccw=true") || tag.contains("sense=false ccw=false") { "cap" } else { "complement" };
                        *errs.entry(format!("{cls}:{k}")).or_default() += 1;
                        continue;
                    }
                    if want > FOUR_PI * r * r / 2.0 {
                        continue;
                    }
                    let rel = (got - want) / want;
                    if rel.abs() >= worst.abs() {
                        worst = rel;
                        worst_tag = tag;
                    }
                }
                println!(
                    "SWEEP R={r} n={n} A={sa:.17e} worst_rel {worst:+.3e} (GB-alone scale {:.1e}) {worst_tag}",
                    2.0 * TAU * f64::EPSILON / sa
                );
            }
        }
    }
    for (k, v) in &errs {
        println!("REFUSED {v:5} {k}");
    }
}

/// Holed faces whose outer region is at most a steradian, under both
/// senses and both storages: the outer cap less an off-centre hole, and
/// a thin concentric annulus.
#[test]
fn verify_4573_holed_small_outer() {
    let c = Point3::new(0.3, -1.1, 0.7);
    let d = unit(Vec3::new(0.8, 0.5, 0.33));
    let r = 2.0;
    let s = sphere(c, r);
    let cases: Vec<(&str, f64, Vec3<f64>, f64)> = vec![
        ("offset hole", 0.5, unit(d + perp(d) * 0.15), 0.2),
        ("tiny outer", 1e-3, unit(d + perp(d) * 2e-4), 3e-4),
        ("annulus 1e-6", 0.5, d, 0.5 - 1e-6),
        ("annulus 1e-9", 0.5, d, 0.5 - 1e-9),
    ];
    for (name, to, dh, th) in cases {
        let want = if name.starts_with("annulus") {
            FOUR_PI * ((to - th) / 2.0).sin() * ((to + th) / 2.0).sin() * r * r
        } else {
            (cap_solid(to) - cap_solid(th)) * r * r
        };
        for (sense, rev) in [(true, false), (true, true), (false, false), (false, true)] {
            // The outer loop runs so the cap is left of it under the
            // outward normal; the hole the other way.
            let outer = cap_loop(c, r, d, to, 3, sense, rev);
            let hole = cap_loop(c, r, dh, th, 2, !sense, !rev);
            let got = curved_face_loops(&s, &[&outer[..], &hole[..]], sense, band());
            match got {
                Ok(f) => println!(
                    "{name} sense={sense} rev={rev}: area rel {:+.3e} (abs {:+.3e})",
                    (f.area - want) / want,
                    f.area - want
                ),
                Err(e) => println!("{name} sense={sense} rev={rev}: ERR {e:?}"),
            }
            // The flipped bit must refuse (SenseInverted).
            let flipped = curved_face_loops(&s, &[&outer[..], &hole[..]], !sense, band());
            println!("   flipped bit: {:?}", flipped.map(|f| f.area));
        }
    }
}

/// Item E: each refusal's type through the public door.
#[test]
fn verify_4573_refusal_types() {
    let c = Point3::new(0.3, -1.1, 0.7);
    let r = 2.0;
    let s = sphere(c, r);
    let d = unit(Vec3::new(0.8, 0.5, 0.33));
    let empty: [&[LoopEdge<f64>]; 0] = [];
    println!("E empty (no loops): {:?}", curved_face_loops(&s, &empty, true, band()).map(|f| f.area));
    let none: [&[LoopEdge<f64>]; 1] = [&[]];
    println!("E empty (one empty loop): {:?}", curved_face_loops(&s, &none, true, band()).map(|f| f.area));
    // An open loop: two arcs of a three-arc cap.
    let lp = cap_loop(c, r, d, 0.4, 3, true, false);
    println!("E open loop: {:?}", curved_face_loops(&s, &[&lp[..2]], true, band()).map(|f| f.area));
    // A span past a whole turn, and a reversed (negative) span.
    let mut wide = cap_loop(c, r, d, 0.4, 1, true, false);
    wide[0].t1 = TAU * 1.5;
    println!("E span past a turn: {:?}", curved_face_loops(&s, &[&wide[..]], true, band()).map(|f| f.area));
    let mut back = cap_loop(c, r, d, 0.4, 1, true, false);
    back[0].t0 = TAU;
    back[0].t1 = 0.0;
    println!("E reversed span: {:?}", curved_face_loops(&s, &[&back[..]], true, band()).map(|f| f.area));
    // A one-loop face whose bit its boundary contradicts.
    let lp = cap_loop(c, r, d, 0.4, 3, true, false);
    println!("E one loop, wrong bit: {:?}", curved_face_loops(&s, &[&lp[..]], false, band()).map(|f| f.area));
    // A holed face, wrong bit.
    let hole = cap_loop(c, r, d, 0.2, 2, false, false);
    println!("E holed, wrong bit: {:?}", curved_face_loops(&s, &[&lp[..], &hole[..]], false, band()).map(|f| f.area));
    println!("E holed, right bit: {:?}", curved_face_loops(&s, &[&lp[..], &hole[..]], true, band()).map(|f| f.area));
}
