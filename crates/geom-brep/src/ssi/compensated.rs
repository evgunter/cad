//! **A NURBS point to a few ulps, with its error bounded.** The
//! midpoint of [`super::enclose::NurbsBoxes::rect_box`] read where the
//! window's net, weights, knots and parameters are all points: the
//! rational sum evaluated in double-double arithmetic (error-free
//! transformations, `hi + lo` carrying about 106 bits), each value
//! carrying an a-priori bound on its distance from the exact value of
//! the same expression over the stored numbers. The enclosure handed
//! back is that value widened by its bound and rounded outward to
//! `f64`: a few ulps of the point, where interval arithmetic over the
//! same sum is as wide as its terms' rounding (`p²` of them).
//!
//! # The bounds
//!
//! A value `x` stands for the reals within `x.err` of `x.hi + x.lo`.
//! Each operation's result is the double-word algorithm's (Joldes,
//! Muller and Popescu 2017, "Tight and rigorous error bounds for basic
//! building blocks of double-word arithmetic"), whose own error is at
//! most `3u²` relative for a sum and `4u²` for a product (`u = 2⁻⁵³`,
//! with a fused multiply-add), taken here as `4u²` and `5u²`. The inputs'
//! bounds propagate as first-order terms plus their product. Every bound
//! is formed in `f64` and then raised by [`up`], which covers the
//! rounding of the bound's own arithmetic.
//!
//! The double-word bounds assume no underflow: a nonzero leading part
//! below `2⁻⁹⁰⁰`, or any value that is not finite, poisons the bound to
//! infinity, and the reading is `None` (the interval readings stand
//! alone).

use geom_core::Interval;
use geom_core::exact::{two_product as two_prod, two_sum};

/// The unit roundoff, `2⁻⁵³`.
const U: f64 = f64::EPSILON / 2.0;

/// The smallest leading part a bound is trusted at.
const FLOOR: f64 = 7.888_609_052_210_118e-271; // 2⁻⁹⁰⁰

/// A bound formed in `f64` raised past its own rounding: at most a few
/// roundings went into it, each at most `u` relative.
fn up(x: f64) -> f64 {
    x * (1.0 + 8.0 * U) + f64::from_bits(1)
}

/// `a + b` as an exact pair, `|a| ≥ |b|` (or `a` zero).
fn fast_two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    (s, b - (s - a))
}

/// A double-word value and its error bound (module docs).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Dd {
    hi: f64,
    lo: f64,
    err: f64,
}

impl Dd {
    /// An exact `f64`.
    pub(crate) fn exact(x: f64) -> Self {
        Self::checked(x, 0.0, 0.0)
    }

    /// `a − b`, exactly.
    pub(crate) fn difference(a: f64, b: f64) -> Self {
        let (hi, lo) = two_sum(a, -b);
        Self::checked(hi, lo, 0.0)
    }

    /// The value with its bound poisoned where the double-word bounds
    /// do not hold (module docs).
    fn checked(hi: f64, lo: f64, err: f64) -> Self {
        let trusted =
            hi.is_finite() && lo.is_finite() && err.is_finite() && (hi == 0.0 || hi.abs() >= FLOOR);
        Self {
            hi,
            lo,
            err: if trusted { err } else { f64::INFINITY },
        }
    }

    /// `|hi| + |lo|`, at least the value's magnitude.
    fn magnitude(self) -> f64 {
        up(self.hi.abs() + self.lo.abs())
    }

    fn neg(self) -> Self {
        Self {
            hi: -self.hi,
            lo: -self.lo,
            err: self.err,
        }
    }

    /// The sum (the accurate double-word sum, at most `3u²` relative).
    pub(crate) fn add(self, y: Self) -> Self {
        let (sh, sl) = two_sum(self.hi, y.hi);
        let (th, tl) = two_sum(self.lo, y.lo);
        let (vh, vl) = fast_two_sum(sh, sl + th);
        let (zh, zl) = fast_two_sum(vh, tl + vl);
        let z = Self::checked(zh, zl, 0.0);
        let err = up(self.err + y.err + 4.0 * U * U * z.magnitude());
        Self::checked(zh, zl, err.max(z.err))
    }

    pub(crate) fn sub(self, y: Self) -> Self {
        self.add(y.neg())
    }

    /// The product (the double-word product with a fused multiply-add,
    /// at most `4u²` relative).
    pub(crate) fn mul(self, y: Self) -> Self {
        let (ch, cl1) = two_prod(self.hi, y.hi);
        let tl0 = self.lo * y.lo;
        let tl1 = self.hi.mul_add(y.lo, tl0);
        let cl2 = self.lo.mul_add(y.hi, tl1);
        let (zh, zl) = fast_two_sum(ch, cl1 + cl2);
        let z = Self::checked(zh, zl, 0.0);
        let err = up(self.magnitude() * y.err
            + y.magnitude() * self.err
            + self.err * y.err
            + 5.0 * U * U * z.magnitude());
        Self::checked(zh, zl, err.max(z.err))
    }

    /// The quotient: `q₀ = hi/y.hi`, the remainder `r = self − q₀·y`
    /// (tracked), `q₁ = r.hi/y.hi`, the pair `q₀ + q₁`. Its distance
    /// from `self/y` over the stored values is `|r − q₁·y|/|y|` with
    /// `|r.hi − q₁·y.hi| ≤ u·|r.hi|`; the operands' own bounds add
    /// `(e_x + |x/y|·e_y)/(|y| − e_y)`. Poisoned where `|y| − e_y` is
    /// not positive.
    ///
    /// As `e_y` approaches `|y|` the bound grows without limit, though it
    /// stays finite and sound: a divisor known only to within its own
    /// size yields a reading too wide to tighten anything, and the
    /// interval readings it is met with stand. In this module the
    /// divisors are knot differences (exact) and the weight sum, whose
    /// bound is `u²` of it. The bound also counts `e_y` twice: `r.err`
    /// carries `|q₀|·e_y` from the remainder's product, and the carried
    /// term adds `|x/y|·e_y` again. That slack is sound, and it is why
    /// no row isolates the `u·|r.hi|` term or [`up`].
    pub(crate) fn div(self, y: Self) -> Self {
        let q0 = self.hi / y.hi;
        let r = self.sub(y.mul(Self::exact(q0)));
        let q1 = r.hi / y.hi;
        let (zh, zl) = two_sum(q0, q1);
        let z = Self::checked(zh, zl, 0.0);
        let below = y.hi.abs() - y.lo.abs() - y.err;
        let below = below * (1.0 - 8.0 * U);
        // A NaN is not positive either.
        if below.is_nan() || below <= 0.0 {
            return Self::checked(zh, zl, f64::INFINITY);
        }
        let own = up(U * r.hi.abs() + r.lo.abs() + q1.abs() * y.lo.abs() + r.err) / below;
        let carried = up(self.err + z.magnitude() * y.err) / below;
        Self::checked(zh, zl, up(own + carried).max(z.err))
    }

    /// The reals the value stands for, rounded outward to `f64`; `None`
    /// where its bound is poisoned.
    pub(crate) fn enclosure(self) -> Option<Interval> {
        if !self.err.is_finite() {
            return None;
        }
        let r = up(self.lo.abs() + self.err);
        let (lo, hi) = ((self.hi - r).next_down(), (self.hi + r).next_up());
        (lo.is_finite() && hi.is_finite()).then(|| Interval::from_bounds(lo, hi))
    }
}

/// The `p + 1` B-spline basis values of degree `p` at `u` on span `s`
/// of the knot list `knots` (Cox–de Boor, as `geom_core`'s basis row):
/// every knot difference and `u` minus a knot exact, the rest
/// double-word. `None` for a span whose knots leave the list.
fn basis(knots: &[f64], s: usize, p: usize, u: f64) -> Option<Vec<Dd>> {
    if s < p || s + p >= knots.len() {
        return None;
    }
    let mut n = vec![Dd::exact(0.0); p + 1];
    n[0] = Dd::exact(1.0);
    for j in 1..=p {
        let mut saved = Dd::exact(0.0);
        for r in 0..j {
            let den = Dd::difference(knots[s + r + 1], knots[s + r + 1 - j]);
            let temp = n[r].div(den);
            let right = Dd::difference(knots[s + r + 1], u);
            let left = Dd::difference(u, knots[s + 1 + r - j]);
            n[r] = saved.add(right.mul(temp));
            saved = left.mul(temp);
        }
        n[j] = saved;
    }
    Some(n)
}

/// One span cell of a NURBS surface: its knot lists, span indices and
/// degrees, and its window of points, `(p_u + 1)·(p_v + 1)` row-major in
/// `u`, each a Cartesian point and its weight.
pub(crate) struct PointCell<'a> {
    pub(crate) knots: (&'a [f64], &'a [f64]),
    pub(crate) spans: (usize, usize),
    pub(crate) degrees: (usize, usize),
    pub(crate) window: &'a [([f64; 3], f64)],
}

/// An enclosure of the cell's point at `(u, v)`, a few ulps wide
/// (module docs); `None` where a bound is poisoned or the cell is
/// malformed.
pub(crate) fn nurbs_point(cell: &PointCell<'_>, (u, v): (f64, f64)) -> Option<[Interval; 3]> {
    let (pu, pv) = cell.degrees;
    if cell.window.len() != (pu + 1) * (pv + 1) {
        return None;
    }
    let bu = basis(cell.knots.0, cell.spans.0, pu, u)?;
    let bv = basis(cell.knots.1, cell.spans.1, pv, v)?;
    let zero = Dd::exact(0.0);
    let (mut num, mut den) = ([zero; 3], zero);
    for (i, bui) in bu.iter().enumerate() {
        for (j, bvj) in bv.iter().enumerate() {
            let (point, weight) = cell.window[i * (pv + 1) + j];
            let cw = bui.mul(*bvj).mul(Dd::exact(weight));
            for (n, x) in num.iter_mut().zip(point) {
                *n = n.add(cw.mul(Dd::exact(x)));
            }
            den = den.add(cw);
        }
    }
    let [x, y, z] = num.map(|n| n.div(den).enclosure());
    Some([x?, y?, z?])
}

#[cfg(test)]
#[allow(
    clippy::expect_used,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss
)]
mod tests {
    use super::{Dd, PointCell, nurbs_point};
    use geom_core::Bounds as _;

    /// **The double-word blocks enclose the exact results.** Sums,
    /// products and quotients of drawn `f64`s against the same
    /// operations in exact rationals (`i128` over a power of two for the
    /// sums and products, cross-multiplied for the quotients), each a
    /// few ulps wide (plus `2⁻⁹⁶` where the result cancels, and the
    /// smallest normal at a zero quotient, whose ends are subnormal
    /// pads). The
    /// operations here are exact or nearly so; the rounding bounds are
    /// held by [`a_cancelled_sum_of_products_keeps_its_rounding_bound`].
    #[test]
    fn the_blocks_enclose_the_exact_results() {
        let mut rng = test_utils::fuzz::start("compensated-blocks");
        for _ in 0..test_utils::fuzz::scaled(2000) {
            // Integers below 2²⁰ scaled by 2⁻²⁰: every exact sum and
            // product is an i128 over 2⁻⁴⁰.
            let a = (rng.range(-1.0, 1.0) * 1_048_576.0).round();
            let b = (rng.range(-1.0, 1.0) * 1_048_576.0).round();
            let scale = 2.0_f64.powi(-20);
            let (x, y) = (Dd::exact(a * scale), Dd::exact(b * scale));
            // (x + y)·y − x, exactly, over 2⁻⁶⁰.
            let exact = ((a as i128 + b as i128) * b as i128) - (a as i128) * (1 << 20);
            let got = x.add(y).mul(y).sub(x).enclosure().expect("finite");
            let want = exact as f64 * 2.0_f64.powi(-40);
            assert!(
                got.lo() <= want && want <= got.hi(),
                "{a}, {b}: {got:?} misses {want:e}"
            );
            assert!(
                got.hi() - got.lo() <= 8.0 * f64::EPSILON * want.abs() + RESOLUTION,
                "{a}, {b}: {got:?} is wide"
            );
            if b != 0.0 {
                // x / y = a / b: the enclosure's ends bracket it,
                // checked by cross-multiplication in exact arithmetic
                // where the ends are dyadics with few enough bits.
                let q = x.div(y).enclosure().expect("finite");
                // `end·b` against `a` in exact integers: `end = m·2^e`.
                // Arbitrary precision: an end of `0/b`'s enclosure is a
                // subnormal pad, `e` near −1074, past any fixed width.
                let check = |end: f64, below: bool| {
                    use num_bigint::BigInt;
                    let (m, e) = split(end);
                    let (bi, ai) = (BigInt::from(b as i128), BigInt::from(a as i128));
                    let lhs = BigInt::from(m) * bi;
                    let shift = e.unsigned_abs() as usize;
                    let (lhs, rhs) = if e < 0 {
                        (lhs, ai << shift)
                    } else {
                        (lhs << shift, ai)
                    };
                    let diff = lhs - rhs;
                    let sign = if b > 0.0 { diff } else { -diff };
                    if below {
                        sign <= BigInt::from(0)
                    } else {
                        sign >= BigInt::from(0)
                    }
                };
                assert!(
                    check(q.lo(), true) && check(q.hi(), false),
                    "{a}/{b}: {q:?}"
                );
                // A zero quotient's ends are subnormal pads.
                assert!(
                    q.hi() - q.lo() <= 8.0 * f64::EPSILON * (a / b).abs() + f64::MIN_POSITIVE,
                    "{a}/{b}: {q:?} is wide"
                );
            }
        }
    }

    /// `x` as an exact big rational `m·2^e`.
    fn big(x: f64) -> (num_bigint::BigInt, i32) {
        let (m, e) = split(x);
        (num_bigint::BigInt::from(m), e)
    }

    /// `Σ cᵢ` over terms `mᵢ·2^{eᵢ}` as one `m·2^e` at the least exponent.
    fn sum(terms: &[(num_bigint::BigInt, i32)]) -> (num_bigint::BigInt, i32) {
        let e = terms.iter().map(|t| t.1).min().unwrap_or(0);
        let m = terms
            .iter()
            .map(|(m, te)| m.clone() << (te - e) as usize)
            .sum();
        (m, e)
    }

    /// Whether `x ≤ m·2^e` (`below`) or `x ≥ m·2^e`, exactly.
    fn bounds(x: f64, (m, e): &(num_bigint::BigInt, i32), below: bool) -> bool {
        let (xm, xe) = big(x);
        let (d, _) = sum(&[(xm, xe), (-m.clone(), *e)]);
        if below {
            d.sign() != num_bigint::Sign::Plus
        } else {
            d.sign() != num_bigint::Sign::Minus
        }
    }

    /// **A sum of double-words keeps its own rounding bound.** Two exact
    /// differences `a₁ − b₁` and `a₂ − b₂` with `b` drawn at `2⁻⁵⁴` to
    /// `2⁻¹⁰⁴` of `a` (so each low part is nonzero), summed: the
    /// double-word sum rounds at `u²` of its value, and its `hi + lo`
    /// stays within its bound of the exact sum, checked in exact
    /// big-integer arithmetic. The bound itself is checked rather than
    /// [`Dd::enclosure`], whose `|lo|` widening (about `u` of the value)
    /// would cover a `u²` term dropped. Red with the sum's `4u²` term
    /// dropped.
    #[test]
    fn a_double_word_sum_keeps_its_own_rounding_bound() {
        let mut rng = test_utils::fuzz::start("compensated-sum");
        for _ in 0..test_utils::fuzz::scaled(20_000) {
            let draw = |rng: &mut test_utils::fuzz::Rng| {
                let a = rng.range(-1.0, 1.0);
                let b = a * rng.range(0.5, 1.0) * 2.0_f64.powi(-54 - rng.below(51) as i32);
                (a, b)
            };
            let ((a1, b1), (a2, b2)) = (draw(&mut rng), draw(&mut rng));
            let s = Dd::difference(a1, b1).add(Dd::difference(a2, b2));
            let exact = sum(&[big(a1), neg(big(b1)), big(a2), neg(big(b2))]);
            // `|exact − (hi + lo)| ≤ err`, exactly: the bound itself, which
            // the enclosure's `|lo|` widening would otherwise swamp.
            let (d, de) = sum(&[exact, neg(big(s.hi)), neg(big(s.lo))]);
            let (em, ee) = big(s.err);
            let (gap, _) = sum(&[(d.magnitude().clone().into(), de), neg((em, ee))]);
            assert!(
                gap.sign() != num_bigint::Sign::Plus,
                "{a1}, {b1}, {a2}, {b2}: the sum is off by more than its bound {:e}; {}",
                s.err,
                test_utils::fuzz::replay()
            );
        }
    }

    /// `−x`, exactly.
    fn neg((m, e): (num_bigint::BigInt, i32)) -> (num_bigint::BigInt, i32) {
        (-m, e)
    }

    /// **A cancelled sum of products keeps its rounding bound.**
    /// `a·b·c + d·e·f` of drawn 53-bit numbers less its `f64` rounding:
    /// the double-word products and their sum round at `u²` of their
    /// values, the difference cancels to about `u` of them, so the
    /// rounding bounds are what keep the exact value inside. Checked in
    /// exact big-integer arithmetic.
    #[test]
    fn a_cancelled_sum_of_products_keeps_its_rounding_bound() {
        let mut rng = test_utils::fuzz::start("compensated-cancellation");
        for _ in 0..test_utils::fuzz::scaled(4000) {
            let [a, b, c, d, e, f] = [0.0; 6].map(|_| rng.range(-1.0, 1.0));
            // `a·b·c + d·e·f` less its `f64` rounding: both products
            // and their sum round in double-word, and the difference
            // cancels to about `u` of them.
            let near = (a * b) * c + (d * e) * f;
            let got = Dd::exact(a)
                .mul(Dd::exact(b))
                .mul(Dd::exact(c))
                .add(Dd::exact(d).mul(Dd::exact(e)).mul(Dd::exact(f)))
                .sub(Dd::exact(near))
                .enclosure()
                .expect("trusted");
            let [(am, ae), (bm, be), (cm, ce), (dm, de), (em, ee), (fm, fe)] =
                [a, b, c, d, e, f].map(big);
            let exact = sum(&[
                (am * bm * cm, ae + be + ce),
                (dm * em * fm, de + ee + fe),
                (-big(near).0, big(near).1),
            ]);
            assert!(
                bounds(got.lo(), &exact, true) && bounds(got.hi(), &exact, false),
                "{a}, {b}, {c}, {d}, {e}, {f}: {got:?} misses the exact value; {}",
                test_utils::fuzz::replay()
            );
        }
    }

    /// `x = m·2^e` exactly, `m` an integer.
    fn split(x: f64) -> (i128, i32) {
        if x == 0.0 {
            return (0, 0);
        }
        let bits = x.to_bits();
        let exp = ((bits >> 52) & 0x7ff) as i32;
        let frac = (bits & ((1 << 52) - 1)) as i128;
        let (m, e) = if exp == 0 {
            (frac, -1074)
        } else {
            (frac | (1 << 52), exp - 1075)
        };
        (if x < 0.0 { -m } else { m }, e)
    }

    /// The double-word bounds' absolute floor on these rows: `16u²` of
    /// terms near one (a sum of nine products, each with its own
    /// double-word error, and the basis's divisions before them).
    const RESOLUTION: f64 = 1.262_177_448_353_619e-29; // 2⁻⁹⁶

    /// **A point is enclosed a few ulps wide.** The biquadratic of
    /// `one_arc`'s touch wall, `z = (x − ½)² + y − ¼` on dyadic control,
    /// read at `(½ ∓ 2⁻ᵏ, ¼)` where its exact height is `2⁻²ᵏ`: the
    /// enclosure holds it and is within four ulps of it plus `2⁻⁹⁶`
    /// (the double-word floor on terms near `¼`) wide, where the
    /// interval sum over the same terms is about `2⁻⁵⁵` wide whatever
    /// `k`, and the lane's `f64` evaluation misses it (`one_arc`'s
    /// `a_point_windows_box_encloses_the_exact_point`).
    #[test]
    fn a_dyadic_point_is_enclosed_a_few_ulps_wide() {
        let knots = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let window: Vec<([f64; 3], f64)> = (0..9)
            .map(|i| {
                let (a, b) = (i / 3, i % 3);
                let z = [0.25, -0.25, 0.25][a] + [-0.25, 0.25, 0.75][b];
                ([0.5 * f64::from(a as u8), 0.5 * f64::from(b as u8), z], 1.0)
            })
            .collect();
        let cell = PointCell {
            knots: (&knots, &knots),
            spans: (2, 2),
            degrees: (2, 2),
            window: &window,
        };
        for k in 20..=40 {
            let h = 2.0_f64.powi(-k);
            for u in [0.5 - h, 0.5 + h] {
                let [_, _, z] = nurbs_point(&cell, (u, 0.25)).expect("a trusted bound");
                let exact = h * h;
                assert!(
                    z.lo() <= exact && exact <= z.hi(),
                    "u = {u:e}: {z:?} misses {exact:e}"
                );
                assert!(
                    z.hi() - z.lo() <= 4.0 * f64::EPSILON * exact + RESOLUTION,
                    "u = {u:e}: {z:?} is wider than four ulps of {exact:e} and 2⁻⁹⁶"
                );
            }
        }
    }
}
