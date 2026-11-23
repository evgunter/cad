//! **`Certification::between` against exact rationals** — a
//! counterexample search over adversarial brackets and ratios: endpoints
//! at zero, subnormals, ±1e300 and ulps from each; ratios at and an ulp
//! past either end of `[0, 1]`, at `½ ± ulp`, and as wide as `[0, 1]`.
//!
//! For every sampled `x`, `y`, `Λ`, the door's answer must contain the
//! exact combination `(1 − λ)·x + λ·y` at every corner of the inputs and
//! at four `λ` across `Λ ∩ [0, 1]`, and must lie in `hull(x, y)`; a `Λ`
//! with no point in `[0, 1]` must refuse. The combination is affine in
//! each of `x` and `y`, so the corners are its extremes over the two
//! brackets at each `λ`. Adopted from a review probe of the door's PR.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-core/src/interval/certification.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/tests/exact_q/"
];

use crate::exact_q::Q;
use geom_core::Bounds;
use geom_core::Interval;
use geom_core::interval::certification::Certification;
use test_utils::fuzz;

fn ulps(x: f64, k: i64) -> f64 {
    let mut y = x;
    for _ in 0..k.unsigned_abs() {
        y = if k > 0 { y.next_up() } else { y.next_down() };
    }
    y
}

fn pick(r: &mut fuzz::Rng, xs: &[f64]) -> f64 {
    xs[r.below(xs.len())]
}

#[allow(clippy::cast_possible_wrap)]
fn scalar(r: &mut fuzz::Rng) -> f64 {
    let base = pick(
        r,
        &[
            0.0, -0.0, 1.0, -1.0, 0.5, 1e-300, 5e-324, 1e-310, 1e300, -1e300, 1e3, 1e-9, 3.0,
        ],
    );
    let v = match r.below(4) {
        0 => base,
        1 => base * r.range(-1.0, 1.0),
        2 => ulps(base, r.below(9) as i64 - 4),
        _ => r.range(-5.0, 5.0),
    };
    if v.is_finite() { v } else { base }
}

#[allow(clippy::cast_possible_wrap)]
fn ordered(a: f64, b: f64) -> (f64, f64) {
    if a <= b { (a, b) } else { (b, a) }
}

#[allow(clippy::cast_possible_wrap)]
fn bracket(r: &mut fuzz::Rng) -> (f64, f64) {
    let a = scalar(r);
    let b = match r.below(3) {
        0 => a,
        1 => ulps(a, r.below(6) as i64),
        _ => scalar(r),
    };
    ordered(a, b)
}

#[allow(clippy::cast_possible_wrap)]
fn ratio(r: &mut fuzz::Rng) -> (f64, f64) {
    let pts = [
        0.0,
        -0.0,
        1.0,
        0.5,
        ulps(0.5, 1),
        ulps(0.5, -1),
        ulps(1.0, 1),
        ulps(0.0, -1),
        5e-324,
        ulps(1.0, -1),
        -1e-17,
        1.0 + 1e-15,
    ];
    let one = |r: &mut fuzz::Rng| {
        if r.below(2) == 0 {
            pick(r, &pts)
        } else {
            r.unit()
        }
    };
    let a = one(r);
    let b = match r.below(3) {
        0 => a,
        1 => ulps(a, r.below(4) as i64),
        _ => one(r),
    };
    ordered(a, b)
}

#[test]
fn between_encloses_every_exact_combination() {
    let mut r = fuzz::start("between-exact");
    let (mut checked, mut refused) = (0usize, 0usize);
    let draws = fuzz::scaled(1_000);
    for _ in 0..draws {
        let ((xl, xh), (yl, yh), (ll, lh)) = (bracket(&mut r), bracket(&mut r), ratio(&mut r));
        let (x, y, l) = (
            Interval::from_bounds(xl, xh),
            Interval::from_bounds(yl, yh),
            Interval::from_bounds(ll, lh),
        );
        let out = Interval::between(x, y, l);
        let (lam_lo, lam_hi) = (ll.max(0.0), lh.min(1.0));
        if lam_lo > lam_hi {
            assert!(
                !out.is_certified(),
                "Λ {l:?} has no point in [0, 1] and certified {out:?} [{}]",
                fuzz::replay()
            );
            continue;
        }
        if !out.is_certified() {
            refused += 1;
            continue;
        }
        assert!(
            out.lo() >= xl.min(yl) && out.hi() <= xh.max(yh),
            "between({x:?}, {y:?}, {l:?}) = {out:?} left the sources' hull [{}]",
            fuzz::replay()
        );
        let one = Q::from_f64(1.0);
        for lam in [
            lam_lo,
            lam_hi,
            0.5f64.clamp(lam_lo, lam_hi),
            0.5 * (lam_lo + lam_hi),
        ] {
            let lq = Q::from_f64(lam);
            let rest = one.sub(&lq);
            for xv in [xl, xh] {
                for yv in [yl, yh] {
                    let v = rest.mul(&Q::from_f64(xv)).add(&lq.mul(&Q::from_f64(yv)));
                    assert!(
                        v.cmp_f64(out.lo()) != core::cmp::Ordering::Less
                            && v.cmp_f64(out.hi()) != core::cmp::Ordering::Greater,
                        "between({x:?}, {y:?}, {l:?}) = {out:?} misses λ={lam:e} x={xv:e} \
                         y={yv:e} [{}]",
                        fuzz::replay()
                    );
                    checked += 1;
                }
            }
        }
    }
    // A finite-input refusal is the overflowing chord, rare by design of
    // the draw: counted aloud and held well under the draw count.
    assert!(
        refused * 20 < draws,
        "{refused} of {draws} finite draws refused [{}]",
        fuzz::replay()
    );
    println!("between: {checked} exact checks, {refused} refusals over {draws} draws");
}

/// **The door's own clamp is the only thing its ratio meets.** For
/// ratios inside, on and outside `[0, 1]` (points and brackets, signed
/// zeros, subnormals, an ulp past either end) and sources of every
/// shape, the door's answer is bit for bit the answer to the same
/// question with the ratio clamped first: a ratio it reads unclamped
/// must be one the clamp leaves as it is.
#[test]
fn a_ratio_is_read_as_its_clamp() {
    let edges = [
        -1.0,
        ulps(0.0, -1),
        -0.0,
        0.0,
        5e-324,
        1e-300,
        0.1,
        1.0 / 3.0,
        ulps(0.5, -1),
        0.5,
        ulps(0.5, 1),
        0.875,
        ulps(1.0, -1),
        1.0,
        ulps(1.0, 1),
        2.0,
    ];
    let sources = [
        (1.0, 1.0),
        (-3.0, 2.5),
        (1e300, 1e300),
        (-0.0, 5e-324),
        (0.1, 0.3),
    ];
    let mut checked = 0;
    for (i, &ll) in edges.iter().enumerate() {
        for &lh in &edges[i..] {
            let l = Interval::from_bounds(ll, lh);
            let clamped = l.clamped_to(0.0, 1.0);
            for &(xl, xh) in &sources {
                for &(yl, yh) in &sources {
                    let (x, y) = (Interval::from_bounds(xl, xh), Interval::from_bounds(yl, yh));
                    assert_eq!(
                        Interval::between(x, y, l).repr_bits(),
                        Interval::between(x, y, clamped).repr_bits(),
                        "between({x:?}, {y:?}, {l:?}) differs from the clamped ratio {clamped:?}"
                    );
                    checked += 1;
                }
            }
        }
    }
    assert_eq!(checked, 136 * 25, "every ratio and source pair was asked");
}
