//! Review probe for PR 3967 (an in-band schedule arm abandons its
//! member): `point_in_loop` on prisms tilted so a face's normal sits a
//! hair off a schedule member, queried at points a few bands from an
//! edge or vertex, against an exact-in-the-local-frame oracle. Run on
//! the merge-base and on the head; `PIL_PROBE_OUT` names the file each
//! run writes its per-case outcomes to, for a diff.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use geom_core::{Point3, Tol, Vec3};
use topo::{LoopContainment, PointInLoopError, point_in_loop};

type V = [f64; 3];

fn add(a: V, b: V) -> V {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sc(a: V, s: f64) -> V {
    [a[0] * s, a[1] * s, a[2] * s]
}
fn dot(a: V, b: V) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn unit(a: V) -> V {
    sc(a, 1.0 / dot(a, a).sqrt())
}

const SCHEDULE: [V; 16] = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0],
    [0.5, 0.25, 1.0],
    [1.0, 0.5, 0.25],
    [0.25, 1.0, 0.5],
    [-0.5, 1.0, 0.125],
    [0.125, -0.5, 1.0],
    [1.0, 0.125, -0.5],
    [0.75, -1.0, 0.375],
    [0.375, 0.75, -1.0],
    [-1.0, 0.375, 0.75],
    [0.625, 0.9375, 0.3125],
    [0.3125, -0.625, 0.9375],
    [0.9375, 0.3125, -0.625],
    [-0.75, -0.25, 1.0],
];

/// Signed distance of `p` to the simple polygon `poly` (positive
/// inside), in f64 in the polygon's own frame.
fn signed_dist(poly: &[(f64, f64)], p: (f64, f64)) -> f64 {
    let n = poly.len();
    let mut d = f64::INFINITY;
    let mut inside = false;
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        let (ex, ey) = (b.0 - a.0, b.1 - a.1);
        let t = (((p.0 - a.0) * ex + (p.1 - a.1) * ey) / (ex * ex + ey * ey)).clamp(0.0, 1.0);
        let (fx, fy) = (a.0 + t * ex - p.0, a.1 + t * ey - p.1);
        d = d.min((fx * fx + fy * fy).sqrt());
        if (a.1 > p.1) != (b.1 > p.1) && p.0 < a.0 + (p.1 - a.1) / (b.1 - a.1) * ex {
            inside = !inside;
        }
    }
    if inside { d } else { -d }
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

#[test]
#[ignore = "review probe; run with --ignored"]
fn review3967_tilted_faces_near_their_boundary() {
    run::<f64>("PIL_PROBE_OUT");
}

#[test]
#[ignore = "review probe; run with --ignored"]
fn review3967_tilted_faces_near_their_boundary_interval() {
    run::<geom_core::Interval>("PIL_PROBE_OUT_I");
}

fn run<T: geom_core::Decide + topo::AtRestPolicy>(env: &str) {
    let tol = Tol::witness();
    let band = geom_core::Band::linear(tol).unwrap();
    let mut out = String::new();
    let (mut wrong, mut answered, mut refused, mut total) = (0, 0, 0, 0);
    let profiles: [(&str, Vec<(f64, f64)>, f64); 4] = [
        ("square", vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0),
        ("sliver", vec![(0.0, 0.0), (1e-4, 0.0), (1e-4, 1e-4), (0.0, 1e-4)], 1e-4),
        ("thin", vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1e-4), (0.0, 1e-4)], 1.0),
        (
            "ell",
            vec![(0.0, 0.0), (2e-3, 0.0), (2e-3, 1e-4), (1e-4, 1e-4), (1e-4, 2e-3), (0.0, 2e-3)],
            1e-4,
        ),
    ];
    let phis: [f64; 10] = [0.0, 1e-12, 1e-10, 2e-9, 5e-9, 2e-8, 1e-7, 1e-6, 5.2e-5, 1e-3];
    let deltas = [0.0, 1e-11, 5e-10, 2e-9, 5e-9, 1.2e-8, 3e-8, 1e-7, 1e-5];
    let mut rng = Lcg(3967);
    for (pname, profile, h) in &profiles {
        for (k, r) in SCHEDULE.iter().enumerate() {
            for &phi in &phis {
                // which = 0: the cap normal near r; 1: the x = max wall's.
                for which in 0..2 {
                    // A hair off r: tilt r by phi about a random axis ⟂ r.
                    let rr = unit(*r);
                    let a = unit(cross(rr, unit([rng.next() - 0.5, rng.next() - 0.5, rng.next() - 0.5])));
                    let target = unit(add(sc(rr, phi.cos()), sc(a, phi.sin())));
                    let other = unit(cross(target, unit([rng.next() - 0.5, rng.next() - 0.5, rng.next() - 0.5])));
                    let third = cross(target, other);
                    // local (x, y, z) → world: cap normal = ez, wall normal = ex.
                    let (ex, ey, ez) = if which == 0 { (other, third, target) } else { (target, other, third) };
                    let off = [rng.next(), rng.next(), rng.next()];
                    let map = move |x: f64, y: f64, z: f64| -> V { add(off, add(add(sc(ex, x), sc(ey, y)), sc(ez, z))) };
                    let mut body = topo::Body::<T>::new();
                    let ops = common::prism_ops(
                        &mut body,
                        profile,
                        (0.0, *h),
                        |x, y, z| {
                            let w = map(x, y, z);
                            Point3::new(w[0], w[1], w[2]).map(T::from_f64)
                        },
                        common::FaceGeometry::Certified,
                        tol,
                    );
                    common::describe_as_intersections(&mut body, tol);
                    // Faces in their own 2-D frame: (polygon, frame → local 3-D, normal).
                    let mut faces: Vec<(String, topo::FaceKey, Vec<(f64, f64)>, Box<dyn Fn(f64, f64) -> V>, V)> = Vec::new();
                    faces.push(("bot".into(), ops.bottom.face, profile.clone(), Box::new(move |x, y| map(x, y, 0.0)), ez));
                    let hh = *h;
                    faces.push(("top".into(), ops.seed.face, profile.clone(), Box::new(move |x, y| map(x, y, hh)), ez));
                    let n = profile.len();
                    for (i, side) in ops.sides.iter().enumerate() {
                        let (p0, p1) = (profile[i], profile[(i + 1) % n]);
                        let len = ((p1.0 - p0.0).powi(2) + (p1.1 - p0.1).powi(2)).sqrt();
                        let (ux, uy) = ((p1.0 - p0.0) / len, (p1.1 - p0.1) / len);
                        let nn = add(sc(ex, uy), sc(ey, -ux));
                        faces.push((
                            format!("side{i}"),
                            side.face,
                            vec![(0.0, 0.0), (len, 0.0), (len, hh), (0.0, hh)],
                            Box::new(move |s, z| map(p0.0 + s * ux, p0.1 + s * uy, z)),
                            nn,
                        ));
                    }
                    for (fname, fk, poly, to3, nrm) in &faces {
                        let lp = body.get_face(*fk).unwrap().outer;
                        let normal = Vec3::new(nrm[0], nrm[1], nrm[2]).map(T::from_f64);
                        let m = poly.len();
                        let mut pts: Vec<(f64, f64)> = Vec::new();
                        for i in 0..m {
                            let (a, b) = (poly[i], poly[(i + 1) % m]);
                            let mid = (0.5 * (a.0 + b.0), 0.5 * (a.1 + b.1));
                            let (tx, ty) = (b.0 - a.0, b.1 - a.1);
                            let l = (tx * tx + ty * ty).sqrt();
                            let (nx, ny) = (ty / l, -tx / l);
                            for &d in &deltas {
                                for sgn in [-1.0, 1.0] {
                                    pts.push((mid.0 + sgn * d * nx, mid.1 + sgn * d * ny));
                                    pts.push((a.0 + sgn * d, a.1 + sgn * d));
                                    pts.push((a.0 + sgn * d, a.1 - sgn * d));
                                    // a few bands along the edge, off it by d
                                    pts.push((a.0 + 3e-8 * tx / l + sgn * d * nx, a.1 + 3e-8 * ty / l + sgn * d * ny));
                                }
                            }
                        }
                        for &p in &pts {
                            let sd = signed_dist(poly, p);
                            let w = to3(p.0, p.1);
                            let got = point_in_loop(&body, lp, normal, Point3::new(w[0], w[1], w[2]).map(T::from_f64), band);
                            total += 1;
                            let tag = match &got {
                                Ok(v) => {
                                    answered += 1;
                                    let bad = match v {
                                        LoopContainment::In => sd < -1e-13,
                                        LoopContainment::Out => sd > 1e-13,
                                        LoopContainment::OnBoundary => sd.abs() > 1.01e-8,
                                    };
                                    if bad {
                                        wrong += 1;
                                        println!("WRONG {pname} k{k} phi{phi:e} w{which} {fname} sd={sd:e}: {v:?}");
                                    }
                                    format!("{v:?}")
                                }
                                Err(PointInLoopError::Escalated { diag, .. }) => {
                                    refused += 1;
                                    format!("Esc({:?})", diag.predicate)
                                }
                                Err(e) => {
                                    refused += 1;
                                    let s = format!("{e:?}");
                                    s.split(' ').next().unwrap().to_string()
                                }
                            };
                            out.push_str(&format!(
                                "{pname} k{k} phi{phi:e} w{which} {fname} ({:e},{:e}) sd={sd:e} {tag}\n",
                                p.0, p.1
                            ));
                        }
                    }
                }
            }
        }
    }
    println!("total {total} answered {answered} refused {refused} wrong {wrong}");
    if let Ok(path) = std::env::var(env) {
        std::fs::write(path, out).unwrap();
    }
    assert_eq!(wrong, 0);
}
