//! **The certified composite Taylor rule** the curved-rim lanes read their
//! edge integrals through: a spiric's ([`super::spiric`]) and a rational
//! spline rim's ([`super::rational_rim`]).
//!
//! `∫_{t₀}^{t₁} g(t) dt` over `pieces` equal pieces: on each, `g`'s Taylor
//! jet at the midpoint `x` integrates the degree-3 part exactly against
//! `∫(t − x)ᵏ dt`, and the degree-4 coefficient taken over the whole piece
//! is the Lagrange remainder, so the enclosure holds the integral at every
//! resolution and its width falls as `h⁵`. The jets carry interval
//! coefficients; their arithmetic is the ring operations, the reciprocal
//! and the square root, each enclosing its exact counterpart. An integrand
//! hands the rule a per-piece context ([`composite_by`]'s `prepare`), so
//! what it forms once per piece (a rim's local polynomial) is formed once,
//! and read at the node and over the piece.

use geom_core::Bounds;
use geom_core::interval::Interval;
use geom_core::interval::certification::Certification;

use super::bracket::{mid, pt};

/// The Taylor coefficients `g⁽ᵏ⁾/k!`, `k = 0..=4`, of a function of `t`,
/// each an enclosure: at a node they are thin; over a piece (inputs
/// enclosing the piece) each encloses its coefficient over the whole
/// piece, which is what the remainder reads.
#[derive(Clone, Copy)]
pub(super) struct Jet(pub(super) [Interval; ORDER]);

/// Coefficients carried: the rule is exact through degree 3 and its
/// remainder reads the degree-4 coefficient.
pub(super) const ORDER: usize = 5;

impl Jet {
    /// The jet of the variable itself at `t`: value `t`, slope `1`.
    pub(super) fn var(t: Interval) -> Self {
        let mut c = [Interval::zero(); ORDER];
        c[0] = t;
        c[1] = Interval::one();
        Self(c)
    }

    pub(super) fn cst(v: Interval) -> Self {
        let mut c = [Interval::zero(); ORDER];
        c[0] = v;
        Self(c)
    }

    pub(super) fn add(self, o: Self) -> Self {
        Self(core::array::from_fn(|k| self.0[k] + o.0[k]))
    }

    pub(super) fn sub(self, o: Self) -> Self {
        Self(core::array::from_fn(|k| self.0[k] - o.0[k]))
    }

    pub(super) fn scale(self, x: Interval) -> Self {
        Self(self.0.map(|c| c * x))
    }

    /// The Cauchy product.
    pub(super) fn mul(self, o: Self) -> Self {
        Self(core::array::from_fn(|k| {
            (0..=k).fold(Interval::zero(), |acc, i| acc + self.0[i] * o.0[k - i])
        }))
    }

    /// The Cauchy square: each cross term once, doubled, and the diagonal
    /// term squared, so the value's enclosure is `a₀²`'s, never below zero,
    /// where [`Jet::mul`] would multiply two independent copies of `a₀`.
    pub(super) fn sqr(self) -> Self {
        Self(core::array::from_fn(|k| {
            let cross =
                (0..k.div_ceil(2)).fold(Interval::zero(), |acc, i| acc + self.0[i] * self.0[k - i]);
            let doubled = cross * pt(2.0);
            if k % 2 == 0 {
                doubled + self.0[k / 2].sqr()
            } else {
                doubled
            }
        }))
    }

    /// `1/x`: `b₀ = 1/a₀`, `bₖ = −(Σ_{i=1..k} aᵢ·bₖ₋ᵢ)/a₀`.
    pub(super) fn recip(self) -> Self {
        let inv = Interval::one() / self.0[0];
        let mut b = [Interval::zero(); ORDER];
        b[0] = inv;
        for k in 1..ORDER {
            let sum = (1..=k).fold(Interval::zero(), |acc, i| acc + self.0[i] * b[k - i]);
            b[k] = -(sum * inv);
        }
        Self(b)
    }

    /// `√x`: `s₀ = √a₀`, `sₖ = (aₖ − Σ_{i=1..k−1} sᵢ·sₖ₋ᵢ)/(2·s₀)`.
    pub(super) fn sqrt(self) -> Self {
        let mut r = [Interval::zero(); ORDER];
        r[0] = self.0[0].sqrt();
        let half_inv = Interval::one() / (pt(2.0) * r[0]);
        for k in 1..ORDER {
            let sum = (1..k).fold(Interval::zero(), |acc, i| acc + r[i] * r[k - i]);
            r[k] = (self.0[k] - sum) * half_inv;
        }
        Self(r)
    }
}

/// The `i`-th of `pieces + 1` equally spaced nodes from `a` to `b`, the
/// last exactly `b`.
#[allow(clippy::cast_precision_loss)]
pub(super) fn node(a: f64, b: f64, pieces: usize, i: usize) -> f64 {
    if i == pieces {
        b
    } else {
        a + (b - a) * (i as f64 / pieces as f64)
    }
}

/// **The rule** (module docs) over `[t₀, t₁]` in `pieces` pieces, with
/// per component the part of its width a later round removes (the
/// remainders'). `prepare(p, q)` forms what the integrand reads on the
/// piece `[p, q]`, once per piece; `g(ctx, t)` forms the integrand's jets
/// from the enclosure of `t` it is handed — the thin node, or the piece —
/// and, at non-thin ends, the span's hull for the endpoint pad (prepared
/// over that hull). One rule for every integrand of one variable the
/// curved-rim lanes read.
pub(super) fn composite_by<const N: usize, C>(
    t0: Interval,
    t1: Interval,
    pieces: usize,
    prepare: impl Fn(f64, f64) -> C,
    g: impl Fn(&C, Interval) -> [Jet; N],
) -> ([Interval; N], [f64; N]) {
    let (a, b) = (mid(t0), mid(t1));
    let span = b - a;
    if !(t0.is_certified() && t1.is_certified() && span.is_finite() && span >= 0.0) || pieces == 0 {
        return ([Interval::refused(); N], [f64::INFINITY; N]);
    }
    let mut total = [Interval::zero(); N];
    let mut removable = [0.0_f64; N];
    for i in 0..pieces {
        let (p, q) = (node(a, b, pieces, i), node(a, b, pieces, i + 1));
        let x = p + (q - p) * 0.5;
        let (ri, le) = (pt(q) - pt(x), pt(p) - pt(x));
        // `∫_p^q (t − x)ᵏ dt`, `k = 0..=4`.
        let weight: [Interval; ORDER] = core::array::from_fn(|k| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            let e = k as i32 + 1;
            (ri.powi(e) - le.powi(e)) / pt(f64::from(e))
        });
        let ctx = prepare(p, q);
        let at = g(&ctx, pt(x));
        let over = g(&ctx, Interval::from_bounds(p, q));
        for k in 0..N {
            let exact =
                (0..ORDER - 1).fold(Interval::zero(), |acc, j| acc + at[k].0[j] * weight[j]);
            let remainder = over[k].0[ORDER - 1] * weight[ORDER - 1];
            removable[k] += remainder.width();
            total[k] = total[k] + exact + remainder;
        }
    }
    let widths = (pt(t0.hi()) - pt(a)).hi().max((pt(a) - pt(t0.lo())).hi())
        + (pt(t1.hi()) - pt(b)).hi().max((pt(b) - pt(t1.lo())).hi());
    if widths > 0.0 {
        let hull = Interval::hull(t0, t1);
        let sup = g(&prepare(hull.lo(), hull.hi()), hull);
        for k in 0..N {
            let pad = (sup[k].0[0] * pt(widths)).mag();
            total[k] = total[k] + Interval::from_bounds(-pad, pad);
        }
    }
    (total, removable)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    /// **The square is the product with its value kept off zero**: the
    /// jet of `(1 + t)` at `t ∈ [−2, 2]` squares to the product's
    /// coefficients, and its value's enclosure is `[0, 9]` where the
    /// product's reaches below zero.
    #[test]
    fn a_jets_square_is_its_product_with_its_value_off_zero() {
        let t = Interval::from_bounds(-2.0, 2.0);
        let j = Jet::cst(Interval::one()).add(Jet::var(t));
        let (sq, mul) = (j.sqr(), j.mul(j));
        for k in 1..ORDER {
            assert!(
                sq.0[k].lo() >= mul.0[k].lo() && sq.0[k].hi() <= mul.0[k].hi(),
                "coefficient {k}: {:?} against {:?}",
                sq.0[k],
                mul.0[k]
            );
        }
        assert!(
            sq.0[0].lo() >= 0.0 && mul.0[0].lo() < 0.0,
            "{:?} {:?}",
            sq.0[0],
            mul.0[0]
        );
    }
}
