//! **A point against one carrier**: its foot ([`Curve3::foot`]) and
//! whether it lies on the carrier ([`Foot::on_carrier`]). These are the
//! one-carrier readings of a point on a curve (D3), homed in `geom`
//! beside the carrier's evaluators and its other banded decisions.
//!
//! **The foot** is a parameter of the carrier near the point's closest
//! point, held with the point and the carrier it was found for, and its
//! gap `|q − C(t)|`, which bounds the point's distance from the carrier
//! from above. Only [`Curve3::foot`] (the whole carrier) and
//! [`Curve3::foot_within`] (an interval of it, the foot a point-on-span
//! question reads) make one, so a foot cannot be read against another
//! point or another carrier. Where it is found:
//!
//! - a line: in closed form;
//! - a circle: the angle of the point about the centre, exact, the
//!   radius's sign read into it;
//! - an ellipse: the angle of the point's unit-coordinate image, then
//!   Gauss–Newton. The unit-coordinate angle alone is not a foot: its
//!   point lies up to `(1 + a/b)·d` from a point `d` off the ellipse;
//! - a spiric oval: [`Curve3::param_near`]'s angle, then Gauss–Newton;
//! - a spline: Gauss–Newton from the caller's seed, clamped into the
//!   interval. Over the whole carrier, a clamped spline's foot is held
//!   to its domain, and a periodic cubic's is free to leave its turn,
//!   since the loop runs on.
//!
//! Over an interval, a line's and a circle's foot is the whole carrier's
//! nearest point, which the interval's own reading of its ends places
//! (the interval's nearest point is it, or an end); every other kind's
//! seed is read on the branch about the interval's middle and clamped
//! into it, and each polish iterate is held to it. No sample is read: a
//! foot is a place to read, and a poor one costs an escalation, never a
//! verdict (below).
//! A closed kind's angle is read relative to the seed's direction, or to
//! its opposite where the point lies definitely behind it, so an
//! enclosure of the point's direction never crosses the angle's own cut.
//! Each polish step is kept only where it decides the gap smaller, so
//! the foot's gap is never worse than its seed's. At a scalar that holds
//! enclosures, each polish iterate is a point, its bracket's midpoint.
//! The parameter certifies nothing, so it encloses nothing, and the gap
//! read at a point carries only the width of `q` and of the carrier.
//! An iterate held as an enclosure would carry that width into every
//! later step and compound it.
//!
//! **Nearest.** On a line and a circle the foot is the point's nearest
//! point of the whole carrier ([`Foot::nearest`]). On an ellipse, a
//! spiric oval and a spline it is a polished stationary point, which may
//! be a far branch's (a spline's hairpin, or a seed on the wrong side): a
//! reading that rests on the foot being nearest — that the point lies off
//! a span because its foot does — reads a certificate instead
//! ([`Curve3::clear_of`]).
//!
//! **A foot has no bare parameter.** The point's parameter is read only
//! where something certifies it: [`OnCarrier::On`] (a gap in the band's
//! zero) or [`Foot::nearest`] (the kind). [`Foot::seed`] is the parameter
//! as found, for a further search, and certifies nothing. `On` certifies
//! the point up to the band's zero: the carrier passes within the zero of
//! it at `t`. It certifies the branch too only where the carrier's
//! branches are decided apart. A carrier that comes within the zero of
//! itself near the point (a hairpin narrower than the zero) reads `On` on
//! either branch, so a reader of the branch's own geometry (a tangent)
//! rests on the carrier's branches being apart, not on `On`. No reader
//! rests on which of two tied polish candidates was kept, and the
//! polish's comparisons are a heuristic's, decided outside the log.
//!
//! **On the carrier** is decided on the foot's gap, zero in the band
//! (`curve_carrier_gap`). **Off the carrier** is decided only on a lower
//! bound on the point's distance from the whole carrier, past the band
//! (`curve_carrier_clear`), so a poor foot can cost an escalation but
//! never a wrong `Off`. The lower bound, per kind:
//!
//! - a line and a circle: the exact distance;
//! - an ellipse: `2|F|/(g + √(g² + 4|F|/s²))` in its plane, with the
//!   height above the plane folded in. Here `F = X²/a² + Y²/b² − 1` at the
//!   point's in-plane image, `g = |∇F|`, and `s` is the smaller
//!   semi-axis. Along the segment from the image to its foot, `F` falls
//!   to zero while `|∇F|` grows at most at the Hessian's norm `2/s²`, so
//!   `|F| ≤ g·d + d²/s²`, whose positive root is the bound;
//! - a spiric oval: the greatest of the point's distance from the
//!   cutting plane, its distance from the torus, and how far it lies
//!   short of the oval's side of the plane's trace;
//! - a spline: the certified piece walk ([`geom_core::bisect`]) over the
//!   whole domain. A piece is cleared when its middle point's distance
//!   less its reach (its half-span times the carrier's greatest speed,
//!   [`crate::carrier_speed`]) decides positive (`curve_carrier_piece_clear`).
//!   The point is off only where every piece clears, so the walk stops
//!   at the first piece that does not clear with its reach within the
//!   band (`curve_carrier_piece`): no later piece can make it off. A
//!   walk past [`geom_core::bisect::PIECE_BUDGET`] refuses
//!   ([`PieceWalkRefusal::Exhausted`]), and a speed bound that is zero or
//!   poison refuses on kind ([`PieceWalkRefusal::Unbounded`]). A rational
//!   spline's speed bound is certified at `Interval`; at `f64` and
//!   extreme weights it is the same expression in round-to-nearest, not
//!   a bound.
//!
//! **Where the walk keeps a piece.** The spline's walk is also the one
//! home for where on the carrier the point can be near: the piece it
//! stops at (the first, in parameter order, whose reach is within the
//! band and that is not decided clear) is a place the carrier may come
//! within the band of the point. So where the foot's gap does not decide
//! zero, the polish is restarted once, from that piece's middle, and reads
//! `On` there where its gap decides zero. The seed order is the caller's
//! seed, then the walk's kept piece: the restart reads the certificate the
//! Off already reads rather than guessing a start, and the walk stops
//! where it always has, so it reads no more pieces than the Off does. A
//! restart whose gap does not decide zero is no answer, its reading
//! superseded, and the walk's reading of the kept piece stands as before:
//! off where every piece clears, and otherwise the straddle below.
//!
//! The lower bound on the band's zero side and the gap past it straddle
//! the whole band (`curve_carrier_straddle`): only a carrier bending
//! tighter than the band resolves, or a poor foot, allows it, and it
//! escalates.

use geom_core::bisect::{Step, bisect};
use geom_core::k_stats::{Pick, choose, decide, decide_reported, straddle};
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, SupSpeed,
};

use super::Curve3;

/// The Gauss–Newton steps a foot is polished by.
const POLISH_STEPS: usize = 6;

/// A point's foot on a carrier, made only by [`Curve3::foot`] and
/// [`Curve3::foot_within`], holding the point and the carrier it was
/// found for (module docs).
#[derive(Clone, Copy, Debug)]
pub struct Foot<'c, T: Real> {
    carrier: &'c Curve3<T>,
    q: Point3<T>,
    t: T,
    gap: T,
}

impl<'c, T: Decide> Foot<'c, T> {
    /// The carrier it was found on.
    pub fn carrier(&self) -> &'c Curve3<T> {
        self.carrier
    }

    /// The point it is the foot of.
    pub fn point(&self) -> Point3<T> {
        self.q
    }

    /// **The parameter as found, certifying nothing**: a seed for a
    /// further search of the carrier near the point, never the point's
    /// parameter. Where two candidates tie, which one is kept is the
    /// polish's choice, and the two can lie on different branches of a
    /// spline. The point's parameter is [`OnCarrier::On`]'s or
    /// [`Self::nearest`]'s. A closed carrier's is not reduced to any
    /// branch.
    pub fn seed(&self) -> T {
        self.t
    }

    /// **The parameter of the point's nearest point of the whole
    /// carrier**, where the kind makes the foot that point (module docs:
    /// a line, a circle); `None` on every other kind. A closed carrier's
    /// is not reduced to any branch.
    pub fn nearest(&self) -> Option<T> {
        self.carrier.foot_is_nearest().then_some(self.t)
    }

    /// **Of two feet of one point on one carrier, the one whose gap is
    /// decided smaller**, `self` at a tie: the heuristic's choice between
    /// sound places that the foot's own polish makes (ledger row F32), for
    /// a reader holding feet found over several intervals (`geom-brep`'s
    /// site reading over its tube pieces). No reader rests on which is
    /// kept: the kept foot is read as any foot is ([`Self::on_carrier`]).
    #[must_use]
    pub fn least(self, other: Self) -> Self {
        if smaller(other.gap, self.gap) {
            other
        } else {
            self
        }
    }

    /// `|q − C(t)|`, an upper bound on the point's distance from the
    /// carrier: what [`Self::on_carrier`] decides `On` on.
    pub fn gap(&self) -> T {
        self.gap
    }

    /// **Whether the point lies on the carrier** (module docs): on by the
    /// gap at the foot, off by a lower bound on its distance from the
    /// whole carrier.
    ///
    /// # Errors
    ///
    /// [`CarrierRefusal`]: an escalation where neither decides, the lower
    /// bound in the band's zero and the gap past it straddling the band,
    /// or a spline's walk that cannot run.
    pub fn on_carrier(&self, band: Band) -> Result<OnCarrier<T>, CarrierRefusal> {
        let gap = decide_reported("curve_carrier_gap", Margin::of(self.gap), band);
        if let Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) = gap
        {
            return Ok(OnCarrier::On { t: self.t, margin });
        }
        let clear = match self.carrier.clearance(self.q) {
            Some(lower) => decide_reported("curve_carrier_clear", Margin::of(lower), band),
            None => {
                match self.carrier.walk_kept(
                    self.q,
                    self.carrier.domain(),
                    Clear::Point,
                    band,
                    |mid| self.restart(mid, band),
                )? {
                    (Clearance::Clear, _) => return Ok(OnCarrier::Off),
                    (_, Some(Some(on))) => return Ok(on),
                    (Clearance::Near(near), _) => near,
                }
            }
        };
        match (clear, gap) {
            (
                Ok(Decided {
                    sign: Sign::Positive,
                    ..
                }),
                _,
            ) => Ok(OnCarrier::Off),
            (_, Err(diag)) | (Err(diag), _) => Err(diag.into()),
            (Ok(low), Ok(high)) => Err(CarrierRefusal::Escalated(CarrierEscalation::Straddle(
                straddle("curve_carrier_straddle", low, high, band),
            ))),
        }
    }
}

impl<T: Decide> Foot<'_, T> {
    /// **The polish restarted from a piece the Off's walk keeps** (module
    /// docs, "Where the walk keeps a piece"): from `mid`, the kept piece's
    /// middle, held to the carrier's domain, `On` where its gap decides
    /// zero (`curve_carrier_gap`). A gap that does not is no answer: its
    /// reading is superseded, since the walk's decides.
    fn restart(&self, mid: T, band: Band) -> Option<OnCarrier<T>> {
        let c = self.carrier;
        // Held to the domain as the whole carrier's foot is: a periodic
        // cubic's runs on past its turn.
        let hold = c.as_periodic().is_none().then(|| c.domain());
        let foot = c.polish(self.q, c.foot_at(self.q, mid), hold);
        let (gap, record) = geom_core::k_stats::detached(|| {
            decide_reported("curve_carrier_gap", Margin::of(foot.gap), band)
        });
        if let Ok(Decided {
            sign: Sign::Zero,
            margin,
        }) = gap
        {
            geom_core::k_stats::splice(record);
            return Some(OnCarrier::On { t: foot.t, margin });
        }
        geom_core::k_stats::splice_superseded(record);
        None
    }
}

/// What [`Curve3::clear_of`] finds of a point against an interval of the
/// carrier.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Clearance {
    /// Every piece of the interval is decided past the band from the
    /// point: no point of the interval lies within the band of it.
    Clear,
    /// The first piece, in parameter order, whose reach is within the
    /// band and that is not decided clear: its clearance's reading (zero,
    /// negative, or the escalation).
    Near(Result<Decided, Indeterminate>),
}

/// Where [`Foot::on_carrier`] places a point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OnCarrier<T> {
    /// Within the band of the carrier at the foot `t`; `margin` is the
    /// gap reading's.
    On {
        /// The point's parameter: the foot's, certified by a gap in the
        /// band's zero. A closed carrier's is not reduced to any branch.
        t: T,
        /// The gap reading's margin.
        margin: MarginDiag,
    },
    /// Definitely past the band from the whole carrier.
    Off,
}

/// Why [`Foot::on_carrier`] gives no answer: the band's escalation, or a
/// spline's walk that could not run. The two are one fact about the
/// refusal, so a consumer sorts them by these two arms
/// ([`Self::escalation`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CarrierRefusal {
    /// The band's escalation.
    Escalated(CarrierEscalation),
    /// A spline's piece walk could not run: no closed-form carrier
    /// walks. It stands as the walk's refusal does
    /// ([`PieceWalkRefusal::refusal`]).
    Unwalked(PieceWalkRefusal),
}

/// How the band escalated a carrier reading ([`CarrierRefusal::Escalated`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CarrierEscalation {
    /// A decision the answer rests on fell in the band's gap.
    InBand(Indeterminate),
    /// The lower bound on the band's zero side and the gap past it
    /// straddle the whole band (`curve_carrier_straddle`): two decided
    /// readings of one distance that no margin of either says.
    Straddle(Indeterminate),
}

impl CarrierEscalation {
    /// The diagnostics, either way.
    #[must_use]
    pub const fn diag(self) -> Indeterminate {
        match self {
            Self::InBand(diag) | Self::Straddle(diag) => diag,
        }
    }
}

/// Why a spline's piece walk could not run ([`CarrierRefusal::Unwalked`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(any(test, feature = "test-support"), derive(strum::VariantArray))]
pub enum PieceWalkRefusal {
    /// The walk ran past its budget.
    Exhausted,
    /// The spline's greatest speed is zero or poison: its pieces have no
    /// reach to clear by.
    Unbounded,
}

impl PieceWalkRefusal {
    /// **This refusal's standing**, stated once for every reader of one
    /// ([`geom_core::ladder::Refusal`]): a walk past its budget is
    /// unsettled (no margin, and a coarser band may settle it); a spline
    /// with no speed bound is a limit no tolerance moves.
    #[must_use]
    pub const fn refusal(self) -> geom_core::ladder::Refusal {
        match self {
            Self::Exhausted => geom_core::ladder::Refusal::Unsettled,
            Self::Unbounded => geom_core::ladder::Refusal::Limit,
        }
    }

    /// **The recourse every rendering of this refusal ends with**: a walk
    /// past its budget is a kernel approximation limit, whose last resort
    /// is the tolerance; a spline with no speed bound is a carrier no walk
    /// reads yet. It is the standing's own ending ([`Self::refusal`],
    /// [`geom_core::ladder::Refusal::recourse`]), so the two cannot part.
    #[must_use]
    pub fn recourse(self) -> std::borrow::Cow<'static, str> {
        self.refusal().recourse().render()
    }
}

impl PieceWalkRefusal {
    /// **The one sentence this refusal is rendered by**, for a site that
    /// carries a `&'static str` ([`core::fmt::Display`] writes it too).
    #[must_use]
    pub const fn sentence(self) -> &'static str {
        match self {
            Self::Exhausted => "a spline's piece walk ran past its budget",
            Self::Unbounded => "a spline's speed bound is zero or poison",
        }
    }
}

impl core::fmt::Display for PieceWalkRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.sentence())
    }
}

impl CarrierRefusal {
    /// **The band's escalation this refusal is**, its diagnostics
    /// ([`CarrierEscalation::diag`]); `Err` where a spline's walk could not
    /// run.
    ///
    /// # Errors
    ///
    /// [`PieceWalkRefusal`]: the refusal is no escalation.
    pub fn escalation(self) -> Result<Indeterminate, PieceWalkRefusal> {
        match self {
            Self::Escalated(how) => Ok(how.diag()),
            Self::Unwalked(why) => Err(why),
        }
    }
}

impl From<Indeterminate> for CarrierRefusal {
    fn from(diag: Indeterminate) -> Self {
        Self::Escalated(CarrierEscalation::InBand(diag))
    }
}

impl From<PieceWalkRefusal> for CarrierRefusal {
    fn from(why: PieceWalkRefusal) -> Self {
        Self::Unwalked(why)
    }
}

/// A carrier kind whose foot is its nearest point, found in closed form
/// ([`Curve3::closed_form`]).
enum ClosedForm<'c, T: Real> {
    /// A line.
    Line(&'c crate::Line<T>),
    /// A circle.
    Circle(&'c crate::Circle<T>),
}

/// What a [`Curve3::point_walk`] visit makes of a piece.
pub enum PieceStep<R> {
    /// Nothing more to read in it.
    Settled,
    /// Read its two halves, the earlier first.
    Halve,
    /// The walk's question is answered, by `R`: stop.
    Stop(R),
}

/// A piece of a [`Curve3::point_walk`], as its visit reads it.
#[derive(Clone, Copy, Debug)]
pub struct PieceRead<T> {
    /// The piece's parameter run.
    pub span: (T, T),
    /// Its middle parameter ([`crate::mid_param`]).
    pub mid: T,
    /// The distance from the walk's point to the carrier's point at
    /// `mid`.
    pub gap: T,
    /// The most metres a point of the piece lies from that point
    /// ([`geom_core::bisect::reach`] at the walk's speed bound), so
    /// `gap − reach` is a floor on the piece's distance from the point.
    pub reach: T,
}

impl<T: Decide> Curve3<T> {
    /// **The point-against-span piece walk**: `(t0, t1)` halved depth
    /// first ([`geom_core::bisect`]), each piece read against `q` as
    /// [`PieceRead`], with `hi` a bound on the carrier's speed. A point's
    /// distance from an interval of a line or a circle is closed-form
    /// ([`Self::distance_within`]); the walk is for what that does not
    /// answer, a kind with no closed form or a question other than the
    /// distance. What a reading settles is `visit`'s, and every caller of
    /// the walk is such a reading ([`Self::clear_of`] among them).
    ///
    /// # Errors
    ///
    /// [`geom_core::bisect::Exhausted`] past the piece budget.
    pub fn point_walk<R>(
        &self,
        q: Point3<T>,
        (t0, t1): (T, T),
        hi: SupSpeed<T>,
        mut visit: impl FnMut(PieceRead<T>) -> PieceStep<R>,
    ) -> Result<Option<R>, geom_core::bisect::Exhausted> {
        bisect([(t0, t1)], |(a, b)| {
            let mid = crate::mid_param(a, b);
            let read = PieceRead {
                span: (a, b),
                mid,
                gap: (self.eval(mid) - q).norm(),
                reach: geom_core::bisect::reach(hi, (a, b)),
            };
            match visit(read) {
                PieceStep::Settled => Step::Settled,
                PieceStep::Halve => Step::Halve([(a, mid), (mid, b)]),
                PieceStep::Stop(answer) => Step::Stop(answer),
            }
        })
    }

    /// **Whether a point's foot on the carrier is its nearest point of
    /// the whole carrier** (module docs): on a line and a circle, where
    /// it is found in closed form. The one list of those kinds, matched
    /// exhaustively so a new kind decides here ([`Self::closed_form`]).
    fn foot_is_nearest(&self) -> bool {
        self.closed_form().is_some()
    }

    /// **The kinds whose foot is their nearest point, found in closed
    /// form** (a line, a circle): the one list [`Self::foot_is_nearest`]
    /// and [`Self::distance_within`] read, matched exhaustively so a new
    /// kind decides here.
    fn closed_form(&self) -> Option<ClosedForm<'_, T>> {
        match self {
            Self::Line(line) => Some(ClosedForm::Line(line)),
            Self::Circle(circle) => Some(ClosedForm::Circle(circle)),
            Self::Ellipse(_) | Self::Spiric { .. } | Self::Nurbs(_) | Self::Periodic(_) => None,
        }
    }

    /// **The parameter of `q`'s nearest point of the whole carrier**,
    /// where the kind makes the foot that point: [`Self::param_near`] on
    /// the kinds it is nearest on (a line, a circle), the same parameter
    /// as [`Foot::nearest`]. `None` on every other kind.
    pub fn nearest(&self, q: Point3<T>, near: T) -> Option<T> {
        if !self.foot_is_nearest() {
            return None;
        }
        self.param_near(q, near)
    }

    /// **`q`'s foot on the whole carrier** (module docs): [`Self::foot_within`]
    /// over the whole carrier, held to nothing but a spline's domain.
    /// `near` is a seed: the parameter a consumer already holds for `q`.
    pub fn foot(&self, q: Point3<T>, near: T) -> Foot<'_, T> {
        self.foot_held(q, None, near)
    }

    /// **`q`'s foot over the interval `(t0, t1)` of the carrier** (module
    /// docs): on a line and a circle the whole carrier's nearest point,
    /// in closed form, whose place against the interval says whether the
    /// interval's nearest point is it or an end; on any other kind
    /// polished from `near` clamped into the interval, each iterate held
    /// to it. The foot is no farther from `q` than the interval's own
    /// ends: an end nearer than the polish reached is polished in turn.
    /// A periodic cubic's ([`Curve3::Periodic`]) is held to the interval
    /// like any other spline's
    /// (`rows::a_periodic_cubics_foot_over_a_span_is_held_to_it`); its
    /// whole-carrier foot ([`Self::foot`]) is free, held to nothing, since
    /// the loop runs on past its base turn
    /// (`rows::a_periodic_cubics_whole_carrier_foot_runs_on_past_its_turn`).
    /// No sample is read: a poor foot costs an escalation, never a
    /// verdict, since `On` rests on its gap and `Off` on a lower bound on
    /// the distance from the whole carrier.
    pub fn foot_within(&self, q: Point3<T>, (t0, t1): (T, T), near: T) -> Foot<'_, T> {
        self.foot_held(q, Some((t0.min(t1), t0.max(t1))), near)
    }

    /// [`Self::foot`] and [`Self::foot_within`]'s one search, `hold` the
    /// interval the foot is held to, `None` for the whole carrier: a
    /// clamped spline's whole carrier is its domain, and a periodic cubic's
    /// runs on past its turn, held to nothing. A nearest kind's foot is
    /// held to nothing either: clamped, a circle's would name an end where
    /// a box of the point's angle lies astride the arc's gap, which only
    /// the place reads, by the copy a turn away. Over an interval the seed
    /// is read on the branch about its middle ([`Self::branch_near`]) and
    /// clamped into it.
    fn foot_held(&self, q: Point3<T>, hold: Option<(T, T)>, near: T) -> Foot<'_, T> {
        let into = |t: T, (lo, hi): (T, T)| {
            self.branch_near(t, crate::mid_param(lo, hi))
                .max(lo)
                .min(hi)
        };
        if let Some(net) = self.whole_net() {
            let hold = hold.or_else(|| {
                self.as_periodic()
                    .is_none()
                    .then(|| net.knot_values().domain())
            });
            let Some(hold) = hold else {
                return self.polish(q, self.foot_at(q, near), None);
            };
            return self.held(q, into(near, hold), hold);
        }
        let Some(seed) = self.param_near(q, near) else {
            unreachable!("only a spline reads no parameter near a point")
        };
        if self.foot_is_nearest() {
            return self.foot_at(q, seed);
        }
        let Some(hold) = hold else {
            return self.polish(q, self.foot_at(q, seed), None);
        };
        self.held(q, into(seed, hold), hold)
    }

    /// **The foot held to `(lo, hi)`**, no farther from `q` than the
    /// interval's own ends: polished from `seed`, and from an end whose
    /// gap is decided smaller than the polished foot's. A seed clamped
    /// onto an end where the distance grows into the interval does not
    /// move, though the other end be nearer
    /// (`rows::a_foot_over_an_interval_is_no_farther_than_its_ends`); an
    /// end that wins is polished, held, so a nearer stationary point by it
    /// is found.
    fn held<'c>(&'c self, q: Point3<T>, seed: T, (lo, hi): (T, T)) -> Foot<'c, T> {
        let mut best = self.polish(q, self.foot_at(q, seed), Some((lo, hi)));
        for end in [lo, hi] {
            let at = self.foot_at(q, end);
            if smaller(at.gap, best.gap) {
                best = self.polish(q, at, Some((lo, hi)));
            }
        }
        best
    }

    /// The foot at `t` as read, unpolished.
    fn foot_at(&self, q: Point3<T>, t: T) -> Foot<'_, T> {
        Foot {
            carrier: self,
            q,
            t,
            gap: (q - self.eval(t)).norm(),
        }
    }

    /// Gauss–Newton from `from`, each step kept only where it decides the
    /// gap smaller, held to `domain` where there is one. Each iterate is a
    /// point where the scalar can hold a wider value (module docs).
    fn polish<'c>(
        &'c self,
        q: Point3<T>,
        from: Foot<'c, T>,
        domain: Option<(T, T)>,
    ) -> Foot<'c, T> {
        let mut best = from;
        for _ in 0..POLISH_STEPS {
            let (p, d) = self.ders1(best.t);
            let mut t = best.t - (p - q).dot(d) / d.dot(d);
            if let Some((lo, hi)) = domain {
                t = t.max(lo).min(hi);
            }
            if t.point_value().is_none()
                && let Some((lo, hi)) = t.structure_bracket()
            {
                t = T::from_computed(crate::mid_param(lo, hi));
            }
            let next = self.foot_at(q, t);
            if !smaller(next.gap, best.gap) {
                break;
            }
            best = next;
        }
        best
    }

    /// **Whether `q` lies on the carrier**, read at its foot seeded at
    /// `near` ([`Self::foot`], [`Foot::on_carrier`]).
    ///
    /// # Errors
    ///
    /// [`Foot::on_carrier`]'s.
    pub fn on_carrier(
        &self,
        q: Point3<T>,
        near: T,
        band: Band,
    ) -> Result<OnCarrier<T>, CarrierRefusal> {
        self.foot(q, near).on_carrier(band)
    }

    /// **A lower bound on `q`'s distance from the whole carrier**, in
    /// closed form (module docs); `None` on a spline, whose bound is its
    /// piece walk ([`Self::clear_of`]).
    ///
    /// A line reads [`crate::Line::distance`], whose `dir` must be nonzero, as an
    /// axis's must for the conics' arms. That is the carrier's, decided
    /// nowhere here: at rest `topo`'s tier-3 check 1 refuses a zero
    /// direction, and a section's line is minted along a stored axis or a
    /// cone's ruling, neither of them zero.
    pub fn clearance(&self, q: Point3<T>) -> Option<T> {
        Some(match *self {
            Self::Line(line) => line.distance(q),
            Self::Circle(c) => {
                let (h, rho) = c.frame.axis_line().height_and_distance(q);
                (h.powi(2) + (rho - c.radius).powi(2)).sqrt()
            }
            Self::Ellipse(e) => {
                let lower =
                    ellipse_level_clearance(e.frame.in_plane(q), (e.semi_u, e.semi_v)).abs();
                (lower.powi(2) + e.frame.height(q).powi(2)).sqrt()
            }
            Self::Spiric {
                center,
                axis,
                u_ref,
                major_radius,
                minor_radius,
                offset,
            } => {
                let w = q - center;
                let plane = (w.dot(u_ref) - offset).abs();
                let (h, rho) = crate::Line {
                    origin: center,
                    dir: axis,
                }
                .height_and_distance(q);
                let (major, minor) = (major_radius.abs(), minor_radius.abs());
                let torus = ((rho - major).powi(2) + h.powi(2)).sqrt() - minor;
                let side =
                    ((major - minor).powi(2) - offset.powi(2)).sqrt() - w.dot(axis.cross(u_ref));
                plane.max(torus.abs()).max(side)
            }
            Self::Nurbs(_) | Self::Periodic(_) => return None,
        })
    }

    /// **`q`'s distance from the interval `(t0, t1)` of the carrier**, in
    /// closed form on the kinds that have one ([`Self::closed_form`]): a
    /// line's span ([`crate::Line::distance_within`]), and a circle's arc.
    /// `None` on every other kind, whose reading of a point against an
    /// interval is the piece walk ([`Self::point_walk`]).
    ///
    /// On a circle, the arc's nearest point to `q` is `q`'s foot on the
    /// whole circle where that foot lies in the arc, and the nearer end
    /// otherwise; the foot's distance is the whole circle's
    /// [`Self::clearance`]. The foot lies in the arc where the angle
    /// between `q`'s offset in the circle's plane, `v`, and the radius to
    /// the arc's middle, `m`, is at most half the span, held to half a
    /// turn: `|v|·|m|·cos(half) − v·m ≤ 0`. That test reads no azimuth and
    /// takes no foot, so it asks nothing ([`select_le_zero`] is no
    /// decision); a cosine has no cut, so no angle is named a turn away;
    /// and on the axis, where `v` is zero, it reads the clearance, which is
    /// every point's distance there. An arc of a turn or more is all
    /// inside.
    ///
    /// At `Interval`, a box decided inside reads the clearance, each of its
    /// points' distance, and one decided outside reads the nearer end,
    /// each point's nearest; where the test straddles, the reading hulls
    /// both, its low end the lower. So the reading encloses the distance
    /// of every point of a box, and no comparison is taken.
    ///
    /// [`select_le_zero`]: geom_core::Real::select_le_zero
    pub fn distance_within(&self, q: Point3<T>, (t0, t1): (T, T)) -> Option<T> {
        let (lo, hi) = (t0.min(t1), t0.max(t1));
        Some(match self.closed_form()? {
            ClosedForm::Line(line) => line.distance_within(q, (lo, hi)),
            ClosedForm::Circle(c) => {
                let (o, axis) = (c.frame.origin, c.frame.axis);
                let w = q - o;
                let v = w - axis * w.dot(axis);
                let m = self.eval(crate::mid_param(lo, hi)) - o;
                let half = ((hi - lo) * T::from_f64(0.5)).min(T::pi());
                let outside = v.norm() * m.norm() * half.cos() - v.dot(m);
                let ends = (q - self.eval(lo)).norm().min((q - self.eval(hi)).norm());
                outside.select_le_zero(self.clearance(q)?, ends)
            }
        })
    }

    /// The parameter range a whole-carrier walk covers: a spline's
    /// domain, a repeating kind's one period ([`Curve3::period`], which
    /// states which kinds repeat), a line's nothing (a line's clearance is
    /// closed-form).
    fn domain(&self) -> (T, T) {
        match (self, self.period()) {
            (Self::Nurbs(n), _) => n.knot_values().domain(),
            (Self::Periodic(p), _) => p.base_span_in(),
            (_, Some(period)) => (T::zero(), period.length()),
            (_, None) => (T::zero(), T::zero()),
        }
    }

    /// **Whether `q` is clear of the interval `(t0, t1)` of the carrier**
    /// by the certified piece walk (module docs): every piece's middle
    /// point's distance less its reach decides positive
    /// (`curve_carrier_piece_clear`); `Near` at the first piece that does
    /// not, once its reach is within the band (`curve_carrier_piece`). A
    /// lower bound on `q`'s distance from the interval, past the band,
    /// where it is `Clear`.
    ///
    /// Every reading of a point against a span by pieces whose reach is
    /// the carrier's speed bound is a [`Self::point_walk`] whose visit is
    /// its stop rule, and its callers are that list. A line's or a
    /// circle's distance is closed-form ([`Self::distance_within`]), and
    /// `topo`'s `pieced_arc` reads its own tighter ball cover. This one
    /// decides against the band, logged, and returns the deciding reading,
    /// so it stops on a piece's reach.
    ///
    /// # Errors
    ///
    /// [`PieceWalkRefusal::Exhausted`] past the piece budget;
    /// [`PieceWalkRefusal::Unbounded`] where the carrier's greatest speed is
    /// zero or poison.
    pub fn clear_of(
        &self,
        q: Point3<T>,
        span: (T, T),
        band: Band,
    ) -> Result<Clearance, CarrierRefusal> {
        Ok(self.walk_kept(q, span, Clear::Point, band, |_| ())?.0)
    }

    /// **Whether the ball of `radius` metres about `q` is clear of the
    /// interval `(t0, t1)` of the carrier**: [`Self::clear_of`]'s walk,
    /// each piece's middle point's distance less its reach less `radius`
    /// deciding positive (`curve_carrier_piece_clear_by`). A lower bound,
    /// past the band, on the distance from the ball to the interval, in
    /// metres, where it is `Clear`. `Near` at the first piece that does
    /// not clear once its reach is within the band, where some point of
    /// the interval may lie within the ball.
    ///
    /// # Errors
    ///
    /// As [`Self::clear_of`].
    pub fn clear_by(
        &self,
        q: Point3<T>,
        span: (T, T),
        radius: T,
        band: Band,
    ) -> Result<Clearance, CarrierRefusal> {
        Ok(self
            .walk_kept(q, span, Clear::Ball(radius), band, |_| ())?
            .0)
    }

    /// [`Self::clear_of`]'s walk, `kept` read once, at the middle of the
    /// first piece the walk keeps (its reach within the band and not
    /// decided clear, in parameter order), where the walk stops: no later
    /// piece can make the point off. The piece it keeps is where on the
    /// interval the carrier can come within the band of `q` (module docs,
    /// "Where the walk keeps a piece"); `None` where every piece clears.
    ///
    /// # Errors
    ///
    /// As [`Self::clear_of`].
    fn walk_kept<R>(
        &self,
        q: Point3<T>,
        (t0, t1): (T, T),
        clear: Clear<T>,
        band: Band,
        kept: impl FnOnce(T) -> R,
    ) -> Result<(Clearance, Option<R>), CarrierRefusal> {
        let hi = crate::carrier_speed::speed_range(self).1;
        if !geom_core::is_finite_length(hi.get())
            || geom_core::k_stats::prove("curve_carrier_speed", Margin::of(hi.get()))
                != Ok(Sign::Positive)
        {
            return Err(PieceWalkRefusal::Unbounded.into());
        }
        let near = self
            .point_walk(q, (t0, t1), hi, |p| {
                let clear = match clear {
                    Clear::Point => decide_reported(
                        "curve_carrier_piece_clear",
                        Margin::of(p.gap - p.reach),
                        band,
                    ),
                    Clear::Ball(radius) => decide_reported(
                        "curve_carrier_piece_clear_by",
                        Margin::of(p.gap - p.reach - radius),
                        band,
                    ),
                };
                if matches!(
                    clear,
                    Ok(Decided {
                        sign: Sign::Positive,
                        ..
                    })
                ) {
                    return PieceStep::Settled;
                }
                if matches!(
                    decide("curve_carrier_piece", Margin::of(p.reach), band),
                    Ok(Sign::Negative | Sign::Zero)
                ) {
                    return PieceStep::Stop((p.mid, clear));
                }
                PieceStep::Halve
            })
            .map_err(|_| CarrierRefusal::from(PieceWalkRefusal::Exhausted))?;
        Ok(match near {
            Some((mid, clear)) => (Clearance::Near(clear), Some(kept(mid))),
            None => (Clearance::Clear, None),
        })
    }
}

/// What [`Curve3::walk_kept`]'s pieces clear: the point itself
/// ([`Curve3::clear_of`]), or the ball of a radius about it
/// ([`Curve3::clear_by`]).
#[derive(Clone, Copy)]
enum Clear<T> {
    Point,
    Ball(T),
}

/// **A lower bound on an in-plane point's distance from an ellipse,
/// signed as the ellipse's level** (module docs): `(x, y)` the point in
/// the ellipse's frame, `(a, b)` its semi-axes, and the bound
/// `2F/(g + √(g² + 4|F|/s²))`, `F = x²/a² + y²/b² − 1`, `g = |∇F|`, `s` the
/// smaller semi-axis. Negative inside the ellipse, positive outside, zero
/// exactly on it; its magnitude is [`Curve3::clearance`]'s in the plane.
pub fn ellipse_level_clearance<T: Decide>((x, y): (T, T), (a, b): (T, T)) -> T {
    let two = T::from_f64(2.0);
    let (u, v) = (x / a, y / b);
    let f = u.powi(2) + v.powi(2) - T::one();
    let g2 = (two * u / a).powi(2) + (two * v / b).powi(2);
    let small = a.min(b);
    two * f / (g2.sqrt() + (g2 + T::from_f64(4.0) * f.abs() / small.powi(2)).sqrt())
}

/// Whether `a` is decided smaller than `b`: a heuristic's choice
/// ([`geom_core::k_stats::choose`], ledger row F32). **At a tie the
/// candidate already held is kept** (`a` is not smaller): sound because
/// the pick can lie on another branch of a spline, and no reader rests on
/// which, since a foot's parameter is read only where something
/// certifies it ([`OnCarrier::On`], [`Foot::nearest`]).
fn smaller<T: Decide>(a: T, b: T) -> bool {
    match choose("curve_foot_polish", Margin::of(b - a), "F32") {
        Pick::Positive => true,
        Pick::Negative | Pick::Tie => false,
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod rows;
