//! Basic interval arithmetic with outward pads (see `round.rs`).
//! Tightness note: inari's arithmetic is correctly rounded (directed
//! hardware rounding); ours pads at most 1 ulp per inexact endpoint —
//! a documented tightness gap, never a soundness gap.

use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::interval::{DInterval, Decoration};
use crate::ops::{lower_of, upper_of};
use crate::round::{add_hi, add_lo, div_hi, div_lo, mul_hi, mul_lo, sub_hi, sub_lo};

impl Add for DInterval {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if let Some(p) = Self::propagate2(&self, &rhs) {
            return p;
        }
        let dec = self.dec.min(rhs.dec);
        Self::make(add_lo(self.lo, rhs.lo), add_hi(self.hi, rhs.hi), dec)
    }
}

impl Sub for DInterval {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if let Some(p) = Self::propagate2(&self, &rhs) {
            return p;
        }
        let dec = self.dec.min(rhs.dec);
        Self::make(sub_lo(self.lo, rhs.hi), sub_hi(self.hi, rhs.lo), dec)
    }
}

impl Neg for DInterval {
    type Output = Self;
    fn neg(self) -> Self {
        if let Some(p) = Self::propagate1(&self) {
            return p;
        }
        Self::make(-self.hi, -self.lo, self.dec)
    }
}

impl Mul for DInterval {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        if let Some(p) = Self::propagate2(&self, &rhs) {
            return p;
        }
        let dec = self.dec.min(rhs.dec);
        // A point operand's corners coincide, so its corner products are
        // the general fold's own values without the repeats, and the
        // fold's extremes over them are the same bits. That holds for a
        // `[-0, +0]` point too: `mul_lo`/`mul_hi` return `+0` for any zero
        // factor, so its two corners give the same products.
        let (lo, hi) = match (self.lo == self.hi, rhs.lo == rhs.hi) {
            (true, true) => (mul_lo(self.lo, rhs.lo), mul_hi(self.lo, rhs.lo)),
            (true, false) => corner_bounds(&[self.lo], &[rhs.lo, rhs.hi]),
            (false, true) => corner_bounds(&[self.lo, self.hi], &[rhs.lo]),
            (false, false) => corner_bounds(&[self.lo, self.hi], &[rhs.lo, rhs.hi]),
        };
        Self::make(lo, hi, dec)
    }
}

/// The extremes of every corner product `x·y`, `x` from `xs` and `y`
/// from `ys`, each with directed pads; `mul_lo`/`mul_hi` apply the
/// `0·inf := 0` corner convention internally.
fn corner_bounds(xs: &[f64], ys: &[f64]) -> (f64, f64) {
    let corners = || xs.iter().flat_map(|&x| ys.iter().map(move |&y| (x, y)));
    let lo = corners()
        .map(|(x, y)| mul_lo(x, y))
        .fold(f64::INFINITY, |a, b| lower_of(a, b, f64::min));
    let hi = corners()
        .map(|(x, y)| mul_hi(x, y))
        .fold(f64::NEG_INFINITY, |a, b| upper_of(a, b, f64::max));
    (lo, hi)
}

impl Div for DInterval {
    type Output = Self;
    /// IEEE 1788 division: the hull of `{x/y : x in self, y in rhs, y != 0}`.
    /// A divisor touching zero is a domain violation: decoration drops to
    /// `Trv`; a divisor identically `[0,0]` yields Empty.
    fn div(self, rhs: Self) -> Self {
        if let Some(p) = Self::propagate2(&self, &rhs) {
            return p;
        }
        let dec = self.dec.min(rhs.dec);
        if rhs.lo == 0.0 && rhs.hi == 0.0 {
            return Self::empty(); // dec Trv by construction
        }
        if rhs.lo < 0.0 && rhs.hi > 0.0 {
            // Divisor straddles zero: hull is the whole line.
            let mut r = Self::entire();
            r.dec = Decoration::Trv;
            return r;
        }
        // Divisor is one-signed; it may still TOUCH zero at one endpoint
        // (domain violation at that endpoint: unbounded hull, dec Trv).
        let touches_zero = rhs.lo == 0.0 || rhs.hi == 0.0;
        let dec = if touches_zero { Decoration::Trv } else { dec };
        let (lo, hi) = if touches_zero {
            div_touching_zero(self, rhs)
        } else {
            let l = [
                div_lo(self.lo, rhs.lo),
                div_lo(self.lo, rhs.hi),
                div_lo(self.hi, rhs.lo),
                div_lo(self.hi, rhs.hi),
            ];
            let h = [
                div_hi(self.lo, rhs.lo),
                div_hi(self.lo, rhs.hi),
                div_hi(self.hi, rhs.lo),
                div_hi(self.hi, rhs.hi),
            ];
            (
                l.into_iter()
                    .fold(f64::INFINITY, |a, b| lower_of(a, b, f64::min)),
                h.into_iter()
                    .fold(f64::NEG_INFINITY, |a, b| upper_of(a, b, f64::max)),
            )
        };
        Self::make(lo, hi, dec)
    }
}

/// Hull of `x / y` when the one-signed divisor `y` touches zero at one
/// endpoint: the finite bound comes from the nonzero divisor endpoint,
/// the other side is unbounded (unless the numerator contains 0, in which
/// case the hull is the whole line).
fn div_touching_zero(x: DInterval, y: DInterval) -> (f64, f64) {
    if x.lo <= 0.0 && x.hi >= 0.0 {
        return (f64::NEG_INFINITY, f64::INFINITY);
    }
    // The nonzero divisor endpoint (finite-side corner).
    let d = if y.lo == 0.0 { y.hi } else { y.lo };
    let num_pos = x.lo > 0.0;
    let div_pos = d > 0.0;
    match (num_pos, div_pos) {
        // x > 0, y = [0, d>0]: x/y in [x.lo/d, +inf)
        (true, true) => (div_lo(x.lo, d), f64::INFINITY),
        // x > 0, y = [d<0, 0]: x/y in (-inf, x.lo/d]
        (true, false) => (f64::NEG_INFINITY, div_hi(x.lo, d)),
        // x < 0, y = [0, d>0]: x/y in (-inf, x.hi/d]
        (false, true) => (f64::NEG_INFINITY, div_hi(x.hi, d)),
        // x < 0, y = [d<0, 0]: x/y in [x.hi/d, +inf)
        (false, false) => (div_lo(x.hi, d), f64::INFINITY),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::round::TWO_PROD_VALID_MIN;

    /// Endpoints at every edge the corner products have: signed zeros,
    /// the subnormal range and the 2Prod floor, exact and inexact
    /// products, the extremes of the finite range and the infinities.
    const EDGES: [f64; 22] = [
        f64::NEG_INFINITY,
        -f64::MAX,
        -3.0,
        -1.0 / 3.0,
        -TWO_PROD_VALID_MIN,
        -5e-324,
        -0.0,
        0.0,
        5e-324,
        2.2250738585072014e-308,
        TWO_PROD_VALID_MIN,
        1e-300,
        0.1,
        1.0 / 3.0,
        0.375,
        1.0,
        1.0 + f64::EPSILON,
        3.0,
        9007199254740993.0,
        1e300,
        f64::MAX,
        f64::INFINITY,
    ];

    /// **A point operand takes a shorter fold, with the general fold's
    /// bits.** Every product of two intervals over [`EDGES`] (points
    /// included) against the four-corner fold on the same endpoints:
    /// bounds bit for bit, decoration equal.
    #[test]
    fn a_point_product_is_the_four_corner_fold() {
        let intervals: Vec<DInterval> = EDGES
            .iter()
            .flat_map(|&lo| EDGES.iter().map(move |&hi| (lo, hi)))
            .filter(|&(lo, hi)| lo <= hi && lo != f64::INFINITY && hi != f64::NEG_INFINITY)
            .map(|(lo, hi)| DInterval::from_bounds(lo, hi))
            .flat_map(|x| [x, x.with_dec_capped(crate::interval::Decoration::Def)])
            .collect();
        let mut points = 0;
        for &x in &intervals {
            for &y in &intervals {
                points += usize::from(x.lo == x.hi || y.lo == y.hi);
                let got = x * y;
                let (lo, hi) = corner_bounds(&[x.lo, x.hi], &[y.lo, y.hi]);
                let want = DInterval::make(lo, hi, x.dec.min(y.dec));
                assert_eq!(
                    (got.lo.to_bits(), got.hi.to_bits(), got.dec),
                    (want.lo.to_bits(), want.hi.to_bits(), want.dec),
                    "{x:?} * {y:?}"
                );
            }
        }
        assert!(
            points > 1000,
            "the table reaches the point paths: {points} products"
        );
    }
}
