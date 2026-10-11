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
//! ([`BlossomError::OutsideSpan`]).
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
//! - **A bare Bernstein row** on `[0, 1]` ([`restrict_bernstein`]) is a
//!   proof about [`KnotVector::bezier`]`(p, (0, 1))`, whose ratios are
//!   the arguments themselves, so no vector is minted for it.
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
use crate::real::CertifiedBounds;

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

/// The Bernstein row on `[a, b] ⊆ [0, 1]` of the polynomial whose
/// Bernstein row on `[0, 1]` is `row`: [`CoeffWindow::restrict`] on
/// [`KnotVector::bezier`]`(p, (0, 1))`, whose every ratio is the argument
/// itself. A constant (one coefficient) is its own restriction.
///
/// # Errors
///
/// [`BlossomError::EmptyRow`] for an empty row;
/// [`BlossomError::OutsideSpan`] for an end with no point in `[0, 1]`.
///
/// [`KnotVector::bezier`]: super::KnotVector::bezier
pub fn restrict_bernstein(
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
    if row.len() == 1 || (a == s0 && b == s1) {
        return row.to_vec();
    }
    super::KnotVector::bezier(row.len() - 1, (s0, s1))
        .ok()
        .and_then(|kv| {
            kv.span_at(s0)?
                .with_window(row)?
                .restrict((Interval::point(a), Interval::point(b)))
                .ok()
        })
        .unwrap_or_else(|| vec![Interval::refused(); row.len()])
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
fn span_ratio(span: Span<'_>) -> ((f64, f64), impl Fn(usize, usize, Interval) -> Interval) {
    let (p, s, t) = (span.degree(), span.index(), span.knots().knots());
    // Indexing justified: `p ≤ s ≤ last_span() = len − p − 2`, and
    // `i + p + 1 − r ≤ s + p` for `j ≤ p`, `r ≥ 1`.
    let ratio = move |j: usize, r: usize, u: Interval| {
        let i = s - p + j;
        let below = Interval::point(t[i]);
        (u - below) / (Interval::point(t[i + p + 1 - r]) - below)
    };
    ((t[s], t[s + 1]), ratio)
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
    Ok(pyramid(row, (a, b), ratio))
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
            let bez = KnotVector::bezier(p, (0.0, 1.0)).unwrap();
            let bern = &coeffs[..=p];
            let ends = (
                Interval::point(1.0 / 3.0),
                Interval::from_bounds(0.6, 0.6000001),
            );
            let via_pair = bez
                .with_coeffs(bern)
                .unwrap()
                .span(p)
                .unwrap()
                .restrict(ends)
                .unwrap();
            assert_eq!(
                bits(&restrict_bernstein(bern, ends).unwrap()),
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
        let f = <f64 as SpanLocate>::restrict_window(&line, span, (0.0, 1.0));
        assert!(f.iter().all(|c| c.is_nan()), "f64 extrapolated: {f:?}");
        let ok = <f64 as SpanLocate>::restrict_window(&line, span, (0.0, 0.5));
        assert!(ok.iter().all(|c| c.is_finite()), "f64 in-span: {ok:?}");
        let lifted: Vec<Interval> = line.iter().map(|&c| Interval::point(c)).collect();
        let i = <Interval as SpanLocate>::restrict_window(&lifted, span, (0.0, 1.0));
        assert!(i.iter().all(|c| !c.is_certified()), "Interval: {i:?}");
        let short = <f64 as SpanLocate>::restrict_window(&line[..2], span, (0.0, 0.5));
        assert!(
            short.len() == 3 && short.iter().all(|c| c.is_nan()),
            "{short:?}"
        );
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
        let on_unit =
            restrict_bernstein(&row, (Interval::point(0.25), Interval::point(0.75))).unwrap();
        for (x, y) in on_segment.iter().zip(&on_unit) {
            assert!(x.lo() <= y.hi() && y.lo() <= x.hi(), "{x:?} vs {y:?}");
        }
    }
}
