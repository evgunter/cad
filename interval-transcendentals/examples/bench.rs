//! Informational microbenchmark: this crate vs inari(+MPFR) on the
//! inventoried entry points. (Example, not criterion: the numbers are
//! evidence, not science.)
//!
//! It compares against the oracle, so it needs the oracle — and the
//! oracle needs inari's AVX+FMA floor:
//!
//! ```text
//! RUSTFLAGS="-C target-cpu=x86-64-v3" \
//!   cargo run --release --features oracle-inari --example bench
//! ```
//!
//! `Cargo.toml` gives this target `required-features = ["oracle-inari"]`,
//! which is what keeps the oracle's C build out of the default
//! `cargo test` graph. It also means the featureless spellings do not
//! run it: `cargo run --example bench` REFUSES ("requires the features:
//! `oracle-inari`") and `cargo build --examples` matches no target at
//! all. Both are loud; neither is this line's job to hide.

use std::hint::black_box;
use std::time::Instant;

use interval_transcendentals::DInterval;

const N: usize = 1_000_000;

fn inputs() -> Vec<(f64, f64)> {
    // Deterministic mix of tight and wide intervals over ±40.
    (0..N)
        .map(|i| {
            let a = (i as f64).mul_add(7.61803398875e-5, -40.0) % 40.0;
            let w = ((i % 97) as f64) * 1e-3;
            (a, a + w)
        })
        .collect()
}

fn bench<T>(label: &str, f: impl Fn(&(f64, f64)) -> T) {
    let xs = inputs();
    let t0 = Instant::now();
    for x in &xs {
        black_box(f(black_box(x)));
    }
    let dt = t0.elapsed();
    println!(
        "{label:>14}: {:>7.1} ns/op",
        dt.as_nanos() as f64 / N as f64
    );
}

fn main() {
    println!("interval-transcendentals (mine) vs inari+MPFR (oracle), {N} ops each");
    let mine = |lo: f64, hi: f64| DInterval::from_bounds(lo, hi);
    let ora = |lo: f64, hi: f64| inari::DecInterval::try_from((lo, hi)).expect("valid");

    bench("mine sin", |&(a, b)| mine(a, b).sin());
    bench("inari sin", |&(a, b)| ora(a, b).sin());
    bench("mine cos", |&(a, b)| mine(a, b).cos());
    bench("inari cos", |&(a, b)| ora(a, b).cos());
    bench("mine tan", |&(a, b)| mine(a, b).tan());
    bench("inari tan", |&(a, b)| ora(a, b).tan());
    bench("mine atan", |&(a, b)| mine(a, b).atan());
    bench("inari atan", |&(a, b)| ora(a, b).atan());
    bench("mine atan2", |&(a, b)| {
        mine(a, b).atan2(mine(b + 1.0, b + 2.0))
    });
    bench("inari atan2", |&(a, b)| {
        ora(a, b).atan2(ora(b + 1.0, b + 2.0))
    });
    bench("mine asin", |&(a, b)| mine(a / 64.0, b / 64.0).asin());
    bench("inari asin", |&(a, b)| ora(a / 64.0, b / 64.0).asin());
    bench("mine sqrt", |&(a, b)| {
        mine(a.abs(), a.abs() + b.abs()).sqrt()
    });
    bench("inari sqrt", |&(a, b)| {
        ora(a.abs(), a.abs() + b.abs()).sqrt()
    });
    bench("mine powi2", |&(a, b)| mine(a, b).powi(2));
    bench("inari powi2", |&(a, b)| ora(a, b).powi(2));
    bench("mine mul", |&(a, b)| mine(a, b) * mine(b, b + 1.0));
    bench("inari mul", |&(a, b)| ora(a, b) * ora(b, b + 1.0));
}
