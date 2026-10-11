//! Second verifier probe for PR 4573's fix pass (7a1f6fcef): the
//! centroid apex and the residue blend, f64 and interval alike — the
//! bool5r1 k=20 lune's enclosure width, caps of every size in both
//! senses, both traversals and both storages, holed faces with a small
//! outer region, and the neighbourhood of one steradian.
#![allow(clippy::all, clippy::pedantic, clippy::unwrap_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, curved_face, curved_face_loops};
use geom_core::{Bounds, Decide, Interval, Point3, Real, Vec3};

const TAU: f64 = std::f64::consts::TAU;
const FOUR_PI: f64 = 2.0 * TAU;
const RS: f64 = 0.010;
const H: f64 = std::f64::consts::FRAC_PI_2;

/// The bool5r1 k = 20 lune, as `bool5r1_probes::ladder` builds it.
#[test]
fn v2_lune_width() {
    let zero_over_r = eps() / RS;
    for k in [10.01, 20.0, 100.0, 1e4] {
        let delta = k * zero_over_r;
        let exact = 2.0 * RS * RS * delta;
        let ei: Vec<LoopEdge<Interval>> = vec![
            topo::sphere_great(RS, 0.0, H, -H, 0, 1),
            topo::sphere_great(RS, delta, -H, H, 1, 0),
        ];
        let ef: Vec<LoopEdge<f64>> = vec![
            topo::sphere_great(RS, 0.0, H, -H, 0, 1),
            topo::sphere_great(RS, delta, -H, H, 1, 0),
        ];
        let si: Surface<Interval> = crate::shared::surf::sphere(RS);
        let sf: Surface<f64> = crate::shared::surf::sphere(RS);
        let ai = curved_face(&si, &ei, true, band()).map(|f| f.area);
        let af = curved_face(&sf, &ef, true, band()).map(|f| f.area);
        match ai {
            Ok(a) => println!(
                "LUNE eps={:e} k={k} exact={exact:e} lo={:e} hi={:e} rel_half={:.3e} contains={} f64_rel={:?}",
                eps(),
                a.lo(),
                a.hi(),
                (a.hi() - a.lo()) / 2.0 / exact,
                a.lo() <= exact && exact <= a.hi(),
                af.map(|x| (x - exact) / exact)
            ),
            Err(e) => println!("LUNE eps={:e} k={k} ERR {e:?}", eps()),
        }
    }
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

fn cap_loop<T: Real>(
    c: Point3<f64>,
    r: f64,
    d: Vec3<f64>,
    theta: f64,
    n: usize,
    ccw: bool,
    stored_rev: bool,
) -> Vec<LoopEdge<T>> {
    let d = unit(d);
    let u = perp(d);
    let centre = c + d * (r * theta.cos());
    let rho = r * theta.sin();
    let carrier = |axis: Vec3<f64>| Curve3::Circle {
        center: centre.map(T::from_f64),
        axis: axis.map(T::from_f64),
        radius: T::from_f64(rho),
        u_ref: u.map(T::from_f64),
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
                LoopEdge::hand_built(carrier(d), T::from_f64(a), T::from_f64(b), ccw, 0, 0)
            } else {
                LoopEdge::hand_built(
                    carrier(-d),
                    T::from_f64(TAU - b),
                    T::from_f64(TAU - a),
                    !ccw,
                    0,
                    0,
                )
            }
        })
        .collect()
}

fn sphere<T: Real>(c: Point3<f64>, r: f64) -> Surface<T> {
    Surface::Sphere {
        center: c.map(T::from_f64),
        radius: T::from_f64(r),
        axis: Vec3::new(0.0, 0.0, 1.0).map(T::from_f64),
        u_ref: Vec3::new(1.0, 0.0, 0.0).map(T::from_f64),
    }
}

fn cap_solid(theta: f64) -> f64 {
    FOUR_PI * (theta / 2.0).sin().powi(2)
}

fn theta_for(solid: f64) -> f64 {
    2.0 * (solid / FOUR_PI).sqrt().asin()
}

fn faces<T: Decide>(
    c: Point3<f64>,
    r: f64,
    loops: &[Vec<LoopEdge<T>>],
    sense: bool,
) -> Result<T, String> {
    let s = sphere::<T>(c, r);
    let refs: Vec<&[LoopEdge<T>]> = loops.iter().map(|l| &l[..]).collect();
    curved_face_loops(&s, &refs, sense, band())
        .map(|f| f.area)
        .map_err(|e| format!("{e:?}"))
}

/// Caps of every size, every (sense, traversal, storage), f64 and
/// interval: the worst f64 relative error, the worst interval relative
/// half-width, and every interval that misses the closed form by more
/// than the input's own rounding (8 ulps of the sphere's area).
#[test]
fn v2_sweep() {
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
    let mut misses = 0;
    let mut sign_bad = 0;
    for &r in &[1.0, 3.0] {
        for &n in &[1usize, 3, 7] {
            for &sa in &sizes {
                let theta = theta_for(sa);
                let cap = cap_solid(theta) * r * r;
                let mut worst: f64 = 0.0;
                let mut worst_w: f64 = 0.0;
                for (sense, ccw) in [(true, true), (false, false), (true, false), (false, true)] {
                    for rev in [false, true] {
                        let want = if sense == ccw { cap } else { FOUR_PI * r * r - cap };
                        let tag = format!("R={r} n={n} A={sa:e} sense={sense} ccw={ccw} rev={rev}");
                        let lf = cap_loop::<f64>(c, r, d, theta, n, ccw, rev);
                        let li = cap_loop::<Interval>(c, r, d, theta, n, ccw, rev);
                        match faces(c, r, &[lf], sense) {
                            Ok(got) => {
                                if got.signum() != want.signum() {
                                    sign_bad += 1;
                                    println!("SIGN f64 {tag}: {got:e} vs {want:e}");
                                }
                                if want <= FOUR_PI * r * r / 2.0 {
                                    let rel = (got - want) / want;
                                    if rel.abs() > worst.abs() {
                                        worst = rel;
                                    }
                                }
                            }
                            Err(e) => *errs.entry(format!("f64 {}", &e[..e.len().min(70)])).or_default() += 1,
                        }
                        match faces(c, r, &[li], sense) {
                            Ok(got) => {
                                let slack = 8.0 * f64::EPSILON * FOUR_PI * r * r * 1e-2 + 4.0 * f64::EPSILON * want;
                                if got.lo() > want + slack || got.hi() < want - slack {
                                    misses += 1;
                                    println!(
                                        "MISS iv {tag}: [{:e}, {:e}] vs {want:e} (by {:e})",
                                        got.lo(),
                                        got.hi(),
                                        (got.lo() - want).max(want - got.hi())
                                    );
                                }
                                if got.lo() < 0.0 {
                                    sign_bad += 1;
                                    println!("SIGN iv {tag}: lo {:e}", got.lo());
                                }
                                if want <= FOUR_PI * r * r / 2.0 {
                                    let w = (got.hi() - got.lo()) / 2.0 / want;
                                    worst_w = worst_w.max(w);
                                }
                            }
                            Err(e) => *errs.entry(format!("iv {}", &e[..e.len().min(70)])).or_default() += 1,
                        }
                    }
                }
                println!(
                    "SWEEP R={r} n={n} A={sa:.17e} f64_worst_rel {worst:+.3e} iv_worst_rel_half {worst_w:.3e}"
                );
            }
        }
    }
    for (k, v) in &errs {
        println!("REFUSED {v:5} {k}");
    }
    println!("TOTAL misses={misses} sign_bad={sign_bad}");
}

/// Holed faces whose outer region is at most a steradian, both senses
/// and both storages, f64 and interval.
#[test]
fn v2_holed() {
    let c = Point3::new(0.3, -1.1, 0.7);
    let d = unit(Vec3::new(0.8, 0.5, 0.33));
    let r = 2.0;
    let near1 = theta_for(1.0 - 1e-9);
    let cases: Vec<(&str, f64, Vec3<f64>, f64)> = vec![
        ("offset hole", 0.5, unit(d + perp(d) * 0.15), 0.2),
        ("tiny outer", 1e-3, unit(d + perp(d) * 2e-4), 3e-4),
        ("annulus 1e-6", 0.5, d, 0.5 - 1e-6),
        ("annulus 1e-9", 0.5, d, 0.5 - 1e-9),
        ("outer ~1sr", near1, unit(d + perp(d) * 0.1), 0.1),
        ("outer just >1sr", theta_for(1.0 + 1e-9), unit(d + perp(d) * 0.1), 0.1),
    ];
    for (name, to, dh, th) in cases {
        let want = if name.starts_with("annulus") {
            FOUR_PI * ((to - th) / 2.0).sin() * ((to + th) / 2.0).sin() * r * r
        } else {
            (cap_solid(to) - cap_solid(th)) * r * r
        };
        for (sense, rev) in [(true, false), (true, true), (false, false), (false, true)] {
            let of = cap_loop::<f64>(c, r, d, to, 3, sense, rev);
            let hf = cap_loop::<f64>(c, r, dh, th, 2, !sense, !rev);
            let oi = cap_loop::<Interval>(c, r, d, to, 3, sense, rev);
            let hi = cap_loop::<Interval>(c, r, dh, th, 2, !sense, !rev);
            let f = faces(c, r, &[of, hf], sense);
            let i = faces(c, r, &[oi, hi], sense);
            let fs = match f {
                Ok(a) => format!("rel {:+.3e}", (a - want) / want),
                Err(e) => format!("ERR {e}"),
            };
            let is = match i {
                Ok(a) => format!(
                    "contains {} rel_half {:.3e} lo-want {:+.3e} hi-want {:+.3e}",
                    a.lo() <= want && want <= a.hi(),
                    (a.hi() - a.lo()) / 2.0 / want,
                    a.lo() - want,
                    a.hi() - want
                ),
                Err(e) => format!("ERR {e}"),
            };
            println!("HOLED {name} sense={sense} rev={rev}: f64 {fs} | iv {is}");
        }
    }
}
