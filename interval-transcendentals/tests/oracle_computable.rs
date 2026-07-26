//! Second, independent dev-oracle: Evan's `computable` library
//! (exact-real refinement with dyadic bounds). Diversity is the point —
//! it shares no code, no π constant, and no rounding model with inari.
//!
//! Coverage: the subset computable implements today — `sin` (native),
//! `cos` (as `sin(π/2 − x)`, composed), `sqrt` (`nth_root(2)`), `powi`
//! (`pow`, n ≥ 0). Point-sample containment: for dyadic sample points
//! `x` in the input interval, computable's refined enclosure of `f(x)`
//! (which contains the true `f(x)`) must lie INSIDE our interval result.
//! Run with `--features oracle-computable` (path dev-oracle; targeted
//! set, not millions — refinement is far slower than MPFR).
#![cfg(feature = "oracle-computable")]

mod common;

use common::Rng;
use computable::{Binary, Computable, XBinary, ops::pi::pi};
use interval_transcendentals::DInterval;

const RANDOM_CASES: u64 = 1500;

/// Refine `c` until its width is comfortably below `width_hint` (our
/// interval's width), then return the dyadic bounds. `None` if the
/// refinement budget is exhausted (caller counts and skips).
fn refined_bounds(c: &Computable, width_hint: f64) -> Option<(XBinary, XBinary)> {
    let exp = if width_hint > 0.0 && width_hint.is_finite() {
        (width_hint.log2().floor() as i32 - 8).clamp(-1080, 0)
    } else {
        -80
    };
    let p = c.refine_to_default(computable::XI::from_i32(exp)).ok()?;
    Some((p.lower(), p.upper()))
}

/// The containment assertion: computable's enclosure of the true value
/// must sit inside our interval.
fn assert_point_in(ctx: &str, mine: &DInterval, c: &Computable, skips: &mut u64) {
    let width = mine.hi() - mine.lo();
    let Some((lo, hi)) = refined_bounds(c, width) else {
        *skips += 1;
        return;
    };
    let my_lo = XBinary::from_f64(mine.lo()).expect("finite bound");
    let my_hi = XBinary::from_f64(mine.hi()).expect("finite bound");
    assert!(
        my_lo <= lo && hi <= my_hi,
        "{ctx}: computable-oracle CONTAINMENT VIOLATION: oracle in ({lo:?}, {hi:?}), mine {mine:?}"
    );
}

fn c_of(x: f64) -> Computable {
    Computable::from(Binary::from_f64(x).expect("finite dyadic"))
}

/// cos(x) as the composition sin(π/2 − x): exercises computable's pi op
/// and arithmetic combinators too.
fn c_cos(x: f64) -> Computable {
    let half = Computable::from(Binary::from_f64(0.5).expect("dyadic"));
    (pi() * half - c_of(x)).sin()
}

#[test]
fn computable_oracle_sin_cos_points() {
    let mut rng = Rng(0xC0FFEE01);
    let mut skips = 0;
    for i in 0..RANDOM_CASES {
        let x = rng.range(-40.0, 40.0);
        let mine_s = DInterval::point(x).sin();
        assert_point_in(
            &format!("sin({x}) case {i}"),
            &mine_s,
            &c_of(x).sin(),
            &mut skips,
        );
        let mine_c = DInterval::point(x).cos();
        assert_point_in(
            &format!("cos({x}) case {i}"),
            &mine_c,
            &c_cos(x),
            &mut skips,
        );
    }
    println!(
        "computable-oracle sin/cos points: {} cases, {skips} skips",
        2 * RANDOM_CASES
    );
    assert!(
        skips < RANDOM_CASES / 4,
        "too many refinement failures to certify"
    );
}

#[test]
fn computable_oracle_sin_wide_intervals() {
    // Interval-logic check (extremum capture): sample points of a wide
    // input must land inside the interval result.
    let mut rng = Rng(0xC0FFEE02);
    let mut skips = 0;
    for i in 0..500 {
        let a = rng.range(-30.0, 30.0);
        let w = rng.unit() * 8.0;
        let x = DInterval::from_bounds(a, a + w);
        let s = x.sin();
        for t in [a, a + 0.5 * w, a + w] {
            assert_point_in(
                &format!("sin wide case {i} t={t}"),
                &s,
                &c_of(t).sin(),
                &mut skips,
            );
        }
    }
    println!("computable-oracle sin wide: 1500 samples, {skips} skips");
    assert!(skips < 300);
}

#[test]
fn computable_oracle_sqrt_pow_points() {
    let mut rng = Rng(0xC0FFEE03);
    let mut skips = 0;
    for i in 0..RANDOM_CASES {
        let x = rng.log_mag(-80, 80).abs();
        let mine = DInterval::point(x).sqrt();
        let c = c_of(x).nth_root(core::num::NonZeroU32::new(2).expect("2 != 0"));
        assert_point_in(&format!("sqrt({x:e}) case {i}"), &mine, &c, &mut skips);
        let n = (rng.next_u64() % 6 + 2) as u32;
        let y = rng.range(-20.0, 20.0);
        let mine_p = DInterval::point(y).powi(n as i32);
        assert_point_in(
            &format!("pow({y}, {n}) case {i}"),
            &mine_p,
            &c_of(y).pow(n),
            &mut skips,
        );
    }
    println!(
        "computable-oracle sqrt/pow points: {} cases, {skips} skips",
        2 * RANDOM_CASES
    );
    assert!(skips < RANDOM_CASES / 4);
}

#[test]
fn computable_oracle_edge_points() {
    let mut skips = 0;
    let edges = [
        0.0,
        -0.0,
        f64::MIN_POSITIVE,
        1.0,
        core::f64::consts::FRAC_PI_2,
        core::f64::consts::PI,
        -core::f64::consts::PI,
        core::f64::consts::TAU,
        1e6,
        -1e6,
        1e12,
    ];
    for x in edges {
        assert_point_in(
            &format!("edge sin({x})"),
            &DInterval::point(x).sin(),
            &c_of(x).sin(),
            &mut skips,
        );
        assert_point_in(
            &format!("edge cos({x})"),
            &DInterval::point(x).cos(),
            &c_cos(x),
            &mut skips,
        );
    }
    assert_eq!(skips, 0, "edge points must all certify");
}
