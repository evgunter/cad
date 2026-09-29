//! The counterexample SEARCH over the cone's two root lanes: line spans
//! and circles in poses nobody chose, against an oracle that shares
//! none of their algebra — the elevation `ρ cos α − |h| sin α` evaluated
//! directly at points of the carrier, sampled densely, each sign change
//! bisected.
//!
//! A certified answer must be the oracle's: a line's interior roots, a
//! circle's roots over the whole turn, each within `1e-9` m, and a
//! certified miss has no sign change. Refusals are always allowed.
//! "Away from the band": a pose with a root within `1e-6` m of a span
//! end or of another root, or a sampled elevation within `1e-7` m of
//! zero with no sign change there (a graze the grid may straddle), is
//! skipped, since the dense oracle cannot place what the band declines
//! to either.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

// Gated to the code it tests: both lanes and the machinery they call
// (the ray lane's quadratic and quartic ladder, the half-angle
// substitution), and the band, tolerance and vector types every
// decision is made in.
test_utils::gated_to![
    "crates/topo/src/boolean/line_cone.rs",
    "crates/topo/src/boolean/circle_cone.rs",
    "crates/topo/src/boolean/circle_torus.rs",
    "crates/topo/src/boolean/solid_contain.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
    "crates/geom-core/src/real.rs",
    "crates/geom-core/src/linalg/",
];

use geom_core::{Band, Point3, Tol, Vec3};
use test_utils::fuzz;

use super::circle_cone::{CircleConeRoots, circle_cone_roots};
use super::line_cone::{EdgeConeRoots, line_cone_edge_roots};

/// A random cone: apex near the origin, axis uniform, half-angle away
/// from the degenerate ends.
struct Cone {
    apex: Point3<f64>,
    axis: Vec3<f64>,
    alpha: f64,
}

impl Cone {
    fn draw(rng: &mut fuzz::Rng) -> Self {
        Self {
            apex: Point3::new(
                rng.range(-0.5, 0.5),
                rng.range(-0.5, 0.5),
                rng.range(-0.5, 0.5),
            ),
            axis: direction(rng),
            alpha: rng.range(0.2, 1.3),
        }
    }

    fn surface(&self) -> geom::Surface<f64> {
        let u = self.axis.cross(Vec3::new(0.3, 0.5, 0.8)).normalize();
        geom::Surface::Cone {
            apex: self.apex,
            axis: self.axis,
            half_angle: self.alpha,
            u_ref: u,
        }
    }

    fn elevation(&self, p: Point3<f64>) -> f64 {
        let q = p - self.apex;
        let h = q.dot(self.axis);
        (q - self.axis * h).norm() * self.alpha.cos() - h.abs() * self.alpha.sin()
    }
}

/// Uniform on the sphere, by rejection from the ball.
fn direction(rng: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.norm();
        if (1e-3..=1.0).contains(&n) {
            break v / n;
        }
    }
}

/// The oracle: the sign changes of `f` over `[t0, t1]` on `n` cells,
/// bisected, and whether a sampled value came within `1e-7` of zero
/// without a sign change in its cell (a graze the grid may straddle).
fn oracle(f: impl Fn(f64) -> f64, t0: f64, t1: f64, n: u32) -> (Vec<f64>, bool) {
    let mut roots = Vec::new();
    let mut graze = false;
    let at = |i: u32| t0 + (t1 - t0) * f64::from(i) / f64::from(n);
    for i in 0..n {
        let (mut a, mut b) = (at(i), at(i + 1));
        let (fa, fb) = (f(a), f(b));
        if fa * fb >= 0.0 {
            graze |= fa.abs() < 1e-7;
            continue;
        }
        for _ in 0..100 {
            let m = 0.5 * (a + b);
            if f(a) * f(m) <= 0.0 {
                b = m;
            } else {
                a = m;
            }
        }
        roots.push(0.5 * (a + b));
    }
    (roots, graze)
}

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// Line spans against random cones: the certified interior roots are
/// the oracle's sign changes over the span.
#[test]
fn line_spans_agree_with_the_direct_oracle() {
    let mut rng = fuzz::start("boolean::cone_roots_fuzz::lines");
    let (mut answered, mut skipped, mut bad) = (0usize, 0usize, Vec::new());
    let mut crossings = 0usize;
    for case in 0..fuzz::scaled(300) {
        let cone = Cone::draw(&mut rng);
        let origin = Point3::new(
            rng.range(-1.5, 1.5),
            rng.range(-1.5, 1.5),
            rng.range(-1.5, 1.5),
        );
        let dir = direction(&mut rng);
        let len = rng.range(0.2, 3.0);
        let got = line_cone_edge_roots(
            origin,
            dir,
            cone.apex,
            cone.axis,
            cone.alpha,
            1.5,
            (0.0, len),
            band(),
        );
        let (truth, graze) = oracle(|t| cone.elevation(origin + dir * t), 0.0, len, 4000);
        let label = format!(
            "case {case}: {origin:?} + t·{dir:?}, t ∈ [0, {len}], cone α {}",
            cone.alpha
        );
        let interior = |ts: &[f64]| -> Option<Vec<f64>> {
            let mut ts: Vec<f64> = ts.to_vec();
            ts.sort_by(f64::total_cmp);
            if ts.windows(2).any(|w| w[1] - w[0] < 1e-6)
                || ts.iter().any(|t| t.abs() < 1e-6 || (t - len).abs() < 1e-6)
            {
                return None;
            }
            Some(ts.into_iter().filter(|t| *t > 0.0 && *t < len).collect())
        };
        match got {
            Err(_) | Ok(EdgeConeRoots::Door) => {}
            Ok(EdgeConeRoots::Miss) => {
                answered += 1;
                if !truth.is_empty() {
                    bad.push(format!("{label}: a certified miss, oracle {truth:?}"));
                }
            }
            Ok(EdgeConeRoots::Two(ts)) => {
                let Some(inside) = interior(&ts) else {
                    skipped += 1;
                    continue;
                };
                if graze && inside.len() != truth.len() {
                    skipped += 1;
                    continue;
                }
                answered += 1;
                crossings += inside.len();
                if inside.len() != truth.len()
                    || inside.iter().zip(&truth).any(|(a, b)| (a - b).abs() > 1e-9)
                {
                    bad.push(format!("{label}: {inside:?} vs oracle {truth:?}"));
                }
            }
        }
    }
    println!("lines: {answered} answered ({crossings} interior crossings), {skipped} at the band");
    assert!(
        bad.is_empty(),
        "{} disagreements: {bad:#?} — {}",
        bad.len(),
        fuzz::replay()
    );
}

/// Circles against random cones, over the whole turn: generic poses and
/// parallel-axes ones (the closed form), each against the oracle.
#[test]
fn circles_agree_with_the_direct_oracle() {
    let mut rng = fuzz::start("boolean::cone_roots_fuzz::circles");
    let (mut answered, mut skipped, mut bad) = (0usize, 0usize, Vec::new());
    let mut counts = [0usize; 5];
    let tau = core::f64::consts::TAU;
    for case in 0..fuzz::scaled(200) {
        let cone = Cone::draw(&mut rng);
        let center = cone.apex
            + Vec3::new(
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
                rng.range(-1.0, 1.0),
            );
        // Every other case parallel to the cone's axis.
        let axis = if case % 2 == 0 {
            direction(&mut rng)
        } else {
            cone.axis
        };
        let u_ref = axis.cross(direction(&mut rng)).normalize();
        let radius = rng.range(0.05, 1.5);
        let v_ref = axis.cross(u_ref);
        let point = |t: f64| center + u_ref * (radius * t.cos()) + v_ref * (radius * t.sin());
        let t0 = rng.range(-3.0, 3.0);
        let (t0, t1) = (t0, t0 + rng.range(0.5, tau));
        let got = circle_cone_roots(center, axis, radius, u_ref, t0, t1, &cone.surface(), band());
        let mid = 0.5 * (t0 + t1);
        let (truth, graze) = oracle(
            |t| cone.elevation(point(t)),
            mid - tau / 2.0,
            mid + tau / 2.0,
            8_000,
        );
        let label = format!(
            "case {case}: centre {center:?}, axis {axis:?}, ρ {radius}, cone α {} at {:?} along {:?}",
            cone.alpha, cone.apex, cone.axis
        );
        match got {
            Err(_) | Ok(CircleConeRoots::Uncertain) => {}
            Ok(CircleConeRoots::Coaxial) => bad.push(format!("{label}: not coaxial")),
            Ok(CircleConeRoots::Miss) => {
                if graze && truth.is_empty() {
                    skipped += 1;
                    continue;
                }
                answered += 1;
                counts[0] += 1;
                if !truth.is_empty() {
                    bad.push(format!("{label}: a certified miss, oracle {truth:?}"));
                }
            }
            Ok(CircleConeRoots::Certified { count, thetas }) => {
                let mut ts: Vec<f64> = thetas[..count].to_vec();
                ts.sort_by(f64::total_cmp);
                if ts.windows(2).any(|w| (w[1] - w[0]) * radius < 1e-6) {
                    skipped += 1;
                    continue;
                }
                if graze && ts.len() != truth.len() {
                    skipped += 1;
                    continue;
                }
                answered += 1;
                counts[count] += 1;
                if ts.len() != truth.len()
                    || ts
                        .iter()
                        .zip(&truth)
                        .any(|(a, b)| (a - b).abs() * radius > 1e-9)
                {
                    bad.push(format!("{label}: {ts:?} vs oracle {truth:?}"));
                }
            }
        }
    }
    println!("circles: {answered} answered (by root count {counts:?}), {skipped} at the band");
    assert!(
        bad.is_empty(),
        "{} disagreements: {bad:#?} — {}",
        bad.len(),
        fuzz::replay()
    );
}
