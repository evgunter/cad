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
fn f_geom(q: Point3<f64>, d: Vec3<f64>, c: Point3<f64>, a: Vec3<f64>, rr: f64, r: f64, t: f64) -> f64 {
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

/// CLAIM 1 + 5 — the certified count and the biquadratic sign, against
/// the geometric oracle, over a lattice of rays including every
/// adversarial pose the brief names.
#[test]
fn r1_certified_counts_agree_with_a_geometric_oracle() {
    let c = Point3::new(0.0, 0.0, 0.0);
    let a = Vec3::new(0.0, 1.0, 0.0);
    let b = band();
    let mut checked = 0usize;
    let mut certified = 0usize;
    let mut uncertain = 0usize;
    let mut misses = 0usize;
    let mut bad: Vec<String> = Vec::new();

    for (rr, r) in [(1.0f64, 0.3f64), (1.0, 0.9), (1.0, 0.02), (5.0, 0.1), (0.2, 0.19)] {
        // a lattice of origins and directions, plus the named adversarial poses
        let mut poses: Vec<(Point3<f64>, Vec3<f64>)> = Vec::new();
        // through the centre, in the midplane -> four roots
        poses.push((Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)));
        poses.push((Point3::new(0.0, 0.0, 0.0), Vec3::new(0.3, 0.0, 0.954).normalize()));
        // axis-parallel
        for x in [0.0, 0.5, rr - r, rr, rr + r, rr + r + 0.4] {
            poses.push((Point3::new(x, -9.0, 0.0), Vec3::new(0.0, 1.0, 0.0)));
        }
        // midplane rays that do not pass through the centre
        for y in [0.0] {
            for off in [0.0, 0.4, rr - r, rr, rr + r, 2.0 * rr] {
                poses.push((Point3::new(-9.0, y, off), Vec3::new(1.0, 0.0, 0.0)));
            }
        }
        // tangent to the inner / outer equator (in the midplane)
        for off in [rr - r, rr + r] {
            poses.push((Point3::new(-9.0, 0.0, off), Vec3::new(1.0, 0.0, 0.0)));
        }
        // through the centre along the axis
        poses.push((Point3::new(0.0, -9.0, 0.0), Vec3::new(0.0, 1.0, 0.0)));
        // a generic lattice
        for i in 0..7 {
            for j in 0..7 {
                for k in 0..5 {
                    let o = Point3::new(
                        -3.0 + i as f64 * 1.1,
                        -2.0 + j as f64 * 0.7,
                        -3.0 + k as f64 * 1.3,
                    );
                    let dir = Vec3::new(
                        0.3 + 0.21 * i as f64,
                        -0.7 + 0.31 * j as f64,
                        0.45 - 0.17 * k as f64,
                    );
                    if dir.norm() < 1e-6 {
                        continue;
                    }
                    poses.push((o, dir.normalize()));
                }
            }
        }

        for (o, dir) in poses {
            let d = dir.normalize();
            checked += 1;
            let got = line_torus_roots(o, d, c, a, rr, r, b);
            let (oracle, gap) = oracle_roots(o, d, c, a, rr, r);
            match got {
                Ok(TorusRoots::Certified { count, ts }) => {
                    certified += 1;
                    if count != oracle.len() {
                        bad.push(format!(
                            "COUNT R={rr} r={r} o={o:?} d={d:?}: certified {count}, oracle {} \
                             (gap {gap:e}) oracle roots {oracle:?} code roots {:?}",
                            oracle.len(),
                            &ts[..count]
                        ));
                        continue;
                    }
                    let mut mine: Vec<f64> = ts[..count].to_vec();
                    mine.sort_by(|x, y| x.partial_cmp(y).unwrap());
                    for (m, ok) in mine.iter().zip(oracle.iter()) {
                        if (m - ok).abs() > 1e-6 {
                            bad.push(format!(
                                "ROOT R={rr} r={r} o={o:?} d={d:?}: code {m:e} vs oracle {ok:e} \
                                 (gap {gap:e})"
                            ));
                        }
                    }
                }
                Ok(TorusRoots::Miss) => {
                    misses += 1;
                    if !oracle.is_empty() && gap > 1e-3 {
                        bad.push(format!(
                            "MISS R={rr} r={r} o={o:?} d={d:?}: code says miss, oracle found \
                             {oracle:?} (gap {gap:e})"
                        ));
                    }
                }
                Ok(TorusRoots::Uncertain) => uncertain += 1,
                Err(_) => uncertain += 1,
            }
        }
    }
    println!(
        "R1 root-count probe: {checked} rays, {certified} certified, {misses} miss, \
         {uncertain} uncertain/escalated, {} disagreements",
        bad.len()
    );
    for line in bad.iter().take(25) {
        println!("  {line}");
    }
    assert!(bad.is_empty(), "{} disagreements with the oracle", bad.len());
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
