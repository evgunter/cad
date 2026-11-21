//! **`patch_distance_sup` against the dense TRUE distance, randomized**
//! (a counterexample search, shape 1): every kind, near its locus and far
//! from it, across its singular sets (axis, apex, centre, the torus's
//! hole), under random rigid motions, on multi-span nets with varying
//! weights. Every bound must hold against the largest closed-form
//! distance over a dense grid of the patch, less the `f64` oracle's own
//! evaluation rounding. A NaN or `+∞` bound certifies nothing and is
//! sound. Seeded per run from the shared harness (`CAD_FUZZ_SEED`
//! replays one), its counts on the `CAD_FUZZ_EFFORT` dial.
#![allow(clippy::unwrap_used, clippy::panic, clippy::cast_precision_loss)]

test_utils::gated_to![
    "crates/geom-brep/src/patch_carrier.rs",
    "crates/geom-brep/src/pcurve_cache/projected.rs",
    "crates/geom-brep/src/curve_face/composite.rs",
    "crates/geom-core/src/spline/compose/patch.rs",
    "crates/geom-core/src/spline/compose/tensor.rs",
];

use geom::{NurbsSurface, Surface};
use geom_brep::patch_distance_sup;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

use test_utils::fuzz::Rng;

/// A random rotation (unit quaternion) and translation.
#[derive(Clone, Copy)]
struct Motion {
    q: [f64; 4],
    t: Vec3<f64>,
}
impl Motion {
    fn random(r: &mut Rng, reach: f64) -> Self {
        let mut q = [0.0; 4];
        loop {
            for x in &mut q {
                *x = r.range(-1.0, 1.0);
            }
            let n = q.iter().map(|x| x * x).sum::<f64>().sqrt();
            if n > 0.1 && n < 1.0 {
                for x in &mut q {
                    *x /= n;
                }
                break;
            }
        }
        Self {
            q,
            t: Vec3::new(
                r.range(-reach, reach),
                r.range(-reach, reach),
                r.range(-reach, reach),
            ),
        }
    }
    fn rot(&self, v: Vec3<f64>) -> Vec3<f64> {
        let [w, x, y, z] = self.q;
        let m = [
            [
                1.0 - 2.0 * (y * y + z * z),
                2.0 * (x * y - w * z),
                2.0 * (x * z + w * y),
            ],
            [
                2.0 * (x * y + w * z),
                1.0 - 2.0 * (x * x + z * z),
                2.0 * (y * z - w * x),
            ],
            [
                2.0 * (x * z - w * y),
                2.0 * (y * z + w * x),
                1.0 - 2.0 * (x * x + y * y),
            ],
        ];
        Vec3::new(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2] * v.z,
            m[1][0] * v.x + m[1][1] * v.y + m[1][2] * v.z,
            m[2][0] * v.x + m[2][1] * v.y + m[2][2] * v.z,
        )
    }
    fn point(&self, p: Point3<f64>) -> Point3<f64> {
        Point3::origin() + self.rot(p - Point3::origin()) + self.t
    }
    fn surface(&self, s: &Surface<f64>) -> Surface<f64> {
        match *s {
            Surface::Plane {
                origin,
                normal,
                u_ref,
            } => Surface::Plane {
                origin: self.point(origin),
                normal: self.rot(normal),
                u_ref: self.rot(u_ref),
            },
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Surface::Cylinder {
                origin: self.point(origin),
                axis: self.rot(axis),
                radius,
                u_ref: self.rot(u_ref),
            },
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Surface::Sphere {
                center: self.point(center),
                radius,
                axis: self.rot(axis),
                u_ref: self.rot(u_ref),
            },
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Surface::Cone {
                apex: self.point(apex),
                axis: self.rot(axis),
                half_angle,
                u_ref: self.rot(u_ref),
            },
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Surface::Torus {
                center: self.point(center),
                axis: self.rot(axis),
                major_radius,
                minor_radius,
                u_ref: self.rot(u_ref),
            },
            ref o => panic!("{o:?}"),
        }
    }
}

fn rho_h(p: Point3<f64>, o: Point3<f64>, a: Vec3<f64>) -> (f64, f64) {
    let d = p - o;
    let h = d.dot(a);
    ((d - a * h).norm(), h)
}

fn true_distance(surface: &Surface<f64>, p: Point3<f64>) -> f64 {
    match *surface {
        Surface::Plane { origin, normal, .. } => (p - origin).dot(normal).abs(),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => (rho_h(p, origin, axis).0 - radius).abs(),
        Surface::Sphere { center, radius, .. } => ((p - center).norm() - radius).abs(),
        Surface::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => {
            let (rho, h) = rho_h(p, apex, axis);
            let (s, c) = half_angle.sin_cos();
            let ray = |dir: (f64, f64)| {
                let t = rho * dir.0 + h * dir.1;
                if t <= 0.0 {
                    rho.hypot(h)
                } else {
                    (rho * dir.1 - h * dir.0).abs()
                }
            };
            ray((s, c)).min(ray((s, -c)))
        }
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => {
            let (rho, h) = rho_h(p, center, axis);
            ((rho - major_radius).hypot(h) - minor_radius).abs()
        }
        ref other => panic!("{other:?}"),
    }
}

fn dense_sup(patch: &NurbsSurface<f64>, surface: &Surface<f64>, n: u32) -> (f64, Point3<f64>) {
    let (u0, u1) = patch.knot_values_u().domain();
    let (v0, v1) = patch.knot_values_v().domain();
    let mut worst = (0.0_f64, Point3::origin());
    for i in 0..=n {
        for j in 0..=n {
            let u = u0 + (u1 - u0) * f64::from(i) / f64::from(n);
            let v = v0 + (v1 - v0) * f64::from(j) / f64::from(n);
            let p = patch.eval(u, v);
            let d = true_distance(surface, p);
            if d > worst.0 {
                worst = (d, p);
            }
        }
    }
    worst
}

/// A surface of revolution about `+z` through the origin: `profile` in
/// `(ρ, z)` with its own knots/weights, swept through `turns` quarter
/// turns (multi-span in `v` when `turns > 1`).
fn revolved(
    profile: &[(f64, f64)],
    profile_w: &[f64],
    pknots: Vec<f64>,
    pdeg: usize,
    turns: usize,
    start: f64,
) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(pknots, pdeg).unwrap();
    let mut kv = vec![0.0; 3];
    for t in 1..turns {
        kv.push(t as f64);
        kv.push(t as f64);
    }
    kv.extend([turns as f64; 3]);
    let kv = KnotVector::clamped(kv, 2).unwrap();
    let w = core::f64::consts::FRAC_1_SQRT_2;
    let mut turn: Vec<((f64, f64), f64)> = Vec::new();
    for t in 0..turns {
        let a0 = start + (t as f64) * core::f64::consts::FRAC_PI_2;
        let am = a0 + core::f64::consts::FRAC_PI_4;
        let a1 = a0 + core::f64::consts::FRAC_PI_2;
        if t == 0 {
            turn.push(((a0.cos(), a0.sin()), 1.0));
        }
        turn.push((((am.cos()) / w, (am.sin()) / w), w));
        turn.push(((a1.cos(), a1.sin()), 1.0));
    }
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for (&(rho, z), &wp) in profile.iter().zip(profile_w) {
        for &((c, s), wt) in &turn {
            control.push(Point3::new(rho * c, rho * s, z));
            weights.push(wp * wt);
        }
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

fn frame_z() -> (Vec3<f64>, Vec3<f64>) {
    (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0))
}

fn perturbed(p: &NurbsSurface<f64>, r: &mut Rng, dx: f64, dw: f64) -> NurbsSurface<f64> {
    let control: Vec<Point3<f64>> = p
        .control()
        .iter()
        .map(|c| {
            Point3::new(
                c.x + r.range(-dx, dx),
                c.y + r.range(-dx, dx),
                c.z + r.range(-dx, dx),
            )
        })
        .collect();
    let weights: Vec<f64> = p
        .weights()
        .iter()
        .map(|w| w * (1.0 + r.range(-dw, dw)))
        .collect();
    NurbsSurface::new(
        p.knot_values_u().clone(),
        p.knot_values_v().clone(),
        control,
        weights,
    )
    .unwrap()
}

fn moved(p: &NurbsSurface<f64>, m: &Motion) -> NurbsSurface<f64> {
    NurbsSurface::new(
        p.knot_values_u().clone(),
        p.knot_values_v().clone(),
        p.control().iter().map(|&c| m.point(c)).collect(),
        p.weights().to_vec(),
    )
    .unwrap()
}

/// A fully random rational net, bidegree (du, dv), spans (su, sv), in a
/// box of half-size `size` about `centre`.
fn random_net(r: &mut Rng, centre: Point3<f64>, size: f64) -> NurbsSurface<f64> {
    let du = 1 + r.below(3);
    let dv = 1 + r.below(3);
    let su = 1 + r.below(3);
    let sv = 1 + r.below(3);
    let kn = |d: usize, s: usize, r: &mut Rng| {
        let mut k = vec![0.0; d + 1];
        let mut acc = 0.0;
        let mut inner = Vec::new();
        for _ in 1..s {
            acc += r.range(0.2, 1.0);
            inner.push(acc);
        }
        let total = acc + r.range(0.2, 1.0);
        k.extend(inner);
        k.extend(vec![total; d + 1]);
        KnotVector::clamped(k, d).unwrap()
    };
    let ku = kn(du, su, r);
    let kv = kn(dv, sv, r);
    let nu = du + su;
    let nv = dv + sv;
    let mut control = Vec::new();
    let mut weights = Vec::new();
    for _ in 0..nu * nv {
        control.push(Point3::new(
            centre.x + r.range(-size, size),
            centre.y + r.range(-size, size),
            centre.z + r.range(-size, size),
        ));
        weights.push(r.range(0.2, 3.0));
    }
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

struct Tally {
    cases: usize,
    nan: usize,
    inf: usize,
    worst_ratio: f64,
}

fn check(t: &mut Tally, what: &str, patch: &NurbsSurface<f64>, surface: &Surface<f64>) {
    let bound = patch_distance_sup(patch, surface);
    let (truth, at) = dense_sup(patch, surface, 40);
    t.cases += 1;
    if bound.is_nan() {
        t.nan += 1;
        return;
    }
    if bound.is_infinite() {
        t.inf += 1;
        return;
    }
    // The f64 oracle's own evaluation rounding at the patch's scale.
    let scale = patch
        .control()
        .iter()
        .map(|c| c.x.abs().max(c.y.abs()).max(c.z.abs()))
        .fold(1.0_f64, f64::max);
    let slack = 64.0 * f64::EPSILON * scale;
    assert!(
        bound >= truth - slack,
        "{what}: bound {bound:e} < truth {truth:e} at {at:?}\nsurface {surface:?}\npatch \
         {patch:?}\n{}",
        test_utils::fuzz::replay()
    );
    if truth > 0.0 {
        t.worst_ratio = t.worst_ratio.max(bound / truth);
    }
}

fn kinds(r: &mut Rng) -> Vec<(&'static str, Surface<f64>)> {
    let (axis, u_ref) = frame_z();
    vec![
        (
            "plane",
            Surface::Plane {
                origin: Point3::new(0.0, 0.0, r.range(-0.5, 0.5)),
                normal: axis,
                u_ref,
            },
        ),
        (
            "cylinder",
            Surface::Cylinder {
                origin: Point3::origin(),
                axis,
                radius: r.range(0.05, 2.0),
                u_ref,
            },
        ),
        (
            "sphere",
            Surface::Sphere {
                center: Point3::origin(),
                radius: r.range(0.05, 2.0),
                axis,
                u_ref,
            },
        ),
        (
            "cone",
            Surface::Cone {
                apex: Point3::origin(),
                axis,
                half_angle: r.range(0.02, 1.5),
                u_ref,
            },
        ),
        ("torus", {
            let minor = r.range(0.05, 1.0);
            Surface::Torus {
                center: Point3::origin(),
                axis,
                major_radius: minor + r.range(0.01, 2.0),
                minor_radius: minor,
                u_ref,
            }
        }),
    ]
}

/// Random nets anywhere around each kind's singular set (axis, apex,
/// centre, the torus's hole) and across its locus, f of both signs.
#[test]
fn random_nets_around_every_kind_are_bounded() {
    let mut r = test_utils::fuzz::start("patch_distance_sup_random_nets");
    let mut t = Tally {
        cases: 0,
        nan: 0,
        inf: 0,
        worst_ratio: 0.0,
    };
    for i in 0..test_utils::fuzz::scaled(60) {
        let ks = kinds(&mut r);
        let (what, s) = &ks[i % ks.len()];
        let size = [0.01, 0.1, 0.5, 2.0][r.below(4)];
        let c = Point3::new(r.range(-1.5, 1.5), r.range(-1.5, 1.5), r.range(-1.5, 1.5));
        let net = random_net(&mut r, c, size);
        let reach = [0.0, 1.0, 100.0][r.below(3)];
        let m = Motion::random(&mut r, reach);
        check(&mut t, what, &moved(&net, &m), &m.surface(s));
    }
    eprintln!(
        "random nets: {} cases, {} NaN, {} inf, worst bound/truth {:.3e}",
        t.cases, t.nan, t.inf, t.worst_ratio
    );
}

/// Exact and near-exact patches of each kind (multi-span in v, varied
/// profiles), perturbed at several scales, moved rigidly.
#[test]
fn near_locus_patches_of_every_kind_are_bounded() {
    let mut r = test_utils::fuzz::start("patch_distance_sup_near_locus");
    let mut t = Tally {
        cases: 0,
        nan: 0,
        inf: 0,
        worst_ratio: 0.0,
    };
    let mut on_finite = [0usize; 5];
    let mut on_total = [0usize; 5];
    let w = core::f64::consts::FRAC_1_SQRT_2;
    for i in 0..test_utils::fuzz::scaled(40) {
        let (axis, u_ref) = frame_z();
        let kind = i % 5;
        let turns = 1 + r.below(4);
        let start = r.range(0.0, 6.0);
        let (patch, surface) = match kind {
            0 => {
                let rad = r.range(0.05, 3.0);
                let (z0, z1) = (r.range(-1.0, 1.0), r.range(1.2, 2.0));
                (
                    revolved(
                        &[(rad, z0), (rad, z1)],
                        &[1.0, 1.0],
                        vec![0.0, 0.0, 1.0, 1.0],
                        1,
                        turns,
                        start,
                    ),
                    Surface::Cylinder {
                        origin: Point3::origin(),
                        axis,
                        radius: rad,
                        u_ref,
                    },
                )
            }
            1 => {
                let a = r.range(0.02, 1.5);
                let (z0, z1) = if r.unit() < 0.3 {
                    (0.0, r.range(0.1, 2.0)) // to the apex
                } else if r.unit() < 0.5 {
                    (r.range(-2.0, -0.1), r.range(0.1, 2.0)) // through the apex
                } else {
                    (r.range(0.05, 1.0), r.range(1.1, 2.0))
                };
                let t = a.tan();
                // A straight generator: through the apex the profile's ρ
                // goes signed, mirroring onto the other nappe.
                (
                    revolved(
                        &[(z0 * t, z0), (z1 * t, z1)],
                        &[1.0, 1.0],
                        vec![0.0, 0.0, 1.0, 1.0],
                        1,
                        turns,
                        start,
                    ),
                    Surface::Cone {
                        apex: Point3::origin(),
                        axis,
                        half_angle: a,
                        u_ref,
                    },
                )
            }
            2 => {
                let rad = r.range(0.05, 3.0);
                // A meridian quarter, or a half in two spans.
                let (profile, pw, pk): (Vec<(f64, f64)>, Vec<f64>, Vec<f64>) = if r.unit() < 0.5 {
                    (
                        vec![(rad, 0.0), (rad, rad), (0.0, rad)],
                        vec![1.0, w, 1.0],
                        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
                    )
                } else {
                    (
                        vec![(0.0, -rad), (rad, -rad), (rad, 0.0), (rad, rad), (0.0, rad)],
                        vec![1.0, w, 1.0, w, 1.0],
                        vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0],
                    )
                };
                (
                    revolved(&profile, &pw, pk, 2, turns, start),
                    Surface::Sphere {
                        center: Point3::origin(),
                        radius: rad,
                        axis,
                        u_ref,
                    },
                )
            }
            3 => {
                let minor = r.range(0.02, 1.0);
                let major = minor + r.range(0.005, 2.0);
                let s0 = r.below(4) as f64 * core::f64::consts::FRAC_PI_2;
                let arc = |k: f64| {
                    let a = s0 + k * core::f64::consts::FRAC_PI_4;
                    let rr = if (k as i32) % 2 == 1 {
                        minor / w
                    } else {
                        minor
                    };
                    (major + rr * a.cos(), rr * a.sin())
                };
                (
                    revolved(
                        &[arc(0.0), arc(1.0), arc(2.0), arc(3.0), arc(4.0)],
                        &[1.0, w, 1.0, w, 1.0],
                        vec![0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 2.0, 2.0],
                        2,
                        turns,
                        start,
                    ),
                    Surface::Torus {
                        center: Point3::origin(),
                        axis,
                        major_radius: major,
                        minor_radius: minor,
                        u_ref,
                    },
                )
            }
            _ => {
                let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 2.0, 2.0, 2.0], 2).unwrap();
                let mut control = Vec::new();
                let mut weights = Vec::new();
                for iu in 0..4 {
                    for iv in 0..4 {
                        control.push(Point3::new(
                            f64::from(iu) + r.range(-0.3, 0.3),
                            f64::from(iv) + r.range(-0.3, 0.3),
                            0.0,
                        ));
                        weights.push(r.range(0.2, 3.0));
                    }
                }
                (
                    NurbsSurface::new(k.clone(), k, control, weights).unwrap(),
                    Surface::Plane {
                        origin: Point3::origin(),
                        normal: axis,
                        u_ref,
                    },
                )
            }
        };
        let reach = [0.0, 1.0, 300.0][r.below(3)];
        let m = Motion::random(&mut r, reach);
        let exact = moved(&patch, &m);
        let surf = m.surface(&surface);
        on_total[kind] += 1;
        let b = patch_distance_sup(&exact, &surf);
        if b.is_finite() && b < 1e-9 {
            on_finite[kind] += 1;
        }
        check(&mut t, "exact", &exact, &surf);
        for dx in [1e-9, 1e-6, 1e-3, 1e-1] {
            let dw = if r.unit() < 0.5 { 0.0 } else { 0.3 };
            let p = perturbed(&exact, &mut r, dx, dw);
            check(&mut t, "perturbed", &p, &surf);
        }
        // the carrier itself off by a little
        let off = match surf {
            Surface::Cylinder {
                origin,
                axis,
                radius,
                u_ref,
            } => Surface::Cylinder {
                origin,
                axis,
                radius: radius * (1.0 + r.range(-0.05, 0.05)),
                u_ref,
            },
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Surface::Cone {
                apex,
                axis,
                half_angle: half_angle * (1.0 + r.range(-0.05, 0.05)),
                u_ref,
            },
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Surface::Sphere {
                center: center + Vec3::new(r.range(-0.05, 0.05), 0.0, 0.0),
                radius,
                axis,
                u_ref,
            },
            Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius,
                u_ref,
            } => Surface::Torus {
                center,
                axis,
                major_radius,
                minor_radius: minor_radius * (1.0 + r.range(-0.05, 0.05)),
                u_ref,
            },
            other => other,
        };
        check(&mut t, "off carrier", &exact, &off);
    }
    eprintln!(
        "near-locus: {} cases, {} NaN, {} inf, worst bound/truth {:.3e}; exact finite<1e-9 per kind (cyl,cone,sph,tor,plane) {:?} of {:?}",
        t.cases, t.nan, t.inf, t.worst_ratio, on_finite, on_total
    );
}
