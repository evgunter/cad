//! **The one shape of a root finder's answer.** Every finder that
//! reports the certified real roots of a residual along a carrier — the
//! half-angle ladder's subdivision ([`crate::half_angle`]), the depressed
//! quartic ([`crate::quartic`]), and the circle × surface doors above this
//! crate — answers `Result<Roots<T, N, C>, _>`: the value holds decided
//! answers only, and every way of not deciding is a typed refusal in the
//! `Err` ([`RootRefusal`]: a sign in band, a limit
//! ([`Unsettled`]) or a fault ([`RootFault`])). A refusal is never read as a verdict (D4): a
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

use geom_core::{Band, Decide, Indeterminate, Margin, Real, Sign};

/// A root finder's decided answer about a residual along a carrier
/// (module docs).
#[derive(Debug, Clone, Copy)]
pub enum Roots<T, const N: usize, C = Infallible> {
    /// A certified count of zero: the carrier misses the surface.
    Miss,
    /// The residual is a zero constant along the carrier: it lies on the
    /// surface.
    On(C),
    /// A certified count of one to `N` roots.
    Certified(Found<T, N>),
}

/// One to `N` certified roots, unordered. Built only by [`Roots::of`], so
/// a count past `N` (or of none, which is [`Roots::Miss`]) cannot be
/// held, and [`Found::roots`] is the one way to read them.
#[derive(Debug, Clone, Copy)]
pub struct Found<T, const N: usize> {
    count: usize,
    roots: [T; N],
}

impl<T, const N: usize> Found<T, N> {
    /// The roots, unordered.
    pub fn roots(&self) -> &[T] {
        &self.roots[..self.count]
    }

    /// How many roots there are, one to `N`.
    pub fn count(&self) -> usize {
        self.count
    }

    /// The same count of roots, each moved by `f`.
    #[must_use]
    pub fn map(self, f: impl Fn(T) -> T) -> Self {
        Self {
            count: self.count,
            roots: self.roots.map(f),
        }
    }
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

impl<T: Real, const N: usize, C> Roots<T, N, C> {
    /// `roots` as a decided answer: [`Roots::Miss`] for none,
    /// [`Roots::Certified`] for one to `N`, and `None` for more than `N`,
    /// a count the finder cannot hold.
    #[must_use]
    pub fn of(roots: &[T]) -> Option<Self> {
        if roots.is_empty() {
            return Some(Self::Miss);
        }
        let count = roots.len();
        if count > N {
            return None;
        }
        let mut held = [T::zero(); N];
        held[..count].copy_from_slice(roots);
        Some(Self::Certified(Found { count, roots: held }))
    }
}

impl<T, const N: usize, C> Roots<T, N, C> {
    /// Exactly `N ≥ 1` certified roots.
    #[must_use]
    pub fn all(roots: [T; N]) -> Self {
        const { assert!(N > 0, "a certified count is at least one") };
        Self::Certified(Found { count: N, roots })
    }
}

impl<T: Real, const M: usize, C: Into<OnCarrier>> Roots<T, M, C> {
    /// The same answer in a finder's shape with room for `N ≥ M` roots
    /// and an [`OnCarrier`] arm.
    #[must_use]
    pub fn widen<const N: usize>(self) -> Roots<T, N, OnCarrier> {
        const { assert!(M <= N, "a root answer widens into no fewer roots") };
        match self {
            Self::Miss => Roots::Miss,
            Self::On(on) => Roots::On(on.into()),
            Self::Certified(found) => {
                let mut wide = [T::zero(); N];
                wide[..M].copy_from_slice(&found.roots);
                Roots::Certified(Found {
                    count: found.count,
                    roots: wide,
                })
            }
        }
    }
}

/// Why a root finder gave no decided answer about the geometry: a limit
/// of the finder at this precision, a definite reading it has no arm
/// for, or a pose it has no reading for. Never a miss, never a count,
/// and never a reading in the band's gap, which is
/// [`RootRefusal::Touch`]: this is the typed frontier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unsettled {
    /// A touch decided in the zero band: a double root, an extreme of
    /// the residual or a stretch of it read zero, a straddle of a value
    /// and its error, or two roots the band reads at one point. Definite,
    /// and not a crossing at any order the finder reads.
    Tangency,
    /// A meter refused: the residual's rounding, or a located root's
    /// slack, is not definitely inside the band.
    Meter,
    /// The walk ran past [`geom_core::bisect::PIECE_BUDGET`].
    Exhausted,
    /// A piece at the walk's floor whose enclosure the band cannot decide:
    /// an enclosure wider than the band, not a reading in it.
    Floor,
    /// The quartic's resolvent has no positive largest root, which its own
    /// constant term contradicts but for rounding; a legitimately small
    /// root reaches it too.
    Resolvent,
    /// The quartic's constructed roots disagree in number with the count
    /// its sign ladder certified: the ladder's discriminant carries
    /// rounding its band does not charge, which a legal far ray reaches.
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
    /// A meeting within the band of a crease of a spline carrier, where
    /// it has no tangent to read a crossing by.
    Crease,
}

/// Two of a root finder's own readings contradict where no legal input
/// takes them apart: a charge the finder states is exceeded, a kernel
/// fault. Only [`crate::half_angle::certified_subdivision`] raises one;
/// a count past a finder's capacity is ruled out by construction, at
/// each site, and is no fault but an `unreachable!`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootFault {
    /// The subdivision located a count the residual cannot have: odd
    /// round a closed turn, or more than `2N` for a residual of degree
    /// `N`. A sign change on a piece read clear, the residual and its
    /// harmonics disagreeing by more than the noise the walk charges.
    ImpossibleCount,
}

impl RootFault {
    /// The two readings that contradict, in words.
    #[must_use]
    pub fn readings(self) -> &'static str {
        match self {
            Self::ImpossibleCount => {
                "a root finder located a count of roots its residual cannot have"
            }
        }
    }
}

/// Why a root finder gave no decided answer.
#[derive(Debug, Clone, Copy)]
pub enum RootRefusal {
    /// A classifying sign the finder needed fell in the band's escalation
    /// gap: the caller's decision escalates.
    Escalated(Indeterminate),
    /// The residual reads in the band's gap where a root can lie: at
    /// every cut the walk can place there, over a piece the band cannot
    /// cut further, or at a located root. A reading in band of the
    /// geometry, which escalates where the carrier is the model's, and is
    /// the probe's where the caller chose the carrier (a ray or a path it
    /// can cast again).
    Touch(Indeterminate),
    /// The finder reached a limit ([`Unsettled`]).
    Unsettled(Unsettled),
    /// The finder's readings contradict ([`RootFault`]).
    Fault(RootFault),
}

impl From<Unsettled> for RootRefusal {
    fn from(why: Unsettled) -> Self {
        Self::Unsettled(why)
    }
}

/// **The refusal of a walk that cut nowhere definite**: a touch on the
/// first reading it met in the band's gap (`gap`), told as the caller's
/// `touch`, or [`Unsettled::Tangency`] where every reading it tried
/// decided zero. The one spelling of that rule for the walks
/// ([`crate::half_angle::certified_subdivision`],
/// [`crate::curve_face::composite`]).
pub fn touch_or_tangency<R: From<Unsettled>>(
    gap: Option<Indeterminate>,
    touch: impl FnOnce(Indeterminate) -> R,
) -> R {
    gap.map_or_else(|| Unsettled::Tangency.into(), touch)
}

impl From<RootFault> for RootRefusal {
    fn from(fault: RootFault) -> Self {
        Self::Fault(fault)
    }
}

impl From<Indeterminate> for RootRefusal {
    fn from(diag: Indeterminate) -> Self {
        Self::Escalated(diag)
    }
}

/// The signs of both ends of `[value − spread, value + spread]`, each
/// decided under `row`: the one reading of a value known only to within
/// `spread`.
///
/// # Errors
///
/// The band's escalation when either end lies in its gap.
pub fn spread_signs<T: Decide>(
    row: &'static str,
    value: T,
    spread: T,
    band: Band,
) -> Result<(Sign, Sign), Indeterminate> {
    let (below, above) = spread_readings(row, value, spread, band)?;
    Ok((below.sign, above.sign))
}

/// [`spread_signs`], keeping each end's reading, for a caller that names
/// a straddle of the two ([`geom_core::k_stats::straddle`]).
///
/// # Errors
///
/// As [`spread_signs`].
pub fn spread_readings<T: Decide>(
    row: &'static str,
    value: T,
    spread: T,
    band: Band,
) -> Result<(geom_core::Decided, geom_core::Decided), Indeterminate> {
    let below = geom_core::k_stats::decide_reported(row, Margin::of(value - spread), band)?;
    let above = geom_core::k_stats::decide_reported(row, Margin::of(value + spread), band)?;
    Ok((below, above))
}

/// The sign of a residual's extreme read as `value` with error at most
/// `noise` ([`spread_signs`]): ends that disagree read `Zero`. `noise`
/// must bound the whole error, rounding included.
///
/// # Errors
///
/// As [`spread_signs`].
pub fn extreme_sign<T: Decide>(
    row: &'static str,
    value: T,
    noise: T,
    band: Band,
) -> Result<Sign, Indeterminate> {
    let (below, above) = spread_signs(row, value, noise, band)?;
    Ok(if below == above { below } else { Sign::Zero })
}

/// The angle from an extreme of a first harmonic `c₀ + A·cos ψ` to its
/// roots, where that extreme has crossed zero by `ratio` of the swing
/// `2A` (`ratio ∈ [0, 1]`): `2·asin(√ratio)`, which reads the extreme to
/// its own relative precision.
pub fn arc_past<T: Real>(ratio: T) -> T {
    T::from_f64(2.0) * ratio.sqrt().asin()
}
