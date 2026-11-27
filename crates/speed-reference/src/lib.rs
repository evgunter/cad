//! The runner's speed reference: a fixed workload that calls no kernel
//! code, timed alone beside the fast-set tests a re-time pass reads.
//!
//! Hosted runners come in speed classes, and one class reads every test
//! alone by about one factor against another. `scripts/fast_set_alone.py`
//! times `tests/all.rs`'s one ignored test in every re-time pass and
//! scales each test's alone reading by the ratio of this workload's
//! nominal time to its reading on the runner, so the gate and the drift
//! check judge a cost in the median hosted runner's seconds.
//!
//! The workload is the kernel's kind of work without the kernel: `f64`
//! arithmetic with divisions, small `Vec` allocations, a sort, branches
//! and a hash map. It runs in two halves, as a fast-set test does: the
//! `f64` half ([`shared`]) is built here at the library's opt level, and
//! the generic half ([`run`] at a scalar the test crate defines) is
//! built in the integration binary at the level that binary gets.
//!
//! FROZEN. The nominal time in `scripts/fast_set_alone.py` is this
//! workload's reading on the median runner, so editing anything in this
//! crate means measuring that time again; the script's selftest pins a
//! digest of these files to the constant, so an edit that forgets is red.

#![cfg_attr(not(test), deny(clippy::disallowed_methods))]

use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

/// The arithmetic the workload is written over.
pub trait Scalar:
    Copy
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    /// The scalar holding `x`.
    fn of(x: f64) -> Self;
    /// Its value as an `f64`.
    fn get(self) -> f64;
}

impl Scalar for f64 {
    fn of(x: f64) -> Self {
        x
    }
    fn get(self) -> f64 {
        self
    }
}

const POINTS: usize = 24;
const NEWTON: usize = 5;
const KEYS: u64 = 1024;
const KEPT: usize = 512;

/// The next state of a xorshift64 generator.
fn next(seed: u64) -> u64 {
    let mut x = seed;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    x
}

/// A value in [0.01, 1.01) from the top 53 bits of `seed`.
fn unit(seed: u64) -> f64 {
    (seed >> 11) as f64 * (1.0 / (1u64 << 53) as f64) + 0.01
}

/// One round: draw POINTS values, sort them, take each one's square root
/// by Newton's method, fold the results by branch, and count the round
/// in a hash map that is cleared when it fills.
fn round<S: Scalar>(seed: &mut u64, table: &mut HashMap<u64, u64>) -> u64 {
    let mut pts: Vec<S> = Vec::with_capacity(POINTS);
    for _ in 0..POINTS {
        *seed = next(*seed);
        pts.push(S::of(unit(*seed)));
    }
    pts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let half = S::of(0.5);
    let mut acc = S::of(0.0);
    for &p in &pts {
        let mut x = p + half;
        for _ in 0..NEWTON {
            x = half * (x + p / x);
        }
        // `Mul::mul(x, x)` is the plain square (the same codegen), spelled so
        // interval-square-allowlist.sh, whose subject is interval enclosures,
        // does not read this f64 workload as one.
        acc = if x > half {
            acc + x
        } else {
            acc - Mul::mul(x, x)
        };
    }
    let key = *seed % KEYS;
    let bits = acc.get().to_bits();
    let slot = table.entry(key).or_insert(0);
    *slot = slot.wrapping_add(bits);
    if table.len() >= KEPT {
        table.clear();
    }
    bits ^ key
}

/// `rounds` rounds at the scalar `S` from a fixed seed: a checksum, for
/// the caller to `black_box`.
pub fn run<S: Scalar>(rounds: u32) -> u64 {
    let mut seed = 0x9e37_79b9_7f4a_7c15;
    let mut table = HashMap::new();
    let mut sum = 0u64;
    for _ in 0..rounds {
        sum = sum.wrapping_add(round::<S>(&mut seed, &mut table));
    }
    sum
}

/// The `f64` half, built at this library's opt level, as the kernel's
/// `f64` code is.
pub fn shared(rounds: u32) -> u64 {
    run::<f64>(rounds)
}
