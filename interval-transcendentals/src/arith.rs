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
    /// **Exact by a point power of two** wherever the scaled endpoints
    /// stay normal ([`DInterval::pow2_product`]), so a halving pads
    /// nothing at any normal magnitude;
    /// everywhere else the four-corner fold, read on two corners where
    /// either operand is a one-signed non-point interval
    /// ([`DInterval::signed_corners`]), bit for bit the fold.
    fn mul(self, rhs: Self) -> Self {
        if let Some(p) = Self::propagate2(&self, &rhs) {
            return p;
        }
        self.pow2_product(rhs)
            .or_else(|| rhs.pow2_product(self))
            .unwrap_or_else(|| self.general_product(rhs))
    }
}

impl DInterval {
    /// The general product, on operands neither poisoned nor empty: the
    /// four-corner fold, read on two corners where it can be.
    fn general_product(self, rhs: Self) -> Self {
        self.signed_corners(rhs)
            .or_else(|| rhs.signed_corners(self))
            .unwrap_or_else(|| self.corner_product(rhs))
    }

    /// **`self · k` for `k` a point `±2ⁿ` at a normal exponent**, exact,
    /// where both scaled endpoints are zero or strictly above the
    /// smallest normal ([`exact_scale`]); `None` elsewhere, for the
    /// general product to decide.
    ///
    /// Scaling a finite `x` by a power of two only moves its exponent,
    /// so `x·k` is exact whenever the exact result is normal. Against the
    /// general product:
    ///
    /// - at `|x·k| >= TWO_PROD_VALID_MIN` that product certifies the
    ///   same exact `x·k` through its 2Prod witness, so the two agree
    ///   bit for bit, decoration included;
    /// - between the smallest normal and that floor (about `2⁻⁹⁵⁹`) the
    ///   witness cannot certify, so the general product pads one step
    ///   outward and this is the exact value inside it;
    /// - an endpoint scaled into the subnormal range, where
    ///   round-to-nearest would not be outward, an overflow and an
    ///   infinite endpoint are `None`.
    ///
    /// A negative `k` is the positive case negated, with a zero endpoint
    /// kept `+0.0` as [`mul_lo`]/[`mul_hi`] return it.
    fn pow2_product(self, w: Self) -> Option<Self> {
        if w.lo != w.hi || !is_normal_pow2(w.lo.abs()) {
            return None;
        }
        let k = w.lo.abs();
        let (lo, hi) = (exact_scale(self.lo, k)?, exact_scale(self.hi, k)?);
        let dec = self.dec.min(w.dec);
        let neg = |v: f64| if v == 0.0 { 0.0 } else { -v };
        Some(if w.lo > 0.0 {
            Self::make(lo, hi, dec)
        } else {
            Self::make(neg(hi), neg(lo), dec)
        })
    }

    /// The general product's four-corner fold, on operands neither
    /// poisoned nor empty.
    fn corner_product(self, rhs: Self) -> Self {
        let dec = self.dec.min(rhs.dec);
        let (lo, hi) = corner_bounds::<Product>(self, rhs);
        Self::make(lo, hi, dec)
    }

    /// **`self · w` on two corners, where `w` is one-signed**, `None`
    /// where the fold must decide: a point `w` (whose fold already reads
    /// two corners), a `w` reaching zero, or a selected corner with a
    /// zero factor. A positive `w` is [`DInterval::mul_nonneg`]'s case
    /// and its argument; a negative one is that case mirrored, since
    /// every directed corner bound is odd in each factor away from zero
    /// (`mul_lo(a, −b) = −mul_hi(a, b)`: the rounded product negates
    /// exactly, the 2Prod witness reads `|r|` and a residual that
    /// negates with it, and `next_down(−r) = −next_up(r)`), so the
    /// fold's least over `self · w` is the negated greatest over
    /// `self · (−w)`. Operands are neither poisoned nor empty.
    fn signed_corners(self, w: Self) -> Option<Self> {
        if w.lo == w.hi {
            return None;
        }
        if w.lo > 0.0 {
            self.nonneg_corners(w)
        } else if w.hi < 0.0 {
            self.nonneg_corners(-w).map(|p| -p)
        } else {
            None
        }
    }

    /// [`DInterval::mul_nonneg`]'s selection for `w.lo >= 0`, `None`
    /// where a selected corner has a zero factor.
    fn nonneg_corners(self, w: Self) -> Option<Self> {
        let (a, b) = if self.lo >= 0.0 {
            (self.lo, w.lo)
        } else {
            (self.lo, w.hi)
        };
        let (c, e) = if self.hi >= 0.0 {
            (self.hi, w.hi)
        } else {
            (self.hi, w.lo)
        };
        if a == 0.0 || b == 0.0 || c == 0.0 || e == 0.0 {
            return None;
        }
        Some(Self::make(mul_lo(a, b), mul_hi(c, e), self.dec.min(w.dec)))
    }
}

impl DInterval {
    /// **`self · k`**, the product by the point `k`: exact for `k` a
    /// normal power of two wherever the scaled endpoints stay normal
    /// ([`DInterval::pow2_product`] states each case), the general
    /// product everywhere else.
    #[must_use]
    pub fn scale_pow2(self, k: f64) -> Self {
        self * Self::point(k)
    }
}

impl DInterval {
    /// **`self · w`, bit for bit `self * w`, reading two corners where
    /// `w.lo >= 0`.** Total: every other operand — a `w` with a negative
    /// lower end, a point operand (whose product already folds two
    /// corners, or scales exactly by a power of two), a poisoned or
    /// empty operand — IS `self * w`. Below, "the general product" is
    /// that product for a non-point `w`.
    ///
    /// With `w` non-negative, the extreme EXACT corners are fixed by the
    /// signs of `self`'s endpoints: the least is `self.lo · w.lo` when
    /// `self.lo >= 0` and `self.lo · w.hi` otherwise, the greatest
    /// `self.hi · w.hi` when `self.hi >= 0` and `self.hi · w.lo`
    /// otherwise. The four-corner fold takes the least of the PADDED
    /// corners ([`mul_lo`]); that is the selected corner's own bound
    /// `v`, because no other corner's bound reads below it (mirror for
    /// `hi`):
    ///
    /// - **`v` padded** (`down1` of the rounded selected product): every
    ///   other corner's exact value is at or above the selected one, so
    ///   rounds at or above that rounded value, and pads to at or above
    ///   `v`.
    /// - **`v` exact and nonzero**: [`mul_lo`] certifies only a finite
    ///   product at `|v| >= TWO_PROD_VALID_MIN`, so `v` is normal. A
    ///   neighbouring corner replaces one factor, say `y`, by the other
    ///   endpoint `y'`, at least one float away on the side that moves the
    ///   product off `v`, so it lies at least `|x|·s(y)` from `v`, `s(z)`
    ///   being the float spacing next to `z` on that side. For normal `z`, `s(z) >= 2⁻⁵³·|z|`, equal only
    ///   when `z` is a power of two and the step is toward zero, and
    ///   `s(z) <= 2⁻⁵²·|z|`, equal only when `z` is a power of two and
    ///   the step is away from zero (a subnormal `y` has
    ///   `s(y) = 2⁻¹⁰⁷⁴ > 2⁻⁵³·|y|`). So `|x|·s(y) >= 2⁻⁵³·|x·y| >=
    ///   s(v)/2`, and the two equalities need the step toward zero at
    ///   `y` and away from zero at `v`, while one step moves both the
    ///   same way: the gap is STRICTLY more than half the spacing at
    ///   `v`. The neighbour rounds to a float past `v` and pads at most
    ///   back to `v`, and the far corner, beyond a neighbour, rounds no
    ///   nearer. So this case needs no guard.
    /// - **`v` an exact zero** (a zero factor): a neighbour can underflow
    ///   to zero and pad one step past it, so the general product
    ///   decides. This is the case the primitive's first draft got
    ///   wrong (`[-∞, -1/3] · [-0, 5e-324]`: the fold's `hi` is
    ///   `5e-324`, a bare selection's `0`).
    #[must_use]
    pub fn mul_nonneg(self, w: Self) -> Self {
        if Self::propagate2(&self, &w).is_some() || w.lo < 0.0 || w.lo == w.hi || self.lo == self.hi
        {
            return self * w;
        }
        self.nonneg_corners(w)
            .unwrap_or_else(|| self.corner_product(w))
    }
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

    /// The general product, propagation included: `*` without its exact
    /// power-of-two path.
    fn general(x: DInterval, y: DInterval) -> DInterval {
        DInterval::propagate2(&x, &y).unwrap_or_else(|| x.general_product(y))
    }

    /// **A point operand takes a shorter fold, with the four-corner
    /// fold's bits.** Every general product over [`intervals`] against
    /// [`four_corners`]: bounds bit for bit, decoration equal. (`*` by a
    /// point power of two leaves the fold where the fold pads:
    /// [`a_power_of_two_product_is_the_general_product_where_that_is_exact`].)
    #[test]
    fn a_point_product_is_the_four_corner_fold() {
        let all = intervals();
        let mut points = 0;
        for &x in &all {
            for &y in &all {
                points += usize::from(x.lo == x.hi || y.lo == y.hi);
                let want = four_corners(x, y, mul_lo, mul_hi);
                assert_eq!(bits(general(x, y)), bits(want), "{x:?} * {y:?}");
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

    /// **A product by a point power of two is the general product
    /// wherever that product is exact, and inside it where it pads.**
    /// Every interval over [`intervals`] at every scale in [`SCALES`],
    /// and every pair of [`intervals`] (whose points include powers of
    /// two), `x * w` in either order against the general product. Where
    /// one operand is a point `±2ⁿ` and both scaled endpoints stay
    /// normal, the product is the other operand's endpoints times it,
    /// computed here in `f64` (exact there), and lies inside the general
    /// product, bit for bit equal to it wherever that product's
    /// endpoints are unpadded (zero, or at or above the 2Prod floor);
    /// everywhere else — the subnormal range, overflow, infinite
    /// endpoints, a scale that is not a power of two — it is the general
    /// product.
    #[test]
    fn a_power_of_two_product_is_the_general_product_where_that_is_exact() {
        let (mut agreed, mut inside) = (0, 0);
        let floor = |v: f64| v == 0.0 || v.abs() >= TWO_PROD_VALID_MIN;
        let all = intervals();
        let scaled: Vec<(DInterval, DInterval)> = all
            .iter()
            .flat_map(|&x| SCALES.iter().map(move |&k| (x, DInterval::point(k))))
            .chain(all.iter().flat_map(|&x| all.iter().map(move |&y| (x, y))))
            .collect();
        for (x, w) in scaled {
            let (got, want) = (x * w, general(x, w));
            assert_eq!(bits(got), bits(w * x), "{x:?} * {w:?}, either order");
            if w.lo == w.hi && w.lo.is_finite() && w.dec == DInterval::point(w.lo).dec {
                assert_eq!(
                    bits(got),
                    bits(x.scale_pow2(w.lo)),
                    "{x:?} * {w:?}, the door"
                );
            }
            let pow2 = |s: DInterval| s.lo == s.hi && is_normal_pow2(s.lo.abs());
            let exact = |v: DInterval, k: f64| {
                let m = k.abs();
                let scale = |e: f64| (e == 0.0).then_some(0.0).or_else(|| exact_scale(e, m));
                let (lo, hi) = (scale(v.lo)?, scale(v.hi)?);
                let neg = |e: f64| if e == 0.0 { 0.0 } else { -e };
                Some(if k > 0.0 {
                    (lo, hi)
                } else {
                    (neg(hi), neg(lo))
                })
            };
            let path = (DInterval::propagate2(&x, &w).is_none())
                .then(|| {
                    pow2(w)
                        .then(|| exact(x, w.lo))
                        .flatten()
                        .or_else(|| pow2(x).then(|| exact(w, x.lo)).flatten())
                })
                .flatten();
            match path {
                Some((lo, hi)) if !(floor(lo) && floor(hi)) => {
                    assert_eq!(
                        (got.lo.to_bits(), got.hi.to_bits(), got.dec),
                        (lo.to_bits(), hi.to_bits(), want.dec),
                        "{x:?} * {w:?}"
                    );
                    assert!(want.lo <= got.lo && got.hi <= want.hi, "{x:?} * {w:?}");
                    inside += 1;
                }
                _ => {
                    assert_eq!(bits(got), bits(want), "{x:?} * {w:?}");
                    agreed += 1;
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

    /// **The product is the four-corner fold, bit for bit**: every
    /// interval over [`intervals`] times every other, the general product
    /// (which reads two corners where either operand is one-signed) against
    /// [`DInterval::corner_product`], bounds and decoration. Red under a
    /// mirrored selection that rounds or picks a corner the fold does
    /// not, or a zero's sign the fold does not give.
    #[test]
    fn the_product_is_the_four_corner_fold() {
        let all = intervals();
        let mut signed = 0;
        for &x in &all {
            for &y in &all {
                if x.is_empty() || y.is_empty() || x.is_nai() || y.is_nai() {
                    continue;
                }
                assert_eq!(
                    bits(x.general_product(y)),
                    bits(x.corner_product(y)),
                    "{x:?} * {y:?}"
                );
                if x.signed_corners(y)
                    .or_else(|| y.signed_corners(x))
                    .is_some()
                {
                    signed += 1;
                }
            }
        }
        assert!(
            signed > 10_000,
            "the table reaches the two-corner read: {signed}"
        );
    }

    /// **A product by a non-negative multiplier is the general
    /// product.** Every interval over [`intervals`] times every one of
    /// them with a non-negative lower end, [`DInterval::mul_nonneg`]
    /// against `x * w`, bounds and decoration, across signed zeros, the
    /// subnormal range, the 2Prod floor, the extremes and the
    /// infinities. Then the operands the shortcut must hand back: a
    /// multiplier with a negative lower end (selected on `w`'s sign
    /// alone, `[-0.1, 0.3]·[-0.7, 0.3]` would read `lo = -0.1·0.3`, not
    /// the true `0.3·-0.7`), and an empty or NaI operand.
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
        assert!(n > 10_000, "the table reaches the shortcut: {n} products");
        let (x, w) = (
            DInterval::from_bounds(-0.1, 0.3),
            DInterval::from_bounds(-0.7, 0.3),
        );
        assert_eq!(bits(x.mul_nonneg(w)), bits(x * w), "a negative multiplier");
        let w = DInterval::from_bounds(0.5, 2.0);
        for odd in [DInterval::empty(), DInterval::nai()] {
            assert_eq!(bits(odd.mul_nonneg(w)), bits(odd * w), "{odd:?} * {w:?}");
            assert_eq!(bits(w.mul_nonneg(odd)), bits(w * odd), "{w:?} * {odd:?}");
        }
    }
}
