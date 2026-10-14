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
        let (lo, hi) = corner_bounds::<Product>(self, rhs);
        Self::make(lo, hi, dec)
    }
}

impl DInterval {
    /// **`self · k` for `k` a positive normal power of two**, exact
    /// wherever the result is above the smallest normal, without the
    /// general product's corner fold or its exactness witnesses.
    ///
    /// Scaling a finite `x` by a power of two only moves its exponent,
    /// so `x·k` is exact whenever the exact result is normal, and the
    /// shortcut returns `[lo·k, hi·k]` unpadded when both ROUNDED
    /// endpoints land strictly above the smallest normal (or are zero):
    /// an exact product just under it rounds up onto it
    /// ([`exact_scale`] reads the boundary). Against the general product
    /// `self * DInterval::point(k)`:
    ///
    /// - at `|x·k| >= TWO_PROD_VALID_MIN` that product certifies the
    ///   same exact `x·k` through its 2Prod witness, so the two agree
    ///   bit for bit;
    /// - between the smallest normal and that floor the witness cannot
    ///   certify, so the general product pads one step outward and the
    ///   shortcut is the exact value inside it;
    /// - every other case — an endpoint scaled into the subnormal range,
    ///   where round-to-nearest would not be outward; an overflow; an
    ///   infinite endpoint; a poisoned or empty operand; a `k` that is
    ///   not a positive normal power of two — IS the general product.
    #[must_use]
    pub fn scale_pow2(self, k: f64) -> Self {
        if Self::propagate1(&self).is_none()
            && is_normal_pow2(k)
            && let (Some(lo), Some(hi)) = (exact_scale(self.lo, k), exact_scale(self.hi, k))
        {
            return Self::make(lo, hi, self.dec);
        }
        self * Self::point(k)
    }
}

impl DInterval {
    /// **`self · w` for a multiplier `w` with `w.lo >= 0`**, bit for bit
    /// the general product, reading two corners where it can.
    ///
    /// With `w` non-negative, the extreme EXACT corners are fixed by the
    /// signs of `self`'s endpoints: the least is `self.lo · w.lo` when
    /// `self.lo >= 0` and `self.lo · w.hi` otherwise, the greatest
    /// `self.hi · w.hi` when `self.hi >= 0` and `self.hi · w.lo`
    /// otherwise. The four-corner fold takes the least of the PADDED
    /// corners, and that is the selected corner's own [`mul_lo`] unless
    /// the selected product is exact: a padded `mul_lo` is `down1` of a
    /// rounded value, and every other corner rounds at or above that
    /// value, so none pads below it; but an exact selected corner keeps
    /// its value unpadded, and a neighbour that rounds onto the same
    /// value and pads one step reads below it (and a neighbour that
    /// underflows to zero pads past an exact zero). So the selected
    /// corner's bound is returned when it was padded, and the general
    /// product decides when it was exact (mirror for `hi`). A point `w`
    /// is the general product too, which already folds two corners
    /// there, as is a `w` with a negative lower end, and a poisoned or
    /// empty operand.
    #[must_use]
    pub fn mul_nonneg(self, w: Self) -> Self {
        if Self::propagate2(&self, &w).is_some() || w.lo < 0.0 || w.lo == w.hi {
            return self * w;
        }
        let (a, b) = if self.lo >= 0.0 {
            (self.lo, w.lo)
        } else {
            (self.lo, w.hi)
        };
        let lo = mul_lo(a, b);
        let (c, e) = if self.hi >= 0.0 {
            (self.hi, w.hi)
        } else {
            (self.hi, w.lo)
        };
        let hi = mul_hi(c, e);
        if unpadded(a, b, lo) || unpadded(c, e, hi) {
            return self * w;
        }
        Self::make(lo, hi, self.dec.min(w.dec))
    }
}

/// Whether a corner bound `v` of `x · y` is the product itself rather
/// than a padded step: a zero factor (its exact zero), or `v` equal to
/// the rounded product.
fn unpadded(x: f64, y: f64, v: f64) -> bool {
    x == 0.0 || y == 0.0 || v == x * y
}

/// `k > 0` with an all-zero significand at a normal exponent.
fn is_normal_pow2(k: f64) -> bool {
    const SIGNIFICAND: u64 = (1 << 52) - 1;
    k > 0.0 && k.is_normal() && k.to_bits() & SIGNIFICAND == 0
}

/// `x·k` when it is exact for `k` a power of two: an exact zero (as
/// `+0.0`, which is what [`mul_lo`]/[`mul_hi`] return for a zero
/// factor), or a finite product STRICTLY above the smallest normal.
///
/// The test reads the rounded product, so the boundary is strict: an
/// exact product at or above the smallest normal is representable,
/// but one just below it (`MIN_POSITIVE − 2⁻¹⁰⁷⁵`) rounds UP onto
/// `MIN_POSITIVE`, so a rounded product of exactly `MIN_POSITIVE` may
/// not be exact. Anything rounding strictly above it was normal before
/// rounding, hence exact. The one exact product given up, `±MIN_POSITIVE`
/// itself, takes the general product.
fn exact_scale(x: f64, k: f64) -> Option<f64> {
    if x == 0.0 {
        return Some(0.0);
    }
    let r = x * k;
    (r.is_finite() && r.abs() > f64::MIN_POSITIVE).then_some(r)
}

/// A directed corner operation: the lower and the upper bound of one
/// corner value `x ∘ y`, one type per operation so the two bounds come
/// as a pair and cannot be swapped.
trait CornerOp {
    fn lo(x: f64, y: f64) -> f64;
    fn hi(x: f64, y: f64) -> f64;
}

/// `×`'s corners, with `0·inf := 0` built into `mul_lo`/`mul_hi`.
struct Product;

impl CornerOp for Product {
    fn lo(x: f64, y: f64) -> f64 {
        mul_lo(x, y)
    }
    fn hi(x: f64, y: f64) -> f64 {
        mul_hi(x, y)
    }
}

/// `÷`'s corners, for a divisor that keeps clear of zero.
struct Quotient;

impl CornerOp for Quotient {
    fn lo(x: f64, y: f64) -> f64 {
        div_lo(x, y)
    }
    fn hi(x: f64, y: f64) -> f64 {
        div_hi(x, y)
    }
}

/// The extremes of `Op` over the corners of `x` and `y`: `Op::lo` folded
/// to the least and `Op::hi` to the greatest, from `±inf`.
///
/// A point operand's two endpoints are one, so its corners repeat, and
/// each operand contributes only its distinct endpoints. The repeated
/// values change neither extreme (`min` and `max` are idempotent, and
/// `lower_of`/`upper_of`'s signed-zero rule asks only whether a `-0` or
/// `+0` occurs), so the bits are the four-corner fold's. A `[-0, +0]`
/// point is one point here, which is safe because `mul_lo`/`mul_hi` and
/// `div_lo`/`div_hi` return `+0` for a zero factor or numerator at either
/// sign. (No lower corner is ever `-0`, so the least's zero rule never
/// fires.)
fn corner_bounds<Op: CornerOp>(x: DInterval, y: DInterval) -> (f64, f64) {
    match (x.lo == x.hi, y.lo == y.hi) {
        (true, true) => fold::<Op, 1>([(x.lo, y.lo)]),
        (true, false) => fold::<Op, 2>([(x.lo, y.lo), (x.lo, y.hi)]),
        (false, true) => fold::<Op, 2>([(x.lo, y.lo), (x.hi, y.lo)]),
        (false, false) => fold::<Op, 4>([(x.lo, y.lo), (x.lo, y.hi), (x.hi, y.lo), (x.hi, y.hi)]),
    }
}

fn fold<Op: CornerOp, const N: usize>(corners: [(f64, f64); N]) -> (f64, f64) {
    let least = corners
        .iter()
        .map(|&(a, b)| Op::lo(a, b))
        .fold(f64::INFINITY, |a, b| lower_of(a, b, f64::min));
    let greatest = corners
        .iter()
        .map(|&(a, b)| Op::hi(a, b))
        .fold(f64::NEG_INFINITY, |a, b| upper_of(a, b, f64::max));
    (least, greatest)
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
            corner_bounds::<Quotient>(self, rhs)
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

    /// Every interval over [`EDGES`], points included, at two decorations.
    fn intervals() -> Vec<DInterval> {
        EDGES
            .iter()
            .flat_map(|&lo| EDGES.iter().map(move |&hi| (lo, hi)))
            .filter(|&(lo, hi)| lo <= hi && lo != f64::INFINITY && hi != f64::NEG_INFINITY)
            .map(|(lo, hi)| DInterval::from_bounds(lo, hi))
            .flat_map(|x| [x, x.with_dec_capped(crate::interval::Decoration::Def)])
            .collect()
    }

    /// The oracle, spelled out here rather than read through
    /// [`corner_bounds`] so that a fault in the shared fold cannot hide
    /// in its own reference: all four corners, folded from `±inf`.
    fn four_corners(
        x: DInterval,
        y: DInterval,
        lo: fn(f64, f64) -> f64,
        hi: fn(f64, f64) -> f64,
    ) -> DInterval {
        let l = [
            lo(x.lo, y.lo),
            lo(x.lo, y.hi),
            lo(x.hi, y.lo),
            lo(x.hi, y.hi),
        ];
        let h = [
            hi(x.lo, y.lo),
            hi(x.lo, y.hi),
            hi(x.hi, y.lo),
            hi(x.hi, y.hi),
        ];
        DInterval::make(
            l.into_iter()
                .fold(f64::INFINITY, |a, b| lower_of(a, b, f64::min)),
            h.into_iter()
                .fold(f64::NEG_INFINITY, |a, b| upper_of(a, b, f64::max)),
            x.dec.min(y.dec),
        )
    }

    fn bits(v: DInterval) -> (u64, u64, crate::interval::Decoration) {
        (v.lo.to_bits(), v.hi.to_bits(), v.dec)
    }

    /// **A point operand takes a shorter fold, with the four-corner
    /// fold's bits.** Every product over [`intervals`] against
    /// [`four_corners`]: bounds bit for bit, decoration equal.
    #[test]
    fn a_point_product_is_the_four_corner_fold() {
        let all = intervals();
        let mut points = 0;
        for &x in &all {
            for &y in &all {
                points += usize::from(x.lo == x.hi || y.lo == y.hi);
                let want = four_corners(x, y, mul_lo, mul_hi);
                assert_eq!(bits(x * y), bits(want), "{x:?} * {y:?}");
            }
        }
        assert!(
            points > 1000,
            "the table reaches the point paths: {points} products"
        );
    }

    /// **The same for a quotient whose divisor keeps clear of zero**, the
    /// branch that folds corners: every quotient over [`intervals`] with
    /// a one-signed divisor that does not touch zero, against
    /// [`four_corners`].
    #[test]
    fn a_point_quotient_is_the_four_corner_fold() {
        let all = intervals();
        let mut points = 0;
        for &x in &all {
            for &y in all.iter().filter(|y| y.lo > 0.0 || y.hi < 0.0) {
                points += usize::from(x.lo == x.hi || y.lo == y.hi);
                let want = four_corners(x, y, div_lo, div_hi);
                assert_eq!(bits(x / y), bits(want), "{x:?} / {y:?}");
            }
        }
        assert!(
            points > 1000,
            "the table reaches the point paths: {points} quotients"
        );
    }

    /// The scales [`DInterval::scale_pow2`] is read at: the extremes of
    /// the normal powers of two, the halvings the quotient pieces use,
    /// and — through the general product — values that are not
    /// positive normal powers of two.
    const SCALES: [f64; 14] = [
        2.2250738585072014e-308,
        9.313225746154785e-10,
        0.0078125,
        0.25,
        0.5,
        1.0,
        2.0,
        1024.0,
        8.98846567431158e307,
        3.0,
        0.1,
        -0.5,
        5e-324,
        0.0,
    ];

    /// **A power-of-two scale is the general product wherever that
    /// product is exact, and inside it where it pads.** Every interval
    /// over [`intervals`] at every scale in [`SCALES`], against
    /// `x * DInterval::point(k)`: where the general product's endpoints
    /// are its unpadded corners `x·k` (zero, or at or above the 2Prod
    /// floor) the two agree bit for bit, decoration included; between
    /// the smallest normal and that floor the shortcut is `x·k` itself,
    /// inside the general product's padded step; everywhere else — the
    /// subnormal range, overflow, infinite endpoints, a `k` that is not
    /// a positive normal power of two — it is the general product.
    #[test]
    fn a_power_of_two_scale_is_the_general_product_where_that_is_exact() {
        let (mut agreed, mut inside) = (0, 0);
        let floor = |v: f64| v == 0.0 || v.abs() >= TWO_PROD_VALID_MIN;
        for &x in &intervals() {
            for &k in &SCALES {
                let (got, want) = (x.scale_pow2(k), x * DInterval::point(k));
                let shortcut = is_normal_pow2(k)
                    && exact_scale(x.lo, k).is_some()
                    && exact_scale(x.hi, k).is_some();
                if !shortcut || (floor(x.lo * k) && floor(x.hi * k)) {
                    assert_eq!(bits(got), bits(want), "{x:?} scaled by {k:e}");
                    agreed += 1;
                } else {
                    let exact = |v: f64| if v == 0.0 { 0.0 } else { v * k };
                    assert_eq!(
                        (got.lo.to_bits(), got.hi.to_bits(), got.dec),
                        (exact(x.lo).to_bits(), exact(x.hi).to_bits(), want.dec),
                        "{x:?} scaled by {k:e}"
                    );
                    assert!(want.lo <= got.lo && got.hi <= want.hi, "{x:?} by {k:e}");
                    inside += 1;
                }
            }
        }
        assert!(
            agreed > 500 && inside > 10,
            "{agreed} agreed, {inside} inside"
        );
    }

    /// **In the normal range the scale is exact**: no pad, so halving
    /// and doubling back returns the operand's own endpoints — down to
    /// the smallest normal, under the 2Prod floor where the general
    /// product pads.
    #[test]
    fn a_normal_range_scale_encloses_exactly() {
        let least = f64::MIN_POSITIVE;
        for (lo, hi) in [
            (-3.0, 0.375),
            (1e-300, 1e300),
            (-f64::MAX / 4.0, -1e-200),
            (8.0 * least, 8.0 * least),
        ] {
            let x = DInterval::from_bounds(lo, hi);
            for k in [0.5, 0.25, 2.0] {
                let y = x.scale_pow2(k);
                assert_eq!((y.lo, y.hi), (lo * k, hi * k), "[{lo:e}, {hi:e}] by {k}");
                let back = y.scale_pow2(1.0 / k);
                assert_eq!(
                    (back.lo, back.hi),
                    (lo, hi),
                    "[{lo:e}, {hi:e}] by {k} and back"
                );
            }
        }
    }

    /// **Scaled into the subnormal range it still encloses.** `3` and
    /// `5` subnormal steps halved are `1.5` and `2.5` steps, which round
    /// to `2` under round-to-nearest-even: a lower bound above the true
    /// value and an upper bound below it. The result, doubled back
    /// (exact at these magnitudes), must still bracket the operand; so
    /// must the smallest normal halved, which lands on a subnormal.
    #[test]
    fn a_scale_into_the_subnormal_range_still_encloses() {
        let step = 5e-324;
        let least = f64::MIN_POSITIVE;
        for (lo, hi) in [
            (3.0 * step, 5.0 * step),
            (-5.0 * step, -3.0 * step),
            (least + step, least + 3.0 * step),
        ] {
            let x = DInterval::from_bounds(lo, hi);
            let y = x.scale_pow2(0.5);
            assert!(
                y.lo * 2.0 <= lo && y.hi * 2.0 >= hi,
                "[{lo:e}, {hi:e}] halved reads [{:e}, {:e}]",
                y.lo,
                y.hi
            );
        }
    }

    /// **A product that rounds UP onto the smallest normal is not
    /// exact.** `0x001F_FFFF_FFFF_FFFF` is `MIN_POSITIVE · (2 − 2⁻⁵²)`;
    /// halved it is `MIN_POSITIVE − 2⁻¹⁰⁷⁵`, which is not representable
    /// and rounds up to `MIN_POSITIVE` itself. A test on the rounded
    /// product's magnitude that admits `MIN_POSITIVE` takes the
    /// shortcut there and returns a lower bound above the true value
    /// (an upper bound below it, on the negative mirror). The result,
    /// doubled back (exact here), must still bracket the operand.
    #[test]
    fn a_product_rounding_up_onto_the_smallest_normal_still_encloses() {
        let x = f64::from_bits(0x001F_FFFF_FFFF_FFFF);
        assert_eq!(
            x * 0.5,
            f64::MIN_POSITIVE,
            "the case rounds onto the smallest normal"
        );
        for (lo, hi) in [(x, x), (-x, -x)] {
            let y = DInterval::from_bounds(lo, hi).scale_pow2(0.5);
            assert!(
                y.lo * 2.0 <= lo && y.hi * 2.0 >= hi,
                "[{lo:e}, {hi:e}] halved reads [{:e}, {:e}]",
                y.lo,
                y.hi
            );
        }
    }

    /// **A product by a non-negative multiplier is the general product,
    /// bit for bit.** Every interval over [`intervals`] times every one
    /// of them with a non-negative lower end: [`DInterval::mul_nonneg`]'s
    /// two corners against `x * w`'s four, bounds and decoration,
    /// across signed zeros, the subnormal range, the 2Prod floor, the
    /// extremes and the infinities.
    #[test]
    fn a_nonnegative_multiplier_is_the_general_product() {
        let all = intervals();
        let mut n = 0;
        for &x in &all {
            for &w in all.iter().filter(|w| w.lo >= 0.0) {
                assert_eq!(bits(x.mul_nonneg(w)), bits(x * w), "{x:?} * {w:?}");
                n += 1;
            }
        }
        // A multiplier below zero is handed to the general product.
        let (x, w) = (
            DInterval::from_bounds(-2.0, 3.0),
            DInterval::from_bounds(-1.0, 0.5),
        );
        assert_eq!(bits(x.mul_nonneg(w)), bits(x * w));
        assert!(n > 10_000, "the table reaches the shortcut: {n} products");
    }

    /// **The same against the weights the Bernstein products read**, at
    /// operands drawn to provoke ties: a binomial quotient `C·C'/C''` as
    /// an enclosure, against intervals built from dyadic values with
    /// few significant bits (whose products are often exact, the case
    /// that falls back to four corners), near-zero values and random
    /// doubles, both signs. Bounds and decoration, bit for bit.
    #[test]
    fn a_binomial_weight_product_is_the_general_product() {
        let mut s: u64 = 0x9E37_79B9_7F4A_7C15;
        let mut next = || {
            s ^= s << 13;
            s ^= s >> 7;
            s ^= s << 17;
            s
        };
        let draw = |n: &mut dyn FnMut() -> u64| -> f64 {
            let r = n();
            let sign = if r & 1 == 0 { 1.0 } else { -1.0 };
            let m = ((r >> 8) % 64) as f64;
            let e = ((r >> 20) % 40) as i32 - 20;
            match (r >> 40) % 4 {
                0 => sign * m * 2f64.powi(e),
                1 => sign * f64::from_bits((r >> 2) & 0x3FEF_FFFF_FFFF_FFFF),
                2 => sign * m * 2f64.powi(-1060),
                _ => 0.0,
            }
        };
        let binom =
            |n: u64, k: u64| (0..k).fold(1.0_f64, |acc, i| acc * (n - i) as f64 / (i + 1) as f64);
        let (mut n, mut fell_back) = (0usize, 0usize);
        for _ in 0..200_000 {
            let (x0, x1) = (draw(&mut next), draw(&mut next));
            let x = DInterval::from_bounds(x0.min(x1), x0.max(x1));
            let r = next();
            let (da, db) = (1 + r % 12, 1 + (r >> 8) % 12);
            let k = (r >> 16) % (da + db + 1);
            let i = k.saturating_sub(db) + (r >> 24) % (k.min(da) - k.saturating_sub(db) + 1);
            let w = DInterval::point(binom(da, i) * binom(db, k - i))
                / DInterval::point(binom(da + db, k));
            let got = x.mul_nonneg(w);
            let want = x * w;
            assert_eq!(bits(got), bits(want), "{x:?} * {w:?}");
            n += 1;
            fell_back += usize::from(w.lo == w.hi);
        }
        assert!(
            n == 200_000 && fell_back < n,
            "{fell_back} point weights of {n}"
        );
    }
}
