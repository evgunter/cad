//! **The blossom of a span window** — the one certified primitive that
//! restricts a span's polynomial piece, in certification arithmetic.
//!
//! On a span `s` of a degree-`p` vector, the piece's coefficients
//! `c_{s−p} … c_s` define its blossom `f(u_1, …, u_p)`: symmetric,
//! affine in each argument, and the piece's value on the diagonal. De
//! Boor's recurrence computes it with argument `u_r` at level `r`,
//! every step a two-point combination at the ratio
//! `(u_r − U_i)/(U_{i+p+1−r} − U_i)`. For `u_r` in the span's closure
//! that ratio is in `[0, 1]` for every `i` the recurrence touches
//! (`U_i ≤ U_s` and `U_{i+p+1−r} ≥ U_{s+1}`), which is the precondition
//! of [`Certification::between`], the step every level goes through.
//!
//! **The argument contract.** A piece's arguments lie in its span's
//! closure, and every door here meets each argument with that closure
//! before using it. The meet is sound only under the contract: an
//! argument that straddles a knot because its TRUE value lies past it
//! would be clamped to the wrong polynomial, and no door can tell that
//! from an argument whose enclosure merely overhangs. The callers hold
//! it by construction — a break of the piece's own span, or a ratio of
//! the piece's own ends — so it is stated, not checked. An argument
//! with no point in the closure refuses typed
//! ([`BlossomError::OutsideSpan`]). The one caller that cannot hold it
//! by construction, a quadrature node that may lie past its span (a
//! vertex pad's), goes through [`diagonal_in_span`], which checks the
//! node lies whole in the closure and answers `None` otherwise.
//!
//! Everything else is this door at chosen arguments:
//!
//! - **Evaluation** at `t` is the blossom at `(t, …, t)`.
//! - **Restriction** to `[a, b]` ([`CoeffWindow::restrict`],
//!   [`SpanRow::restrict`]) is the Bernstein row whose coefficient `k`
//!   is `f(a^{p−k}, b^k)`; at the span's own ends it is the span's
//!   Bézier segment. The `p + 1` coefficients share their `a` levels —
//!   coefficient `k` is the pyramid's level `p − k` at `a` carried `k`
//!   levels further at `b` — so each is still exactly the `p` steps of
//!   its own blossom, and the sharing changes no bit.
//! - **A Bézier segment's row** on `[s0, s1]` ([`restrict_segment`])
//!   is a proof about [`KnotVector::bezier`]`(p, (s0, s1))`, whose
//!   ratio at each cut end is formed once, so no vector is minted for
//!   it.
//!
//! So no coefficient is more than `p` steps deep from the window it was
//! read from, wherever the arguments fall and however many pieces a
//! span is cut into.
//!
//! [`KnotVector::bezier`]: super::KnotVector::bezier

use super::hull::CoeffWindow;
use super::knots::Span;
use crate::interval::Interval;
use crate::interval::certification::Certification;
use crate::readable::Readable;
use crate::real::{Bounds, CertifiedBounds};

/// A typed refusal of the blossom doors (fail-loud; the kernel never
/// panics).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlossomError {
    /// The blossom of a degree-`degree` piece takes exactly `degree`
    /// arguments.
    ArgumentCount {
        /// The window's degree.
        degree: usize,
        /// The number of arguments given.
        got: usize,
    },
    /// Argument `index` lies wholly outside the span `[lo, hi]`: the
    /// window answers for its own piece, and the piece's arguments lie
    /// in its span by contract (module docs).
    OutsideSpan {
        /// Which argument.
        index: usize,
        /// The span's lower knot.
        lo: f64,
        /// The span's upper knot.
        hi: f64,
    },
    /// A bare Bernstein row with no coefficient names no polynomial.
    EmptyRow,
}

impl core::fmt::Display for BlossomError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::ArgumentCount { degree, got } => write!(
                f,
                "a degree-{degree} blossom takes {degree} arguments, not {got}"
            ),
            Self::OutsideSpan { index, lo, hi } => write!(
                f,
                "blossom argument {index} lies wholly outside the span [{}, {}]",
                Readable(lo),
                Readable(hi)
            ),
            Self::EmptyRow => write!(f, "a Bernstein row with no coefficient"),
        }
    }
}

impl core::error::Error for BlossomError {}

/// A span's `p + 1` window coefficients held beside the span — the row a
/// piece's blossom reads. Minted by [`Span::with_window`], which checks
/// the one relation a length can state, so the row and the span name the
/// same piece by the caller's construction and the same degree by check.
/// The shape for a row that is a caller's own intermediate (a line of a
/// cell block, one homogeneous channel); a coefficient array minted
/// against the whole vector reads through [`CoeffWindow`] instead.
#[derive(Clone, Copy, Debug)]
pub struct SpanRow<'a> {
    span: Span<'a>,
    row: &'a [Interval],
}

impl<'a> Span<'a> {
    /// `row` held as this span's window — `None` unless it is
    /// `degree + 1` long.
    #[must_use]
    pub fn with_window(self, row: &'a [Interval]) -> Option<SpanRow<'a>> {
        (row.len() == self.degree() + 1).then_some(SpanRow { span: self, row })
    }
}

impl SpanRow<'_> {
    /// The piece's blossom at `args` ([`CoeffWindow::blossom`]).
    ///
    /// # Errors
    ///
    /// As [`CoeffWindow::blossom`].
    pub fn blossom(self, args: &[Interval]) -> Result<Interval, BlossomError> {
        span_blossom(self.span, self.row.to_vec(), args)
    }

    /// The piece's Bernstein row on `[a, b]` ([`CoeffWindow::restrict`]).
    ///
    /// # Errors
    ///
    /// As [`CoeffWindow::restrict`].
    pub fn restrict(self, ends: (Interval, Interval)) -> Result<Vec<Interval>, BlossomError> {
        span_restrict(self.span, self.row, ends)
    }
}

impl<E: CertifiedBounds> CoeffWindow<'_, E> {
    /// The piece's blossom at `args`, enclosed: an enclosure of
    /// `f(u_1, …, u_p)` for every `u_r ∈ args[r − 1]`.
    ///
    /// Each argument is met with the span's closure first (the module
    /// docs' argument contract), which keeps every level's ratio in
    /// `[0, 1]`. A refused argument or coefficient flows through as NaI.
    ///
    /// # Errors
    ///
    /// [`BlossomError::ArgumentCount`] unless there are exactly `p`
    /// arguments; [`BlossomError::OutsideSpan`] for a certified argument
    /// with no point in the span.
    pub fn blossom(self, args: &[Interval]) -> Result<Interval, BlossomError> {
        span_blossom(self.span(), self.row(), args)
    }

    /// The Bernstein row of the piece restricted to `[a, b]`:
    /// coefficient `k` is the blossom at `a` taken `p − k` times and `b`
    /// taken `k` times (module docs), so each is `p` steps deep. At the
    /// span's own ends it is the span's Bézier segment.
    ///
    /// # Errors
    ///
    /// As [`CoeffWindow::blossom`]: an end wholly outside the span.
    pub fn restrict(self, ends: (Interval, Interval)) -> Result<Vec<Interval>, BlossomError> {
        span_restrict(self.span(), &self.row(), ends)
    }

    /// The window's coefficients crossed into certification arithmetic.
    fn row(self) -> Vec<Interval> {
        self.coeffs()
            .iter()
            .map(|&c| Interval::from_certified(c))
            .collect()
    }
}

/// The arithmetic `restrict_bernstein` had before its one caller
/// (`geom-brep`'s trim bisection) moved to [`restrict_segment`] on
/// `[0, 1]`: the pyramid on the row at the admitted ends, every ratio
/// the argument itself. The oracle for that caller's bit-identity row,
/// read through `crate::test_support`.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn bernstein_restriction_as_retired(
    row: &[Interval],
    (a, b): (Interval, Interval),
) -> Result<Vec<Interval>, BlossomError> {
    let p = row.len().checked_sub(1).ok_or(BlossomError::EmptyRow)?;
    let (a, b) = (
        admit(a, 0, (0.0, 1.0))?,
        admit(b, p.saturating_sub(1), (0.0, 1.0))?,
    );
    Ok(pyramid(row, (a, b), |_, _, u| u))
}

/// The Bernstein row on `[a, b]` of the Bézier segment whose row on
/// `[s0, s1]` is `row`: [`SpanRow::restrict`] on
/// [`KnotVector::bezier`]`(p, (s0, s1))` at the exact cut ends, so
/// coefficient `k` is the segment's blossom at `(a^(p−k), b^k)`, `p`
/// steps of [`Certification::between`] deep from the segment's own row
/// wherever the cut falls. The whole segment comes back as it came, and
/// a constant (one coefficient) is its own restriction.
///
/// A cut out of order comes back refused, every coefficient of it: an
/// end outside the segment, a reversed cut (sound, blossoms being
/// symmetric, and refused by contract), an empty or unbounded segment,
/// a cut that is not a number. An empty row comes back empty.
///
/// **The restriction door for a row that is read as it comes**: the
/// SSI section's cuts and halves and the trim bisection's halves.
/// It is the door for them by width: of restriction's three homes it is
/// the tightest on those callers. Measured on the section's and props'
/// real inputs (2026-10-20), [`split_restrict`] is up to 98× wider than
/// it on the section's general cuts and 14× at its cut-and-derivative
/// step, and `super::derivative`'s `Grid::halved` arithmetic (unscaled
/// sums and one exact `2⁻ʳ` per output, its generic halving at ½) is up
/// to 2× wider at the section's exact-half cuts. At the section's own
/// halves of a `[0, 1]` row the other two are wider on 0.01% of
/// coefficients, by at most 1.6× (3 ulp); at the trim bisection's
/// halves all three agree to the bit. The other two are kept for what
/// this one is not: `split_restrict` is the cut's fast form, `O(p²)`
/// against `O(p³)`, sound beside it because its reader meets every cut
/// with the whole cell's; `Grid::halved` is `super::derivative`'s
/// halving, generic over `T: Real` with exact `2⁻ʳ` steps, where this
/// reads certification scalars only. The numbers are a dated
/// measurement nothing computes with, so no row guards them; the table
/// and how it was taken are in
/// `work/nurbs/binomial-bernstein-product-and-blossom-have-two-homes-each.md`.
///
/// [`KnotVector::bezier`]: super::KnotVector::bezier
// The negated guard refuses a NaN cut, where `a < s0` would pass it.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
#[must_use]
pub fn restrict_segment(
    row: &[Interval],
    (s0, s1): (f64, f64),
    (a, b): (f64, f64),
) -> Vec<Interval> {
    if row.is_empty() || !(s0 <= a && a <= b && b <= s1 && s0 < s1) {
        return vec![Interval::refused(); row.len()];
    }
    if row.len() == 1 || (a == s0 && b == s1 && row.iter().all(|c| c.is_certified())) {
        return row.to_vec();
    }
    // `KnotVector::bezier(p, (s0, s1))` refuses only a non-finite end
    // here. On it every level ratio of the span is `(u − s0)/(s1 − s0)`
    // (`slice_ratio`: `t_i = s0` and `t_{i+p+1−r} = s1` for every entry
    // `j ≥ r` a level writes), so the ratio at each cut end is formed
    // once, the same expression, and the pyramid runs on it.
    if !(s0.is_finite() && s1.is_finite()) {
        return vec![Interval::refused(); row.len()];
    }
    let p = row.len() - 1;
    let (Ok(a), Ok(b)) = (
        admit(Interval::point(a), 0, (s0, s1)),
        admit(Interval::point(b), p - 1, (s0, s1)),
    ) else {
        return vec![Interval::refused(); row.len()];
    };
    let below = Interval::point(s0);
    let width = Interval::point(s1) - below;
    pyramid(row, ((a - below) / width, (b - below) / width), |_, _, u| u)
}

/// A split's weights `(t, 1 − t)` for the thin convex step
/// `x·(1 − t) + y·t`, three outward interval operations per entry.
/// [`Certification::between`]'s monotone and convex forms met are built
/// for the blossom door's general ratios; a window's ratio is the
/// enclosure of one point's, a few ulps wide. The plain step encloses
/// the exact step for every `t` in that enclosure (`[0, 1]` after the
/// clamp); what it costs in width is measured on the cut's meets.
fn convex(t: Interval) -> (Interval, Interval) {
    (t, Interval::point(1.0) - t)
}

/// The thin convex step `x·(1 − t) + y·t` at the weights `(t, 1 − t)`
/// ([`convex`]), and `x` itself where `y` is the same enclosure: a run
/// of equal coefficients (a flat wall's constant derivative) stays
/// exactly what it was, as [`Certification::between`] keeps it, where
/// the plain step would widen it by `t`'s own width at every level.
/// Sound: a convex combination of two values in one interval is in it.
/// The same enclosure means the same ends AND the same decoration
/// ([`Interval::same_enclosure`]): a certified `x` beside a refused `y`
/// with its ends, or a `Com` one beside a `Def` one, takes the general
/// step, which carries the weaker, rather than coming back as `x`.
fn step(x: Interval, y: Interval, (t, s): (Interval, Interval)) -> Interval {
    if x.same_enclosure(y) {
        x
    } else {
        // Both weights' lower ends are non-negative (`t` is clamped to
        // `[0, 1]`; `s = 1 − t` reaches `1 + 2⁻⁵²` above at a tiny
        // positive `t.lo`),
        // which is all `Interval::mul_nonneg` reads two corners on, bit
        // for bit the general product.
        x.mul_nonneg(s) + y.mul_nonneg(t)
    }
}

/// The Bernstein row on `[a, b]` of the segment whose row on `[s0, s1]`
/// is `row`, by two de Casteljau splits: at `(a − s0)/(s1 − s0)`,
/// keeping the right part (the row on `[a, s1]`), then at
/// `(b − a)/(s1 − a)`, keeping the left. Each step is the thin convex
/// step ([`convex`]) at an interval ratio clamped to `[0, 1]`, so at
/// `Interval` every coefficient encloses the exact restriction's,
/// `2p` steps deep; `O(p²)` against the blossom pyramid's `O(p³)`
/// ([`restrict_segment`]). A split at an end of its segment is
/// skipped. A cut out of order, or a segment that is empty or not
/// finite, refuses every coefficient.
///
/// **The restriction cut's fast form, for a reader that meets it**:
/// `derivative`'s cut pieces meet every cut box with the whole cell's,
/// which absorbs its width. Unmet it is up to 417× wider per
/// coefficient than the pyramid on the same inputs (measured at
/// `cut_pieces`), so a row read as it comes restricts at
/// [`restrict_segment`].
// The negated guard refuses a NaN cut, where `a < s0` would pass it.
#[allow(clippy::neg_cmp_op_on_partial_ord)]
pub(crate) fn split_restrict(
    row: &[Interval],
    (s0, s1): (f64, f64),
    (a, b): (f64, f64),
) -> Vec<Interval> {
    if row.is_empty()
        || !(s0 <= a && a <= b && b <= s1 && s0 < s1 && s0.is_finite() && s1.is_finite())
    {
        return vec![Interval::refused(); row.len()];
    }
    let ratio = |x: f64, lo: f64| {
        let lo = Interval::point(lo);
        ((Interval::point(x) - lo) / (Interval::point(s1) - lo)).clamped_to(0.0, 1.0)
    };
    let mut d = row.to_vec();
    let p = d.len() - 1;
    if a > s0 {
        // The right part: entry `k` is level `p − k`'s entry `k`, the
        // last each level writes from the low end up.
        let (t, s) = convex(ratio(a, s0));
        for r in 1..=p {
            for k in 0..=p - r {
                d[k] = step(d[k], d[k + 1], (t, s));
            }
        }
    }
    if b < s1 {
        // The left part: entry `k` is level `k`'s entry `0`, the last
        // each level writes from the high end down.
        let (t, s) = convex(ratio(b, a));
        for r in 1..=p {
            for k in (r..=p).rev() {
                d[k] = step(d[k - 1], d[k], (t, s));
            }
        }
    }
    d
}

/// `u` met with the closure `[lo, hi]` (module docs' argument contract):
/// a refusal flows on, a certified argument with no point there refuses.
fn admit(u: Interval, index: usize, (lo, hi): (f64, f64)) -> Result<Interval, BlossomError> {
    if !u.is_certified() {
        return Ok(u);
    }
    let inside = u.clamped_to(lo, hi);
    if inside.is_certified() {
        Ok(inside)
    } else {
        Err(BlossomError::OutsideSpan { index, lo, hi })
    }
}

/// The span's closure and its level ratio: at level `r`, entry `j` of
/// the window combines at `(u − U_i)/(U_{i+p+1−r} − U_i)`, `i = s − p + j`.
/// A span's degree, index and knot list, the one read both the ratio
/// and the Bézier test take.
fn span_knots(span: Span<'_>) -> (usize, usize, &[f64]) {
    (
        span.degree(),
        span.index(),
        span.knots_f64_interim().knots(),
    )
}

fn span_ratio(span: Span<'_>) -> ((f64, f64), impl Fn(usize, usize, Interval) -> Interval) {
    let (p, s, t) = span_knots(span);
    // Indexing justified: `p ≤ s ≤ last_span() = len − p − 2`.
    ((t[s], t[s + 1]), slice_ratio(t, p, s))
}

/// The recurrence's ratio at entry `j` of level `r`, on the span `s` of
/// the knot list `t`: `(u − t_i)/(t_{i+p+1−r} − t_i)` with
/// `i = s − p + j`, the knots read as exact points. The caller keeps
/// `s ≥ p` and `s + p < t.len()`, so `i + p + 1 − r ≤ s + p` is in range
/// for `j ≤ p`, `r ≥ 1`.
fn slice_ratio(t: &[f64], p: usize, s: usize) -> impl Fn(usize, usize, Interval) -> Interval {
    move |j: usize, r: usize, u: Interval| {
        let i = s - p + j;
        let below = Interval::point(t[i]);
        (u - below) / (Interval::point(t[i + p + 1 - r]) - below)
    }
}

/// The value at `t` of the degree-`p` piece over the knot list `knots`
/// whose span starts at control `first`, from its `p + 1` coefficients
/// `window`: the blossom on the diagonal `(t, …, t)`, every level
/// [`Certification::between`], when `t` lies whole in the span's
/// closure `[knots[first + p], knots[first + p + 1]]`. `None` when it
/// does not (refused, or reaching past either end), and for a window
/// that is not `p + 1` long or a list too short for the span: a piece's
/// value past its span is its polynomial extension, no combination
/// between its sources, so it is the caller's to answer.
///
/// The list need not be a clamped vector's (a derivative's raw knot
/// list, `derivative_knot_slice`'s), only nondecreasing, which keeps
/// every ratio in `[0, 1]` inside the closure. This door holds the
/// argument contract by its explicit check (module docs); the result
/// is bit-identical to [`CoeffWindow::blossom`] at `(t, …, t)` on the
/// same span.
#[must_use]
pub fn diagonal_in_span(
    knots: &[f64],
    p: usize,
    first: usize,
    window: &[Interval],
    t: Interval,
) -> Option<Interval> {
    let s = first + p;
    if window.len() != p + 1 || s + p.max(1) >= knots.len() {
        return None;
    }
    let (lo, hi) = (knots[s], knots[s + 1]);
    if !(t.is_certified() && Bounds::lo(t) >= lo && Bounds::hi(t) <= hi) {
        return None;
    }
    let ratio = slice_ratio(knots, p, s);
    let mut d = window.to_vec();
    for r in 1..=p {
        level(&mut d, r, t, &ratio);
    }
    Some(d[p])
}

/// One level `r` of the recurrence at `u`, in place: entries `p … r`.
fn level(
    d: &mut [Interval],
    r: usize,
    u: Interval,
    ratio: &impl Fn(usize, usize, Interval) -> Interval,
) {
    for j in (r..d.len()).rev() {
        d[j] = Interval::between(d[j - 1], d[j], ratio(j, r, u));
    }
}

fn span_blossom(
    span: Span<'_>,
    mut d: Vec<Interval>,
    args: &[Interval],
) -> Result<Interval, BlossomError> {
    let p = span.degree();
    if args.len() != p {
        return Err(BlossomError::ArgumentCount {
            degree: p,
            got: args.len(),
        });
    }
    let (closure, ratio) = span_ratio(span);
    for (index, &u) in args.iter().enumerate() {
        level(&mut d, index + 1, admit(u, index, closure)?, &ratio);
    }
    Ok(d[p])
}

fn span_restrict(
    span: Span<'_>,
    row: &[Interval],
    (a, b): (Interval, Interval),
) -> Result<Vec<Interval>, BlossomError> {
    let p = span.degree();
    let (closure, ratio) = span_ratio(span);
    let (a, b) = (admit(a, 0, closure)?, admit(b, p - 1, closure)?);
    if whole_bezier_span(span, (a, b)) && row.iter().all(|c| c.is_certified()) {
        return Ok(row.to_vec());
    }
    Ok(pyramid(row, (a, b), ratio))
}

/// Whether `(a, b)` restricts a Bézier segment to its own whole span:
/// both ends exact points on the span's two knots, and the span's
/// window reading the left knot `p` times and the right `p` times. The
/// restriction is then the identity in exact arithmetic — the blossom
/// `B(a^{p−k}, b^k)` of a Bézier segment over `[a, b]` is its `k`-th
/// coefficient — so the window's own coefficients are the restriction,
/// with none of the pyramid's rounding. A closed form's enclosing net
/// (`ORDER`-fold interior knots) and every span the composite has cut
/// at its own breaks is such a segment.
///
/// **The shortcut is taken only on a row whose every coefficient is
/// certified.** It is not the pyramid bit for bit: it is tighter at the
/// ulp, and the pyramid mixes every coefficient into every output, so a
/// row holding a refused (uncertified) coefficient comes back refused
/// in every output, where the window returned whole would keep its
/// certified neighbours certified. Such a row runs the pyramid, so the
/// shortcut never changes what refuses
/// (`a_whole_span_restriction_refuses_as_the_pyramid_does`).
fn whole_bezier_span(span: Span<'_>, (a, b): (Interval, Interval)) -> bool {
    use crate::real::Bounds as _;
    let (p, s, t) = span_knots(span);
    let point_at = |x: Interval, k: f64| x.is_certified() && x.lo() == k && x.hi() == k;
    // Indexing justified as `span_ratio`'s: `p ≤ s` and `s + p < t.len()`.
    point_at(a, t[s])
        && point_at(b, t[s + 1])
        && t[s + 1 - p..=s].iter().all(|&k| k == t[s])
        && t[s + 1..=s + p].iter().all(|&k| k == t[s + 1])
}

/// The restriction to `[a, b]` of the row: the pyramid of `a` levels,
/// each coefficient `k` its level `p − k` carried `k` levels on at `b`.
fn pyramid(
    row: &[Interval],
    (a, b): (Interval, Interval),
    ratio: impl Fn(usize, usize, Interval) -> Interval,
) -> Vec<Interval> {
    let p = row.len() - 1;
    let mut at_a = Vec::with_capacity(p + 1);
    at_a.push(row.to_vec());
    for r in 1..=p {
        let mut d = at_a[r - 1].clone();
        level(&mut d, r, a, &ratio);
        at_a.push(d);
    }
    (0..=p)
        .map(|k| {
            let mut d = at_a[p - k].clone();
            for r in p - k + 1..=p {
                level(&mut d, r, b, &ratio);
            }
            d[p]
        })
        .collect()
}

/// [`super::SpanLocate::restrict_window`] at the certification scalar:
/// [`SpanRow::restrict`] on the window, or `p + 1` refusals for a window
/// of the wrong length or an end outside the span.
pub(crate) fn restrict_window(
    window: &[Interval],
    span: Span<'_>,
    (a, b): (f64, f64),
) -> Vec<Interval> {
    span.with_window(window)
        .and_then(|w| w.restrict((Interval::point(a), Interval::point(b))).ok())
        .unwrap_or_else(|| vec![Interval::refused(); span.degree() + 1])
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::real::Bounds;
    use crate::spline::{KnotVector, SpanLocate};

    fn bits(v: &[Interval]) -> Vec<(u64, u64)> {
        v.iter()
            .map(|c| (c.lo().to_bits(), c.hi().to_bits()))
            .collect()
    }

    /// **The whole-span shortcut changes no refusal**
    /// (`whole_bezier_span`): a certified row restricted to its own whole
    /// span comes back as it came, bit for bit, through both doors (the
    /// span row and the bare segment); a row holding one refused
    /// coefficient runs the pyramid instead, and comes back as the
    /// pyramid gives it — every output refused, no certified neighbour
    /// kept. Red under a shortcut taken whatever the row's decoration.
    #[test]
    fn a_whole_span_restriction_refuses_as_the_pyramid_does() {
        let p = 3usize;
        let kv = KnotVector::bezier(p, (2.0, 6.0)).unwrap();
        let certified: Vec<Interval> = [0.5, 0.75, 1.0, 1.25].map(Interval::point).to_vec();
        let mut holed = certified.clone();
        holed[1] = Interval::from_bounds(-1.0, 1.44).sqrt();
        assert!(
            !holed[1].is_certified(),
            "the plant is refused: {:?}",
            holed[1]
        );
        let ends = (Interval::point(2.0), Interval::point(6.0));
        let span = kv.with_coeffs(&certified).unwrap().span(p).unwrap();
        let whole = span.restrict(ends).unwrap();
        assert_eq!(
            bits(&whole),
            bits(&certified),
            "a certified row comes back as it came"
        );
        assert!(whole.iter().all(|c| c.is_certified()));
        assert_eq!(
            bits(&restrict_segment(&certified, (2.0, 6.0), (2.0, 6.0))),
            bits(&certified),
            "the bare segment's door takes the same shortcut"
        );
        let held = kv.with_coeffs(&holed).unwrap().span(p).unwrap();
        let got = held.restrict(ends).unwrap();
        let (closure, ratio) = span_ratio(held.span());
        let (a, b) = (
            admit(ends.0, 0, closure).unwrap(),
            admit(ends.1, p - 1, closure).unwrap(),
        );
        let pyramid_row = pyramid(&holed, (a, b), ratio);
        assert_eq!(
            bits(&got),
            bits(&pyramid_row),
            "a holed row is the pyramid's"
        );
        assert_eq!(
            got.iter().map(|c| c.is_certified()).collect::<Vec<_>>(),
            pyramid_row
                .iter()
                .map(|c| c.is_certified())
                .collect::<Vec<_>>(),
            "the pyramid's decoration, output for output"
        );
        assert!(
            got.iter().all(|c| !c.is_certified()),
            "the pyramid refuses every output of a holed row: {got:?}"
        );
        let bare = restrict_segment(&holed, (2.0, 6.0), (2.0, 6.0));
        assert!(
            bare.iter().all(|c| !c.is_certified()),
            "the bare segment's door refuses a holed row whole: {bare:?}"
        );
        // A refused end whose ordinary ends sit on the knot is no exact
        // point: the certified row restricted at it is the pyramid's, and
        // the first output, read at that end alone, refuses. Red under a
        // shortcut that reads the end's bounds alone.
        let refused_end =
            Interval::from_bounds(-1.0, 1.0).sqrt() * Interval::point(0.0) + Interval::point(2.0);
        assert!(
            !refused_end.is_certified() && refused_end.lo() == 2.0 && refused_end.hi() == 2.0,
            "the plant is a refused point bracket on the knot: {refused_end:?}"
        );
        let at_refused = span.restrict((refused_end, ends.1)).unwrap();
        let (closure, ratio) = span_ratio(span.span());
        let pyramid_at = pyramid(
            &certified,
            (admit(refused_end, 0, closure).unwrap(), ends.1),
            ratio,
        );
        assert_eq!(
            bits(&at_refused),
            bits(&pyramid_at),
            "a refused end is the pyramid's"
        );
        assert!(
            !at_refused[0].is_certified(),
            "the output at the refused end refuses: {at_refused:?}"
        );
    }

    /// **The doors are one arithmetic, bit for bit.** The restriction's
    /// shared pyramid equals the `p + 1` blossoms it shares levels
    /// between (so each coefficient is still `p` steps deep from the
    /// window); a span row equals the coefficient window over the same
    /// coefficients; and the bare-row door equals the restriction on
    /// `KnotVector::bezier(p, (0, 1))` it stands for. Non-dyadic knots and
    /// cuts, widened coefficients, and enclosure arguments, at degrees
    /// 1–6.
    #[test]
    fn the_blossom_doors_agree_bitwise() {
        for p in 1..=6usize {
            let mut knots = vec![0.0; p + 1];
            knots.extend([0.3, 1.0 / 3.0, 0.7]);
            knots.extend(vec![1.0; p + 1]);
            let kv = KnotVector::clamped(knots, p).unwrap();
            #[allow(clippy::cast_precision_loss)]
            let coeffs: Vec<Interval> = (0..kv.control_count())
                .map(|i| {
                    let c = (0.37 * i as f64).sin();
                    Interval::from_bounds(c - 1e-12, c + 1e-12)
                })
                .collect();
            let pair = kv.with_coeffs(&coeffs).unwrap();
            let win = pair.span_at(0.31).unwrap();
            let (a, b) = (
                Interval::from_bounds(0.31, 0.3100000001),
                Interval::point(0.32),
            );
            let restricted = win.restrict((a, b)).unwrap();
            let blossoms: Vec<Interval> = (0..=p)
                .map(|k| {
                    let args: Vec<Interval> =
                        (0..p).map(|r| if r < p - k { a } else { b }).collect();
                    win.blossom(&args).unwrap()
                })
                .collect();
            assert_eq!(
                bits(&restricted),
                bits(&blossoms),
                "p={p}: pyramid vs blossoms"
            );
            let row = win.span().with_window(win.coeffs()).unwrap();
            assert_eq!(
                bits(&row.restrict((a, b)).unwrap()),
                bits(&restricted),
                "p={p}: span row vs coefficient window"
            );
            let span = win.span();
            let diagonal = diagonal_in_span(
                span.knots_f64_interim().knots(),
                p,
                span.first_control(),
                win.coeffs(),
                a,
            )
            .unwrap();
            assert_eq!(
                bits(&[diagonal]),
                bits(&[win.blossom(&vec![a; p]).unwrap()]),
                "p={p}: the raw-list diagonal vs the window's blossom"
            );
            let bez = KnotVector::bezier(p, (0.0, 1.0)).unwrap();
            let bern = &coeffs[..=p];
            let ends = (Interval::point(1.0 / 3.0), Interval::point(0.6));
            let via_pair = bez
                .with_coeffs(bern)
                .unwrap()
                .span(p)
                .unwrap()
                .restrict(ends)
                .unwrap();
            assert_eq!(
                bits(&restrict_segment(bern, (0.0, 1.0), (1.0 / 3.0, 0.6))),
                bits(&via_pair),
                "p={p}: bare row vs its Bézier pair"
            );
        }
    }

    /// **Every scalar refuses an end outside the span.** The piece's
    /// polynomial past its span is not the curve; at `f64` the default
    /// restriction answers NaN where the certification scalar refuses,
    /// rather than extrapolating.
    #[test]
    fn a_window_restricted_past_its_span_refuses_at_every_scalar() {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
        let span = kv.span_at(0.1).unwrap();
        let line = [0.1, 0.7, 1.3];
        let held = kv.knots();
        let at =
            |ends: (f64, f64)| <f64 as SpanLocate>::restrict_window(&line, span, held, ends, ends);
        let f = at((0.0, 1.0));
        assert!(f.iter().all(|c| c.is_nan()), "f64 extrapolated: {f:?}");
        let ok = at((0.0, 0.5));
        assert!(ok.iter().all(|c| c.is_finite()), "f64 in-span: {ok:?}");
        let lifted: Vec<Interval> = line.iter().map(|&c| Interval::point(c)).collect();
        let held_i: Vec<Interval> = held.iter().map(|&k| Interval::point(k)).collect();
        let pt = |(a, b): (f64, f64)| (Interval::point(a), Interval::point(b));
        let i = <Interval as SpanLocate>::restrict_window(
            &lifted,
            span,
            &held_i,
            (0.0, 1.0),
            pt((0.0, 1.0)),
        );
        assert!(i.iter().all(|c| !c.is_certified()), "Interval: {i:?}");
        let short =
            <f64 as SpanLocate>::restrict_window(&line[..2], span, held, (0.0, 0.5), (0.0, 0.5));
        assert!(
            short.len() == 3 && short.iter().all(|c| c.is_nan()),
            "{short:?}"
        );
        // Held knots that are not the vector's length refuse at both.
        let f =
            <f64 as SpanLocate>::restrict_window(&line, span, &held[1..], (0.0, 0.5), (0.0, 0.5));
        assert!(f.iter().all(|c| c.is_nan()), "f64 short held: {f:?}");
        let i = <Interval as SpanLocate>::restrict_window(
            &lifted,
            span,
            &held_i[1..],
            (0.0, 0.5),
            pt((0.0, 0.5)),
        );
        assert!(
            i.iter().all(|c| !c.is_certified()),
            "Interval short held: {i:?}"
        );
    }

    /// **A split restriction encloses the exact restriction, `2p` steps
    /// deep.** Rows of degree 1–8 with dyadic coefficients spread over
    /// `2^{±10}` (a wide weight power's span), cut at dyadic ends whose
    /// two split ratios are dyadic too, so the exact restriction is
    /// `f64` de Casteljau with no rounding: every coefficient of
    /// [`split_restrict`] contains it. Red under the second split
    /// at `(b − s0)/(s1 − s0)` (the ratio on the wrong segment), or under
    /// either split keeping the other part. And on drawn non-dyadic cuts
    /// it meets the blossom pyramid's restriction coefficient by
    /// coefficient (two enclosures of one row).
    #[test]
    fn a_split_restriction_encloses_the_exact_restriction() {
        // The exact de Casteljau split of a dyadic row at a dyadic `t`:
        // the right part (`right`) or the left.
        fn split(row: &[f64], t: f64, right: bool) -> Vec<f64> {
            let mut levels = vec![row.to_vec()];
            for r in 1..row.len() {
                let prev = &levels[r - 1];
                levels.push(
                    (0..prev.len() - 1)
                        .map(|i| (1.0 - t) * prev[i] + t * prev[i + 1])
                        .collect(),
                );
            }
            let p = row.len() - 1;
            (0..=p)
                .map(|k| {
                    if right {
                        levels[p - k][k]
                    } else {
                        levels[k][0]
                    }
                })
                .collect()
        }
        let mut rng = test_utils::fuzz::start("split-restrict-exact");
        // (s0, s1, a, b) with (a − s0)/(s1 − s0) and (b − a)/(s1 − a)
        // dyadic.
        let cuts = [
            (0.0, 1.0, 0.25, 0.625),
            (0.0, 1.0, 0.5, 0.75),
            (2.0, 6.0, 3.0, 4.5),
            (-1.0, 1.0, -0.5, 0.25),
            (0.0, 1.0, 0.0, 0.375),
            (0.0, 1.0, 0.75, 1.0),
        ];
        for _ in 0..test_utils::fuzz::scaled(200) {
            let p = 1 + rng.below(8);
            let row: Vec<f64> = (0..=p)
                .map(|_| {
                    let mag = 2.0_f64.powi(rng.below(21) as i32 - 10);
                    (rng.range(-8.0, 8.0)).round() * mag
                })
                .collect();
            let lifted: Vec<Interval> = row.iter().map(|&c| Interval::point(c)).collect();
            for (s0, s1, a, b) in cuts {
                let ta = (a - s0) / (s1 - s0);
                let tb = (b - a) / (s1 - a);
                let exact = split(&split(&row, ta, true), tb, false);
                let got = split_restrict(&lifted, (s0, s1), (a, b));
                for (k, (g, e)) in got.iter().zip(&exact).enumerate() {
                    assert!(
                        g.is_certified() && g.lo() <= *e && *e <= g.hi(),
                        "degree {p} on [{s0}, {s1}] cut to [{a}, {b}], coefficient {k}: \
                         {g:?} misses {e:e}; {}",
                        test_utils::fuzz::replay()
                    );
                }
            }
            let (x, y) = (rng.range(0.0, 1.0), rng.range(0.0, 1.0));
            let (a, b) = (x.min(y), x.max(y));
            let blossom = restrict_segment(&lifted, (0.0, 1.0), (a, b));
            let got = split_restrict(&lifted, (0.0, 1.0), (a, b));
            for (g, w) in got.iter().zip(&blossom) {
                assert!(
                    g.lo() <= w.hi() && w.lo() <= g.hi(),
                    "[{a}, {b}]: {g:?} and the pyramid's {w:?} are disjoint; {}",
                    test_utils::fuzz::replay()
                );
            }
        }
    }

    /// **A split restriction encloses the exact restriction where its
    /// ratios round.** Drawn non-dyadic segments and cuts, so both split
    /// ratios `(a − s0)/(s1 − s0)` and `(b − a)/(s1 − a)` are inexact in
    /// `f64`; drawn rows of degree 1–6. Every coefficient of
    /// [`split_restrict`] holds the exact restriction, two de Casteljau
    /// splits at the exact rational ratios over the stored numbers,
    /// computed in big-integer fractions. Red under either ratio taken
    /// as a rounded point instead of an outward interval.
    #[test]
    fn a_split_restriction_holds_the_exact_rational_restriction() {
        use num_bigint::BigInt;
        /// An exact rational `n/d`, `d > 0`.
        #[derive(Clone)]
        struct Q(BigInt, BigInt);
        impl Q {
            fn of(x: f64) -> Self {
                let bits = x.to_bits();
                let exp = ((bits >> 52) & 0x7ff) as i32;
                let frac = BigInt::from(bits & ((1 << 52) - 1));
                let (m, e) = if exp == 0 {
                    (frac, -1074)
                } else {
                    (frac + (BigInt::from(1) << 52), exp - 1075)
                };
                let m = if x < 0.0 { -m } else { m };
                if e >= 0 {
                    Q(m << e as usize, BigInt::from(1))
                } else {
                    Q(m, BigInt::from(1) << (-e) as usize)
                }
            }
            fn add(&self, o: &Q) -> Q {
                Q(&self.0 * &o.1 + &o.0 * &self.1, &self.1 * &o.1)
            }
            fn sub(&self, o: &Q) -> Q {
                Q(&self.0 * &o.1 - &o.0 * &self.1, &self.1 * &o.1)
            }
            fn mul(&self, o: &Q) -> Q {
                Q(&self.0 * &o.0, &self.1 * &o.1)
            }
            fn div(&self, o: &Q) -> Q {
                let (n, d) = (&self.0 * &o.1, &self.1 * &o.0);
                if d.sign() == num_bigint::Sign::Minus {
                    Q(-n, -d)
                } else {
                    Q(n, d)
                }
            }
            /// `self ≤ o`.
            fn le(&self, o: &Q) -> bool {
                &self.0 * &o.1 <= &o.0 * &self.1
            }
        }
        // The exact split: `right` keeps `[t, 1]`, else `[0, t]`.
        fn split(row: &[Q], t: &Q, right: bool) -> Vec<Q> {
            let one_t = Q::of(1.0).sub(t);
            let mut levels = vec![row.to_vec()];
            for r in 1..row.len() {
                let prev = &levels[r - 1];
                let next = (0..prev.len() - 1)
                    .map(|i| one_t.mul(&prev[i]).add(&t.mul(&prev[i + 1])))
                    .collect();
                levels.push(next);
            }
            let p = row.len() - 1;
            (0..=p)
                .map(|k| {
                    if right {
                        levels[p - k][k].clone()
                    } else {
                        levels[k][0].clone()
                    }
                })
                .collect()
        }
        let mut rng = test_utils::fuzz::start("split-restrict-rational");
        for _ in 0..test_utils::fuzz::scaled(60) {
            let p = 1 + rng.below(6);
            let row: Vec<f64> = (0..=p).map(|_| rng.range(-3.0, 3.0)).collect();
            let s0 = rng.range(-2.0, 1.0);
            let s1 = s0 + rng.range(0.1, 3.0);
            let (x, y) = (rng.range(s0, s1), rng.range(s0, s1));
            let (a, b) = (x.min(y), x.max(y));
            let got = split_restrict(
                &row.iter().map(|&c| Interval::point(c)).collect::<Vec<_>>(),
                (s0, s1),
                (a, b),
            );
            let (qs0, qs1, qa, qb) = (Q::of(s0), Q::of(s1), Q::of(a), Q::of(b));
            let ta = qa.sub(&qs0).div(&qs1.sub(&qs0));
            let tb = qb.sub(&qa).div(&qs1.sub(&qa));
            let exact = split(
                &split(
                    &row.iter().map(|&c| Q::of(c)).collect::<Vec<_>>(),
                    &ta,
                    true,
                ),
                &tb,
                false,
            );
            for (k, (g, e)) in got.iter().zip(&exact).enumerate() {
                assert!(
                    g.is_certified() && Q::of(g.lo()).le(e) && e.le(&Q::of(g.hi())),
                    "degree {p} on [{s0}, {s1}] cut to [{a}, {b}], coefficient {k}: {g:?} \
                     misses the exact value; {}",
                    test_utils::fuzz::replay()
                );
            }
        }
    }

    /// **Equal neighbours step to themselves exactly.** A constant row
    /// of a drawn non-dyadic value, cut at drawn non-dyadic ends of a
    /// non-dyadic segment, comes back the same value bit for bit, where
    /// the plain thin step `x·(1 − t) + y·t` would widen it by the
    /// ratio's own width at every level. Red with the `x == y` arm of
    /// `step` removed.
    #[test]
    fn a_constant_row_is_cut_to_itself_exactly() {
        let mut rng = test_utils::fuzz::start("split-restrict-constant");
        for _ in 0..test_utils::fuzz::scaled(200) {
            let p = 1 + rng.below(8);
            let c = Interval::point(rng.range(-3.0, 3.0));
            let s0 = rng.range(-2.0, 1.0);
            let s1 = s0 + rng.range(0.1, 3.0);
            let (x, y) = (rng.range(s0, s1), rng.range(s0, s1));
            let (a, b) = (x.min(y), x.max(y));
            for got in split_restrict(&vec![c; p + 1], (s0, s1), (a, b)) {
                assert!(
                    got.lo().to_bits() == c.lo().to_bits()
                        && got.hi().to_bits() == c.hi().to_bits(),
                    "degree {p} on [{s0}, {s1}] cut to [{a}, {b}]: {got:?} is not {c:?}"
                );
            }
        }
    }

    /// **The thin step's products are the general products to the bit.**
    /// `step` reads `x·(1 − t) + y·t` through `Interval::mul_nonneg`;
    /// against `x * s + y * t` over drawn coefficients of every sign
    /// (below, above and straddling zero, thin and wide) and weights from
    /// interior ratios, ratios at an end of `[0, 1]`, wide ratios, tiny
    /// ratios (whose `1 − t` reaches above one) and point ratios (the
    /// general product's fallback), every result has the same ends
    /// and decoration. Red on a product that pads or picks a corner
    /// otherwise.
    #[test]
    fn the_thin_step_reads_the_general_products_to_the_bit() {
        use interval_transcendentals::Decoration;
        let bits = |x: Interval| -> (u64, u64, Decoration) {
            (x.lo().to_bits(), x.hi().to_bits(), x.decoration())
        };
        let mut rng = test_utils::fuzz::start("thin-step-nonneg");
        let coefficient = |rng: &mut test_utils::fuzz::Rng| {
            let c = rng.range(-3.0, 3.0);
            let w = match rng.below(3) {
                0 => 0.0,
                1 => rng.range(0.0, 1e-9),
                _ => rng.range(0.0, 2.0),
            };
            Interval::from_bounds(c, c + w)
        };
        let mut above = 0usize;
        for _ in 0..test_utils::fuzz::scaled(20_000) {
            let (x, y) = (coefficient(&mut rng), coefficient(&mut rng));
            let ratio = match rng.below(6) {
                0 => Interval::point(rng.range(0.0, 1.0)),
                1 => Interval::point(if rng.below(2) == 0 { 0.0 } else { 1.0 }),
                // A wide ratio: end weights far apart.
                2 => {
                    let (r0, r1) = (rng.range(0.0, 1.0), rng.range(0.0, 1.0));
                    Interval::from_bounds(r0.min(r1), r0.max(r1))
                }
                // A tiny positive `t.lo`, at most `2⁻⁵⁴`: `1 − t` reads
                // `1 + 2⁻⁵²` above.
                3 => {
                    let tiny = 2f64.powi(-54);
                    let lo = rng.range(0.0, tiny).max(f64::from_bits(1));
                    Interval::from_bounds(lo, lo + rng.range(0.0, tiny))
                }
                _ => {
                    let (s0, s1) = (rng.range(-2.0, 1.0), rng.range(1.0, 3.0));
                    let a = rng.range(s0, s1);
                    let lo = Interval::point(s0);
                    ((Interval::point(a) - lo) / (Interval::point(s1) - lo)).clamped_to(0.0, 1.0)
                }
            };
            let (t, s) = convex(ratio);
            above += usize::from(s.hi() > 1.0);
            let general = x * s + y * t;
            let got = step(x, y, (t, s));
            if x.same_enclosure(y) {
                continue;
            }
            assert!(
                bits(got) == bits(general),
                "x {x:?}, y {y:?}, t {t:?}: {got:?} against {general:?}"
            );
        }
        assert!(above > 0, "no draw read `1 − t` above one");
    }

    /// **A refused neighbour is not laundered by the equal arm.** A
    /// certified `[0, 2]` beside `sqrt([-1, 4])` — the same ends, refused
    /// (the domain clamp records `Trv`) — cut strictly inside, where
    /// every coefficient of the cut reads both, comes back refused in
    /// either order, as [`Certification::between`] refuses
    /// it; the pair of certified `[0, 2]`s stays certified. Red with the
    /// equal arm comparing ends alone: the row `[certified, refused]`
    /// came back certified.
    #[test]
    fn a_refused_neighbour_with_the_same_ends_stays_refused() {
        let ok = Interval::from_bounds(0.0, 2.0);
        let refused = Interval::from_bounds(-1.0, 4.0).sqrt();
        assert!(ok.is_certified() && !refused.is_certified());
        assert!(
            ok.lo().to_bits() == refused.lo().to_bits()
                && ok.hi().to_bits() == refused.hi().to_bits(),
            "the probe needs the same ends: {ok:?} {refused:?}"
        );
        for cut in [(0.25, 0.5), (0.5, 0.75), (0.125, 0.875), (0.375, 0.375)] {
            for row in [[ok, refused], [refused, ok]] {
                let got = split_restrict(&row, (0.0, 1.0), cut);
                assert!(
                    got.iter().all(|c| !c.is_certified()),
                    "{row:?} cut to {cut:?} came back {got:?}"
                );
            }
            let got = split_restrict(&[ok, ok], (0.0, 1.0), cut);
            assert!(got.iter().all(|c| c.is_certified()), "{got:?}");
        }
    }

    /// **A weaker certified neighbour is not upgraded by the equal arm.**
    /// A `Com` `[2, 3]` beside `floor([2.5, 3.5])` — the same ends, `Def`
    /// (the hull across a step) — cut strictly inside comes back at `Def`
    /// or below in either order, the level the general step carries; two
    /// `Def` ones stay `Def`, and two `Com` ones `Com`. Red with the equal
    /// arm comparing ends and `is_certified()` alone: the row
    /// `[Com, Def]` came back `Com`.
    #[test]
    fn a_weaker_neighbour_with_the_same_ends_is_not_upgraded() {
        use crate::real::Real;
        use interval_transcendentals::Decoration;
        let com = Interval::from_bounds(2.0, 3.0);
        let def = Interval::from_bounds(2.5, 3.5).floor();
        assert!(com.decoration() == Decoration::Com && def.decoration() == Decoration::Def);
        assert!(
            com.lo().to_bits() == def.lo().to_bits() && com.hi().to_bits() == def.hi().to_bits(),
            "the probe needs the same ends: {com:?} {def:?}"
        );
        for cut in [(0.25, 0.5), (0.5, 0.75), (0.125, 0.875), (0.375, 0.375)] {
            for row in [[com, def], [def, com]] {
                let got = split_restrict(&row, (0.0, 1.0), cut);
                assert!(
                    got.iter().all(|c| c.decoration() <= Decoration::Def),
                    "{row:?} cut to {cut:?} came back {got:?}"
                );
            }
            for (x, level) in [(com, Decoration::Com), (def, Decoration::Def)] {
                let got = split_restrict(&[x, x], (0.0, 1.0), cut);
                assert!(got.iter().all(|c| c.decoration() == level), "{got:?}");
            }
        }
    }

    /// **A segment cut is the Bézier span's restriction, to the bit.**
    /// `restrict_segment` forms each cut end's ratio once rather than at
    /// every entry of every level; on `KnotVector::bezier(p, (s0, s1))`
    /// every ratio is that one expression, so the rows agree bit for bit
    /// (and in certification) with [`SpanRow::restrict`] there: drawn
    /// widened rows of degree 1–8, non-dyadic segments at several
    /// magnitudes, cuts anywhere inside them. Red under the ratio formed
    /// any other way (`(u − s0)·(1/(s1 − s0))`).
    #[test]
    fn a_segment_cut_is_the_bezier_spans_restriction_to_the_bit() {
        let mut rng = test_utils::fuzz::start("restrict-segment-hoisted-ratio");
        for _ in 0..test_utils::fuzz::scaled(400) {
            let p = 1 + rng.below(8);
            let row: Vec<Interval> = (0..=p)
                .map(|_| {
                    let c = rng.range(-3.0, 3.0);
                    let w = rng.range(0.0, 1e-9);
                    Interval::from_bounds(c - w, c + w)
                })
                .collect();
            let scale = [1e-3, 1.0, 37.0, 1e4][rng.below(4)];
            let s0 = rng.range(-1.0, 1.0) * scale;
            let s1 = s0 + rng.range(0.01, 2.0) * scale;
            let (x, y) = (rng.range(s0, s1), rng.range(s0, s1));
            let (a, b) = (x.min(y), x.max(y));
            let got = restrict_segment(&row, (s0, s1), (a, b));
            let kv = KnotVector::bezier(p, (s0, s1)).unwrap();
            let want = kv
                .span_at(s0)
                .unwrap()
                .with_window(&row)
                .unwrap()
                .restrict((Interval::point(a), Interval::point(b)))
                .unwrap();
            for (g, w) in got.iter().zip(&want) {
                assert!(
                    g.lo().to_bits() == w.lo().to_bits()
                        && g.hi().to_bits() == w.hi().to_bits()
                        && g.is_certified() == w.is_certified(),
                    "degree {p} on [{s0}, {s1}] cut to [{a}, {b}]: {g:?} against {w:?}; {}",
                    test_utils::fuzz::replay()
                );
            }
        }
    }

    /// **A cut out of order refuses its row** (carried from `compose`'s
    /// segment-cut row when the segment cut moved onto the door).
    /// Outside the segment the cut is no restriction of it; a reversed
    /// cut inside it is refused by contract. Red under the guard
    /// removed: the reversed cut comes back certified.
    #[test]
    fn a_segment_cut_out_of_order_refuses_its_row() {
        let row: Vec<Interval> = [1.0, -2.0, 3.0].map(Interval::point).to_vec();
        for (seg, cut) in [
            ((0.0, 1.0), (0.5, 0.25)),
            ((0.0, 1.0), (-0.5, 0.5)),
            ((0.0, 1.0), (0.5, 1.5)),
            ((1.0, 1.0), (1.0, 1.0)),
            ((0.0, 1.0), (f64::NAN, 0.5)),
        ] {
            let got = restrict_segment(&row, seg, cut);
            assert_eq!(got.len(), row.len(), "{seg:?} {cut:?}");
            assert!(
                got.iter().all(|c| !c.is_certified()),
                "{seg:?} cut at {cut:?} came back certified: {got:?}"
            );
        }
        assert!(
            restrict_segment(&[], (0.0, 1.0), (0.25, 0.5)).is_empty(),
            "an empty row cut inside its segment comes back empty"
        );
        let point = restrict_segment(&row, (0.0, 1.0), (0.25, 0.25));
        assert!(
            point.iter().all(|c| c.is_certified()),
            "a point cut inside the segment reads: {point:?}"
        );
        let whole = restrict_segment(&row, (2.0, 6.0), (2.0, 6.0));
        assert!(
            whole
                .iter()
                .zip(&row)
                .all(|(x, y)| x.lo().to_bits() == y.lo().to_bits()
                    && x.hi().to_bits() == y.hi().to_bits()),
            "the whole segment comes back as it came: {whole:?}"
        );
        // The segment's restriction is the [0, 1] row's at the mapped
        // ratios, which are exact here.
        let on_segment = restrict_segment(&row, (2.0, 6.0), (3.0, 5.0));
        let on_unit = restrict_segment(&row, (0.0, 1.0), (0.25, 0.75));
        for (x, y) in on_segment.iter().zip(&on_unit) {
            assert!(x.lo() <= y.hi() && y.lo() <= x.hi(), "{x:?} vs {y:?}");
        }
    }
}
