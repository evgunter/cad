//! **No certified root of a shallow crossing is placed past the band.**
//! Circles posed to cross a torus at a chosen depth (log-uniform from
//! `1e-11` to `1e-3` m, either side) at a point anywhere on the tube —
//! against the unit torus, a placed torus at the ten-metre scale, and a
//! near-parallel carrier whose tilt can land in the band's gap — decided
//! at `ε = 1e-6, 1e-9, 1e-12` on both lanes, against the exact oracle
//! ([`crate::boolean::conic_oracle::exact`]). On every pose at least
//! `Kε` deep, a certified answer has the true count, and each certified
//! root lies within `Kε` of arc of a true one and each true one within
//! `Kε` of a certified one; a `Miss` has no true root; nothing else is
//! certified. A refusal is always allowed.

test_utils::gated_to![
    "crates/topo/src/boolean/circle_torus.rs",
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
use crate::boolean::conic_oracle::exact;
use geom_core::{Bounds, Interval, Real};
use test_utils::fuzz::{self, Rng};

/// A circle, its arc and a torus, as stored.
#[derive(Clone, Copy, Debug)]
struct Pose {
    center: [f64; 3],
    axis: [f64; 3],
    radius: f64,
    u_ref: [f64; 3],
    t0: f64,
    t1: f64,
    t_center: [f64; 3],
    t_axis: [f64; 3],
    big: f64,
    small: f64,
}

fn v3<T: Real>(a: [f64; 3]) -> Vec3<T> {
    Vec3::new(T::from_f64(a[0]), T::from_f64(a[1]), T::from_f64(a[2]))
}

fn p3<T: Real>(a: [f64; 3]) -> Point3<T> {
    Point3::new(T::from_f64(a[0]), T::from_f64(a[1]), T::from_f64(a[2]))
}

impl Pose {
    fn carrier<T: Real>(&self) -> geom::Curve3<T> {
        geom::Curve3::Circle {
            center: p3(self.center),
            axis: v3(self.axis),
            radius: T::from_f64(self.radius),
            u_ref: v3(self.u_ref),
        }
    }

    fn torus<T: Real>(&self) -> geom::Surface<T> {
        let axis = Vec3::from_array(self.t_axis);
        geom::Surface::Torus {
            center: p3(self.t_center),
            axis: v3(self.t_axis),
            major_radius: T::from_f64(self.big),
            minor_radius: T::from_f64(self.small),
            u_ref: v3(axis.orthonormal_basis().0.to_array()),
        }
    }

    /// The door's certified roots as `(lo, hi)` parameter pairs (`pair`
    /// reads one), or what it answered instead.
    fn door<T: Decide>(
        &self,
        band: Band,
        pair: fn(T) -> (f64, f64),
    ) -> Result<Vec<(f64, f64)>, String> {
        let at = |x: f64| T::from_f64(x);
        match circle_torus_roots(
            &self.carrier(),
            at(self.t0),
            at(self.t1),
            &self.torus(),
            band,
        ) {
            Ok(CircleRoots::Certified { count, thetas }) => {
                Ok(thetas[..count].iter().map(|&t| pair(t)).collect())
            }
            Ok(other) => Err(format!("{other:?}")),
            Err(e) => Err(format!("refused: {e:?}")),
        }
    }
}

fn log_uniform(rng: &mut Rng, lo: f64, hi: f64) -> f64 {
    10f64.powf(rng.range(lo.log10(), hi.log10()))
}

fn arc(rng: &mut Rng) -> (f64, f64) {
    let (m, h) = (rng.range(-PI, PI), rng.range(0.01, PI));
    (m - h, m + h)
}

/// A circle of radius `rho` through the point `depth` off the torus
/// `(c, a, big, small)` along its normal, tangent there to the tube in a
/// random direction and turned about that tangent at random.
fn graze(rng: &mut Rng, (c, a, big, small): ([f64; 3], Vec3<f64>, f64, f64), rho: f64) -> Pose {
    let (e1, e2) = a.orthonormal_basis();
    let (phi, psi) = (rng.range(0.0, TAU), rng.range(0.0, TAU));
    let radial = e1 * phi.cos() + e2 * phi.sin();
    let on = Point3::from_array(c) + radial * (big + small * psi.cos()) + a * (small * psi.sin());
    let normal = radial * psi.cos() + a * psi.sin();
    let along_phi = e2 * phi.cos() - e1 * phi.sin();
    let along_psi = a * psi.cos() - radial * psi.sin();
    let g = rng.range(0.0, TAU);
    let tangent = (along_phi * g.cos() + along_psi * g.sin()).normalize();
    let b = rng.range(0.0, TAU);
    let toward = (normal * b.cos() + normal.cross(tangent) * b.sin()).normalize();
    let depth = if rng.unit() < 0.5 { -1.0 } else { 1.0 } * log_uniform(rng, 1e-11, 1e-3);
    let center = on + normal * depth - toward * rho;
    let axis = toward.cross(tangent).normalize();
    let (t0, t1) = arc(rng);
    Pose {
        center: center.to_array(),
        axis: axis.to_array(),
        radius: rho,
        u_ref: toward.to_array(),
        t0,
        t1,
        t_center: c,
        t_axis: a.to_array(),
        big,
        small,
    }
}

/// A carrier whose axis is within `1e-15..1e-3` of the unit torus's,
/// its radius set to cross a contour circle by `1e-11..1e-3` m.
fn near_parallel(rng: &mut Rng) -> Pose {
    let small = 0.25;
    let h0 = if rng.unit() < 0.5 {
        rng.range(-small, small)
    } else {
        (small - log_uniform(rng, 1e-11, 1e-3)) * if rng.unit() < 0.5 { -1.0 } else { 1.0 }
    };
    let off = rng.range(0.0, 2.0);
    let side = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
    let contour = 1.0 + side * (small * small - h0 * h0).max(0.0).sqrt();
    let dd = if rng.unit() < 0.5 { -1.0 } else { 1.0 } * log_uniform(rng, 1e-11, 1e-3);
    let rho = (contour + dd + if rng.unit() < 0.5 { -off } else { off })
        .abs()
        .max(1e-3);
    let z = Vec3::new(0.0, 0.0, 1.0);
    let tilt = log_uniform(rng, 1e-15, 1e-3);
    let towards = z.cross(crate::boolean::conic_oracle::unit(rng)).normalize();
    let axis = (z + towards * tilt).normalize();
    let u_ref = axis
        .cross(crate::boolean::conic_oracle::unit(rng))
        .normalize();
    let ang = rng.range(0.0, TAU);
    let (t0, t1) = arc(rng);
    Pose {
        center: [off * ang.cos(), off * ang.sin(), h0],
        axis: axis.to_array(),
        radius: rho,
        u_ref: u_ref.to_array(),
        t0,
        t1,
        t_center: [0.0; 3],
        t_axis: z.to_array(),
        big: 1.0,
        small,
    }
}

fn draw(rng: &mut Rng, family: usize) -> Pose {
    match family {
        0 => {
            let rho = log_uniform(rng, 0.05, 5.0);
            graze(rng, ([0.0; 3], Vec3::new(0.0, 0.0, 1.0), 1.0, 0.25), rho)
        }
        1 => {
            let c = [
                rng.range(-10.0, 10.0),
                rng.range(-10.0, 10.0),
                rng.range(-10.0, 10.0),
            ];
            let a = crate::boolean::conic_oracle::unit(rng);
            let big = rng.range(0.3, 3.0);
            let small = big * rng.range(0.05, 0.6);
            let rho = log_uniform(rng, 0.1, 10.0);
            graze(rng, (c, a, big, small), rho)
        }
        _ => near_parallel(rng),
    }
}

/// The arc length from `(lo, hi)` to `root` round the turn, on a circle of
/// radius `rho`: zero when the root is inside.
fn arc_off((lo, hi): (f64, f64), root: f64, rho: f64) -> f64 {
    let (mid, half) = ((lo + hi) / 2.0, (hi - lo) / 2.0);
    let turn = (mid - root).rem_euclid(TAU);
    (turn.min(TAU - turn) - half).max(0.0) * rho
}

/// What is wrong with `got` against `truth` at `reach` metres, if
/// anything.
fn judge(
    got: &Result<Vec<(f64, f64)>, String>,
    truth: &exact::Truth,
    rho: f64,
    reach: f64,
) -> Option<String> {
    match got {
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
                        .map(|&t| arc_off(r, t, rho))
                        .fold(f64::INFINITY, f64::min)
                })
                .chain(truth.roots.iter().map(|&t| {
                    roots
                        .iter()
                        .map(|&r| arc_off(r, t, rho))
                        .fold(f64::INFINITY, f64::min)
                }))
                .fold(0.0, f64::max);
            (worst > reach).then(|| {
                format!(
                    "a root {worst:.3e} m from the truth ({roots:?} vs {:?})",
                    truth.roots
                )
            })
        }
        Err(what) if what.starts_with("Miss") && !truth.roots.is_empty() => {
            Some(format!("Miss on {} true roots", truth.roots.len()))
        }
        Err(what)
            if what.starts_with("Miss") || what.starts_with("refused") || what == "Uncertain" =>
        {
            None
        }
        Err(what) => Some(format!("certified {what}")),
    }
}

#[test]
fn no_certified_root_of_a_shallow_crossing_is_placed_past_the_band() {
    let mut rng = fuzz::start("circle_torus_shallow_sweep");
    let n = fuzz::scaled(40);
    let mut wrong = Vec::new();
    let (mut judged, mut refused) = (0usize, 0usize);
    for i in 0..3 * n {
        let pose = draw(&mut rng, i % 3);
        let truth = exact::truth(&pose.carrier(), &pose.torus());
        for eps in [1e-6, 1e-9, 1e-12] {
            let band = Band::new(eps, 10.0 * eps).unwrap();
            if truth.depth < 10.0 * eps {
                continue;
            }
            for (lane, got) in [
                ("f64", pose.door(band, |t: f64| (t, t))),
                ("Interval", pose.door(band, |t: Interval| (t.lo(), t.hi()))),
            ] {
                judged += 1;
                refused += usize::from(matches!(&got, Err(w) if !w.starts_with("Miss")));
                if let Some(why) = judge(&got, &truth, pose.radius, 10.0 * eps) {
                    wrong.push(format!(
                        "{lane} at ε {eps:e}, depth {:.3e}: {why}\n  {pose:?}",
                        truth.depth
                    ));
                }
            }
        }
    }
    println!("{judged} answers judged outside the band, {refused} refused");
    assert!(
        wrong.is_empty(),
        "{} wrong answers outside the band ({}):\n{}",
        wrong.len(),
        fuzz::replay(),
        wrong.join("\n")
    );
}

/// **The exact oracle is the mpmath oracle where both read**: on the
/// pinned shallow poses ([`super::tests::SHALLOW_POSES`], mpmath at 70
/// digits) it finds the same crossings to `1e-13` m of arc, and each
/// pose at least `Kε` deep at `ε = 1e-12` — so the sweep's verdicts rest
/// on an oracle that resolves the band it judges.
#[test]
fn the_exact_oracle_agrees_with_mpmath_on_the_pinned_poses() {
    for (label, v, want) in super::tests::SHALLOW_POSES {
        let pose = Pose {
            center: [v[2], v[3], v[4]],
            axis: [v[5], v[6], v[7]],
            radius: v[8],
            u_ref: [v[9], v[10], v[11]],
            t0: v[0],
            t1: v[1],
            t_center: [v[12], v[13], v[14]],
            t_axis: [v[15], v[16], v[17]],
            big: v[18],
            small: v[19],
        };
        let truth = exact::truth(&pose.carrier(), &pose.torus());
        let mut want = want.to_vec();
        want.sort_by(f64::total_cmp);
        assert_eq!(
            truth.roots.len(),
            want.len(),
            "{label}: {:?} vs {want:?}",
            truth.roots
        );
        for (got, want) in truth.roots.iter().zip(&want) {
            let off = arc_off((*got, *got), *want, pose.radius);
            assert!(
                off < 1e-13,
                "{label}: the exact oracle's {got} is {off:.3e} m from mpmath's {want}"
            );
        }
        assert!(
            (1e-11..1e-6).contains(&truth.depth),
            "{label}: a shallow pose outside the band at 1e-12, depth {:.3e}",
            truth.depth
        );
    }
}
