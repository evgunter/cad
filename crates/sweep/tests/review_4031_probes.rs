//! Review probes for PR 4031 (parallel cylinder germ pair). Each line is
//! one run judged by `common::differential::outcome` against a closed
//! form. `#[ignore]`d: run with `--ignored --nocapture` and diff trees.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::common::differential::outcome;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use topo::{Body, BooleanError, BooleanResult};

fn rod(r: f64, (x, y): (f64, f64), (z0, z1): (f64, f64)) -> Body<f64> {
    sweep::test_support::prism_at(
        vec![(Point2::new(x - r, y), 1.0), (Point2::new(x + r, y), 1.0)],
        z0,
        z1 - z0,
        Tol::witness(),
    )
}

/// Lens area of discs `(0, r1)` and `(d, r2)`, with the disjoint and
/// nested cases.
fn lens_area(r1: f64, d: f64, r2: f64) -> f64 {
    if d >= r1 + r2 {
        return 0.0;
    }
    if d <= (r1 - r2).abs() {
        return PI * r1.min(r2).powi(2);
    }
    let h1 = (d * d + r1 * r1 - r2 * r2) / (2.0 * d);
    let h2 = d - h1;
    let seg = |r: f64, h: f64| r * r * (h / r).clamp(-1.0, 1.0).acos() - h * (r * r - h * h).max(0.0).sqrt();
    seg(r1, h1) + seg(r2, h2)
}

type Run = (&'static str, Result<BooleanResult<f64>, BooleanError>, f64);

fn runs(a: &Body<f64>, b: &Body<f64>, va: f64, vb: f64, s: f64) -> [Run; 6] {
    let tol = Tol::witness();
    [
        ("U ab", topo::union(a, b, tol), va + vb - s),
        ("U ba", topo::union(b, a, tol), va + vb - s),
        ("I ab", topo::intersect(a, b, tol), s),
        ("I ba", topo::intersect(b, a, tol), s),
        ("S ab", topo::subtract(a, b, tol), va - s),
        ("S ba", topo::subtract(b, a, tol), vb - s),
    ]
}

/// Drum `R` about z over `[z0d, z1d]` against a rod `r` at `(d, 0)` over
/// `span`, optionally both turned by `turn` (axis, angle) about the origin.
#[allow(clippy::too_many_arguments)]
fn pose(
    tag: &str,
    big_r: f64,
    (z0d, z1d): (f64, f64),
    r: f64,
    c: (f64, f64),
    span: (f64, f64),
    turn: Option<(Vec3<f64>, f64)>,
) {
    let mut a = rod(big_r, (0.0, 0.0), (z0d, z1d));
    let mut b = rod(r, c, span);
    if let Some((axis, angle)) = turn {
        let m = Affine3::rotation_about_axis(Point3::new(0.0, 0.0, 0.0), axis, angle);
        a = topo::transform_rigid(&a, &m, Tol::witness()).unwrap();
        b = topo::transform_rigid(&b, &m, Tol::witness()).unwrap();
    }
    let d = c.0.hypot(c.1);
    let overlap = (span.1.min(z1d) - span.0.max(z0d)).max(0.0);
    let s = lens_area(big_r, d, r) * overlap;
    let va = PI * big_r * big_r * (z1d - z0d);
    let vb = PI * r * r * (span.1 - span.0);
    for (op, res, want) in runs(&a, &b, va, vb, s) {
        println!(
            "R4031 {tag} R={big_r} r={r} c={c:?} span={span:?} turn={turn:?} {op} => {}",
            outcome(res, want, Tol::witness())
        );
    }
}

const DELTAS: [f64; 12] = [
    1e-3, -1e-3, 1e-4, -1e-4, 1e-5, -1e-5, 1e-6, -1e-6, 1e-7, -1e-7, 1e-8, -1e-8,
];

#[test]
#[ignore = "review battery"]
fn r4031_near_tangent() {
    let z = (-1.0, 1.0);
    for r in [0.2, 0.3, 0.8] {
        for dl in DELTAS {
            // external tangency
            pose("ext", 0.5, z, r, (0.5 + r + dl, 0.0), (-0.5, 0.5), None);
            // internal tangency
            pose("int", 0.5, z, r, ((0.5 - r).abs() + dl, 0.0), (-0.5, 0.5), None);
        }
    }
    // tilted external tangency, off every axis
    for dl in [1e-4, -1e-4, 1e-6, -1e-6, -1e-8] {
        pose(
            "ext-turned",
            0.5,
            z,
            0.25,
            ((0.75 + dl) * 0.6, (0.75 + dl) * 0.8),
            (-0.5, 0.5),
            Some((Vec3::new(1.0, 2.0, 0.5), 0.7)),
        );
    }
}

#[test]
#[ignore = "review battery"]
fn r4031_equal_radii_and_coaxial() {
    let z = (-1.0, 1.0);
    for off in [0.3, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 4e-6, 1e-7, 1e-8] {
        pose("eqr", 0.5, z, 0.5, (off, 0.0), (-0.5, 0.5), None);
        pose("eqr-long", 0.5, z, 0.5, (0.0, off), (-1.5, 1.5), None);
        pose("coax", 0.5, z, 0.2, (off, 0.0), (-0.5, 0.5), None);
        pose("coax-nr", 0.5, z, 0.5 - 1e-5, (off, 0.0), (-0.5, 0.5), None);
    }
}

#[test]
#[ignore = "review battery"]
fn r4031_cap_edge_and_turned() {
    let z = (-1.0, 1.0);
    // a rod crossing the drum's top rim, its cap above, near and at the cap plane
    for top in [1.5, 1.0 + 1e-3, 1.0 + 1e-6, 1.0 - 1e-6, 1.0 - 1e-3] {
        pose("cap", 0.5, z, 0.2, (0.5, 0.0), (0.2, top), None);
        pose("cap-wide", 0.5, z, 0.8, (0.9, 0.0), (0.0, top), None);
    }
    // a rod whose wall passes through the drum's rim circle point at the cap
    pose("cap-low", 0.5, z, 0.2, (0.6, 0.1), (-1.5, -0.2), None);
    for (axis, ang) in [
        (Vec3::new(1.0, 0.0, 0.0), PI / 2.0),
        (Vec3::new(0.3, -1.0, 0.2), 1.3),
        (Vec3::new(1.0, 1.0, 1.0), 2.5),
        (Vec3::new(0.0, 1.0, 0.0), 1e-3),
    ] {
        for (r, c) in [(0.2, (0.5, 0.0)), (0.5, (0.0, 0.7)), (0.8, (-0.9, 0.1)), (0.15, (0.0, 0.5))] {
            pose("turned", 0.5, z, r, c, (-0.5, 0.5), Some((axis, ang)));
        }
    }
}

/// Two rods cut one after the other (and unioned), disjoint from each other.
#[test]
#[ignore = "review battery"]
fn r4031_two_rods() {
    let tol = Tol::witness();
    let drum = rod(0.5, (0.0, 0.0), (-1.0, 1.0));
    let vd = PI * 0.25 * 2.0;
    for ((r1, c1), (r2, c2)) in [
        ((0.2, (0.5, 0.0)), (0.2, (-0.5, 0.0))),
        ((0.2, (0.5, 0.0)), (0.15, (0.0, 0.5))),
        ((0.3, (0.6, 0.1)), (0.2, (-0.3, -0.5))),
    ] {
        let p = rod(r1, c1, (-0.5, 0.5));
        let q = rod(r2, c2, (-0.7, 0.3));
        let (l1, l2) = (
            lens_area(0.5, f64::hypot(c1.0, c1.1), r1),
            lens_area(0.5, f64::hypot(c2.0, c2.1), r2),
        );
        let (vp, vq) = (PI * r1 * r1, PI * r2 * r2);
        let tag = format!("two {r1}@{c1:?} {r2}@{c2:?}");
        let first = topo::subtract(&drum, &p, tol).map(|r| r.body().unwrap().body.clone());
        match first {
            Ok(d1) => {
                let want = vd - l1 - l2;
                println!("R4031 {tag} S S => {}", outcome(topo::subtract(&d1, &q, tol), want, tol));
                println!("R4031 {tag} S U => {}", outcome(topo::union(&d1, &q, tol), vd - l1 + vq - l2, tol));
                println!("R4031 {tag} S I => {}", outcome(topo::intersect(&q, &d1, tol), l2, tol));
            }
            Err(e) => println!("R4031 {tag} first ERR {e:?}"),
        }
        match topo::union(&drum, &p, tol).map(|r| r.body().unwrap().body.clone()) {
            Ok(u1) => {
                println!("R4031 {tag} U U => {}", outcome(topo::union(&u1, &q, tol), vd + vp - l1 + vq - l2, tol));
                println!("R4031 {tag} U S => {}", outcome(topo::subtract(&u1, &q, tol), vd + vp - l1 - l2, tol));
            }
            Err(e) => println!("R4031 {tag} firstU ERR {e:?}"),
        }
    }
}

/// A rod tipped off parallel by `theta` about its own centre (x axis),
/// long, so the axis drift over the rod reaches many eps.
#[test]
#[ignore = "review battery"]
fn r4031_near_parallel_tilt() {
    let tol = Tol::witness();
    for (half, theta) in [
        (0.5, 1e-5),
        (0.5, 1e-6),
        (0.5, 1e-7),
        (20.0, 1e-6),
        (20.0, 5e-7),
        (20.0, 2e-7),
        (20.0, 1e-7),
        (20.0, 1e-8),
        (200.0, 1e-8),
        (200.0, 1e-9),
    ] {
        let a = rod(0.5, (0.0, 0.0), (-half - 1.0, half + 1.0));
        let b0 = rod(0.2, (0.5, 0.0), (-half, half));
        let m = Affine3::rotation_about_axis(Point3::new(0.5, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), theta);
        let b = topo::transform_rigid(&b0, &m, tol).unwrap();
        let s = lens_area(0.5, 0.5, 0.2) * 2.0 * half;
        let va = PI * 0.25 * (2.0 * half + 2.0);
        let vb = PI * 0.04 * 2.0 * half;
        for (op, res, want) in runs(&a, &b, va, vb, s) {
            println!("R4031 tilt half={half} theta={theta} drift={:e} {op} => {}", half * theta, outcome(res, want, tol));
        }
    }
}

// ---- join1_delta_arc_battery re-judged by differential::outcome ----

fn zprism(pts: &[(f64, f64, f64)], d: (f64, f64), z: (f64, f64)) -> Body<f64> {
    use profile::{Profile, SketchPlane, test_support::bulge_loop};
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z.0)));
    let lp = bulge_loop(
        pts.iter()
            .map(|&(x, y, b)| (Point2::new(x + d.0, y + d.1), b))
            .collect(),
    );
    let p = Profile::new(plane, vec![lp]).validate(Tol::witness()).unwrap();
    sweep::extrude(
        &p,
        sweep::Extrusion::Distance {
            depth: z.1 - z.0,
            side: sweep::ExtrudeSide::Along,
        },
        Tol::witness(),
    )
    .unwrap()
    .body
}

fn polygon(pts: &[(f64, f64, f64)], d: (f64, f64), n: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for i in 0..pts.len() {
        let (x0, y0, b) = pts[i];
        let (x1, y1, _) = pts[(i + 1) % pts.len()];
        out.push((x0 + d.0, y0 + d.1));
        if b == 0.0 {
            continue;
        }
        let theta = 4.0 * b.atan();
        let (mx, my) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
        let (cx, cy) = (x1 - x0, y1 - y0);
        let chord = cx.hypot(cy);
        let r = chord / (2.0 * (theta / 2.0).sin());
        let h = r * (theta / 2.0).cos();
        let (nx, ny) = (-cy / chord, cx / chord);
        let (ccx, ccy) = (mx + nx * h, my + ny * h);
        let a0 = (y0 - ccy).atan2(x0 - ccx);
        for k in 1..n {
            let t = a0 + theta * (k as f64) / (n as f64);
            out.push((ccx + r.abs() * t.cos() + d.0, ccy + r.abs() * t.sin() + d.1));
        }
    }
    out
}

fn parea(p: &[(f64, f64)]) -> f64 {
    let n = p.len();
    0.5 * (0..n)
        .map(|i| p[i].0 * p[(i + 1) % n].1 - p[(i + 1) % n].0 * p[i].1)
        .sum::<f64>()
}

fn clip(subject: &[(f64, f64)], clipper: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut out = subject.to_vec();
    for i in 0..clipper.len() {
        let (a, b) = (clipper[i], clipper[(i + 1) % clipper.len()]);
        let side = |p: (f64, f64)| (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0);
        let inp = out.clone();
        out.clear();
        for j in 0..inp.len() {
            let (p, q) = (inp[(j + inp.len() - 1) % inp.len()], inp[j]);
            let (sp, sq) = (side(p), side(q));
            if sq >= 0.0 {
                if sp < 0.0 {
                    let t = sp / (sp - sq);
                    out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
                }
                out.push(q);
            } else if sp >= 0.0 {
                let t = sp / (sp - sq);
                out.push((p.0 + t * (q.0 - p.0), p.1 + t * (q.1 - p.1)));
            }
        }
        if out.is_empty() {
            return out;
        }
    }
    out
}

/// Richardson-extrapolated area (chord error is O(n⁻²)).
fn rich<F: Fn(usize) -> f64>(f: F) -> f64 {
    let (a1, a2) = (f(1024), f(2048));
    (4.0 * a2 - a1) / 3.0
}

fn bulge(deg: f64) -> f64 {
    (deg.to_radians() / 4.0).tan()
}

fn arc_shapes() -> Vec<(&'static str, Vec<(f64, f64, f64)>)> {
    vec![
        ("half", vec![(0.5, 0.0, bulge(180.0)), (-0.5, 0.0, 0.0)]),
        ("lower", vec![(-0.5, 0.0, bulge(180.0)), (0.5, 0.0, 0.0)]),
        ("shallow", vec![(0.5, 0.0, bulge(90.0)), (-0.5, 0.0, 0.0)]),
        ("quarter", vec![(0.0, 0.0, 0.0), (0.5, 0.0, bulge(90.0)), (0.0, 0.5, 0.0)]),
        ("square", vec![(-0.5, -1.0, 0.0), (0.5, -1.0, 0.0), (0.5, 0.0, 0.0), (-0.5, 0.0, 0.0)]),
        ("tri", vec![(0.5, 0.0, 0.0), (0.0, 0.5, 0.0), (-0.5, 0.0, 0.0)]),
        ("lens", vec![(0.5, 0.0, bulge(90.0)), (-0.5, 0.0, bulge(90.0))]),
    ]
}

/// Closed-form sanity of the extrapolated oracle: half disc against the
/// half disc shifted (0.5, 0) is half the r = 0.5 lens at d = 0.5.
#[test]
#[ignore = "review battery"]
fn r4031_arc_oracle_check() {
    let s = arc_shapes();
    let got = rich(|n| parea(&clip(&polygon(&s[0].1, (0.0, 0.0), n), &polygon(&s[0].1, (0.5, 0.0), n))).max(0.0));
    let want = lens_area(0.5, 0.5, 0.5) / 2.0;
    println!("R4031ORACLE half/half d=0.5: {got:.15} closed {want:.15} diff {:e}", got - want);
    let got = rich(|n| parea(&polygon(&s[2].1, (0.0, 0.0), n)));
    let r: f64 = 0.5 / (PI / 4.0).sin();
    let want = r * r * (PI / 2.0) / 2.0 - 0.25;
    println!("R4031ORACLE shallow seg: {got:.15} closed {want:.15} diff {:e}", got - want);
}

#[test]
#[ignore = "review battery"]
fn r4031_arc_battery_differential() {
    use topo::flush::{declare_all, find_flush_candidates};
    let tol = Tol::witness();
    let shapes = arc_shapes();
    let offsets = [(0.0, 0.0), (0.5, 0.0), (-0.5, 0.0), (0.0, 0.5), (1.0, 0.0)];
    let za = (0.0, 2.0);
    let zbs = [(0.0, 2.0), (1.0, 3.0), (2.0, 4.0), (-1.0, 1.0), (0.5, 1.5)];
    for (na, pa) in &shapes {
        let a = zprism(pa, (0.0, 0.0), za);
        let aa = rich(|n| parea(&polygon(pa, (0.0, 0.0), n)));
        for (nb, pb) in &shapes {
            for &d in &offsets {
                let ab = rich(|n| parea(&clip(&polygon(pa, (0.0, 0.0), n), &polygon(pb, d, n))).max(0.0));
                let bb = rich(|n| parea(&polygon(pb, d, n)));
                for &zb in &zbs {
                    let b = zprism(pb, d, zb);
                    let (va, vb) = (aa * 2.0, bb * (zb.1 - zb.0));
                    let vi = ab * (za.1.min(zb.1) - za.0.max(zb.0)).max(0.0);
                    for decl in [false, true] {
                        let dd = if decl {
                            match find_flush_candidates(&a, &b, tol) {
                                Ok(f) => Some(declare_all(&f)),
                                Err(_) => continue,
                            }
                        } else {
                            None
                        };
                        for (op, want) in [("U", va + vb - vi), ("S", va - vi), ("I", vi)] {
                            let res = match (&dd, op) {
                                (None, "U") => topo::union(&a, &b, tol),
                                (None, "S") => topo::subtract(&a, &b, tol),
                                (None, _) => topo::intersect(&a, &b, tol),
                                (Some(x), "U") => topo::union_with(&a, &b, x, tol),
                                (Some(x), "S") => topo::subtract_with(&a, &b, x, tol),
                                (Some(x), _) => topo::intersect_with(&a, &b, x, tol),
                            };
                            println!(
                                "R4031ARC {na} {nb} d={d:?} zb={zb:?} decl={decl} {op} => {}",
                                outcome(res, want, tol)
                            );
                        }
                    }
                }
            }
        }
    }
}

/// The frame admits "parallel" on ‖a₁×a₂‖ levered by the RADIUS; the
/// axis drift over a long rod is θ·L. Tilt angles straddling the frame's
/// zero band, rods long enough that θ·L clears it by orders.
#[test]
#[ignore = "review battery"]
fn r4031_lever() {
    let tol = Tol::witness();
    for half in [50.0, 200.0, 1000.0] {
        for theta in [0.0, 2e-10, 5e-10, 1e-9, 1.5e-9, 3e-9] {
            for (r, c) in [(0.2, 0.5), (0.3, 0.6), (0.5, 0.7)] {
                let a = rod(0.5, (0.0, 0.0), (-half - 1.0, half + 1.0));
                let b0 = rod(r, (c, 0.0), (-half, half));
                let m = Affine3::rotation_about_axis(Point3::new(c, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), theta);
                let b = if theta == 0.0 { b0 } else { topo::transform_rigid(&b0, &m, tol).unwrap() };
                let s = lens_area(0.5, c, r) * 2.0 * half;
                let va = PI * 0.25 * (2.0 * half + 2.0);
                let vb = PI * r * r * 2.0 * half;
                for (op, res, want) in runs(&a, &b, va, vb, s) {
                    let line = outcome(res, want, tol);
                    println!("R4031 lever half={half} theta={theta} r={r} c={c} drift={:e} {op} => {}", half * theta, line.chars().take(200).collect::<String>());
                }
            }
        }
    }
}

/// Smaller tilts on shorter rods: where does the arm stop building?
#[test]
#[ignore = "review battery"]
fn r4031_lever_small() {
    let tol = Tol::witness();
    for half in [0.5, 5.0, 50.0] {
        for theta in [1e-13, 1e-12, 1e-11, 1e-10] {
            let (r, c) = (0.3, 0.6);
            let a = rod(0.5, (0.0, 0.0), (-half - 1.0, half + 1.0));
            let b0 = rod(r, (c, 0.0), (-half, half));
            let m = Affine3::rotation_about_axis(Point3::new(c, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), theta);
            let b = topo::transform_rigid(&b0, &m, tol).unwrap();
            let s = lens_area(0.5, c, r) * 2.0 * half;
            let va = PI * 0.25 * (2.0 * half + 2.0);
            let vb = PI * r * r * 2.0 * half;
            for (op, res, want) in runs(&a, &b, va, vb, s) {
                let line = outcome(res, want, tol);
                println!("R4031 lsmall half={half} theta={theta} drift={:e} {op} => {}", half * theta, line.chars().take(160).collect::<String>());
            }
        }
    }
}
