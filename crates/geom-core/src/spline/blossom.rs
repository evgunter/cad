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
//! Everything else is this door at chosen arguments:
//!
//! - **Evaluation** at `t` is the blossom at `(t, …, t)`.
//! - **Restriction** to `[a, b]` ([`CoeffWindow::restrict`]) is the
//!   Bernstein row whose coefficient `k` is `f(a^{p−k}, b^k)`; at the
//!   span's own ends it is the span's Bézier segment.
//! - **A bare Bernstein row** on `[s0, s1]` is a proof about
//!   [`KnotVector::bezier`]`(p, (s0, s1))`, minted like any other pair.
//!
//! So no coefficient is more than `p` steps deep from the window it was
//! read from, wherever the arguments fall and however many pieces a
//! span is cut into.
//!
//! [`KnotVector::bezier`]: super::KnotVector::bezier

use super::hull::CoeffWindow;
use crate::interval::Interval;
use crate::interval::certification::Certification;
use crate::readable::Readable;
use crate::real::CertifiedBounds;

/// A typed refusal of [`CoeffWindow::blossom`] (fail-loud; the kernel
/// never panics).
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
    /// in its span by contract.
    OutsideSpan {
        /// Which argument.
        index: usize,
        /// The span's lower knot.
        lo: f64,
        /// The span's upper knot.
        hi: f64,
    },
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
        }
    }
}

impl core::error::Error for BlossomError {}

impl<E: CertifiedBounds> CoeffWindow<'_, E> {
    /// The piece's blossom at `args`, enclosed: an enclosure of
    /// `f(u_1, …, u_p)` for every `u_r ∈ args[r − 1]`.
    ///
    /// Each argument is met with the span's closure first: the piece's
    /// arguments lie in its span by contract, so the meet is sound, and
    /// it is what keeps every level's ratio in `[0, 1]` (module docs).
    /// A refused argument or coefficient flows through as NaI.
    ///
    /// # Errors
    ///
    /// [`BlossomError::ArgumentCount`] unless there are exactly `p`
    /// arguments; [`BlossomError::OutsideSpan`] for a certified argument
    /// with no point in the span.
    pub fn blossom(self, args: &[Interval]) -> Result<Interval, BlossomError> {
        let span = self.span();
        let (p, s) = (span.degree(), span.index());
        if args.len() != p {
            return Err(BlossomError::ArgumentCount {
                degree: p,
                got: args.len(),
            });
        }
        let t = span.knots().knots();
        // Indexing justified: `p ≤ s ≤ last_span() = len − p − 2`.
        let (lo, hi) = (t[s], t[s + 1]);
        let mut d: Vec<Interval> = self
            .coeffs()
            .iter()
            .map(|&c| Interval::from_certified(c))
            .collect();
        for (level, &u) in args.iter().enumerate() {
            let u = if u.is_certified() {
                let inside = u.clamped_to(lo, hi);
                if !inside.is_certified() {
                    return Err(BlossomError::OutsideSpan {
                        index: level,
                        lo,
                        hi,
                    });
                }
                inside
            } else {
                u
            };
            let r = level + 1;
            for j in (r..=p).rev() {
                // `i = s − p + j ≥ s − p` and `i + p + 1 − r ≤ s + p`,
                // inside the vector's `len = control_count + p + 1`.
                let i = s - p + j;
                let below = Interval::point(t[i]);
                let ratio = (u - below) / (Interval::point(t[i + p + 1 - r]) - below);
                d[j] = Interval::between(d[j - 1], d[j], ratio);
            }
        }
        Ok(d[p])
    }

    /// The Bernstein row of the piece restricted to `[a, b]`:
    /// coefficient `k` is the blossom at `a` taken `p − k` times and `b`
    /// taken `k` times (module docs), so each is at most `p` steps deep.
    /// At the span's own ends it is the span's Bézier segment.
    ///
    /// # Errors
    ///
    /// As [`CoeffWindow::blossom`]: an end wholly outside the span.
    pub fn restrict(self, (a, b): (Interval, Interval)) -> Result<Vec<Interval>, BlossomError> {
        let p = self.span().degree();
        let mut args = vec![a; p];
        (0..=p)
            .map(|k| {
                if k > 0 {
                    args[p - k] = b;
                }
                self.blossom(&args)
            })
            .collect()
    }
}

/// [`super::SpanLocate::restrict_window`] at the certification scalar:
/// `window` minted as a coefficient array of `span`'s vector (refused
/// outside the window, which the span never reads) and restricted
/// through the window. A refused row is `p + 1` refusals.
pub(crate) fn restrict_window(
    window: &[Interval],
    span: super::Span<'_>,
    (a, b): (f64, f64),
) -> Vec<Interval> {
    let p = span.degree();
    let first = span.first_control();
    if window.len() != p + 1 {
        return vec![Interval::refused(); p + 1];
    }
    span.knots()
        .with_coeffs_from_fn(
            |i| {
                i.checked_sub(first)
                    .and_then(|k| window.get(k))
                    .copied()
                    .unwrap_or_else(Interval::refused)
            },
            |pair| {
                pair.span(span.index())?
                    .restrict((Interval::point(a), Interval::point(b)))
                    .ok()
            },
        )
        .unwrap_or_else(|| vec![Interval::refused(); p + 1])
}
