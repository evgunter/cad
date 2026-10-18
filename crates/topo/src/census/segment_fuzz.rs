//! **Pass 5's edge pairs never read a meeting apart**: random segment
//! pairs built to meet, read by [`interiors_meet`] at `f64` and at
//! [`Interval`], in both orders, at ε = 1e-6, 1e-9 and 1e-12 (K = 10).
//!
//! Two families, each over every angle from far inside the parallel
//! zero to right:
//!
//! - **Crossings**: two segments through one point at least `3Kε` inside
//!   both spans. A definite no is a missed crossing.
//! - **Overlaps**: two segments along one line, overlapping by at least
//!   `3Kε`, every point of the overlap within `ε/3` of the other edge.
//!   A definite no is a missed overlap.
//!
//! An escalation is never wrong here; a definite no always is. Interval
//! is checked at every ε. `f64` is checked where its rounding holds,
//! which excludes the crossings at ε = 1e-12: for two edges that share
//! no point key, the closest point of their lines is read through the
//! closed form from their starts ([`ee_lines_closest`]), whose `f64`
//! rounding is about `|d|·u / sin φ`. At 1e-12 that exceeds `Kε` for
//! the nearly parallel crossings, which then read a definite span
//! outside an edge, while Interval escalates the same pairs
//! (`work/restread/the-census-crossing-lane-reads-a-pair-sharing-no-point-key-from-their-starts.md`).
//! Those are counted and logged, never asserted.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

// Gated to the readings it checks, and the band they decide in.
test_utils::gated_to![
    "crates/topo/src/census.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/tolerance.rs",
    "crates/geom-core/src/interval.rs",
];

use super::*;
use geom_core::Interval;
use test_utils::fuzz::Rng;

const K: f64 = 10.0;
const EPS: [f64; 3] = [1e-6, 1e-9, 1e-12];

type V = [f64; 3];

fn cross(a: V, b: V) -> V {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn norm(a: V) -> f64 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}

fn add(a: V, b: V, s: f64) -> V {
    [a[0] + b[0] * s, a[1] + b[1] * s, a[2] + b[2] * s]
}

fn unit(r: &mut Rng) -> V {
    loop {
        let v = [r.range(-1.0, 1.0), r.range(-1.0, 1.0), r.range(-1.0, 1.0)];
        let n = norm(v);
        if n > 0.1 && n < 1.0 {
            return v.map(|x| x / n);
        }
    }
}

/// A direction at angle `phi` off `a`.
fn turned(r: &mut Rng, a: V, phi: f64) -> V {
    let mut w = cross(a, unit(r));
    while norm(w) < 0.1 {
        w = cross(a, unit(r));
    }
    let w = w.map(|x| x / norm(w));
    add(a.map(|x| x * phi.cos()), w, phi.sin())
}

fn meets<T: Decide>((a0, a1): (V, V), (b0, b1): (V, V), band: Band) -> Option<bool> {
    let p = |v: V| Point3::new(T::from_f64(v[0]), T::from_f64(v[1]), T::from_f64(v[2]));
    let mut errors = Vec::new();
    interiors_meet(
        &segment((p(a0), p(a1))),
        &segment((p(b0), p(b1))),
        band,
        &mut errors,
    )
}

/// Each pair built by `pair` at every ε, read in both orders: the
/// readings that said a definite no, at Interval and at `f64` (the
/// latter only where `f64_holds`), and the `f64` noes excused.
fn falsify(
    label: &str,
    count: usize,
    f64_holds: fn(f64) -> bool,
    mut pair: impl FnMut(&mut Rng, f64) -> Option<((V, V), (V, V), String)>,
) {
    let mut r = test_utils::fuzz::start(label);
    let mut bad = Vec::new();
    for eps in EPS {
        let band = Band::new(eps, K * eps).unwrap();
        let mut excused = 0usize;
        for _ in 0..test_utils::fuzz::scaled(count) {
            let Some((a, b, what)) = pair(&mut r, eps) else {
                continue;
            };
            for (p, q) in [(a, b), (b, a)] {
                let iv = meets::<Interval>(p, q, band);
                let f = meets::<f64>(p, q, band);
                let f64_no = f == Some(false);
                if iv == Some(false) || (f64_no && f64_holds(eps)) {
                    bad.push(format!(
                        "ε {eps:e} {what}: f64 {f:?}, Interval {iv:?}, a {p:?}, b {q:?}"
                    ));
                } else if f64_no {
                    excused += 1;
                }
            }
        }
        println!("[fuzz] {label} at ε {eps:e}: {excused} f64 noes excused");
    }
    assert!(
        bad.is_empty(),
        "{label}: {} definite noes on pairs that meet; {}:\n{}",
        bad.len(),
        test_utils::fuzz::replay(),
        bad[..bad.len().min(8)].join("\n")
    );
}

/// **Two segments crossing strictly inside both spans never read
/// apart.**
#[test]
fn crossings_strictly_inside_both_spans_never_read_apart() {
    falsify(
        "census_segment_crossings",
        500,
        |eps| eps > 1e-12,
        |r, eps| {
            let x = [r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)];
            let a = unit(r);
            let phi = r.log_uniform(1e-14, 10f64.powf(0.2));
            let b = turned(r, a, phi);
            let m = 3.0 * K * eps;
            let la = r.log_uniform(7.0 * m, 10f64.powf(0.5));
            let lb = r.log_uniform(7.0 * m, 10f64.powf(0.5));
            let sa = r.range(m, la - m);
            let sb = r.range(m, lb - m);
            Some((
                (add(x, a, -sa), add(x, a, la - sa)),
                (add(x, b, -sb), add(x, b, lb - sb)),
                format!("φ {phi:e} la {la:e} lb {lb:e} sa {sa:e} sb {sb:e}"),
            ))
        },
    );
}

/// **Two segments overlapping within the zero never read apart.**
#[test]
fn overlaps_within_the_zero_never_read_apart() {
    falsify(
        "census_segment_overlaps",
        500,
        |_| true,
        |r, eps| {
            let x = [r.range(-2.0, 2.0), r.range(-2.0, 2.0), r.range(-2.0, 2.0)];
            let a = unit(r);
            let m = 3.0 * K * eps;
            let la = r.log_uniform(3.0 * m, 10f64.powf(0.5));
            let lb = r.log_uniform(3.0 * m, 10f64.powf(0.5));
            // `b` starts before `a`'s far end by at least `m`.
            let s = r.range(-lb + m, la - m);
            if la.min(s + lb) - s.max(0.0) < m {
                return None;
            }
            // Its turn and offset keep the overlap within ε/3 of `a`.
            let psi = r.unit() * (eps / 6.0) / la.max(lb);
            let b = turned(r, a, psi);
            let off = cross(a, unit(r));
            let off = off.map(|v| v / norm(off) * (r.unit() * eps / 6.0));
            let b0 = add(add(x, a, s), off, 1.0);
            Some((
                (x, add(x, a, la)),
                (b0, add(b0, b, lb)),
                format!("ψ {psi:e} la {la:e} lb {lb:e} s {s:e}"),
            ))
        },
    );
}
