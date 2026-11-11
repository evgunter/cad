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
use super::trig::{rotate, spread_cos, spread_sin, trig_at};

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
        let x = geom::bracket_mid((p, q));
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

/// **The rule over consecutive intervals**: [`composite_by`] over each
/// `(a, b, item, pieces)` of `intervals` (an edge's Bézier spans, or its carrier
/// interval cut at breaks), each at its own piece count, summed with their
/// removable widths; `prepare` reads the interval's item. The one home of
/// the per-interval accumulator every curved-rim lane cuts an edge by.
pub(super) fn composite_each<S, C, const N: usize>(
    intervals: impl IntoIterator<Item = (f64, f64, S, usize)>,
    prepare: impl Fn(&S, f64, f64) -> C,
    g: impl Fn(&C, Interval) -> [Jet; N],
) -> ([Interval; N], [f64; N]) {
    let mut total = [Interval::zero(); N];
    let mut removable = [0.0_f64; N];
    for (a, b, item, pieces) in intervals {
        let (part, rem) = composite_by(pt(a), pt(b), pieces, |p, q| prepare(&item, p, q), &g);
        for k in 0..N {
            total[k] = total[k] + part[k];
            removable[k] += rem[k];
        }
    }
    (total, removable)
}

/// **The turn one piece of a span may take about a chart's axis**: a
/// sixteenth of a turn, what a full-turn harmonic rim's round-0 piece
/// turns ([`super::quad::EDGE_INIT_PIECES`]). The rotational
/// volume lane reads it as the azimuth's rate times the piece's length,
/// and the incidence read of a net's twin as its piece's control box
/// over the box's distance from the axis
/// (`crate::pcurve_cache::Parts::ByTurn`).
#[allow(clippy::cast_precision_loss)]
pub(crate) const PIECE_TURN: f64 = core::f64::consts::TAU / super::quad::EDGE_INIT_PIECES as f64;

/// **The fewest equal pieces `[a, b]` is read in**, a power of two up
/// to `most`, for which `fits(ctx, piece, h)` holds on every piece (the
/// context prepared over it, the piece's enclosure, its length): `most`
/// where none does. Each count is tried in full, so the reading is a
/// function of the interval and `fits` alone: the one home of how many
/// pieces a span needs, each reader passing its own `fits`.
pub(crate) fn pieces_where<C>(
    (a, b): (f64, f64),
    most: usize,
    prepare: impl Fn(f64, f64) -> C,
    fits: impl Fn(&C, Interval, f64) -> bool,
) -> usize {
    let mut pieces = 1;
    while pieces < most
        && !(0..pieces).all(|i| {
            let (p, q) = (node(a, b, pieces, i), node(a, b, pieces, i + 1));
            fits(&prepare(p, q), Interval::from_bounds(p, q), q - p)
        })
    {
        pieces *= 2;
    }
    pieces
}

/// **`Σ h·bound` over the pieces of consecutive intervals**, as
/// [`composite_each`] cuts them: `bound` reads the jets enclosing a piece
/// and answers an upper bound of a nonnegative rate over it, so the sum
/// bounds that rate's integral from above (a length, where the rate is a
/// speed).
pub(super) fn rate_bound_over<S, C>(
    intervals: impl IntoIterator<Item = (f64, f64, S)>,
    pieces: usize,
    prepare: impl Fn(&S, f64, f64) -> C,
    bound: impl Fn(&C, Interval) -> f64,
) -> f64 {
    let mut total = 0.0_f64;
    for (a, b, item) in intervals {
        for i in 0..pieces {
            let (p, q) = (node(a, b, pieces, i), node(a, b, pieces, i + 1));
            let rate = bound(&prepare(&item, p, q), Interval::from_bounds(p, q));
            total += (pt(rate) * (pt(q) - pt(p))).hi();
        }
    }
    total
}

/// The jets of `t`, `cos t` and `sin t` over the bracket `t`, the
/// values read through [`trig_of`], so a refused `t` gives refused jets.
pub(super) fn trig_jets(t: Interval) -> (Jet, Jet, Jet) {
    let (c, s) = trig_of(t);
    let z = Interval::zero();
    (
        Jet([t, Interval::one(), z, z, z]),
        Jet([c, -s, -c / pt(2.0), s / pt(6.0), c / pt(24.0)]),
        Jet([s, c, -s / pt(2.0), -c / pt(6.0), s / pt(24.0)]),
    )
}

/// `(cos t, sin t)` over every `t ∈ [lo, hi]`, exact `f64` ends: rotated
/// from the exact `(1, 0)` at `t = 0` to the midpoint, then widened over
/// the half-width by the interval rotation — the one widening every
/// enclosure here reads (a parameter bracket, a piece, an edge's span).
/// Exact for `lo = hi`; the whole circle past a half-width of `1.5`.
pub(super) fn trig_range(lo: f64, hi: f64) -> (Interval, Interval) {
    let m = geom::bracket_mid((lo, hi));
    if !(m.is_finite() && lo <= hi) {
        return (Interval::refused(), Interval::refused());
    }
    let at = trig_at((Interval::one(), Interval::zero()), m);
    let hw = (pt(hi) - pt(m)).hi().max((pt(m) - pt(lo)).hi());
    rotate(at.0, at.1, spread_cos(hw), spread_sin(hw))
}

/// [`trig_range`] over a parameter bracket.
pub(super) fn trig_of(x: Interval) -> (Interval, Interval) {
    if !x.is_certified() {
        return (Interval::refused(), Interval::refused());
    }
    trig_range(x.lo(), x.hi())
}

/// `∫_a^b g` for `N` integrands at once, `pieces` equal pieces (the rule
/// in the module docs), plus the endpoint-bracket pad: `a`, `b` are the
/// brackets' midpoints, and `widths` their summed half-widths, paid at
/// `sup|g|` over the span's own window; and per integrand the summed
/// width of its remainder terms `g₄·∫(t − x)⁴`, the part of its width a
/// later round removes. The integrands see `t`, `cos t` and `sin t` as
/// jets ([`composite_by`] over them).
pub(super) fn composite<const N: usize>(
    t0: Interval,
    t1: Interval,
    pieces: usize,
    g: impl Fn(Jet, Jet, Jet) -> [Jet; N],
) -> ([Interval; N], [f64; N]) {
    composite_by(
        t0,
        t1,
        pieces,
        |_, _| (),
        |(), t| {
            let (tj, cj, sj) = trig_jets(t);
            g(tj, cj, sj)
        },
    )
}

// ---------------------------------------------------------------------
// The work of a piece
// ---------------------------------------------------------------------

/// Weight of one piece of one spiric edge (and of any integrand read
/// through the trig jets and one reciprocal: a projected circle), in
/// [`super::quad::FACE_WORK_WALL`]'s unit.
pub(super) const W_SPIRIC: usize = 46;

/// Weight of one piece of one Bézier span of a rational rim edge
/// ([`super::rational_rim`]), in [`super::quad::FACE_WORK_WALL`]'s unit,
/// by the carrier's degree: a piece shifts four power forms of the
/// degree's length to the piece and reads them by Horner twice, so its
/// cost grows with the degree. Measured in release on this box (a timing
/// probe, reverted), 50 calls of 256 pieces on one span of each degree,
/// against a spiric piece's 13.2 µs measured on the same box, which
/// [`W_SPIRIC`] weighs at 46: the weight is the time in that ratio,
/// rounded up. Past degree 7, each degree adds the last step's 60.
///
/// | degree | µs a piece | weight |
/// |---|---|---|
/// | 1 | 34.5 | 120 |
/// | 2 | 44.5 | 155 |
/// | 3 | 63.0 | 220 |
/// | 4 | 77.4 | 270 |
/// | 5 | 90.6 | 316 |
/// | 6 | 116 | 404 |
/// | 7 | 132 | 460 |
pub(super) const fn w_rational(degree: usize) -> usize {
    match degree {
        0 | 1 => 120,
        2 => 155,
        3 => 220,
        4 => 270,
        5 => 316,
        6 => 404,
        7 => 460,
        _ => 460 + 60 * (degree - 7),
    }
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

    /// **The trig jets of an uncertified bracket refuse**: `√[−1, 4]` is
    /// `[0, 2]` with ordinary ends and a clamped domain, and every
    /// coefficient of its `cos` and `sin` jets refuses, where the same
    /// ends read bare (`trig_range`) certify.
    #[test]
    fn the_trig_jets_of_an_uncertified_bracket_refuse() {
        let t = Interval::from_bounds(-1.0, 4.0).sqrt();
        assert!(
            !t.is_certified() && t.lo().is_finite() && t.hi().is_finite(),
            "the fixture is an uncertified bracket with ordinary ends: {t:?}"
        );
        let (_, cj, sj) = trig_jets(t);
        for (name, jet) in [("cos", cj), ("sin", sj)] {
            for (k, c) in jet.0.iter().enumerate() {
                assert!(
                    !c.is_certified(),
                    "{name} jet coefficient {k} certifies: {c:?}"
                );
            }
        }
        let (c, s) = trig_range(t.lo(), t.hi());
        assert!(
            c.is_certified() && s.is_certified(),
            "the bare ends certify, so the refusal is trig_jets' own: {c:?} {s:?}"
        );
    }
}
