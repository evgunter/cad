//! Reviewer probe (PR 7146): prints `less_sin` at every scalar for an
//! external high-precision oracle. Asserts nothing.
#![allow(clippy::unwrap_used, clippy::print_stdout)]
use geom_core::interval::Interval;
use geom_core::{Bounds, Dual, Real};

fn boxes() -> Vec<(f64, f64)> {
    let mut v = vec![
        (0.0, 0.0), (-0.0, 0.0), (5e-324, 5e-324), (-1e-300, 1e-300), (1e-160, 1e-160),
        (1e-8, 1e-8), (1e-3, 1e-3), (-1e-3, 2e-3), (-2.0, 2.0), (-2.0, -2.0), (2.0, 2.0),
        (1.999_999_999_999_999_8, 2.0), (2.0, 2.000_000_000_000_000_4), (1.9, 2.1), (-2.1, -1.9),
        (-3.0, 3.0), (-1e-9, 2.5), (0.5, 1.5), (-7.0, 7.0), (1e10, 1e10 + 1.0), (-1e300, 1e300),
        (1.0, f64::INFINITY), (f64::NEG_INFINITY, f64::INFINITY), (0.7226382429702543, 0.7226382429702543),
        (1.0, 1.0), (1.5, 1.75), (-1.0, -0.5), (3.0, 3.0),
    ];
    let mut s: u64 = 0x2545_f491_4f6c_dd1d;
    let mut r = || {
        s ^= s << 13; s ^= s >> 7; s ^= s << 17;
        (s >> 11) as f64 / (1u64 << 53) as f64
    };
    for _ in 0..4000 {
        let scale = 2f64.powi(-((r() * 40.0) as i32)) * 2.2;
        let a = (2.0 * r() - 1.0) * scale;
        let b = a + r() * scale * [1e-12, 1e-6, 1e-2, 1.0][(r() * 4.0) as usize];
        v.push((a, b));
    }
    v
}

#[test]
#[ignore = "reviewer probe"]
fn review_less_sin_dump() {
    for x in [0.0, -0.0, 5e-324, 1e-300, 1e-160, 1e-103, 1e-8, 1e-3, 0.5, 1.0, 1.999_999_999_999_999_8, 2.0,
              2.000_000_000_000_000_4, 3.0, 1e300, f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
        println!("F {x:e} {:e}", x.less_sin());
    }
    for (lo, hi) in boxes() {
        let i = Interval::from_bounds(lo, hi);
        let o = i.less_sin();
        println!("I {lo:e} {hi:e} {:e} {:e} {}", o.lo(), o.hi(), o.is_certified());
        let d = Dual::variable(i).less_sin();
        println!("DI {lo:e} {hi:e} {:e} {:e} {:e} {:e}", d.value.lo(), d.value.hi(), d.deriv.lo(), d.deriv.hi());
    }
    for x in [1e-300, 1e-160, 1e-8, 1e-3, 0.5, 1.9, 2.0, 2.5, 3.0, 6.0, -1e-5] {
        let d = Dual::variable(x).less_sin();
        println!("D {x:e} {:e} {:e} {}", d.value, d.deriv, d.value.to_bits() == x.less_sin().to_bits());
    }
}
