//! **No certified root of a shallow ladder crossing is placed past the
//! band.** Circles tilted to a wall, and ellipses against a sphere or a
//! wall, posed to cross the surface at a chosen depth (log-uniform from
//! `1e-11` to `1e-3` m, either side) at a random point, tangent there in
//! a random direction — the poses the ladder arm answers — decided at
//! `ε = 1e-6, 1e-9, 1e-12` on both lanes against the exact oracle
//! ([`crate::boolean::conic_oracle::exact`]). On every pose at least `Kε`
//! deep, a certified answer has the true count and each certified root
//! lies within `Kε` of arc of a true one and each true one within `Kε` of
//! a certified one; a `Miss` has no true root; nothing else is
//! certified. A refusal is always allowed.

test_utils::gated_to![
    "crates/topo/src/boolean/conic_quadric/mod.rs",
    "crates/topo/src/boolean/circle_roots.rs",
    "crates/topo/src/boolean/conic_oracle.rs",
    "crates/topo/src/boolean/conic_oracle/",
    "crates/topo/src/boolean/solid_contain.rs",
    "crates/geom-brep/src/implicit.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/running.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/interval.rs",
];

use core::f64::consts::{PI, TAU};

use super::*;
use crate::boolean::conic_oracle::{exact, unit};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz::{self, Rng};

fn log_uniform(rng: &mut Rng, lo: f64, hi: f64) -> f64 {
    10f64.powf(rng.range(lo.log10(), hi.log10()))
}

/// A pose: the carrier, its arc, the surface, and the carrier's top speed.
struct Pose {
    carrier: geom::Curve3<f64>,
    surface: geom::Surface<f64>,
    t0: f64,
    t1: f64,
    speed: f64,
}

/// A circle against a wall (`family` 0), an ellipse against a sphere (1)
/// or against a wall (2), through the point `depth` off the surface along
/// its normal and tangent there to it.
fn draw(rng: &mut Rng, family: usize) -> Pose {
    let o = Point3::new(
        rng.range(-3.0, 3.0),
        rng.range(-3.0, 3.0),
        rng.range(-3.0, 3.0),
    );
    let r = log_uniform(rng, 0.01, 10.0);
    let (surface, on, normal, tangent) = if family == 1 {
        let n = unit(rng);
        let t = n.cross(unit(rng)).normalize();
        let s = geom::Surface::Sphere {
            center: o,
            radius: r,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        (s, o + n * r, n, t)
    } else {
        let axis = unit(rng);
        let (e1, e2) = axis.orthonormal_basis();
        let phi = rng.range(0.0, TAU);
        let radial = e1 * phi.cos() + e2 * phi.sin();
        let along = e2 * phi.cos() - e1 * phi.sin();
        let g = rng.range(0.0, TAU);
        let s = geom::Surface::Cylinder {
            origin: o,
            axis,
            radius: r,
            u_ref: e1,
        };
        let on = o + radial * r + axis * rng.range(-2.0, 2.0);
        (
            s,
            on,
            radial,
            (along * g.cos() + axis * g.sin()).normalize(),
        )
    };
    let b = rng.range(0.0, TAU);
    let toward = (normal * b.cos() + normal.cross(tangent) * b.sin()).normalize();
    let major = log_uniform(rng, 0.01, 10.0);
    let minor = if family == 0 {
        major
    } else {
        major * rng.range(0.2, 0.999)
    };
    let depth = if rng.unit() < 0.5 { -1.0 } else { 1.0 } * log_uniform(rng, 1e-11, 1e-3);
    let center = on + normal * depth - toward * major;
    let axis = toward.cross(tangent).normalize();
    let carrier = if family == 0 {
        geom::Curve3::Circle {
            center,
            axis,
            radius: major,
            u_ref: toward,
        }
    } else {
        geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref: toward,
        }
    };
    let (m, h) = (rng.range(-PI, PI), rng.range(0.01, PI));
    Pose {
        carrier,
        surface,
        t0: m - h,
        t1: m + h,
        speed: major,
    }
}

fn lift_p(p: Point3<f64>) -> Point3<Interval> {
    Point3::new(
        Interval::from_f64(p.x),
        Interval::from_f64(p.y),
        Interval::from_f64(p.z),
    )
}

fn lift_v(v: Vec3<f64>) -> Vec3<Interval> {
    Vec3::new(
        Interval::from_f64(v.x),
        Interval::from_f64(v.y),
        Interval::from_f64(v.z),
    )
}

/// `pose` at the `Interval` scalar.
#[allow(clippy::panic)]
fn lifted(pose: &Pose) -> (geom::Curve3<Interval>, geom::Surface<Interval>) {
    let at = Interval::from_f64;
    let carrier = match pose.carrier {
        geom::Curve3::Circle {
            center,
            axis,
            radius,
            u_ref,
        } => geom::Curve3::Circle {
            center: lift_p(center),
            axis: lift_v(axis),
            radius: at(radius),
            u_ref: lift_v(u_ref),
        },
        geom::Curve3::Ellipse {
            center,
            axis,
            major,
            minor,
            u_ref,
        } => geom::Curve3::Ellipse {
            center: lift_p(center),
            axis: lift_v(axis),
            major: at(major),
            minor: at(minor),
            u_ref: lift_v(u_ref),
        },
        _ => panic!("the sweep draws circles and ellipses"),
    };
    let surface = match pose.surface {
        geom::Surface::Sphere {
            center,
            radius,
            axis,
            u_ref,
        } => geom::Surface::Sphere {
            center: lift_p(center),
            radius: at(radius),
            axis: lift_v(axis),
            u_ref: lift_v(u_ref),
        },
        geom::Surface::Cylinder {
            origin,
            axis,
            radius,
            u_ref,
        } => geom::Surface::Cylinder {
            origin: lift_p(origin),
            axis: lift_v(axis),
            radius: at(radius),
            u_ref: lift_v(u_ref),
        },
        _ => panic!("the sweep draws spheres and walls"),
    };
    (carrier, surface)
}

/// The door's certified roots as `(lo, hi)` pairs, or what it answered
/// instead.
fn answer<T: Decide + Bounds>(
    got: Result<CircleRoots<T>, BooleanError>,
) -> Result<Vec<(f64, f64)>, String> {
    match got {
        Ok(CircleRoots::Certified { count, thetas }) => {
            Ok(thetas[..count].iter().map(|t| (t.lo(), t.hi())).collect())
        }
        Ok(other) => Err(format!("{other:?}")),
        Err(e) => Err(format!("refused: {e:?}")),
    }
}

/// The arc length from `(lo, hi)` to `root` round the turn, at `speed`.
fn arc_off((lo, hi): (f64, f64), root: f64, speed: f64) -> f64 {
    let (mid, half) = ((lo + hi) / 2.0, (hi - lo) / 2.0);
    let turn = (mid - root).rem_euclid(TAU);
    (turn.min(TAU - turn) - half).max(0.0) * speed
}

#[test]
fn no_certified_root_of_a_shallow_ladder_crossing_is_placed_past_the_band() {
    let mut rng = fuzz::start("conic_quadric_shallow_sweep");
    let n = fuzz::scaled(40);
    let mut wrong = Vec::new();
    let mut judged = 0usize;
    for i in 0..3 * n {
        let pose = draw(&mut rng, i % 3);
        let truth = exact::truth(&pose.carrier, &pose.surface);
        let (carrier_i, surface_i) = lifted(&pose);
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            if truth.depth < 10.0 * eps {
                continue;
            }
            let at = Interval::from_f64;
            for (lane, got) in [
                (
                    "f64",
                    answer(conic_quadric_roots(
                        &pose.carrier,
                        pose.t0,
                        pose.t1,
                        &pose.surface,
                        band,
                    )),
                ),
                (
                    "Interval",
                    answer(conic_quadric_roots(
                        &carrier_i,
                        at(pose.t0),
                        at(pose.t1),
                        &surface_i,
                        band,
                    )),
                ),
            ] {
                judged += 1;
                let why = match &got {
                    Ok(roots) if roots.len() != truth.roots.len() => Some(format!(
                        "certified {} roots, the truth has {}",
                        roots.len(),
                        truth.roots.len()
                    )),
                    Ok(roots) => {
                        let worst = roots
                            .iter()
                            .map(|&r| {
                                truth
                                    .roots
                                    .iter()
                                    .map(|&t| arc_off(r, t, pose.speed))
                                    .fold(f64::INFINITY, f64::min)
                            })
                            .chain(truth.roots.iter().map(|&t| {
                                roots
                                    .iter()
                                    .map(|&r| arc_off(r, t, pose.speed))
                                    .fold(f64::INFINITY, f64::min)
                            }))
                            .fold(0.0, f64::max);
                        (worst > 10.0 * eps).then(|| format!("a root {worst:.3e} m from the truth"))
                    }
                    Err(what) if what == "Miss" => (!truth.roots.is_empty())
                        .then(|| format!("Miss on {} true roots", truth.roots.len())),
                    Err(what) if what == "Uncertain" || what.starts_with("refused") => None,
                    Err(what) => Some(format!("certified {what}")),
                };
                if let Some(why) = why {
                    wrong.push(format!(
                        "{lane} at ε {eps:e}, depth {:.3e}: {why}\n  {:?} {:?} [{}, {}]",
                        truth.depth, pose.carrier, pose.surface, pose.t0, pose.t1
                    ));
                }
            }
        }
    }
    println!("{judged} answers judged outside the band");
    assert!(
        wrong.is_empty(),
        "{} wrong answers outside the band ({}):\n{}",
        wrong.len(),
        fuzz::replay(),
        wrong.join("\n")
    );
}
