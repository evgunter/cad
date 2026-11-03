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
//! - a spline: the best of the caller's seed and a few samples on each
//!   knot span, then Gauss–Newton, held to the domain; and Gauss–Newton
//!   from each end of the domain, the smallest gap kept. A closed
//!   spline's two ends are one point, so a descent held at one end
//!   continues from the other.
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
//! as found, for a further search, and certifies nothing. So no reader
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
//!   [`crate::carrier_speed`]) decides positive (`curve_carrier_piece_clear`),
//!   and stops once its reach is within the band
//!   (`curve_carrier_piece`). The point is off only where every piece
//!   clears. A walk past [`geom_core::bisect::PIECE_BUDGET`] refuses
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
    /// A piece within the band's reach of the point is not decided clear:
    /// its clearance's reading (zero, negative, or the escalation).
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
    /// where the kind makes the foot that point: [`Foot::nearest`] of the
    /// foot seeded at `near`, asked of the kind before the foot is found,
    /// so a kind with none searches nothing. `None` on every other kind.
    pub fn nearest(&self, q: Point3<T>, near: T) -> Option<T> {
        if !self.foot_is_nearest() {
            return None;
        }
        self.foot(q, near).nearest()
    }

    /// **`q`'s foot on the carrier** (module docs). `near` is a seed: the
    /// parameter a consumer already holds for `q`, or any parameter of
    /// the interval it reads.
    pub fn foot(&self, q: Point3<T>, near: T) -> Foot<'_, T> {
        let seed = match self {
            Self::Line(line) => line.param(q),
            Self::Circle(c) => {
                let (x, y) = c.frame.in_plane(q);
                return self.foot_at(q, angle_near(x, y, near));
            }
            Self::Ellipse(e) => {
                let (x, y) = crate::Conic::unit_in_plane(e, q);
                angle_near(x, y, near)
            }
            Self::Spiric {
                center,
                axis,
                major_radius,
                ..
            } => {
                // The torus's minor angle about the tube's centre circle,
                // as [`Curve3::param_near`] reads it.
                let w = q - *center;
                let h = w.dot(*axis);
                let rho = (w - *axis * h).norm();
                angle_near(rho - *major_radius, h, near)
            }
            Self::Nurbs(n) => {
                let knots = n.knot_values();
                let (lo, hi) = knots.domain();
                let breaks: Vec<T> = knots.runs().map(|(k, _)| k).collect();
                let mut best = self.foot_at(q, near.max(lo).min(hi));
                for k in breaks.windows(2) {
                    for s in SPLINE_SAMPLES {
                        let f = self.foot_at(q, k[0] + (k[1] - k[0]) * T::from_f64(s));
                        if smaller(f.gap, best.gap) {
                            best = f;
                        }
                    }
                }
                let domain = Some((lo, hi));
                let mut foot = self.polish(q, best, domain);
                for end in [lo, hi] {
                    let f = self.polish(q, self.foot_at(q, end), domain);
                    if smaller(f.gap, foot.gap) {
                        foot = f;
                    }
                }
                return foot;
            }
        };
        let at = self.foot_at(q, seed);
        match self {
            Self::Line(_) => at,
            _ => self.polish(q, at, None),
        }
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
            Self::Line(crate::Line { origin, dir }) => {
                let w = q - origin;
                let u = dir * (T::one() / dir.norm());
                (w - u * w.dot(u)).norm()
            }
            Self::Circle(c) => {
                let (h, rho) = height_and_radius(q - c.frame.center, c.frame.axis);
                (h.powi(2) + (rho - c.radius).powi(2)).sqrt()
            }
            Self::Ellipse(e) => {
                let lower =
                    ellipse_level_clearance(e.frame.in_plane(q), (e.semi_u, e.semi_v)).abs();
                (lower.powi(2) + (q - e.frame.center).dot(e.frame.axis).powi(2)).sqrt()
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
    /// (`curve_carrier_piece_clear`), a piece stopping once its reach is
    /// within the band (`curve_carrier_piece`). A lower bound on `q`'s
    /// distance from the interval, past the band, where it is `Clear`.
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
        let mut kept: Option<Result<Decided, Indeterminate>> = None;
        self.point_walk(
            q,
            (t0, t1),
            hi,
            |p| -> PieceStep<core::convert::Infallible> {
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
                    kept = kept.or(Some(clear));
                    return PieceStep::Settled;
                }
                PieceStep::Halve
            },
        )
        .map_err(|_| CarrierRefusal::from(PieceWalkRefusal::Exhausted))?;
        Ok(kept.map_or(Clearance::Clear, Clearance::Near))
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

/// **The angle of `(x, y)`, read about `near`**: relative to `near`'s
/// direction, or to its opposite where the point lies definitely behind
/// it, so the angle's own branch cut is never crossed and an enclosure of
/// the point's direction stays an enclosure of one angle. The result lies
/// within a turn and a half of `near` (half a turn of `near` or of
/// `near + π`); a reader that needs a branch chooses it. The flip is
/// decided outside the log (ledger row F31): where `along` is undecided
/// the point lies a quarter turn from `near`, clear of both formulas'
/// cuts, or at the centre, where `across` is undecided too and both
/// formulas read an enclosure equally wide; no foot rests on the outcome.
fn angle_near<T: Decide>(x: T, y: T, near: T) -> T {
    let (sin, cos) = near.sin_cos();
    let (along, across) = (x * cos + y * sin, y * cos - x * sin);
    let behind =
        check_unlogged("curve_foot_branch", along, Band::least(), "F31") == Ok(Sign::Negative);
    if behind {
        near + T::pi() + (T::zero() - across).atan2(T::zero() - along)
    } else {
        near + across.atan2(along)
    }
}

/// `w`'s height along `axis` and its distance from the axis.
fn height_and_radius<T: Decide>(w: Vec3<T>, axis: Vec3<T>) -> (T, T) {
    let h = w.dot(axis);
    (h, (w - axis * h).norm())
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
