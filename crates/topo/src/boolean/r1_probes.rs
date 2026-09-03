//! R1 review probes for BOOL-3 (issue 1011, torus half). NOT part of
//! the shipped tree: these exist to falsify the PR's claims by
//! execution, against oracles independent of the code under test.
//!
//! * the CERTIFIED ROOT COUNT, against a geometric root counter that
//!   never touches the quartic's coefficients (it samples the torus's
//!   own implicit `F` along the ray and bisects every sign change);
//! * the `sqrt`-chain CUBE ROOT, against `f64::cbrt`;
//! * the BIQUADRATIC SIGN, through the same geometric oracle on the
//!   rays that reach that arm.

#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use super::*;
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The torus's own implicit function along the ray — the geometry, not
/// the quartic. `F(t) = (|w|² + R² − r²)² − 4R²ρ²`.
fn f_geom(
    q: Point3<f64>,
    d: Vec3<f64>,
    c: Point3<f64>,
    a: Vec3<f64>,
    rr: f64,
    r: f64,
    t: f64,
) -> f64 {
    let w = (q - c) + d * t;
    let h = w.dot(a);
    let rho2 = w.norm_squared() - h * h;
    (w.norm_squared() + rr * rr - r * r).powi(2) - 4.0 * rr * rr * rho2
}

/// An INDEPENDENT root counter: dense sampling of `f_geom` plus
/// bisection. Returns the sorted roots, and the smallest gap between
/// consecutive ones (a proxy for how near a tangency the pose is).
fn oracle_roots(
    q: Point3<f64>,
    d: Vec3<f64>,
    c: Point3<f64>,
    a: Vec3<f64>,
    rr: f64,
    r: f64,
) -> (Vec<f64>, f64) {
    let b = (q - c).dot(d);
    let ext = rr + r;
    let (lo, hi) = (-b - ext * 1.5, -b + ext * 1.5);
    let n = 400_000usize;
    let mut roots = Vec::new();
    let mut prev_t = lo;
    let mut prev = f_geom(q, d, c, a, rr, r, lo);
    for i in 1..=n {
        let t = lo + (hi - lo) * (i as f64) / (n as f64);
        let cur = f_geom(q, d, c, a, rr, r, t);
        if (prev < 0.0 && cur > 0.0) || (prev > 0.0 && cur < 0.0) {
            // bisect
            let (mut x0, mut x1) = (prev_t, t);
            let f0 = prev;
            for _ in 0..200 {
                let mid = 0.5 * (x0 + x1);
                let fm = f_geom(q, d, c, a, rr, r, mid);
                if (fm < 0.0) == (f0 < 0.0) {
                    x0 = mid;
                } else {
                    x1 = mid;
                }
            }
            roots.push(0.5 * (x0 + x1));
        } else if cur == 0.0 {
            roots.push(t);
        }
        prev_t = t;
        prev = cur;
    }
    roots.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut gap = f64::INFINITY;
    for w in roots.windows(2) {
        gap = gap.min(w[1] - w[0]);
    }
    (roots, gap)
}

/// CLAIM 2 — the `sqrt`-chain cube root. Compared against `f64::cbrt`
/// over 12 decades either side of 1 and both signs.
#[test]
fn r1_cbrt_chain_tracks_the_true_cube_root() {
    let mut worst = 0.0f64;
    let mut worst_x = 0.0f64;
    for e in -30..=30 {
        for mant in [1.0, 1.7, 3.3, 6.1, 9.4] {
            for sgn in [1.0, -1.0] {
                let x = sgn * mant * 10f64.powi(e);
                let got = cbrt(x);
                let want = x.cbrt();
                let rel = ((got - want) / want).abs();
                if rel > worst {
                    worst = rel;
                    worst_x = x;
                }
            }
        }
    }
    println!("R1 cbrt: worst relative error {worst:e} at x = {worst_x:e}");
    assert_eq!(cbrt(0.0), 0.0, "cbrt(0) must be 0");
    assert!(
        worst < 1e-12,
        "the sqrt-chain cube root drifts from the true one by {worst:e} at {worst_x:e}"
    );
}

/// The truncation is a SYSTEMATIC offset, not an enclosure: the chain
/// computes `x^((1−4^-27)/3)`, and this measures how far that is from
/// `x^(1/3)` at the magnitudes the resolvent actually feeds it.
#[test]
fn r1_cbrt_truncation_is_a_bias_not_a_containment() {
    for x in [1e-18f64, 1e-6, 1.0, 1e6, 1e18, 1e36] {
        let exact = x.powf((1.0 - 4f64.powi(-27)) / 3.0);
        let truth = x.cbrt();
        println!(
            "R1 cbrt truncation: x={x:e} chain-exponent value={exact:e} true={truth:e} rel={:e}",
            ((exact - truth) / truth).abs()
        );
    }
}

/// The torus centre and axis every probe below sweeps about.
fn centre_and_axis() -> (Point3<f64>, Vec3<f64>) {
    (Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0))
}

/// The five `(R, r)` shapes the probes sweep — one per conditioning
/// regime of the quartic: generic; a fat ring; a thin ring; a large-`R`
/// thin ring; and a ring whose inner equator all but closes
/// (`R - r = 0.01`, so the near-tangency is built into the shape rather
/// than into the pose).
const SHAPES: [(f64, f64); 5] = [(1.0, 0.3), (1.0, 0.9), (1.0, 0.02), (5.0, 0.1), (0.2, 0.19)];

/// The running census of one sweep: how the certifier answered, and
/// every disagreement with the geometric oracle.
struct Tally {
    checked: usize,
    certified: usize,
    misses: usize,
    uncertain: usize,
    bad: Vec<String>,
}

impl Tally {
    fn new() -> Self {
        Self {
            checked: 0,
            certified: 0,
            misses: 0,
            uncertain: 0,
            bad: Vec::new(),
        }
    }

    /// One pose against both counters. Every line `bad` collects opens
    /// with the REGIME that produced it, so a red names the pose class
    /// and not only its coordinates.
    fn compare(&mut self, regime: &str, rr: f64, r: f64, o: Point3<f64>, dir: Vec3<f64>) {
        let (c, a) = centre_and_axis();
        let d = dir.normalize();
        self.checked += 1;
        let got = line_torus_roots(o, d, c, a, rr, r, band());
        let (oracle, gap) = oracle_roots(o, d, c, a, rr, r);
        match got {
            Ok(TorusRoots::Certified { count, ts }) => {
                self.certified += 1;
                if count != oracle.len() {
                    self.bad.push(format!(
                        "COUNT [{regime}] R={rr} r={r} o={o:?} d={d:?}: certified {count}, \
                         oracle {} (gap {gap:e}) oracle roots {oracle:?} code roots {:?}",
                        oracle.len(),
                        &ts[..count]
                    ));
                    return;
                }
                let mut mine: Vec<f64> = ts[..count].to_vec();
                mine.sort_by(|x, y| x.partial_cmp(y).unwrap());
                for (m, ok) in mine.iter().zip(oracle.iter()) {
                    if (m - ok).abs() > 1e-6 {
                        self.bad.push(format!(
                            "ROOT [{regime}] R={rr} r={r} o={o:?} d={d:?}: code {m:e} vs \
                             oracle {ok:e} (gap {gap:e})"
                        ));
                    }
                }
            }
            Ok(TorusRoots::Miss) => {
                self.misses += 1;
                if !oracle.is_empty() && gap > 1e-3 {
                    self.bad.push(format!(
                        "MISS [{regime}] R={rr} r={r} o={o:?} d={d:?}: code says miss, oracle \
                         found {oracle:?} (gap {gap:e})"
                    ));
                }
            }
            Ok(TorusRoots::Uncertain) => self.uncertain += 1,
            Err(_) => self.uncertain += 1,
        }
    }

    fn report(&self, label: &str) {
        println!(
            "R1 root-count probe ({label}): {} rays, {} certified, {} miss, {} \
             uncertain/escalated, {} disagreements",
            self.checked,
            self.certified,
            self.misses,
            self.uncertain,
            self.bad.len()
        );
        for line in self.bad.iter().take(25) {
            println!("  {line}");
        }
    }
}

/// The pose REGIMES of a `(R, r)` torus: the rays whose root structure
/// the geometry names, rather than rays drawn at large. Every entry is
/// a distinct configuration of the quartic — a ray through the hole, a
/// ray tangent to one of the two equators, a ray down the tube's
/// centre, a ray that clears the shape — in each of the two symmetry
/// planes plus two obliques through the centre.
fn regime_poses(rr: f64, r: f64) -> Vec<(&'static str, Point3<f64>, Vec3<f64>)> {
    vec![
        // Through the centre: the four-root rays.
        (
            "centre, midplane, axis-aligned",
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            "centre, midplane, oblique",
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.3, 0.0, 0.954),
        ),
        (
            "centre, out of the midplane",
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.3, 0.5, 0.812),
        ),
        // Parallel to the axis: the offset walks the hole, the inner
        // equator, the tube's centre, the outer equator, and clear.
        (
            "axis-parallel, through the hole",
            Point3::new(0.0, -9.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            "axis-parallel, inner-equator tangent",
            Point3::new(rr - r, -9.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            "axis-parallel, through the tube centre",
            Point3::new(rr, -9.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            "axis-parallel, outer-equator tangent",
            Point3::new(rr + r, -9.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        (
            "axis-parallel, clear of the shape",
            Point3::new(rr + r + 0.4, -9.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        ),
        // In the midplane, offset from the centre: the same walk with
        // the ray in the plane of the ring instead of along its axis.
        (
            "midplane, through the hole",
            Point3::new(-9.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            "midplane, inner-equator tangent",
            Point3::new(-9.0, 0.0, rr - r),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            "midplane, through the tube centre",
            Point3::new(-9.0, 0.0, rr),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            "midplane, outer-equator tangent",
            Point3::new(-9.0, 0.0, rr + r),
            Vec3::new(1.0, 0.0, 0.0),
        ),
        (
            "midplane, beyond the outer equator",
            Point3::new(-9.0, 0.0, 2.0 * rr),
            Vec3::new(1.0, 0.0, 0.0),
        ),
    ]
}

/// CLAIM 1 + 5 — the certified count and the biquadratic sign, against
/// the geometric oracle, over the ENUMERATION of pose regimes: for each
/// of the five torus shapes, every ray configuration the geometry names
/// (`regime_poses`). Deterministic and exhaustive over that table;
/// generic poses are the sibling sweep's, not this row's.
#[test]
fn r1_certified_counts_agree_with_a_geometric_oracle() {
    let mut tally = Tally::new();
    for (rr, r) in SHAPES {
        for (regime, o, dir) in regime_poses(rr, r) {
            tally.compare(regime, rr, r, o, dir);
        }
    }
    tally.report("pose regimes");
    assert!(
        tally.bad.is_empty(),
        "{} disagreements with the oracle over the pose-regime enumeration",
        tally.bad.len()
    );
}

/// CLAIM 1 + 5 at poses nobody chose — a counterexample search over
/// rays drawn uniformly through the box the shapes occupy, with a
/// direction uniform on the sphere.
///
/// A varying seed, because this is the search shape: successive runs
/// explore new poses instead of replaying one lattice forever. The
/// count is on the workspace `CAD_FUZZ_EFFORT` dial, shipped at the
/// smoke level a gated run should cost; `CAD_FUZZ_EFFORT=60` restores
/// roughly the ray count the fixed lattice used to run.
///
/// Specific to `crates/topo/src/boolean/solid_contain.rs` (the
/// certifier `line_torus_roots` lives there) and to this file.
#[test]
fn r1_generic_poses_agree_with_the_geometric_oracle() {
    use test_utils::fuzz;
    let mut rng = fuzz::start("boolean::r1_probes::generic_poses");
    let per_shape = fuzz::scaled(4);
    let mut tally = Tally::new();
    for (rr, r) in SHAPES {
        for _ in 0..per_shape {
            let o = Point3::new(
                rng.range(-3.0, 3.6),
                rng.range(-2.0, 2.2),
                rng.range(-3.0, 2.2),
            );
            // Uniform on the sphere by rejection from the ball: a
            // direction drawn per-component and normalized would
            // over-weight the cube's diagonals, which is the bias the
            // retired lattice's arithmetic directions already had.
            let dir = loop {
                let v = Vec3::new(
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                    rng.range(-1.0, 1.0),
                );
                let n = v.norm();
                if (1e-3..=1.0).contains(&n) {
                    break v / n;
                }
            };
            tally.compare("generic", rr, r, o, dir);
        }
    }
    tally.report("generic poses");
    assert!(
        tally.bad.is_empty(),
        "{} disagreements with the oracle at generic poses — {}",
        tally.bad.len(),
        fuzz::replay()
    );
}

/// The four-root ray through the hole, asserted at the ROOT level
/// rather than through `point_in_solid` — the shipped suite pins the
/// consequence, not the count.
#[test]
fn r1_the_hole_ray_certifies_exactly_four_roots() {
    let (rr, r) = (1.0f64, 0.3f64);
    let c = Point3::new(0.0, 0.0, 0.0);
    let a = Vec3::new(0.0, 1.0, 0.0);
    let d = Vec3::new(1.0, 0.0, 0.0);
    match line_torus_roots(Point3::new(0.0, 0.0, 0.0), d, c, a, rr, r, band()) {
        Ok(TorusRoots::Certified { count, ts }) => {
            let mut v = ts[..count].to_vec();
            v.sort_by(|x, y| x.partial_cmp(y).unwrap());
            println!("R1 hole ray: count={count} ts={v:?}");
            assert_eq!(count, 4);
            for (got, want) in v.iter().zip([-(rr + r), -(rr - r), rr - r, rr + r]) {
                assert!((got - want).abs() < 1e-9, "{got} != {want}");
            }
        }
        other => panic!("the four-root ray through the hole did not certify: {other:?}"),
    }
}
