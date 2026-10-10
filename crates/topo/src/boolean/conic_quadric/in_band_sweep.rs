//! **The half-angle door answers no pose inside the band, and places
//! every root it certifies within the zero band.** Circles tilted to a
//! cylinder wall — the poses the ladder arm takes — decided at
//! `ε = 1e-6, 1e-9, 1e-12` against the exact oracle
//! ([`crate::boolean::conic_oracle::exact`]):
//!
//! - a circle tangent to the wall, or crossing or clearing it by less
//!   than `Kε`, at any tilt, is escalated or `Uncertain`, never a `Miss`
//!   or a certified count (D1's trilean), and `OnSurface` only where it
//!   lies on the wall to within the band all round;
//! - a circle of radius 1500 against a unit wall, whose harmonics' noise
//!   reads in the band's escalation gap at the default band, has each
//!   certified root within the zero band of a true one, and no `Miss` on
//!   a true crossing.

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
];

use core::f64::consts::{PI, TAU};

use super::*;
use crate::boolean::conic_oracle::exact;
use geom_core::{Point3, Vec3};
use test_utils::fuzz::{self, Rng};

const EPS_ROWS: [f64; 3] = [1e-6, 1e-9, 1e-12];

fn log_uniform(rng: &mut Rng, lo: f64, hi: f64) -> f64 {
    10f64.powf(rng.range(lo.log10(), hi.log10()))
}

/// The unit-axis wall of radius `r` about the `z` axis.
fn wall(r: f64) -> geom::Surface<f64> {
    geom::Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        radius: r,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// A circle of radius `rho` through the point `depth` off the wall of
/// radius `r` along its normal at `(r, 0, 0)`, tangent there in the
/// direction `ŷ cos g + ẑ sin g`, its centre toward
/// `x̂ cos b + (x̂ × t̂) sin b` from that point: `b` and `g` set its tilt
/// to the wall's axis, from square (`g = 0`, `b = 0`) to through it.
fn tangent_circle(r: f64, rho: f64, depth: f64, g: f64, b: f64) -> geom::Curve3<f64> {
    let normal = Vec3::new(1.0, 0.0, 0.0);
    let tangent = Vec3::new(0.0, g.cos(), g.sin());
    let toward = (normal * b.cos() + normal.cross(tangent) * b.sin()).normalize();
    geom::Curve3::Circle {
        center: Point3::new(r, 0.0, 0.0) + normal * depth - toward * rho,
        axis: toward.cross(tangent).normalize(),
        radius: rho,
        u_ref: toward,
    }
}

/// The arc length between the parameters `a` and `b` round the turn.
fn arc_between(a: f64, b: f64, rho: f64) -> f64 {
    let d = (a - b).rem_euclid(TAU);
    d.min(TAU - d) * rho
}

/// The carrier's farthest distance from the wall of radius `r`, sampled
/// round the turn.
fn farthest(c: &geom::Curve3<f64>, r: f64) -> f64 {
    (0..4096)
        .map(|k| {
            let p = c.eval(TAU * f64::from(k) / 4096.0);
            (p.x.hypot(p.y) - r).abs()
        })
        .fold(0.0, f64::max)
}

/// The harmonics' noise as the ladder's meter reads it (metres).
fn noise_reading(c: &geom::Curve3<f64>, r: f64) -> f64 {
    let conic = geom_brep::Conic::of(c).unwrap();
    let h =
        geom_brep::conic_cylinder_harmonics(&conic, Point3::origin(), Vec3::new(0.0, 0.0, 1.0), r);
    rounding_charge(h.terms) / h.per / h.floor
}

/// **No in-band pose is answered** (module docs). Each pose crosses,
/// grazes or clears the wall by a depth drawn inside the zero band,
/// inside its escalation gap, or zero (an exact tangency), at a tilt
/// drawn over the whole turn — from square to the axis, which the
/// first-harmonic arm takes, through every tilt the ladder arm takes —
/// and is judged where the exact oracle reads its shallowest extreme
/// inside `Kε`. The ladder alone certified a `Miss` on crossings up to
/// `0.41ε` deep and on exact tangencies tilted past `1.01ε`.
#[test]
fn no_pose_inside_the_band_is_answered() {
    let mut rng = fuzz::start("conic_quadric_in_band_sweep");
    let n = fuzz::scaled(100);
    let mut wrong = Vec::new();
    for eps in EPS_ROWS {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for i in 0..3 * n {
            let r = log_uniform(&mut rng, 0.01, 100.0);
            let rho = log_uniform(&mut rng, 0.01, 2000.0);
            let (g, b) = (rng.range(0.0, TAU), rng.range(0.0, TAU));
            let side = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
            let depth = match i % 3 {
                0 => side * rng.range(0.0, eps),
                1 => side * rng.range(eps, 10.0 * eps),
                _ => 0.0,
            };
            let c = tangent_circle(r, rho, depth, g, b);
            let truth = exact::truth(&c, &wall(r));
            if truth.depth >= 10.0 * eps {
                continue;
            }
            let (m, h) = (rng.range(-PI, PI), rng.range(0.01, PI));
            let got = conic_quadric_roots(&c, m - h, m + h, &wall(r), band);
            match got {
                Ok(CircleRoots::Uncertain) | Err(_) => continue,
                Ok(CircleRoots::OnSurface) if farthest(&c, r) <= 10.0 * eps => continue,
                _ => {}
            }
            wrong.push(format!(
                "ε {eps:e}, depth {depth:e} (true {:.3e}, {} true roots): {got:?}\n  {c:?} wall r {r}",
                truth.depth,
                truth.roots.len()
            ));
        }
    }
    assert!(
        wrong.is_empty(),
        "{} in-band poses answered ({}):\n{}",
        wrong.len(),
        fuzz::replay(),
        wrong.join("\n")
    );
}

/// **An exact tangency is escalated at every tilt** (an edge table, so
/// no search): a circle touching a wall at `(r, 0, 0)` — from outside,
/// from inside, and crossing its normal section — its plane tilted to
/// square by an arc `s = ρ·sin α` from just past the zero band
/// (`1.01ε`, where the ladder alone certified `Miss`) through to a
/// circle whose plane holds the wall's axis. Square to the axis the
/// first-harmonic arm escalates it on the exact range; tilted, the
/// subdivision splits the piece holding the double root down to the
/// band's width and answers `Uncertain`.
#[test]
fn an_exact_tangency_is_escalated_at_every_tilt() {
    let mut wrong = Vec::new();
    for eps in EPS_ROWS {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for (r, rho) in [
            (1.0, 2.0),
            (1.0, 0.3),
            (5.0, 50.0),
            (0.1, 3.0),
            (1.0, 1500.0),
        ] {
            for s in [1.01, 10.0, 1e3, 1e6]
                .map(|k| k * eps)
                .into_iter()
                .chain([0.1 * rho, rho])
            {
                let tilt = (s / rho).min(1.0).asin();
                for b in [0.0, PI, 0.5 * PI] {
                    let c = tangent_circle(r, rho, 0.0, tilt, b);
                    let got = conic_quadric_roots(&c, -1.0, 1.0, &wall(r), band);
                    if !matches!(got, Ok(CircleRoots::Uncertain) | Err(_)) {
                        wrong.push(format!(
                            "ε {eps:e}, wall r {r}, ρ {rho}, tilt arc {s:e}, b {b}: {got:?}"
                        ));
                    }
                }
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "{} tangencies answered:\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

/// **The ρ = 1500 family reads its noise in the band's escalation gap,
/// and that reading licenses a true answer**: at the default band the
/// ladder's noise meter reads this crossing's harmonics in the gap and
/// lets the ladder run, and the answer is the subdivision's — two roots,
/// each within the zero band of the exact oracle's.
#[test]
fn a_circle_of_radius_1500_reading_its_noise_in_the_gap_is_answered_truly() {
    let (rho, eps) = (1500.0, 1e-9);
    let band = Band::new(eps, 10.0 * eps).unwrap();
    let c = tangent_circle(1.0, rho, -1e-6, 1.2, 0.7);
    let noise = noise_reading(&c, 1.0);
    assert!(
        decide("bool_conic_quadric_ladder_noise", Margin::of(noise), band).is_err(),
        "the noise reading {noise:e} m lies in the gap of the band [1e-9, 1e-8]"
    );
    let truth = exact::truth(&c, &wall(1.0));
    assert_eq!(truth.roots.len(), 2, "the pose crosses the wall twice");
    let got = conic_quadric_roots(&c, -1.0, 1.0, &wall(1.0), band);
    let Ok(CircleRoots::Certified { count: 2, thetas }) = got else {
        panic!("two certified crossings, got {got:?}");
    };
    for &t in &thetas[..2] {
        let off = truth
            .roots
            .iter()
            .map(|&x| arc_between(t, x, rho))
            .fold(f64::INFINITY, f64::min);
        assert!(off <= eps, "root {t} lies {off:e} m from the truth");
    }
}

/// **A circle of radius 1500 against a unit wall places every certified
/// root within the zero band of a true one** (module docs), at depths
/// from a hundredth of the zero band to 0.1 m either side and at any
/// tilt, on a reading of its harmonics' noise that lies in the band's
/// gap at the default band. The ladder alone certified roots 1.09ε to
/// 1.23ε off at ε = 1e-9 on that reading. The judge is the zero band plus
/// the `f64` parameter's own grid at this radius, `ρ·ulp(2π)` per side
/// (`1.3e-12` m: the door's root and the oracle's are each an `f64` angle,
/// the oracle's read off `φ₀ + 2·atan t` round the turn), which only the
/// `1e-12` row feels.
#[test]
fn a_circle_of_radius_1500_places_its_roots_within_the_zero_band() {
    let rho = 1500.0;
    let mut rng = fuzz::start("conic_quadric_rho_1500_sweep");
    let n = fuzz::scaled(150);
    let grid = 2.0 * rho * f64::EPSILON * TAU;
    let mut wrong = Vec::new();
    for eps in EPS_ROWS {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for _ in 0..n {
            let side = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
            let depth = side * log_uniform(&mut rng, 1e-2 * eps, 1e-1);
            let c = tangent_circle(1.0, rho, depth, rng.range(0.0, TAU), rng.range(0.0, TAU));
            let truth = exact::truth(&c, &wall(1.0));
            let got = conic_quadric_roots(&c, -1.0, 1.0, &wall(1.0), band);
            match got {
                Ok(CircleRoots::Miss) if !truth.roots.is_empty() => wrong.push(format!(
                    "ε {eps:e}, depth {depth:e}: Miss on {} true roots",
                    truth.roots.len()
                )),
                Ok(CircleRoots::Certified { count, thetas }) => {
                    if count != truth.roots.len() {
                        wrong.push(format!(
                            "ε {eps:e}, depth {depth:e}: {count} certified, {} true",
                            truth.roots.len()
                        ));
                    }
                    for &t in &thetas[..count] {
                        let off = truth
                            .roots
                            .iter()
                            .map(|&x| arc_between(t, x, rho))
                            .fold(f64::INFINITY, f64::min);
                        if off > eps + grid {
                            wrong.push(format!(
                                "ε {eps:e}, depth {depth:e}: root {t} lies {off:e} m from the truth"
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "{} wrong answers ({}):\n{}",
        wrong.len(),
        fuzz::replay(),
        wrong.join("\n")
    );
}
