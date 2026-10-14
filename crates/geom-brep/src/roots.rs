//! **The one shape of a root finder's answer.** Every finder that
//! reports the certified real roots of a residual along a carrier — the
//! half-angle ladder's subdivision ([`crate::half_angle`]), the depressed
//! quartic ([`crate::quartic`]), and the circle × surface doors above this
//! crate — answers `Result<Roots<T, N, C>, _>`: the value holds decided
//! answers only, and every way of not deciding is a typed refusal in the
//! `Err` ([`Unsettled`], or [`RootRefusal`] where the finder's own band
//! decisions can escalate). A refusal is never read as a verdict (D4): a
//! caller takes it down a path that decides the question itself, or
//! refuses in turn.
//!
//! `N` is the most roots the finder can certify, and `C` is what its
//! [`Roots::On`] arm carries: [`OnCarrier`] for a finder whose residual
//! can be a zero constant (a circle on a sphere, wall or torus), and the
//! uninhabited [`Infallible`] for one whose residual cannot, so that arm
//! needs no match and cannot be built. [`Roots::widen`] is the one bridge
//! between them.

use core::convert::Infallible;

use geom_core::Indeterminate;

/// A root finder's decided answer about a residual along a carrier
/// (module docs).
#[derive(Debug, Clone, Copy)]
pub enum Roots<T, const N: usize, C = Infallible> {
    /// A certified count of zero: the carrier misses the surface.
    Miss,
    /// The residual is a zero constant along the carrier: it lies on the
    /// surface.
    On(C),
    /// A certified count (at most `N`) and the roots, unordered, in
    /// `roots[..count]`.
    Certified {
        /// How many of `roots` are roots.
        count: usize,
        /// The roots, `roots[..count]`.
        roots: [T; N],
    },
}

/// [`Roots::On`]'s payload where a finder's residual can be a zero
/// constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnCarrier;

impl From<Infallible> for OnCarrier {
    fn from(never: Infallible) -> Self {
        match never {}
    }
}

impl<T: geom_core::Real, const M: usize, C: Into<OnCarrier>> Roots<T, M, C> {
    /// The same answer in a finder's shape with room for `N ≥ M` roots
    /// and an [`OnCarrier`] arm.
    #[must_use]
    pub fn widen<const N: usize>(self) -> Roots<T, N, OnCarrier> {
        const { assert!(M <= N, "a root answer widens into no fewer roots") };
        match self {
            Self::Miss => Roots::Miss,
            Self::On(on) => Roots::On(on.into()),
            Self::Certified { count, roots } => {
                let mut wide = [T::zero(); N];
                wide[..M].copy_from_slice(&roots);
                Roots::Certified { count, roots: wide }
            }
        }
    }
}

/// Why a root finder gave no decided answer: the finder could not
/// certify a count at this band. Never a miss and never a count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unsettled {
    /// A double root: a piece the band cannot cut further that is neither
    /// clear nor monotone, an extreme of the residual in the zero band, or
    /// a repeated root of the quartic's sign ladder.
    Tangency,
    /// A sign the finder must read definite at a point it chose (a cut, a
    /// split, a located root read ON) did not read so.
    SignInBand,
    /// A meter refused: the residual's rounding, or a located root's
    /// slack, is not definitely inside the band.
    Meter,
    /// The walk ran past [`geom_core::bisect::PIECE_BUDGET`].
    Exhausted,
    /// The roots located are a count the residual cannot have (odd round
    /// a closed turn, or more than its degree allows).
    ImpossibleCount,
    /// The quartic's resolvent has no positive largest root, which its own
    /// constant term contradicts.
    Resolvent,
    /// The quartic's constructed roots disagree in number with the count
    /// its sign ladder certified.
    CountDisagrees {
        /// The count the sign ladder certified.
        certified: usize,
        /// The count the factorization built.
        constructed: usize,
    },
    /// A root within the band of a cone's apex, where the surface has no
    /// tangent plane.
    AtApex,
    /// The residual's form degenerates where the band reads its leading
    /// term zero (a line along a cone's generator).
    Degenerate,
}

/// Why a finder whose own band decisions can escalate gave no decided
/// answer.
#[derive(Debug, Clone, Copy)]
pub enum RootRefusal {
    /// A classifying sign the finder needed fell in the band's
    /// escalation gap: the caller's decision escalates.
    Escalated(Indeterminate),
    /// The finder could not certify a count ([`Unsettled`]).
    Unsettled(Unsettled),
}

impl From<Unsettled> for RootRefusal {
    fn from(why: Unsettled) -> Self {
        Self::Unsettled(why)
    }
}

impl From<Indeterminate> for RootRefusal {
    fn from(diag: Indeterminate) -> Self {
        Self::Escalated(diag)
    }
}
