//! **A point against one carrier**: its foot ([`Curve3::foot`]) and
//! whether it lies on the carrier ([`Foot::on_carrier`]). These are the
//! one-carrier readings of a point on a curve (D3), homed in `geom`
//! beside the carrier's evaluators and its other banded decisions.
//!
//! **The foot** is a parameter of the carrier near the point's closest
//! point, held with the point and the carrier it was found for, and its
//! gap `|q − C(t)|`, which bounds the point's distance from the carrier
//! from above. Only [`Curve3::foot`] makes one, so a foot cannot be read
//! against another point or another carrier. Where it is found:
//!
//! - a line: in closed form;
//! - a circle: the angle of the point about the centre, exact, the
//!   radius's sign read into it;
//! - an ellipse: the angle of the point's unit-coordinate image, then
//!   Gauss–Newton. The unit-coordinate angle alone is not a foot: its
//!   point lies up to `(1 + a/b)·d` from a point `d` off the ellipse;
//! - a spiric oval: [`Curve3::param_near`]'s angle, then Gauss–Newton;
//! - a spline: Gauss–Newton, held to the domain, from the caller's seed
//!   and from some of a few samples on each knot span and the domain's
//!   ends, the smallest gap kept. The samples polished from are the
//!   ends, each local minimum of the sampled gaps, and the few least
//!   gaps, so a branch whose nearest sample sits beside a nearer one of
//!   another branch is still descended. A closed spline's two ends are
//!   one point, so a descent held at one end continues from the other.
//!
//! A closed kind's angle is read relative to the seed's direction, or to
//! its opposite where the point lies definitely behind it, so an
//! enclosure of the point's direction never crosses the angle's own cut.
//! Each polish step is kept only where it decides the gap smaller, so
//! the foot's gap is never worse than its seed's.
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
//! The lower bound on the band's zero side and the gap past it straddle
//! the whole band (`curve_carrier_straddle`): only a carrier bending
//! tighter than the band resolves, or a poor foot, allows it, and it
//! escalates.

use geom_core::bisect::{Step, bisect};
use geom_core::k_stats::{check_unlogged, decide, decide_reported, straddle};
use geom_core::{
    Band, Decide, Decided, Indeterminate, Margin, MarginDiag, Point3, Real, Sign, SupSpeed, Vec3,
};

use super::Curve3;

/// The Gauss–Newton steps a foot is polished by.
const POLISH_STEPS: usize = 6;

/// How many of a spline's least-gap samples its foot search polishes
/// from, beside each local minimum of the sampled gaps.
const LEAST_SAMPLES: usize = 4;

/// Where a spline's foot search samples each knot span, as fractions of
/// it.
const SPLINE_SAMPLES: [f64; 4] = [0.125, 0.375, 0.625, 0.875];

/// A point's foot on a carrier, made only by [`Curve3::foot`], holding
/// the point and the carrier it was found for (module docs).
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
            None => match self.carrier.clear_of(self.q, self.carrier.domain(), band)? {
                Clearance::Clear => return Ok(OnCarrier::Off),
                Clearance::Near(kept) => kept,
            },
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
    /// walks.
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
pub enum PieceWalkRefusal {
    /// The walk ran past its budget.
    Exhausted,
    /// The spline's greatest speed is zero or poison: its pieces have no
    /// reach to clear by.
    Unbounded,
}

impl PieceWalkRefusal {
    /// **The recourse every rendering of this refusal ends with**: a walk
    /// past its budget is a kernel approximation limit, whose last resort
    /// is the tolerance; a spline with no speed bound is a carrier no walk
    /// reads yet.
    #[must_use]
    pub const fn recourse(self) -> &'static str {
        match self {
            Self::Exhausted => geom_core::KERNEL_LIMIT_RECOURSE,
            Self::Unbounded => geom_core::NOT_YET_ENDING,
        }
    }
}

impl core::fmt::Display for PieceWalkRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(match self {
            Self::Exhausted => "a spline's piece walk ran past its budget",
            Self::Unbounded => "a spline's speed bound is zero or poison",
        })
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
    /// [`PieceRead`], with `hi` a bound on the carrier's speed. What a
    /// reading settles is `visit`'s: the one walk behind
    /// [`Self::clear_of`] and every other reading of a point against a
    /// span by its pieces.
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
    /// exhaustively so a new kind decides here.
    fn foot_is_nearest(&self) -> bool {
        match self {
            Self::Line(_) | Self::Circle(_) => true,
            Self::Ellipse(_) | Self::Spiric { .. } | Self::Nurbs(_) => false,
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

    /// **`q`'s foot on the carrier** (module docs). `near` is a seed: the
    /// parameter a consumer already holds for `q`, or any parameter of
    /// the interval it reads.
    pub fn foot(&self, q: Point3<T>, near: T) -> Foot<'_, T> {
        let Self::Nurbs(n) = self else {
            let Some(seed) = self.param_near(q, near) else {
                unreachable!("only a spline reads no parameter near a point")
            };
            let at = self.foot_at(q, seed);
            return if self.foot_is_nearest() {
                at
            } else {
                self.polish(q, at, None)
            };
        };
        let knots = n.knot_values();
        let (lo, hi) = knots.domain();
        let breaks: Vec<T> = knots.runs().map(|(k, _)| k).collect();
        let mut samples = vec![self.foot_at(q, lo)];
        for k in breaks.windows(2) {
            samples.extend(
                SPLINE_SAMPLES.map(|s| self.foot_at(q, k[0] + (k[1] - k[0]) * T::from_f64(s))),
            );
        }
        samples.push(self.foot_at(q, hi));
        let domain = Some((lo, hi));
        let mut foot = self.polish(q, self.foot_at(q, near.max(lo).min(hi)), domain);
        for i in spline_starts(&samples) {
            let f = self.polish(q, samples[i], domain);
            if smaller(f.gap, foot.gap) {
                foot = f;
            }
        }
        foot
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
    /// gap smaller, held to `domain` where there is one.
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
    pub fn clearance(&self, q: Point3<T>) -> Option<T> {
        Some(match *self {
            Self::Line(line) => line.offset(q).norm(),
            Self::Circle(c) => {
                let (h, rho) = height_and_radius(q - c.frame.origin, c.frame.axis);
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
                let (h, rho) = height_and_radius(w, axis);
                let (major, minor) = (major_radius.abs(), minor_radius.abs());
                let torus = ((rho - major).powi(2) + h.powi(2)).sqrt() - minor;
                let side =
                    ((major - minor).powi(2) - offset.powi(2)).sqrt() - w.dot(axis.cross(u_ref));
                plane.max(torus.abs()).max(side)
            }
            Self::Nurbs(_) => return None,
        })
    }

    /// The parameter range a whole-carrier walk covers: a spline's
    /// domain, a closed kind's one turn, a line's nothing (a line's
    /// clearance is closed-form).
    fn domain(&self) -> (T, T) {
        match self {
            Self::Nurbs(n) => n.knot_values().domain(),
            Self::Circle(_) | Self::Ellipse(_) | Self::Spiric { .. } => (T::zero(), T::tau()),
            Self::Line(_) => (T::zero(), T::zero()),
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
    /// # Errors
    ///
    /// [`PieceWalkRefusal::Exhausted`] past the piece budget;
    /// [`PieceWalkRefusal::Unbounded`] where the carrier's greatest speed is
    /// zero or poison.
    pub fn clear_of(
        &self,
        q: Point3<T>,
        (t0, t1): (T, T),
        band: Band,
    ) -> Result<Clearance, CarrierRefusal> {
        let hi = crate::carrier_speed::speed_range(self).1;
        if !geom_core::is_finite_length(hi.get())
            || decide("curve_carrier_speed", Margin::of(hi.get()), Band::least())
                != Ok(Sign::Positive)
        {
            return Err(PieceWalkRefusal::Unbounded.into());
        }
        let near = self
            .point_walk(q, (t0, t1), hi, |p| {
                let clear = decide_reported(
                    "curve_carrier_piece_clear",
                    Margin::of(p.gap - p.reach),
                    band,
                );
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
                    return PieceStep::Stop(clear);
                }
                PieceStep::Halve
            })
            .map_err(|_| CarrierRefusal::from(PieceWalkRefusal::Exhausted))?;
        Ok(near.map_or(Clearance::Clear, Clearance::Near))
    }
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

/// `w`'s height along `axis` and its distance from the axis.
pub(super) fn height_and_radius<T: Decide>(w: Vec3<T>, axis: Vec3<T>) -> (T, T) {
    let h = w.dot(axis);
    (h, (w - axis * h).norm())
}

/// **The samples a spline's foot search polishes from**, by index, in
/// order: both ends of the domain, each local minimum of the sampled
/// gaps, and the [`LEAST_SAMPLES`] least gaps. A local minimum finds a
/// branch whose samples dip toward the point; the least gaps find one
/// whose nearest sample sits beside a nearer one on another branch
/// (a cusp, or a hairpin narrower than the sampling).
fn spline_starts<T: Decide>(samples: &[Foot<'_, T>]) -> Vec<usize> {
    let gap = |i: usize| samples[i].gap;
    let not_below = |i: usize, j: Option<usize>| j.is_none_or(|j| !smaller(gap(j), gap(i)));
    let mut starts: Vec<usize> = (0..samples.len())
        .filter(|&i| {
            i == 0 || i + 1 == samples.len() || {
                not_below(i, i.checked_sub(1)) && not_below(i, Some(i + 1))
            }
        })
        .collect();
    for _ in 0..LEAST_SAMPLES {
        let least = (0..samples.len())
            .filter(|i| !starts.contains(i))
            .reduce(|a, b| if smaller(gap(b), gap(a)) { b } else { a });
        starts.extend(least);
    }
    starts.sort_unstable();
    starts
}

/// Whether `a` is decided smaller than `b`: a heuristic's choice, read
/// on the least band so it takes the sign alone, and decided outside the
/// log (ledger row F32). An undecided pick keeps the candidate already
/// held, which can lie on another branch of a spline; no reader rests on
/// which, since a foot's parameter is read only where something
/// certifies it ([`OnCarrier::On`], [`Foot::nearest`]).
fn smaller<T: Decide>(a: T, b: T) -> bool {
    check_unlogged("curve_foot_polish", b - a, Band::least(), "F32") == Ok(Sign::Positive)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod rows;
