//! **The ladder is sound where the lane choice would not send it**
//! (`[verdicts.consumers]`, C5's conic × quadric lane choice): with the
//! choice forced to the ladder (`curve_face::pose::ladder_lane::forcing`), a second
//! harmonic in the zero band or at zero is read by the subdivision, and
//! no answer it gives contradicts the exact oracle
//! ([`crate::curve_face::conic_oracle::exact`]):
//!
//! - a `Miss` or a certified count stands only on a pose whose shallowest
//!   extreme the oracle reads outside the zero band, and only with the
//!   oracle's count, each root within the escalation band of a true one
//!   along the carrier;
//! - an `On` only where the carrier lies within the escalation band of
//!   the surface all round.
//!
//! The poses: circles within the gap of square to a wall, coaxial ones
//! included; near-circular ellipses on a sphere; a circle exactly square
//! to a wall, coaxial, at depths across the band; and circles nearly
//! square to a cone's axis, whose `F = Q/R` is the cone's, its
//! half-angle wide, narrow down to `1e-3` or near `π/2`, where the floor
//! on `|res|/|F|` falls toward zero.
//!
//! Each family must draw definite answers on a tenth of its poses at
//! least: a ladder that refused everything would contradict nothing.

test_utils::gated_to![
    "crates/geom-brep/src/curve_face/conic_quadric.rs",
    "crates/geom-brep/src/curve_face/pose.rs",
    "crates/geom-brep/src/curve_face/circle_roots.rs",
    "crates/geom-brep/src/half_angle.rs",
    "crates/geom-brep/src/curve_face/conic_oracle/",
    "crates/geom-brep/src/implicit.rs",
];

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use super::*;
use crate::curve_face::conic_oracle::{ArcTruth, arc_truth, exact};
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

const EPS_ROWS: [f64; 3] = [1e-6, 1e-9, 1e-12];

/// The unit-axis wall of radius `r` about the `z` axis.
fn wall(r: f64) -> geom::Surface<f64> {
    geom::Surface::Cylinder {
        frame: geom::Frame::new(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        radius: r,
    }
}

fn sphere(r: f64) -> geom::Surface<f64> {
    geom::Surface::Sphere {
        frame: geom::Frame::new(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        radius: r,
    }
}

/// The cone of half-angle `alpha` with its apex at the origin, opening
/// up the `z` axis.
fn cone(alpha: f64) -> geom::Surface<f64> {
    geom::Surface::Cone {
        frame: geom::Frame::new(
            Point3::origin(),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        half_angle: alpha,
    }
}

/// The unit `z` axis tilted by `tilt` toward the azimuth `toward`.
fn tilted(tilt: f64, toward: f64) -> Vec3<f64> {
    Vec3::new(
        tilt.sin() * toward.cos(),
        tilt.sin() * toward.sin(),
        tilt.cos(),
    )
}

/// The least and greatest `|residual|` of `c` against `s` sampled round
/// the turn, the residual the band classifies: `d + d²/2r` at a true
/// distance `d` from a sphere or a wall of radius `r`, and
/// `ρ cos α − |h| sin α` on a cone. The least is the depth of a constant
/// residual, at which the oracle finds no extreme ([`contradiction`]).
fn reach(c: &geom::Curve3<f64>, s: &geom::Surface<f64>) -> (f64, f64) {
    (0..4096)
        .map(|k| {
            let p = c.eval(TAU * f64::from(k) / 4096.0);
            match *s {
                geom::Surface::Cone { half_angle, .. } => {
                    p.x.hypot(p.y) * half_angle.cos() - p.z.abs() * half_angle.sin()
                }
                geom::Surface::Sphere { radius, .. } | geom::Surface::Cylinder { radius, .. } => {
                    let d = crate::curve_face::conic_oracle::distance(s, p);
                    d + d * d / (2.0 * radius)
                }
                _ => unreachable!("the row reads spheres, walls and cones"),
            }
            .abs()
        })
        .fold((f64::INFINITY, 0.0), |(lo, hi), x| (lo.min(x), hi.max(x)))
}

/// The arc along `c` between the parameters `a` and `b`, at its top
/// speed, the shorter way round.
fn arc(c: &geom::Curve3<f64>, a: f64, b: f64) -> f64 {
    let d = (a - b).rem_euclid(TAU);
    d.min(TAU - d) * c.conic().unwrap().speed_hi()
}

/// Why the forced ladder's answer `got` contradicts the oracle's `truth`
/// at `eps` (module docs), or `None`.
fn contradiction(
    got: &Result<Result<CircleRoots<f64>, Unsettled>, ConicQuadricRefusal>,
    on_arc: &ArcTruth,
    (c, span): (&geom::Curve3<f64>, (f64, f64)),
    (_, farthest): (f64, f64),
    eps: f64,
) -> Option<&'static str> {
    let escalate = 10.0 * eps;
    // The oracle's depth at the residual's extremes on the arc and its
    // ends. An arc whose end stands in band is the end's to read, and is
    // set aside.
    if on_arc.at_an_end {
        return None;
    }
    let nearest = on_arc.least;
    let truth = &on_arc.roots;
    let placed = |found: &[f64]| {
        found
            .iter()
            .copied()
            .filter(|&t| crate::curve_face::conic_oracle::on_arc(span, t))
            .collect::<Vec<_>>()
    };
    match got {
        Ok(Err(_)) | Err(ConicQuadricRefusal::Sphere(_) | ConicQuadricRefusal::Cylinder(_)) => None,
        Err(ConicQuadricRefusal::Cone(_)) => None,
        Err(ConicQuadricRefusal::Invariant(_)) => Some("a kernel fault"),
        Ok(Ok(CircleRoots::On(_))) => {
            (farthest > escalate).then_some("On, off the surface past the band")
        }
        Ok(Ok(CircleRoots::Miss)) => {
            if !truth.is_empty() {
                Some("a Miss on a true crossing")
            } else if nearest < eps {
                Some("a Miss on an extreme in the zero band")
            } else {
                None
            }
        }
        Ok(Ok(CircleRoots::Certified(found))) => {
            if nearest < eps {
                Some("a count on an extreme in the zero band")
            } else if placed(found.roots()).len() != truth.len() {
                Some("a count the oracle does not hold")
            } else if placed(found.roots())
                .iter()
                .any(|&t| truth.iter().all(|&truth_t| arc(c, t, truth_t) > escalate))
            {
                Some("a root past the band of every true one")
            } else {
                None
            }
        }
    }
}

/// One forced reading of `c` against `s` over a random arc, and its
/// complaint, if any, as a line.
#[allow(clippy::too_many_arguments)]
fn judge(
    rng: &mut fuzz::Rng,
    (what, definite): (&str, &mut usize),
    c: &geom::Curve3<f64>,
    s: &geom::Surface<f64>,
    eps: f64,
    band: Band,
) -> Option<String> {
    let (nearest, farthest) = reach(c, s);
    let truth = exact::truth(c, s);
    let (m, h) = (rng.range(-PI, PI), rng.range(0.01, PI));
    let got = crate::curve_face::pose::ladder_lane::forcing(|| {
        conic_quadric_roots(c, m - h, m + h, s, band)
    });
    if matches!(got, Ok(Ok(CircleRoots::Miss | CircleRoots::Certified(_)))) {
        *definite += 1;
    }
    let on_arc = arc_truth(
        |t| crate::implicit_residual(s, c.eval(t)),
        (m - h, m + h),
        &truth.roots,
        10.0 * eps,
    );
    let why = contradiction(&got, &on_arc, (c, (m - h, m + h)), (nearest, farthest), eps)?;
    Some(format!(
        "{what}, ε {eps:e}: {why}: {got:?}\n  truth: {} roots {:?}, depth {:.3e}, \
         residual depth {:.3e}, sampled {nearest:.3e} to {farthest:.3e}\n  {c:?}\n  {s:?}",
        truth.roots.len(),
        truth.roots,
        truth.depth,
        truth.residual_depth,
    ))
}

/// A depth drawn inside the zero band, inside the escalation gap, past
/// it, or zero, on either side.
fn depth(rng: &mut fuzz::Rng, i: usize, eps: f64) -> f64 {
    let side = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
    side * match i % 4 {
        0 => rng.range(0.0, eps),
        1 => rng.range(eps, 10.0 * eps),
        2 => rng.range(10.0 * eps, 100.0 * eps),
        _ => 0.0,
    }
}

/// **The forced ladder contradicts no oracle** (module docs), on every
/// pose family, at three bands.
#[test]
fn the_forced_ladder_contradicts_no_oracle() {
    let mut rng = fuzz::start("conic_quadric_ladder_lane");
    let n = fuzz::scaled(60);
    let mut wrong = Vec::new();
    // The definite answers (a `Miss`, a certified count) each family
    // drew: a ladder that refused everything would contradict nothing.
    let mut definite = [0usize; 4];
    for eps in EPS_ROWS {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for i in 0..4 * n {
            // A circle within the gap of square to a wall: its second
            // harmonic `(ρ sin α)²/4r` drawn in `[0, Kε]`, its centre on
            // the axis (a coaxial rim) or off it by a reach that grazes,
            // crosses or clears.
            let r = rng.log_uniform(0.01, 100.0);
            let rho = r * rng.log_uniform(0.05, 1.0);
            let a2 = rng.range(0.0, 10.0 * eps);
            let tilt = ((4.0 * r * a2).sqrt() / rho).min(1.0).asin();
            let normal = tilted(tilt, rng.range(0.0, TAU));
            let z = rng.range(-r, r);
            let d = depth(&mut rng, i, eps);
            let off = match i % 3 {
                0 => 0.0,
                1 => r - rho + d,
                _ => rng.range(0.0, r + rho),
            };
            let (off, rho) = if off == 0.0 { (0.0, r + d) } else { (off, rho) };
            let az = rng.range(0.0, TAU);
            let center = Point3::new(off * az.cos(), off * az.sin(), z);
            let u = normal.orthonormal_basis().0;
            let c = geom::Curve3::circle(center, normal, rho, u);
            let s = wall(r);
            wrong.extend(judge(
                &mut rng,
                ("a circle near square to a wall", &mut definite[0]),
                &c,
                &s,
                eps,
                band,
            ));

            // A circle exactly square to a wall, coaxial, at a depth
            // across the band: a constant residual.
            let rim = geom::Curve3::circle(
                Point3::new(0.0, 0.0, z),
                Vec3::new(0.0, 0.0, 1.0),
                r + d,
                Vec3::new(1.0, 0.0, 0.0),
            );
            wrong.extend(judge(
                &mut rng,
                ("a rim of a wall", &mut definite[1]),
                &rim,
                &s,
                eps,
                band,
            ));

            // A near-circular ellipse on a sphere: `|a² − b²|/4R` drawn in
            // `[0, Kε]`, in a plane at a height `h` off the centre, its
            // centre on the normal through it or off it.
            let big = rng.log_uniform(0.01, 100.0);
            let h = big * rng.range(-0.95, 0.95);
            let section = (big * big - h * h).sqrt();
            let a = section + d;
            let b = (a * a - 4.0 * big * rng.range(0.0, 10.0 * eps))
                .max(0.0)
                .sqrt();
            let normal = tilted(rng.range(0.0, PI), rng.range(0.0, TAU));
            let (e1, e2) = normal.orthonormal_basis();
            let shift = if i % 2 == 0 {
                0.0
            } else {
                rng.range(0.0, section)
            };
            let spin = rng.range(0.0, TAU);
            let e = geom::Curve3::Ellipse(geom::Ellipse::new(
                geom::Frame::new(
                    Point3::origin() + normal * h + e1 * shift,
                    normal,
                    e1 * spin.cos() + e2 * spin.sin(),
                ),
                a,
                b,
            ));
            let s = sphere(big);
            wrong.extend(judge(
                &mut rng,
                ("a near-circular ellipse on a sphere", &mut definite[2]),
                &e,
                &s,
                eps,
                band,
            ));

            // A circle nearly square to a cone's axis, its centre on the
            // axis or off it, at a height off the apex: the cone's `F`.
            // Its half-angle wide, narrow down to `1e-3` (the floor on
            // `|res|/|F|` about `sin 2α`), or near `π/2`.
            let alpha = match i % 3 {
                0 => rng.range(0.1, 1.4),
                1 => rng.log_uniform(1e-3, 0.1),
                _ => FRAC_PI_2 - rng.log_uniform(1e-3, 0.1),
            };
            let height = rng.log_uniform(0.01, 10.0);
            let reach = height * alpha.tan();
            let tilt = rng.log_uniform(1e-12, 1e-3);
            let normal = tilted(tilt, rng.range(0.0, TAU));
            let off = if i % 2 == 0 {
                0.0
            } else {
                rng.range(0.0, reach)
            };
            let az = rng.range(0.0, TAU);
            let c = geom::Curve3::circle(
                Point3::new(off * az.cos(), off * az.sin(), height),
                normal,
                (reach - off + d).max(reach * 1e-3),
                normal.orthonormal_basis().0,
            );
            wrong.extend(judge(
                &mut rng,
                ("a circle near square to a cone", &mut definite[3]),
                &c,
                &cone(alpha),
                eps,
                band,
            ));
        }
    }
    let drawn = 4 * n * EPS_ROWS.len();
    eprintln!("definite answers per family, of {drawn}: {definite:?}");
    // A tenth of each family's draws: at effort 1 the fewest a family
    // drew was 18% (the cones), the most 72%.
    assert!(
        definite.iter().all(|&k| 10 * k >= drawn),
        "a family drew under a tenth definite answers, of {drawn}: {definite:?} ({})",
        fuzz::replay()
    );
    assert!(
        wrong.is_empty(),
        "{} forced-ladder answers the oracle contradicts ({}):\n{}",
        wrong.len(),
        fuzz::replay(),
        wrong.join("\n")
    );
}
