//! **A periodic channel's period, and the one rule a whole turn is
//! decided by.**
//!
//! [`Period`] answers every whole-period question of a channel: which
//! deck element a joint is on ([`Period::decide`]), how far `k` periods
//! shift ([`Period::times`]), and whether an extent along the channel is
//! short of one turn, a whole turn, or over one ([`Period::turn`]).
//!
//! # The turn rule
//!
//! The quantity is the headroom `h = period − extent`, crossed to metres
//! through the rate pair (`[tolerance.margins]` clause (iii); the module
//! docs of [`crate::predicate`], *The rate pair*). It is read for three
//! different questions — "not over", "strictly partial", "a whole turn"
//! — so the rule fixes the reading and leaves the disposition to the
//! caller: [`Turn`] has three values, and each question has its one
//! home ([`Period::not_over`], [`Period::strictly_partial`],
//! [`Period::whole_turn`]) for a site that asks it.
//!
//! The rate is typed. An [`Exact`] rate (a circle's radius, an authored
//! angle on its arm) reads `h` once. A [`Bounded`] rate (an ellipse, a
//! spiric, a chart image) reads it at both ends, each verdict on the end
//! that makes it sound:
//! - [`Turn::Over`] needs `h` at the inf rate definitely negative: a
//!   lower bound on the overlap's length past the band;
//! - [`Turn::Short`] needs `h` at the inf rate definitely positive;
//! - [`Turn::Whole`] needs `h` at the sup rate within the zero band: only
//!   an upper bound certifies that the extent is metrically a whole turn.
//!
//! Any other pair is [`TurnUndecided`]: on the inf reading's own margin
//! where it is in the band, on the caller's straddle row where the two
//! readings are decided and straddle the band
//! ([`crate::k_stats::straddle`]). At an `Exact` rate the two reads are
//! one, so `Whole` is `Zero` and no straddle can arise; that is why an
//! `Exact` site names no straddle row ([`TurnRows`]).
//!
//! # The extent cannot be reversed
//!
//! The extent enters as an [`Extent`], whose constructors each say why
//! it is not negative: [`Extent::forward`] decides `0 < t1 − t0` at a
//! positive rate, [`Extent::hull`] folds a hull's least and greatest
//! itself, [`Extent::decide_signed`] reads a signed net quantity's
//! magnitude after deciding its sign, and a certified edge's comes from
//! its certificate; a sum of extents is one. The field is private, and
//! the rule reads only a headroom it forms itself ([`Headroom`]), so no
//! bare value reaches it:
//!
//! ```compile_fail,E0423
//! let _ = geom_core::period::Extent(1.0_f64);
//! ```
//!
//! ```compile_fail,E0423
//! use geom_core::period::{Exact, Headroom, TurnRate as _, TurnRows};
//! let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
//! let _ = Exact::new(1.0_f64).read(TurnRows::exact("t"), Headroom(7.0), band);
//! ```
//!
//! Their twins, through the doors, compile:
//!
//! ```
//! use geom_core::period::{Exact, Extent, Period, TurnRows};
//! let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
//! let extent = Extent::hull([1.0_f64, 0.0]).unwrap();
//! assert_eq!(extent.get(), 1.0);
//! let _ = Period::new(7.0_f64).turn(TurnRows::exact("t"), extent, Exact::new(1.0), band);
//! ```

use crate::k_stats::{NonzeroSign, decide, decide_nonzero_reported, decide_reported, straddle};
use crate::predicate::{
    Band, Decide, Decided, Indeterminate, InfSpeed, Margin, MarginDiag, Sign, SupSpeed,
};
use crate::real::Real;

/// Why a [`Period`] decided no branch.
#[derive(Clone, Debug)]
pub enum BranchMiss {
    /// A half-period mark is undecided: the gap is not resolved to one
    /// branch at this width (D4 ¶3, escalate-never-guess). A gap that is
    /// not a finite number, or whose branch no `i32` names, decides no
    /// mark either, and reads here with an invalid margin.
    Undecided(Indeterminate),
    /// The gap sits ON a half-period mark (the mark decided Zero): it
    /// is half a period from two branches and is the gap of neither — a
    /// stored image half a period off (`topo::pcurves` reads a sphere's
    /// twin as its own orbit point, with half the period, so a twin is
    /// never this).
    OnMark,
}

/// Where [`Period::decide`] stopped short of a branch: the miss, and the
/// two branches `[j, j + 1]` either side of the mark it stopped on (none
/// for a gap that names no mark at all, or where `j + 1` is past
/// `i32::MAX`).
#[derive(Clone, Debug)]
pub struct WalkStop {
    /// Why no branch was decided.
    pub miss: BranchMiss,
    /// The branches either side of the mark the walk stopped on.
    pub astride: Option<[i32; 2]>,
}

/// **A periodic channel's period**: the length one whole turn of the
/// channel spans, which every whole-period question of that channel is
/// asked of. Today that is τ in azimuth, its half on a sphere joint's
/// orbit (whose twin sits half a period over), and the first-channel
/// period a loop walk decides its joints in. Three questions go through
/// it:
/// - a joint's deck element, decided by the half-period marks
///   ([`Period::decide`]);
/// - a deck element's shift, `k` periods as a length ([`Period::times`]);
/// - whether an extent along the channel is short of one turn, a whole
///   turn, or over one ([`Period::turn`]).
///
/// No whole-period integer is read off a value: [`Period::decide`]
/// decides it through the marks, and every other integer is structure.
/// A chart places its angles in principal windows by deciding turns
/// off them through these marks (`geom::Window`, and `geom_brep`'s
/// placement, which re-exports this type).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Period<T> {
    length: T,
}

impl<T: Real> Period<T> {
    /// The channel's period, one whole turn of it.
    pub fn new(length: T) -> Self {
        Self { length }
    }

    /// The period as a length.
    pub fn length(self) -> T {
        self.length
    }

    /// Half the period, as a length: how far an offset may sit from a
    /// whole number of periods and still be nearer it than the next.
    pub fn half(self) -> T {
        self.length / (T::one() + T::one())
    }

    /// `k` whole periods, as a length: a deck element's shift, or the
    /// advance of a winding of `k`.
    pub fn times(self, k: i32) -> T {
        T::from_f64(f64::from(k)) * self.length
    }

    /// `k` windings of this channel as a rate per unit of a carrier's
    /// parameter: [`Period::times`]`(k)` of this channel's period over
    /// the carrier's `period`. A winding becomes a slope only here, read
    /// through the two periods it is counted in (C4). Exact at `f64`
    /// for the counts a closed-form table holds; at `Interval` it is a
    /// tight enclosure of the ratio.
    pub fn rate(self, k: i32, carrier: Period<T>) -> T {
        self.times(k) / carrier.length()
    }
}

impl<T: Decide> Period<T> {
    /// **A branch, decided as structure**: the whole number of periods
    /// `k` with `|gap − k·length| < length/2`, as the integer it is. It
    /// is read off the half period of room a branch has, by sign
    /// decisions against the half-period marks (`name`, metered through
    /// `meter`), read outward from zero, never by rounding the gap. The
    /// marks either side of zero first; then, on the side the gap lies,
    /// the marks 1, 2, 4, 8, ... periods out until one is not cleared
    /// (decided past, away from zero), and a bisection between it and
    /// the last cleared. A gap that clears a mark clears every mark
    /// nearer zero (the meter grows with the gap's distance from the
    /// mark), so the mark the walk stops on is the first, from zero, that
    /// the gap does not clear; where that mark decides Zero
    /// ([`BranchMiss::OnMark`]) the gap is within the band's zero of it
    /// in the meter's units, and the branches astride it are the gap's
    /// own nearest. Out to two periods the marks asked are the ones a
    /// step-by-step walk asks, in its order.
    ///
    /// No cap: the walk asks O(log k) marks however far out the gap
    /// lies, and stops only on a mark it does not clear. Once images are
    /// placed over their edges' spans (C4's placement, unit B of the
    /// `[ev]` PR 5142 answer), a joint's gap is within about two periods
    /// and the walk is a handful of decisions; until then a carrier
    /// stated far off puts its gap that far out. A gap that is not
    /// finite, or whose branch is past `i32`, decides nothing and stops
    /// as undecided.
    ///
    /// # Errors
    ///
    /// [`WalkStop`]: a mark undecided, or the gap on a mark, each with
    /// the branches astride it.
    pub fn decide(
        self,
        name: &'static str,
        gap: T,
        meter: impl Fn(T) -> Margin<T>,
        band: Band,
    ) -> Result<i32, WalkStop> {
        // A gap that names no branch reads as an unreadable margin, and
        // the funnel records it as it records any. Two paths reach it: a
        // gap that is not finite, which decided no mark, and a gap that
        // clears the mark past `i32::MAX`, whose branch no `i32` names.
        // A stop ON a mark keeps that mark's own miss (`across` below).
        let unnamed = || WalkStop {
            miss: BranchMiss::Undecided(
                match decide(name, Margin::of(T::from_f64(f64::NAN)), band) {
                    Err(cause) => cause,
                    Ok(sign) => unreachable!("an unreadable margin classified {sign:?}"),
                },
            ),
            astride: None,
        };
        if !crate::real::is_finite_length(gap) {
            return Err(unnamed());
        }
        let half = T::from_f64(0.5);
        // The mark `i + ½` periods up (`up`), or `i + ½` down, and
        // whether the gap clears it: decided past it, away from zero.
        // Spelled `k + side` with `k` the branch below or above it, as
        // a step-by-step walk spells it, so the forms a decision builds
        // are that walk's.
        let mark = |i: i32, up: bool| {
            let at = if up {
                T::from_f64(f64::from(i)) + half
            } else {
                T::from_f64(f64::from(-i)) + (T::zero() - half)
            };
            let sign = decide(name, meter(gap - at * self.length), band);
            let cleared = matches!(
                (&sign, up),
                (Ok(Sign::Positive), true) | (Ok(Sign::Negative), false)
            );
            (sign, cleared)
        };
        // The branch across the first mark, from zero, the gap does not
        // clear: `i` periods out where the gap is decided short of it,
        // else the stop on it with the branches astride it.
        let across = |i: i32, up: bool, sign: Result<Sign, Indeterminate>| {
            // The branch is `±i`, which an `i32` always names; the pair
            // astride the mark is named only where both of its ends are
            // (not past `i32::MAX`), and the stop keeps the mark's own
            // miss either way.
            let astride = || {
                let i = i64::from(i);
                let [lo, hi] = if up { [i, i + 1] } else { [-i - 1, -i] };
                Some([i32::try_from(lo).ok()?, i32::try_from(hi).ok()?])
            };
            match sign {
                Ok(Sign::Zero) => Err(WalkStop {
                    miss: BranchMiss::OnMark,
                    astride: astride(),
                }),
                Ok(_) => Ok(if up { i } else { -i }),
                Err(cause) => Err(WalkStop {
                    miss: BranchMiss::Undecided(cause),
                    astride: astride(),
                }),
            }
        };
        let up = match mark(0, true) {
            (_, true) => true,
            (sign @ (Ok(Sign::Zero) | Err(_)), false) => return across(0, true, sign),
            (_, false) => match mark(0, false) {
                (_, true) => false,
                (sign, false) => return across(0, false, sign),
            },
        };
        // Mark 0 is cleared. Gallop out (marks 1, 2, 4, 8, ...) to the
        // first the gap does not clear, then bisect between it and the
        // last it clears: the marks it clears are those nearer zero, so
        // the mark found is the first uncleared from zero, and the walk
        // asks O(log k) marks however far out the gap lies.
        let (mut cleared, mut probe) = (0, 1);
        let mut stop = loop {
            let (sign, past) = mark(probe, up);
            if !past {
                break (probe, sign);
            }
            if probe == i32::MAX {
                return Err(unnamed());
            }
            cleared = probe;
            probe = probe.saturating_mul(2);
        };
        while stop.0 - cleared > 1 {
            let mid = cleared + (stop.0 - cleared) / 2;
            match mark(mid, up) {
                (_, true) => cleared = mid,
                (sign, false) => stop = (mid, sign),
            }
        }
        across(stop.0, up, stop.1)
    }

    /// **The turn rule** (module docs): `extent` against one period, at
    /// `rate`, under the caller's `rows`.
    ///
    /// # Errors
    ///
    /// [`TurnUndecided`] where no reading settles a [`Turn`].
    pub fn turn<R: TurnRate<T>>(
        self,
        rows: TurnRows<R::Straddle>,
        extent: Extent<T>,
        rate: R,
        band: Band,
    ) -> Result<Turn, TurnUndecided> {
        rate.read(rows, Headroom(self.length - extent.0), band)
    }

    /// **"Not over"** (module docs): whether `extent` is not over one
    /// period — [`Turn::Short`], [`Turn::Whole`], or a reading the sup
    /// decides short ([`TurnUndecided::not_over_at_sup`]), whose
    /// escalation the answer supersedes
    /// ([`crate::k_stats::splice_superseded`]).
    ///
    /// # Errors
    ///
    /// Any other [`TurnUndecided`].
    pub fn not_over<R: TurnRate<T>>(
        self,
        rows: TurnRows<R::Straddle>,
        extent: Extent<T>,
        rate: R,
        band: Band,
    ) -> Result<bool, TurnUndecided> {
        let (turn, recording) = crate::k_stats::detached(|| self.turn(rows, extent, rate, band));
        match turn {
            Err(undecided) if undecided.not_over_at_sup() => {
                crate::k_stats::splice_superseded(recording);
                Ok(true)
            }
            turn => {
                crate::k_stats::splice(recording);
                turn.map(|turn| !matches!(turn, Turn::Over(_)))
            }
        }
    }

    /// **"Strictly partial"** (module docs): whether `extent` is
    /// definitely short of one period, [`Turn::Short`].
    ///
    /// # Errors
    ///
    /// [`TurnUndecided`] where no reading settles a [`Turn`].
    pub fn strictly_partial<R: TurnRate<T>>(
        self,
        rows: TurnRows<R::Straddle>,
        extent: Extent<T>,
        rate: R,
        band: Band,
    ) -> Result<bool, TurnUndecided> {
        self.turn(rows, extent, rate, band)
            .map(|turn| matches!(turn, Turn::Short(_)))
    }

    /// **"A whole turn"** (module docs): whether `extent` is one period,
    /// [`Turn::Whole`].
    ///
    /// # Errors
    ///
    /// [`TurnUndecided`] where no reading settles a [`Turn`].
    pub fn whole_turn<R: TurnRate<T>>(
        self,
        rows: TurnRows<R::Straddle>,
        extent: Extent<T>,
        rate: R,
        band: Band,
    ) -> Result<bool, TurnUndecided> {
        self.turn(rows, extent, rate, band)
            .map(|turn| matches!(turn, Turn::Whole(_)))
    }
}

/// **An extent along a periodic channel**, in its parameter units, never
/// negative by construction (module docs, *The extent cannot be
/// reversed*).
#[derive(Clone, Copy, Debug)]
pub struct Extent<T>(T);

/// Why [`Extent::forward`] gave no extent.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NotForward {
    /// The span is in the band.
    Undecided(Indeterminate),
    /// The span is within the zero band: no extent.
    Zero(MarginDiag),
    /// The span runs definitely backwards.
    Reversed(MarginDiag),
}

impl<T: Real> Extent<T> {
    /// The width of the hull of `values`: their greatest less their
    /// least, folded here so that no caller hands in a pair in the wrong
    /// order. `None` for no values. Poison propagates
    /// ([`Real::max`], [`Real::min`]). A span's two ends are not a hull;
    /// they enter through [`Extent::forward`].
    pub fn hull(values: impl IntoIterator<Item = T>) -> Option<Self> {
        let mut values = values.into_iter();
        let first = values.next()?;
        let (lo, hi) = values.fold((first, first), |(lo, hi), x| (lo.min(x), hi.max(x)));
        Some(Self(hi - lo))
    }

    /// The width of the hull of two values, whatever their order:
    /// [`Extent::hull`] of the pair.
    pub fn between(a: T, b: T) -> Self {
        let (lo, hi) = (a.min(b), a.max(b));
        Self(hi - lo)
    }

    /// The extent, in the channel's parameter units.
    pub fn get(self) -> T {
        self.0
    }

    /// A fixture's extent, unchecked: this module's rows only.
    #[cfg(test)]
    pub(crate) fn fixture(x: T) -> Self {
        Self(x)
    }
}

impl<T: Decide> Extent<T> {
    /// A span `t1 − t0` decided forward under `name`: its length at
    /// `rate`, the "definitely apart" door ([`Margin::metered`]), must be
    /// definitely positive. At a rate that is not positive no span is
    /// forward (it reads as its magnitude against that rate).
    ///
    /// # Errors
    ///
    /// [`NotForward`] for a span in the band, within the zero band, or
    /// reversed.
    pub fn forward(
        name: &'static str,
        t0: T,
        t1: T,
        rate: InfSpeed<T>,
        band: Band,
    ) -> Result<Self, NotForward> {
        let span = t1 - t0;
        // A rate that is not positive decides every span not forward: the
        // span's magnitude against it, never a reversed span's product
        // with a negative rate.
        let read = rate.get().select_le_zero(span.abs(), span);
        match decide_reported(name, Margin::metered(read, rate), band) {
            Ok(Decided {
                sign: Sign::Positive,
                ..
            }) => Ok(Self(span)),
            Ok(Decided {
                sign: Sign::Zero,
                margin,
            }) => Err(NotForward::Zero(margin)),
            Ok(Decided {
                sign: Sign::Negative,
                margin,
            }) => Err(NotForward::Reversed(margin)),
            Err(cause) => Err(NotForward::Undecided(cause)),
        }
    }

    /// The magnitude of a signed net quantity (a revolve's angle, a
    /// loop's net turn), with its sign, decided definitely nonzero under
    /// `name` at `rate`.
    ///
    /// # Errors
    ///
    /// [`decide_nonzero_reported`]'s: in the band, or definitely zero.
    pub fn decide_signed(
        name: &'static str,
        net: T,
        rate: InfSpeed<T>,
        band: Band,
    ) -> Result<(NonzeroSign, Self), Indeterminate> {
        let (sign, _) = decide_nonzero_reported(name, Margin::metered(net, rate), band)?;
        Ok(match sign {
            NonzeroSign::Positive => (sign, Self(net)),
            NonzeroSign::Negative => (sign, Self(-net)),
        })
    }
}

impl<T: Real> core::ops::Add for Extent<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

/// **Where an extent stands against one turn** ([`Period::turn`]), with
/// the margin that decided it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Turn {
    /// Definitely short of a whole turn.
    Short(MarginDiag),
    /// A whole turn, within the zero band at the sup rate.
    Whole(MarginDiag),
    /// Definitely over a whole turn.
    Over(MarginDiag),
}

/// A [`Turn`] that is not over: what a certificate stores.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Within {
    /// Definitely short of a whole turn.
    Short,
    /// A whole turn.
    Whole,
}

impl Turn {
    fn of(decided: Decided) -> Self {
        match decided.sign {
            Sign::Positive => Self::Short(decided.margin),
            Sign::Zero => Self::Whole(decided.margin),
            Sign::Negative => Self::Over(decided.margin),
        }
    }

    /// The turn, where it is not over.
    #[must_use]
    pub fn within(self) -> Option<Within> {
        match self {
            Self::Short(_) => Some(Within::Short),
            Self::Whole(_) => Some(Within::Whole),
            Self::Over(_) => None,
        }
    }

    /// The margin that decided it.
    #[must_use]
    pub fn margin(self) -> MarginDiag {
        match self {
            Self::Short(m) | Self::Whole(m) | Self::Over(m) => m,
        }
    }
}

/// **The rows a caller decides a turn under**: its own, so each site's
/// population stays separable in the telemetry. `straddle` is `()` at an
/// [`Exact`] rate, which cannot straddle, and the straddle row's name at
/// a [`Bounded`] one. At a `Bounded` rate `turn` carries both readings
/// of the headroom, the inf's and (where it is read) the sup's: they are
/// two bounds on one quantity, so one row holds them, as the site's own
/// spelling did.
#[derive(Clone, Copy, Debug)]
pub struct TurnRows<S> {
    /// The headroom's row, `(period − extent)` at the rate.
    pub turn: &'static str,
    /// The row a straddle of the two readings is named by.
    pub straddle: S,
}

impl TurnRows<()> {
    /// The rows of an [`Exact`] site.
    #[must_use]
    pub const fn exact(turn: &'static str) -> Self {
        Self { turn, straddle: () }
    }
}

impl TurnRows<&'static str> {
    /// The rows of a [`Bounded`] site.
    #[must_use]
    pub const fn bounded(turn: &'static str, straddle: &'static str) -> Self {
        Self { turn, straddle }
    }
}

/// Which row a [`TurnUndecided`] is named by.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TurnRow {
    /// [`TurnRows::turn`]: a reading in the band.
    Turn,
    /// [`TurnRows::straddle`]: two decided readings straddling the band.
    Straddle,
}

/// **No [`Turn`] settled**: the escalation, the row it is named by, and
/// the sign the sup reading decided, where it was read and decided.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TurnUndecided {
    /// The escalation (the row's name inside).
    pub diag: Indeterminate,
    /// The row it is named by.
    pub on: TurnRow,
    /// The sign of the headroom at the sup rate: `None` at an [`Exact`]
    /// rate (whose one reading is the one undecided) and where the sup
    /// reading is itself in the band.
    pub sup: Option<Sign>,
}

impl TurnUndecided {
    /// Whether the sup reading put the extent definitely over a turn.
    #[must_use]
    pub fn over_at_sup(&self) -> bool {
        self.sup == Some(Sign::Negative)
    }

    /// **Whether "not over" may take this reading as an extent that is
    /// not over a turn**, the one home of that admission: the sup read
    /// the headroom definitely positive, so the extent is short of a
    /// turn in the parameter, whatever the speed. Admitting a reading
    /// the sup does not decide would admit an overlap up to the
    /// escalation band.
    #[must_use]
    pub fn not_over_at_sup(&self) -> bool {
        self.sup == Some(Sign::Positive)
    }
}

/// An exact rate: the speed at every point of the extent, so its inf and
/// its sup are one (a circle's radius, an authored angle on its arm).
#[derive(Clone, Copy, Debug)]
pub struct Exact<T>(T);

impl<T: Real> Exact<T> {
    /// The rate `r`, as a speed: its magnitude, so no rate the rule reads
    /// is negative (a negative one would flip `Over` and `Short`).
    pub fn new(r: T) -> Self {
        Self(r.abs())
    }
}

/// A rate known only between bounds: the least and the greatest speed
/// over the stretch the headroom measures (an ellipse, a spiric, a chart
/// image).
#[derive(Clone, Copy, Debug)]
pub struct Bounded<T> {
    inf: InfSpeed<T>,
    sup: SupSpeed<T>,
}

impl<T: Real> Bounded<T> {
    /// A lower bound `inf` and an upper bound `sup` on the speed over the
    /// headroom's stretch — the caller's premise, which the rule's
    /// soundness rests on. Both are read as speeds: the
    /// inf's magnitude, and the sup never below it (the larger of an
    /// upper and a lower bound on one speed is still an upper bound), so
    /// a pair with `sup < inf` is not representable.
    pub fn new(inf: InfSpeed<T>, sup: SupSpeed<T>) -> Self {
        let inf = inf.get().abs();
        Self {
            inf: InfSpeed::new(inf),
            sup: SupSpeed::new(sup.get().max(inf)),
        }
    }

    /// The upper bound.
    pub fn sup(self) -> SupSpeed<T> {
        self.sup
    }
}

/// The headroom `period − extent` the rule reads, formed only by
/// [`Period::turn`]: no caller hands the rule a bare value.
#[derive(Clone, Copy, Debug)]
pub struct Headroom<T>(T);

mod sealed {
    pub trait Sealed {}
}

impl<T> sealed::Sealed for Exact<T> {}
impl<T> sealed::Sealed for Bounded<T> {}

/// **A typed rate** the turn rule reads at: [`Exact`] or [`Bounded`].
/// Sealed: the rule's readings are these two.
pub trait TurnRate<T: Decide>: sealed::Sealed + Copy {
    /// What a caller names a straddle by: `()` where none can arise.
    type Straddle: Copy;

    /// The lower bound, the rate `Over` and `Short` read the headroom at.
    fn inf(self) -> InfSpeed<T>;

    /// The rule at this rate, on the headroom [`Period::turn`] formed.
    #[doc(hidden)]
    fn read(
        self,
        rows: TurnRows<Self::Straddle>,
        headroom: Headroom<T>,
        band: Band,
    ) -> Result<Turn, TurnUndecided>;
}

impl<T: Decide> TurnRate<T> for Exact<T> {
    type Straddle = ();

    fn inf(self) -> InfSpeed<T> {
        InfSpeed::new(self.0)
    }

    fn read(
        self,
        rows: TurnRows<()>,
        Headroom(headroom): Headroom<T>,
        band: Band,
    ) -> Result<Turn, TurnUndecided> {
        decide_reported(rows.turn, Margin::metered(headroom, self.inf()), band)
            .map(Turn::of)
            .map_err(|diag| TurnUndecided {
                diag,
                on: TurnRow::Turn,
                sup: None,
            })
    }
}

impl<T: Decide> TurnRate<T> for Bounded<T> {
    type Straddle = &'static str;

    fn inf(self) -> InfSpeed<T> {
        self.inf
    }

    fn read(
        self,
        rows: TurnRows<&'static str>,
        Headroom(headroom): Headroom<T>,
        band: Band,
    ) -> Result<Turn, TurnUndecided> {
        let low = decide_reported(rows.turn, Margin::metered(headroom, self.inf), band);
        if let Ok(
            decided @ Decided {
                sign: Sign::Negative | Sign::Positive,
                ..
            },
        ) = low
        {
            return Ok(Turn::of(decided));
        }
        let high = decide_reported(rows.turn, Margin::metered_sup(headroom, self.sup), band);
        let sup = high.ok().map(|decided| decided.sign);
        let undecided = |diag, on| Err(TurnUndecided { diag, on, sup });
        match (low, high) {
            (
                _,
                Ok(
                    whole @ Decided {
                        sign: Sign::Zero, ..
                    },
                ),
            ) => Ok(Turn::of(whole)),
            // The inf reading is Zero: the two are decided and straddle
            // the band, the lower bound on `h`'s length below the upper.
            (Ok(zero), Ok(high)) => {
                let over = high.sign == Sign::Negative;
                let (lower, upper) = if over { (high, zero) } else { (zero, high) };
                undecided(
                    straddle(rows.straddle, lower, upper, band),
                    TurnRow::Straddle,
                )
            }
            (Ok(_), Err(diag)) | (Err(diag), _) => undecided(diag, TurnRow::Turn),
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic)]
mod period_rows {
    use crate::{Band, Interval, Margin, Real, SpanLocate};

    use super::{BranchMiss, Period, WalkStop};

    fn band() -> Band {
        Band::new(1e-9, 1e-8).expect("a band")
    }

    /// **The marks are read from zero, however far out.** A gap 25 and a
    /// fraction periods out decides `25`; seven half periods decide `7`
    /// on an orbit whose step is half a period.
    #[test]
    fn a_far_gap_decides_its_branch_with_no_cap() {
        let tau = f64::tau();
        let period = Period::new(tau);
        assert!(matches!(
            period.decide("t", 25.3 * tau, Margin::of, band()),
            Ok(25)
        ));
        assert!(matches!(
            period.decide("t", -25.3 * tau, Margin::of, band()),
            Ok(-25)
        ));
        let half = Period::new(tau / 2.0);
        assert!(matches!(
            half.decide("t", 3.5 * tau, Margin::of, band()),
            Ok(7)
        ));
    }

    /// **A far gap on a mark names the branches astride it.** A bracket
    /// straddling the mark 40.5 periods out decides neither branch, and
    /// the walk stops on that mark with `[40, 41]`.
    #[test]
    fn a_far_gap_on_a_mark_stops_astride_it() {
        let mark = Interval::from_f64(40.5 * f64::tau());
        let gap =
            (mark - Interval::from_f64(1e-10)).enclosure_hull(mark + Interval::from_f64(1e-10));
        match Period::new(Interval::tau()).decide("t", gap, Margin::of, band()) {
            Err(WalkStop {
                miss: BranchMiss::OnMark | BranchMiss::Undecided(_),
                astride: Some([40, 41]),
            }) => {}
            other => panic!("the walk stops on the mark at 40.5 periods: {other:?}"),
        }
    }

    /// The step-by-step walk the gallop stands for: each mark from zero
    /// in turn, out to the first the gap does not clear.
    fn step_walk(gap: f64, asked: &core::cell::RefCell<Vec<f64>>) -> Result<i32, String> {
        let tau = f64::tau();
        let mark = |at: f64| {
            let m = gap - at * tau;
            asked.borrow_mut().push(m);
            crate::k_stats::decide("t", Margin::of(m), band())
        };
        let on = |k: i32, side: f64| f64::from(k) + side;
        let mut k = 0;
        let (mut below, mut above) = (false, false);
        loop {
            if !above {
                match mark(on(k, 0.5)) {
                    Ok(crate::Sign::Negative) => {}
                    Ok(crate::Sign::Positive) => {
                        (k, below, above) = (k + 1, true, false);
                        continue;
                    }
                    other => return Err(format!("{other:?} [{k}, {}]", k + 1)),
                }
            }
            if !below {
                match mark(on(k, -0.5)) {
                    Ok(crate::Sign::Positive) => {}
                    Ok(crate::Sign::Negative) => {
                        (k, below, above) = (k - 1, false, true);
                        continue;
                    }
                    other => return Err(format!("{other:?} [{}, {k}]", k - 1)),
                }
            }
            return Ok(k);
        }
    }

    /// **The gallop answers as the step-by-step walk does**, on branches,
    /// on marks and in band, from 40 periods down to 40 up; out to two
    /// periods it asks the same marks in the same order.
    #[test]
    fn the_gallop_answers_as_the_step_walk_does() {
        let tau = f64::tau();
        let band = band();
        let mut gaps = Vec::new();
        for k in -40..=40 {
            for frac in [0.0, 0.2, 0.49, -0.3] {
                gaps.push((f64::from(k) + frac) * tau);
            }
            let mark = (f64::from(k) + 0.5) * tau;
            gaps.extend([
                mark,
                mark + 0.5 * band.zero(),
                mark + 0.5 * (band.zero() + band.escalate()),
            ]);
        }
        for gap in gaps {
            let (walked, galloped) = (
                core::cell::RefCell::new(Vec::new()),
                core::cell::RefCell::new(Vec::new()),
            );
            let want = step_walk(gap, &walked);
            let got = Period::new(tau)
                .decide(
                    "t",
                    gap,
                    |m| {
                        galloped.borrow_mut().push(m);
                        Margin::of(m)
                    },
                    band,
                )
                .map_err(|stop| {
                    let [lo, hi] = stop.astride.expect("a finite gap stops on a mark");
                    let sign = match stop.miss {
                        BranchMiss::OnMark => "Ok(Zero)".to_owned(),
                        BranchMiss::Undecided(cause) => format!("{:?}", Err::<(), _>(cause)),
                    };
                    format!("{sign} [{lo}, {hi}]")
                });
            assert_eq!(got, want, "gap {:.6} periods", gap / tau);
            if (gap / tau).abs() <= 2.5 {
                assert_eq!(
                    *galloped.borrow(),
                    *walked.borrow(),
                    "gap {:.6} periods",
                    gap / tau
                );
            }
        }
    }

    /// **A far gap costs O(log k) marks**, and a branch past `i32`
    /// decides nothing: a billion periods out answers in under a hundred
    /// marks, and three billion stops undecided with no mark named.
    #[test]
    fn a_far_gap_costs_log_k_marks() {
        let tau = f64::tau();
        let asked = core::cell::Cell::new(0);
        let meter = |m| {
            asked.set(asked.get() + 1);
            Margin::of(m)
        };
        let period = Period::new(tau);
        assert!(matches!(
            period.decide("t", (1e9 + 0.25) * tau, meter, band()),
            Ok(1_000_000_000)
        ));
        assert!(asked.get() < 100, "{} marks", asked.get());
        assert!(matches!(
            period.decide("t", -3e9 * tau, Margin::of, band()),
            Err(WalkStop {
                miss: BranchMiss::Undecided(_),
                astride: None
            })
        ));
    }

    /// **A branch an `i32` names is answered, to its last value.** A
    /// gap just past `i32::MAX` periods decides `i32::MAX`, and its
    /// mirror `−i32::MAX`: the pair astride a mark is formed only where
    /// the walk stops on one.
    #[test]
    fn a_branch_at_i32_max_is_answered() {
        let tau = f64::tau();
        let k = f64::from(i32::MAX);
        let period = Period::new(tau);
        let up = period.decide("t", (k + 0.1) * tau, Margin::of, band());
        let down = period.decide("t", -(k + 0.1) * tau, Margin::of, band());
        assert!(matches!(up, Ok(i32::MAX)), "up: {up:?}");
        assert!(matches!(down, Ok(-2_147_483_647)), "down: {down:?}");
    }

    test_utils::gated_to!["crates/geom-core/src/period.rs"];

    /// **The gallop answers as the step-by-step walk does at `Interval`,
    /// metered** (a counterexample search: the seed varies). Enclosures
    /// of every width from a point to two periods, centred out to 300
    /// periods, half of them within 1e-7 periods of a half-period mark,
    /// metered at a lever of 0.37: the answer, the miss and the pair
    /// astride it agree with a walk that asks every mark from zero.
    #[test]
    fn the_gallop_answers_as_the_step_walk_does_at_interval() {
        use crate::k_stats::decide;
        use crate::{Real as _, Sign};
        use test_utils::fuzz;
        let mut rng = fuzz::start("period::period_rows::gallop_at_interval");
        let tau = Interval::tau();
        let arm = Interval::from_f64(0.37);
        let band = band();
        let meter = |g| Margin::metered(g, crate::InfSpeed::new(arm));
        let step_walk = |gap: Interval| -> String {
            let half = Interval::from_f64(0.5);
            let mark = |k: i32, side: Interval| {
                decide(
                    "t",
                    meter(gap - (Interval::from_f64(f64::from(k)) + side) * tau),
                    band,
                )
            };
            let (mut k, mut below, mut above) = (0i32, false, false);
            loop {
                if !above {
                    match mark(k, half) {
                        Ok(Sign::Negative) => {}
                        Ok(Sign::Positive) => {
                            (k, below, above) = (k + 1, true, false);
                            continue;
                        }
                        Ok(Sign::Zero) => return format!("OnMark {:?}", [k, k + 1]),
                        Err(_) => return format!("Undecided {:?}", [k, k + 1]),
                    }
                }
                if !below {
                    match mark(k, Interval::zero() - half) {
                        Ok(Sign::Positive) => {}
                        Ok(Sign::Negative) => {
                            (k, below, above) = (k - 1, false, true);
                            continue;
                        }
                        Ok(Sign::Zero) => return format!("OnMark {:?}", [k - 1, k]),
                        Err(_) => return format!("Undecided {:?}", [k - 1, k]),
                    }
                }
                return format!("Ok({k})");
            }
        };
        let widths = [0.0, 1e-12, 1e-9, 3e-8, 1e-3, 0.4, 2.0];
        for trial in 0..fuzz::scaled(20_000) {
            let mut c = rng.range(-300.0, 300.0);
            if rng.unit() < 0.5 {
                c = c.round() + 0.5 + rng.range(-5e-8, 5e-8);
            }
            let w = widths[rng.below(widths.len())];
            let gap = (tau * Interval::from_f64(c)).enclosure_hull(tau * Interval::from_f64(c + w));
            let got = match Period::new(tau).decide("t", gap, meter, band) {
                Ok(k) => format!("Ok({k})"),
                Err(WalkStop {
                    miss: BranchMiss::OnMark,
                    astride: Some(a),
                }) => format!("OnMark {a:?}"),
                Err(WalkStop {
                    miss: BranchMiss::Undecided(_),
                    astride: Some(a),
                }) => format!("Undecided {a:?}"),
                Err(other) => format!("{other:?}"),
            };
            assert_eq!(
                got,
                step_walk(gap),
                "seed {:#x}, trial {trial}: centre {c} periods, width {w}",
                fuzz::seed()
            );
        }
    }

    /// **A gap that names no number decides nothing.** An infinite or
    /// NaN gap stops at once as undecided, with no mark: an infinite
    /// margin would read every mark definite and the walk would never
    /// end.
    #[test]
    fn a_gap_that_is_not_finite_stops_undecided() {
        let period = Period::new(f64::tau());
        for gap in [f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            assert!(
                matches!(
                    period.decide("t", gap, Margin::of, band()),
                    Err(WalkStop {
                        miss: BranchMiss::Undecided(_),
                        astride: None
                    })
                ),
                "{gap}"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
mod turn_rows {
    use crate::k_stats::{Bracket, NonzeroSign};
    use crate::{Band, InfSpeed, MarginDiag, Real, SupSpeed};

    use super::{
        Bounded, Exact, Extent, NotForward, Period, Turn, TurnRow, TurnRows, TurnUndecided, Within,
    };

    const EPS: f64 = 1e-9;
    const ROWS: TurnRows<&str> = TurnRows::bounded("t_turn", "t_straddle");

    fn band() -> Band {
        Band::new(EPS, 10.0 * EPS).expect("a band")
    }

    fn one_turn() -> Period<f64> {
        Period::new(f64::tau())
    }

    /// The extent `τ − h` of a fixture whose headroom is `h` parameter
    /// units: a hull from zero, so a negative `h` is an extent past a turn.
    fn past(h: f64) -> Extent<f64> {
        Extent::fixture(f64::tau() - h)
    }

    fn exact(h: f64) -> Result<Turn, TurnUndecided> {
        one_turn().turn(TurnRows::exact("t_turn"), past(h), Exact::new(1.0), band())
    }

    /// The rule at an inf of 1 and a sup of 20, with what it recorded.
    fn bounded(h: f64) -> (Result<Turn, TurnUndecided>, crate::k_stats::Recorded) {
        let bracket = Bracket::open();
        let rate = Bounded::new(InfSpeed::new(1.0), SupSpeed::new(20.0));
        let got = one_turn().turn(ROWS, past(h), rate, band());
        (got, bracket.finish())
    }

    fn reads(recorded: &crate::k_stats::Recorded) -> usize {
        recorded.verdicts.len() + recorded.escalations.len()
    }

    /// **An exact rate reads the headroom once**: short, whole and over
    /// are its three signs, and a reading in the band is undecided on the
    /// turn row, never over at the sup (which is the same reading).
    #[test]
    fn an_exact_rate_reads_the_headroom_once() {
        assert!(matches!(exact(20.0 * EPS), Ok(Turn::Short(_))), "short");
        assert!(matches!(exact(0.0), Ok(Turn::Whole(_))), "whole");
        assert!(
            matches!(exact(-0.5 * EPS), Ok(Turn::Whole(_))),
            "whole, over by ε/2"
        );
        assert!(matches!(exact(-20.0 * EPS), Ok(Turn::Over(_))), "over");
        for h in [3.0 * EPS, -3.0 * EPS] {
            let got = exact(h);
            assert!(
                matches!(
                    got,
                    Err(TurnUndecided {
                        on: TurnRow::Turn,
                        sup: None,
                        ..
                    })
                ),
                "{h:e}: {got:?}"
            );
        }
    }

    /// **A bounded rate decides each verdict on the bound that makes it
    /// sound** (module docs): a definite inf reading is the answer and the
    /// sup is not read; `Whole` needs the sup reading in the zero band;
    /// an inf `Zero` against a definite sup straddles, the lower bound on
    /// `h`'s length below the upper, and is over at the sup exactly where
    /// that sup reading is negative.
    #[test]
    fn a_bounded_rate_decides_on_the_sound_bound() {
        for (h, short) in [(20.0 * EPS, true), (-20.0 * EPS, false)] {
            let (got, recorded) = bounded(h);
            let want = if short { "short" } else { "over" };
            assert!(
                matches!(
                    (got, short),
                    (Ok(Turn::Short(_)), true) | (Ok(Turn::Over(_)), false)
                ),
                "{want}: {got:?}"
            );
            assert_eq!(reads(&recorded), 1, "{want}: the inf reading alone");
        }
        let (got, recorded) = bounded(0.04 * EPS);
        assert!(matches!(got, Ok(Turn::Whole(_))), "whole: {got:?}");
        assert_eq!(reads(&recorded), 2, "whole: both readings");
        for (h, over) in [(-0.9 * EPS, true), (0.9 * EPS, false)] {
            let (got, recorded) = bounded(h);
            let Err(
                undecided @ TurnUndecided {
                    diag,
                    on: TurnRow::Straddle,
                    ..
                },
            ) = got
            else {
                panic!("{h:e}: a straddle: {got:?}");
            };
            assert_eq!(undecided.over_at_sup(), over, "{h:e}");
            assert_eq!(undecided.not_over_at_sup(), !over, "{h:e}");
            assert_eq!(diag.predicate, Some("t_straddle"), "{h:e}");
            // The headroom the rule read: `τ − (τ − h)`, rounded.
            let read = f64::tau() - (f64::tau() - h);
            let (lo, hi) = if over {
                (read * 20.0, read)
            } else {
                (read, read * 20.0)
            };
            assert_eq!(
                diag.margin,
                MarginDiag::enclosure(lo, hi),
                "{h:e}: the two readings' hull"
            );
            let escalations: Vec<_> = recorded.escalations.iter().map(|e| e.predicate()).collect();
            assert_eq!(
                escalations,
                ["t_straddle"],
                "{h:e}: one escalation, the straddle"
            );
        }
        // The inf reading in the band: undecided on its own margin, over at
        // the sup where the sup reading is definitely negative.
        for (h, over) in [(-3.0 * EPS, true), (3.0 * EPS, false)] {
            let (got, _) = bounded(h);
            assert!(
                matches!(
                    got,
                    Err(u @ TurnUndecided { on: TurnRow::Turn, .. }) if u.over_at_sup() == over
                        && u.not_over_at_sup() == !over
                ),
                "{h:e}: {got:?}"
            );
        }
        // The inf reading Zero, the sup in the band: undecided on the sup's
        // margin, not over.
        let (got, _) = bounded(-0.3 * EPS);
        assert!(
            matches!(
                got,
                Err(TurnUndecided {
                    on: TurnRow::Turn,
                    sup: None,
                    ..
                })
            ),
            "{got:?}"
        );
    }

    /// **The extent's doors**: a forward span decides its sign, refusing
    /// zero and reversed typed; a signed net quantity keeps its sign and
    /// its magnitude, refusing zero; a hull is its width; extents add.
    #[test]
    fn the_extent_doors_decide_what_they_claim() {
        let rate = InfSpeed::new(1.0);
        assert_eq!(
            Extent::forward("t_fwd", 1.0, 3.0, rate, band()).map(Extent::get),
            Ok(2.0)
        );
        assert!(matches!(
            Extent::forward("t_fwd", 1.0, 1.0, rate, band()),
            Err(NotForward::Zero(_))
        ));
        assert!(matches!(
            Extent::forward("t_fwd", 3.0, 1.0, rate, band()),
            Err(NotForward::Reversed(_))
        ));
        assert!(matches!(
            Extent::forward("t_fwd", 1.0, 1.0 + 3.0 * EPS, rate, band()),
            Err(NotForward::Undecided(_))
        ));
        let (sign, extent) = Extent::decide_signed("t_net", -2.0, rate, band()).unwrap();
        assert_eq!((sign, extent.get()), (NonzeroSign::Negative, 2.0));
        assert!(Extent::decide_signed("t_net", 0.0, rate, band()).is_err());
        assert_eq!(Extent::hull([2.0, -1.0, 0.5]).map(Extent::get), Some(3.0));
        assert_eq!(
            Extent::hull([1.0, 0.0]).map(Extent::get),
            Some(1.0),
            "either order"
        );
        assert!(Extent::hull(core::iter::empty::<f64>()).is_none());
        assert_eq!((Extent::fixture(1.0) + Extent::fixture(2.0)).get(), 3.0);
        let hull = one_turn().turn(
            TurnRows::exact("t_turn"),
            Extent::hull([1.0]).unwrap(),
            Exact::new(1.0),
            band(),
        );
        assert!(
            matches!(hull, Ok(Turn::Short(_))),
            "a zero-width hull: {hull:?}"
        );
    }

    /// **No public door yields a negative extent or a rate the rule would
    /// read backwards** (the holes the review executed): at a negative
    /// rate a reversed span is not forward, and neither is a forward one;
    /// a negative exact rate reads as its magnitude, so an extent over a
    /// turn is still over; a bounded pair whose sup is below its inf is
    /// read with the sup lifted to the inf.
    #[test]
    fn no_door_yields_a_reversed_extent_or_rate() {
        let negative = InfSpeed::new(-1.0);
        for (t0, t1) in [(3.0, 1.0), (1.0, 3.0)] {
            assert!(
                matches!(
                    Extent::forward("t_fwd", t0, t1, negative, band()),
                    Err(NotForward::Reversed(_))
                ),
                "({t0}, {t1}) at a negative rate"
            );
        }
        let over = one_turn().turn(
            TurnRows::exact("t_turn"),
            past(-20.0 * EPS),
            Exact::new(-1.0),
            band(),
        );
        assert!(matches!(over, Ok(Turn::Over(_))), "{over:?}");
        let odd = Bounded::new(InfSpeed::new(2.0), SupSpeed::new(1.0));
        assert_eq!(odd.sup().get(), 2.0, "the sup is lifted to the inf");
        let whole = one_turn().turn(ROWS, past(0.25 * EPS), odd, band());
        assert!(matches!(whole, Ok(Turn::Whole(_))), "{whole:?}");
    }

    /// `Turn::within` is the turn a certificate keeps, and `Over` keeps none.
    #[test]
    fn within_drops_only_over() {
        let m = exact(0.0).unwrap().margin();
        assert_eq!(Turn::Short(m).within(), Some(Within::Short));
        assert_eq!(Turn::Whole(m).within(), Some(Within::Whole));
        assert_eq!(Turn::Over(m).within(), None);
    }
}
