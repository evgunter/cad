//! Review R1 probes of PR 5422 (the edge-meeting ray). Evidence, not
//! pins: `cargo nextest run -p topo --release --run-ignored only -E
//! 'test(edge_meeting_ray_review_r1)' --no-capture`, with
//! `CAD_TOLERANCE_EPS` set per run. Uses only API present on the merge
//! base too, so one file runs on base and head.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stdout)]

use geom_core::{Band, Point3, Tol};
use topo::Body;

fn rot(axis: [f64; 3], ang: f64) -> [[f64; 3]; 3] {
    let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
    let (s, c) = ang.sin_cos();
    let k = 1.0 - c;
    [
        [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
        [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
        [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
    ]
}

struct Pose {
    r: [[f64; 3]; 3],
    t: [f64; 3],
}

impl Pose {
    fn fwd(&self, p: [f64; 3]) -> [f64; 3] {
        let at = |i: usize| self.r[i][0] * p[0] + self.r[i][1] * p[1] + self.r[i][2] * p[2] + self.t[i];
        [at(0), at(1), at(2)]
    }
    fn back(&self, p: [f64; 3]) -> [f64; 3] {
        let d = [p[0] - self.t[0], p[1] - self.t[1], p[2] - self.t[2]];
        let at = |i: usize| self.r[0][i] * d[0] + self.r[1][i] * d[1] + self.r[2][i] * d[2];
        [at(0), at(1), at(2)]
    }
}

/// A right prism over `profile` (CCW), `z ∈ [0, h]`, placed by `pose`.
fn prism(profile: &[(f64, f64)], h: f64, pose: &Pose, tol: Tol) -> Body<f64> {
    let mut body = Body::<f64>::new();
    let _ = topo::test_support::prism_ops(
        &mut body,
        profile,
        (0.0, h),
        |x, y, z| {
            let p = pose.fwd([x, y, z]);
            Point3::new(p[0], p[1], p[2])
        },
        topo::test_support::FaceGeometry::Certified,
        tol,
    );
    topo::test_support::describe_as_intersections(&mut body, tol);
    body
}

fn seg_dist(p: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = dx * dx + dy * dy;
    let t = (((p.0 - a.0) * dx + (p.1 - a.1) * dy) / l2).clamp(0.0, 1.0);
    let (cx, cy) = (a.0 + t * dx - p.0, a.1 + t * dy - p.1);
    (cx * cx + cy * cy).sqrt()
}

fn in_poly(p: (f64, f64), poly: &[(f64, f64)]) -> bool {
    let mut inside = false;
    let n = poly.len();
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if (a.1 > p.1) != (b.1 > p.1) && p.0 < a.0 + (p.1 - a.1) * (b.0 - a.0) / (b.1 - a.1) {
            inside = !inside;
        }
    }
    inside
}

/// Truth in the prism's own frame: inside, and the distance to its
/// boundary.
fn truth(profile: &[(f64, f64)], h: f64, p: [f64; 3]) -> (bool, f64) {
    let xy = (p[0], p[1]);
    let n = profile.len();
    let dxy = (0..n)
        .map(|i| seg_dist(xy, profile[i], profile[(i + 1) % n]))
        .fold(f64::INFINITY, f64::min);
    let inp = in_poly(xy, profile);
    let inz = p[2] > 0.0 && p[2] < h;
    if inp && inz {
        (true, dxy.min(p[2]).min(h - p[2]))
    } else {
        let a = if inp { 0.0 } else { dxy };
        let b = if inz { 0.0 } else { (-p[2]).max(p[2] - h) };
        (false, (a * a + b * b).sqrt())
    }
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

fn cases(e: f64) -> Vec<(&'static str, Vec<(f64, f64)>, f64)> {
    let k = 10.0;
    vec![
        ("cube", vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0),
        ("L", vec![(0.0, 0.0), (1.0, 0.0), (1.0, 0.3), (0.3, 0.3), (0.3, 1.0), (0.0, 1.0)], 0.5),
        (
            "U",
            vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.8, 1.0), (0.8, 0.2), (0.2, 0.2), (0.2, 1.0), (0.0, 1.0)],
            0.4,
        ),
        (
            "near-collinear",
            vec![(0.0, 0.0), (0.5, 3.0e-7), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            0.7,
        ),
        (
            "near-collinear-in",
            vec![(0.0, 0.0), (0.5, -3.0e-7), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            0.7,
        ),
        ("sliver-wedge", vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0e4 * e)], 0.6),
        ("slab-25e", vec![(0.0, 0.0), (25.0 * e, 0.0), (25.0 * e, 1.0), (0.0, 1.0)], 1.0),
        ("needle-21e", vec![(0.0, 0.0), (21.0 * e, 0.0), (21.0 * e, 21.0 * e), (0.0, 21.0 * e)], 1.0),
        ("needle-12e", vec![(0.0, 0.0), (12.0 * e, 0.0), (12.0 * e, 12.0 * e), (0.0, 12.0 * e)], 1.0),
        (
            "thin-L-3K",
            vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (1.0, 3.0 * k * e),
                (3.0 * k * e, 3.0 * k * e),
                (3.0 * k * e, 1.0),
                (0.0, 1.0),
            ],
            0.5,
        ),
        (
            "trapezoid-needle",
            vec![(0.0, 0.0), (19.0 * e, 0.0), (15.0 * e, 14.0 * e), (4.0 * e, 14.0 * e)],
            1.0,
        ),
        (
            "star",
            vec![(1.0, 0.0), (0.3, 0.25), (0.3, 0.95), (-0.25, 0.4), (-0.8, 0.6), (-0.4, 0.0), (-0.8, -0.6), (-0.25, -0.4), (0.3, -0.95), (0.3, -0.25)],
            0.3,
        ),
    ]
}

/// `point_in_solid` against the prism's truth near its features, along
/// the axes and turned, both ways out. Any `In`/`Out` against the truth
/// is a wrong answer.
#[test]
#[ignore = "probe: run by hand"]
fn fuzz_point_in_solid_against_truth() {
    let tol = Tol::witness();
    let band = Band::linear(tol).expect("the run's band");
    let e = tol.eps();
    let ke = 10.0 * e;
    let poses = [
        Pose { r: rot([0.0, 0.0, 1.0], 0.0), t: [0.0; 3] },
        Pose { r: rot([37.0, -21.0, 13.0], 0.7), t: [0.3, -0.2, 0.5] },
        Pose { r: rot([1.0, 2.0, 3.0], 1.1), t: [0.1, 0.2, -0.3] },
    ];
    let filter: Option<String> = std::env::var("R1_CASE").ok();
    for (name, profile, h) in cases(e) {
        if filter.as_deref().is_some_and(|f| f != name) {
            continue;
        }
        for (pi, pose) in poses.iter().enumerate() {
            for inverted in [false, true] {
                let Ok(body) = std::panic::catch_unwind(|| prism(&profile, h, pose, tol)) else {
                    println!("R1PIS eps={e:e} {name} pose{pi}: BUILD PANIC");
                    continue;
                };
                let body = if inverted { body.revert().expect("reverts") } else { body };
                let mut rng = Lcg(0x5422 + pi as u64);
                let n = profile.len();
                let (mut wrong, mut read, mut refused, mut skipped) = (0, 0, 0, 0);
                let mut examples: Vec<String> = Vec::new();
                for i in 0..400 {
                    // A feature: a vertex, or a point along an edge (rim or strut).
                    let c = i % n;
                    let (a, b) = (profile[c], profile[(c + 1) % n]);
                    let s = rng.next();
                    let zsel = rng.next();
                    let z = if zsel < 0.35 { 0.0 } else if zsel < 0.7 { h } else { rng.next() * h };
                    let (x, y) = if rng.next() < 0.3 { a } else { (a.0 + s * (b.0 - a.0), a.1 + s * (b.1 - a.1)) };
                    let scale = [2.5, 4.0, 8.0, 20.0, 100.0, 1e4][(rng.next() * 6.0) as usize % 6] * ke;
                    let off = [rng.next() - 0.5, rng.next() - 0.5, rng.next() - 0.5];
                    let on = (off[0] * off[0] + off[1] * off[1] + off[2] * off[2]).sqrt();
                    let p = [x + off[0] / on * scale, y + off[1] / on * scale, z + off[2] / on * scale];
                    let (inside, dist) = truth(&profile, h, p);
                    if dist < 2.0 * ke {
                        skipped += 1;
                        continue;
                    }
                    let want = inside != inverted;
                    let w = pose.fwd(p);
                    let got = topo::point_in_solid(&body, Point3::new(w[0], w[1], w[2]), band, tol);
                    match got {
                        Ok(topo::SolidContainment::In) | Ok(topo::SolidContainment::Out) => {
                            read += 1;
                            let said = matches!(got, Ok(topo::SolidContainment::In));
                            if said != want {
                                wrong += 1;
                                if examples.len() < 3 {
                                    examples.push(format!("p={p:?} dist={dist:e} want_in={want} got={got:?}"));
                                }
                            }
                        }
                        _ => refused += 1,
                    }
                }
                let _ = pose.back([0.0; 3]);
                println!(
                    "R1PIS eps={e:e} {name} pose{pi} inv={inverted}: read {read} refused {refused} skipped {skipped} WRONG {wrong} {examples:?}"
                );
            }
        }
    }
}

/// The side read off each prism by the structural doors, both ways out:
/// any wrong orientation is a wrong answer.
#[test]
#[ignore = "probe: run by hand"]
fn structural_orientation_of_cases() {
    let tol = Tol::witness();
    let e = tol.eps();
    let poses = [
        Pose { r: rot([0.0, 0.0, 1.0], 0.0), t: [0.0; 3] },
        Pose { r: rot([37.0, -21.0, 13.0], 0.7), t: [0.3, -0.2, 0.5] },
        Pose { r: rot([1.0, 2.0, 3.0], 1.1), t: [0.1, 0.2, -0.3] },
    ];
    for (name, profile, h) in cases(e) {
        for (pi, pose) in poses.iter().enumerate() {
            let Ok(body) = std::panic::catch_unwind(|| prism(&profile, h, pose, tol)) else {
                println!("R1ORI eps={e:e} {name} pose{pi}: BUILD PANIC");
                continue;
            };
            let inv = body.revert().expect("reverts");
            println!(
                "R1ORI eps={e:e} {name} pose{pi}: plain {:?} | inverted {:?}",
                topo::validate_geometric_structural(&body, tol),
                topo::validate_geometric_structural(&inv, tol)
            );
        }
    }
}
