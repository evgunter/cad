//! **A carrier span, its reading of a point against its ends, and the
//! point-on-span reading that composes it with the carrier's.**
//!
//! A [`Span`] is a carrier curve and a parameter interval on it, with no
//! topology attached: the part of the carrier an edge covers.
//!
//! **Two readings, one composition.** Whether a point is on a span is two
//! questions, each read at the point's foot on the carrier
//! ([`geom::Curve3::foot`], found here from the point and a seed, so a
//! foot is never another point's). Whether the point lies on the carrier
//! is the carrier's own reading ([`geom::Foot::on_carrier`]): on by the
//! foot's gap, off only by a lower bound on the distance from the whole
//! carrier. Where the foot sits against the span's ends is this module's
//! reading, [`Span::place`], which decides no carrier membership.
//! [`Span::read`] composes the two, once, and decides nothing of its
//! own; [`Span::contact`] is its answer as a side of a meeting (C5,
//! `crates/geom-brep/README.md`).
//!
//! **The place.** On a closed carrier (a circle, an ellipse, a spiric
//! oval) the foot's parameter is first read on the branch within half a
//! turn of the span's middle. Each end is read by the point's raw chord
//! to the end's own point (`curve_pair_span_end_chord`): zero puts the
//! point at that end ([`Place::End`]), whatever the foot. Past the band,
//! the side of each end is the sign of the foot's parameter distance `m`
//! from it, inside the span positive, metered by the carrier's greatest
//! speed (`curve_pair_span_end_side`). A side that reads zero beside a
//! chord past the band escalates as their straddle
//! (`curve_pair_span_end_straddle`).
//!
//! **Past needs a nearest foot.** That the foot lies past the span says
//! the point lies off it only where the foot is the point's nearest point
//! of the carrier, which it is on a line and a circle
//! ([`geom::Foot::nearest`]). On an ellipse, a spiric oval or a spline
//! the foot is a polished stationary point, perhaps a far branch's, so a
//! `Past` there stands only on a certificate that the point is clear of
//! the span itself: the piece walk over the span's own parameters
//! ([`geom::Curve3::clear_of`]). Where the walk keeps a piece near the
//! point the place escalates (`curve_pair_span_past_straddle`), and where
//! it cannot run it refuses as the walk does.
//!
//! A span whose ends are no vertex — a closed edge whose start is not a
//! vertex of its own, [`Span::endless`] — has no end to be near: every
//! foot is inside it.
//!
//! **The composition** ([`Span::read`]): a point the carrier reads off is
//! off the span ([`Reading::Off`]) wherever its foot sits, so its place
//! is not read. No end can be within the band of a point the whole
//! carrier is past the band from: the chord to an end is at least the
//! distance to the carrier. That holds exactly, and certified at an
//! enclosing scalar; at a point scalar it holds where rounding stays
//! inside the band's gap, which is the session's admissible box (D4 ¶4)
//! and not beyond it (a circle of radius 1e8 read at ε = 1e-9 can read
//! an end point off). And a foot need not be defined to say so: on a
//! circle's axis every parameter is a foot, and the one found is the
//! seed turned by the signs of two zeros ([`geom::Curve3::foot`]), so a
//! place read there would decide sides of a half turn the point does not
//! have. Otherwise, at an end is [`Reading::End`]; inside, the carrier's
//! reading decides, on ([`Reading::On`], at the parameter that reading
//! certifies) or undecided (its refusal);
//! past, the point is off the span, on the carrier
//! ([`Reading::Carrier`]) or not ([`Reading::Off`], an undecided carrier
//! included, since no answer to it puts the point on the span). A place
//! the band cannot read is a refusal: the carrier's where it refused
//! too, else the place's.

use geom::cover::Cover;
use geom::{CarrierEscalation, CarrierRefusal, Clearance, Curve3, Foot, OnCarrier};
use geom_core::k_stats::{self, decide_reported, straddle};
use geom_core::{Band, Decide, Indeterminate, InfSpeed, Margin, MarginDiag, Point3, Real, Sign};
use geom_core::{Decided, SupSpeed};

/// A carrier and a parameter interval on it (module docs).
#[derive(Clone, Copy)]
pub struct Span<'a, T: Real> {
    carrier: &'a Curve3<T>,
    t0: T,
    t1: T,
    /// The carrier's points at `t0` and `t1`, evaluated once.
    ends: (Point3<T>, Point3<T>),
    ended: bool,
    lo: InfSpeed<T>,
    hi: SupSpeed<T>,
}

/// One end of a span.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanEnd {
    /// The end at the span's first parameter.
    Start,
    /// The end at its last.
    Stop,
}

/// Where [`Span::place`] puts a point's foot against the span's ends.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Place {
    /// Inside the span, clear of both its ends.
    Inside,
    /// The point within the band of this end's point.
    End(SpanEnd),
    /// Past the span, clear of both its ends.
    Past,
}

/// Whether a run of the carrier about a point lies past the span
/// ([`Span::past_by`]).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PastBy {
    /// The point is not read past the span, or the span has no ends.
    Not,
    /// Past the span, and the whole run clear of both ends.
    Clear,
    /// Past the span, the run's reach to an end in the band's gap of the
    /// chord to it (`curve_pair_span_past_reach_straddle`), or that reach
    /// itself undecided.
    Near(Indeterminate),
}

/// A point on a span, as [`Span::contact`] reads it: what a meeting holds
/// on each side.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hit<T> {
    /// On the carrier and inside the span.
    On {
        /// The parameter, on the branch about the span's middle.
        t: T,
        /// The on-carrier reading's margin.
        margin: MarginDiag,
    },
    /// Within the band of this end's point.
    End(SpanEnd),
}

/// What [`Span::read`] makes of a point: the two readings composed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reading<T> {
    /// On the carrier and inside the span.
    On {
        /// The parameter, on the branch about the span's middle.
        t: T,
        /// The on-carrier reading's margin.
        margin: MarginDiag,
    },
    /// Within the band of this end's point.
    End(SpanEnd),
    /// On the carrier, past the span and clear of both its ends.
    Carrier,
    /// Off the span: off the carrier, or past the span and not read on
    /// the carrier.
    Off,
}

/// Which of [`Span::read`]'s two readings a [`SpanRefusal`] rests on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanReading {
    /// The place read the foot inside the span, and the carrier's
    /// reading refused.
    Carrier,
    /// The place against the span's ends refused, whatever the carrier
    /// read: the refusal carried is the carrier's where both refused.
    Place,
}

/// **Why [`Span::read`] refused**: the refusal, and the reading the
/// answer rests on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpanRefusal {
    /// The refusal.
    pub refused: CarrierRefusal,
    /// The reading it rests on.
    pub of: SpanReading,
}

impl SpanRefusal {
    /// The band's escalation this refusal is ([`CarrierRefusal::escalation`]).
    ///
    /// # Errors
    ///
    /// [`geom::PieceWalkRefusal`]: the refusal is no escalation.
    pub fn escalation(self) -> Result<Indeterminate, geom::PieceWalkRefusal> {
        self.refused.escalation()
    }
}

impl<T> Reading<T> {
    /// The reading as a side of a meeting, or `None` off the span.
    pub fn hit(self) -> Option<Hit<T>> {
        match self {
            Self::On { t, margin } => Some(Hit::On { t, margin }),
            Self::End(which) => Some(Hit::End(which)),
            Self::Carrier | Self::Off => None,
        }
    }
}

/// The side of one end a foot lies on.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    /// Inside the span, clear of this end.
    Clear,
    /// Outside the span past this end, clear of it.
    Past,
}

impl<'a, T: Decide> Span<'a, T> {
    /// The span of `carrier` over `(t0, t1)`, its rate bounds
    /// ([`geom::carrier_speed::speed_range`]) and its end points read
    /// once.
    pub fn new(carrier: &'a Curve3<T>, (t0, t1): (T, T)) -> Self {
        let (lo, hi) = geom::carrier_speed::speed_range(carrier);
        Self {
            carrier,
            t0,
            t1,
            ends: (carrier.eval(t0), carrier.eval(t1)),
            ended: true,
            lo,
            hi,
        }
    }

    /// The same span with no ends: a closed edge whose start is no vertex
    /// of its own, so its end point is its interior too.
    #[must_use]
    pub fn endless(self) -> Self {
        Self {
            ended: false,
            ..self
        }
    }

    /// Whether the span has ends: `false` for [`Self::endless`].
    pub fn ended(&self) -> bool {
        self.ended
    }

    /// The carrier.
    pub fn carrier(&self) -> &'a Curve3<T> {
        self.carrier
    }

    /// **The parameter interval**: the span's one door to its first and
    /// last parameters, which on an endless span are its seam and no end.
    pub fn params(&self) -> (T, T) {
        (self.t0, self.t1)
    }

    /// **The span's ends**, each its parameter and its point, the start
    /// first: both ends of an ended span, and none of an endless one,
    /// whose first and last parameters are a seam in its interior. The
    /// span's one door to its end points.
    pub fn ends(&self) -> impl Iterator<Item = (T, Point3<T>)> {
        let n = if self.ended { 2 } else { 0 };
        [(self.t0, self.ends.0), (self.t1, self.ends.1)]
            .into_iter()
            .take(n)
    }

    /// `which` end's parameter and point, as [`Self::ends`] holds them,
    /// read only where the span is known to have ends.
    fn end(&self, which: SpanEnd) -> (T, Point3<T>) {
        match which {
            SpanEnd::Start => (self.t0, self.ends.0),
            SpanEnd::Stop => (self.t1, self.ends.1),
        }
    }

    /// Where `hit` sits on the span's carrier: its parameter, or its
    /// end's.
    pub fn at(&self, hit: Hit<T>) -> T {
        match hit {
            Hit::On { t, .. } => t,
            Hit::End(which) => self.end(which).0,
        }
    }

    /// The carrier's least and greatest speed.
    pub fn speeds(&self) -> (InfSpeed<T>, SupSpeed<T>) {
        (self.lo, self.hi)
    }

    /// **Where on the span a meeting with a member can lie**: an
    /// interval of the span's parameters outside which no point of the
    /// span stands within the band's escalation of `member`, the
    /// member's cover. On a line, the cover's range along it
    /// ([`Cover::range_along`]) widened by the escalation either way,
    /// within the span. On any other carrier, the hull of the pieces of
    /// the carrier's piece walk not decided farther than the escalation
    /// from the cover's ball ([`Cover::ball`]); the whole span where the
    /// walk cannot run, or for a member with no cover (`None`).
    pub fn window_near(&self, member: Option<&Cover<T>>, band: Band) -> (T, T) {
        let (lo, hi) = (self.t0.min(self.t1), self.t0.max(self.t1));
        let Some(cover) = member else {
            return (lo, hi);
        };
        let Curve3::Line(geom::Line { origin, dir }) = self.carrier else {
            return self.pieces_near(cover, (lo, hi), band);
        };
        let (near, far) = cover.range_along(*dir);
        let base = (*origin - Point3::origin()).dot(*dir);
        let reach = T::from_f64(band.escalate()) * dir.norm();
        let per = dir.norm_squared();
        let within = |t: T| t.max(lo).min(hi);
        (
            within((near - base - reach) / per),
            within((far - base + reach) / per),
        )
    }

    /// [`Self::window_near`] off a line, by the carrier's piece walk
    /// ([`Curve3::point_walk`]) about the cover's ball's centre: a piece is
    /// cleared where its floor on that distance, less the ball's radius,
    /// is decided past the escalation on an exact order; halved while its
    /// reach exceeds the ball's radius and the escalation; and kept once
    /// its reach is within them, so the window's ends are resolved to the
    /// member's own size, or the escalation for a point. The whole span
    /// where the carrier has no finite speed bound or the walk runs past
    /// its budget.
    fn pieces_near(&self, cover: &Cover<T>, (lo, hi): (T, T), band: Band) -> (T, T) {
        let (centre, radius) = cover.ball();
        let reach = radius + T::from_f64(band.escalate());
        let positive = |row, m: T| {
            matches!(
                k_stats::decide(row, Margin::of(m), Band::least()),
                Ok(Sign::Positive)
            )
        };
        if !geom_core::is_finite_length(self.hi.get())
            || !positive("curve_pair_window_speed", self.hi.get())
        {
            return (lo, hi);
        }
        let mut kept: Option<(T, T)> = None;
        let walked = self.carrier.point_walk(centre, (lo, hi), self.hi, |p| {
            if positive("curve_pair_window_clear", p.gap - p.reach - reach) {
                return geom::PieceStep::Settled;
            }
            if positive("curve_pair_window_halve", p.reach - reach) {
                return geom::PieceStep::Halve;
            }
            kept = Some(kept.map_or(p.span, |(from, _)| (from, p.span.1)));
            geom::PieceStep::<()>::Settled
        });
        match walked {
            Ok(_) => kept.unwrap_or((lo, lo)),
            Err(geom_core::bisect::Exhausted) => (lo, hi),
        }
    }

    /// `t` within half a turn of the span's middle on a closed carrier,
    /// `t` itself on an open one ([`Curve3::branch_near`]).
    pub fn branch(&self, t: T) -> T {
        self.carrier
            .branch_near(t, geom::mid_param(self.t0, self.t1))
    }

    /// **The parameters a shared run of this span and another is read
    /// at**: this span's middle, and halfway between each of its ends and
    /// each of `feet`, the feet on this carrier of the other span's ends.
    /// A positive-length overlap of the two spans holds one of them. An
    /// endless span has no ends, so its middle alone: the overlap with an
    /// ended span is that span, whose own middle holds it.
    pub fn witness_params(&self, feet: impl IntoIterator<Item = T>) -> Vec<T> {
        let mut ts = vec![geom::mid_param(self.t0, self.t1)];
        if !self.ended {
            return ts;
        }
        for t in feet {
            let t = self.branch(t);
            ts.extend([geom::mid_param(self.t0, t), geom::mid_param(t, self.t1)]);
        }
        ts
    }

    /// **Where `q`'s foot sits against the span's ends** (module docs),
    /// the foot found from `near`. It decides nothing about whether `q`
    /// lies on the carrier.
    ///
    /// # Errors
    ///
    /// The band's escalation when a decision the answer rests on falls in
    /// its gap (the chord to an end, the side of an end, or a `Past` the
    /// span walk does not certify), or the walk's own refusal.
    pub fn place(&self, q: Point3<T>, near: T, band: Band) -> Result<Place, CarrierRefusal> {
        self.place_at(&self.carrier.foot(q, near), band)
    }

    /// [`Self::place`] at a foot already found on this span's carrier.
    fn place_at(&self, foot: &Foot<'_, T>, band: Band) -> Result<Place, CarrierRefusal> {
        let (q, t) = (foot.point(), self.branch(foot.seed()));
        if !self.ended {
            return Ok(Place::Inside);
        }
        let ends = [(SpanEnd::Start, t - self.t0), (SpanEnd::Stop, self.t1 - t)];
        // An end at the point first, on its raw chord: no side matters
        // beside it.
        let mut chords = Vec::with_capacity(2);
        for (which, _) in ends {
            let chord = q - self.end(which).1;
            let read = decide_reported("curve_pair_span_end_chord", Margin::norm3(chord), band);
            if let Ok(Decided {
                sign: Sign::Zero, ..
            }) = read
            {
                return Ok(Place::End(which));
            }
            chords.push(read);
        }
        let mut sides = [Side::Clear; 2];
        let mut past = None;
        for (side, ((_, m), read)) in sides.iter_mut().zip(ends.into_iter().zip(chords)) {
            let chord = read?;
            *side = self.side(m, chord, band)?;
            if *side == Side::Past {
                past = past.or(Some(chord));
            }
        }
        let Some(chord) = past else {
            return Ok(Place::Inside);
        };
        if foot.nearest().is_some() {
            return Ok(Place::Past);
        }
        match self.carrier.clear_of(q, (self.t0, self.t1), band)? {
            Clearance::Clear => Ok(Place::Past),
            // A kept piece reading zero is no `On`: its reading is the
            // lower bound `|C(m) − q| − ℓ` on `q`'s distance from the
            // piece, so zero puts `q` within `z + 2ℓ` of it, past the
            // zero band `z` the composition's `On` claims, and the place
            // decides no carrier membership of its own (module docs).
            Clearance::Near(Ok(near)) => {
                Err(CarrierRefusal::Escalated(CarrierEscalation::Straddle(
                    straddle("curve_pair_span_past_straddle", near, chord, band),
                )))
            }
            Clearance::Near(Err(diag)) => Err(diag.into()),
        }
    }

    /// The side of one end the foot lies on, `m` its parameter's distance
    /// inside the span from the end and `chord` the point's chord to the
    /// end's point, decided past the band (module docs).
    fn side(&self, m: T, chord: Decided, band: Band) -> Result<Side, Indeterminate> {
        let side = decide_reported(
            "curve_pair_span_end_side",
            Margin::metered_sup(m, self.hi),
            band,
        )?;
        match side.sign {
            Sign::Positive => Ok(Side::Clear),
            Sign::Negative => Ok(Side::Past),
            Sign::Zero => Err(straddle("curve_pair_span_end_straddle", side, chord, band)),
        }
    }

    /// **Whether the window's run lies past the span**
    /// (`curve_pair::pose`'s module docs, "The gap's lower bound" and "The
    /// run"), `q` a point of the carrier at about `near`, the polished
    /// pair's, and every pair of the window within the band's escalation
    /// within `reach` metres of `q` along it. The span's own place reading
    /// says `q` is past it ([`Self::place`]); then each end is one the run
    /// cannot reach or cannot cross:
    ///
    /// - **out of reach**: its chord to `q` less `reach` reads positive
    ///   (`curve_pair_span_past_reach`). A chord is at most the arc it
    ///   spans, so the arc from `q` to the end is longer than `reach`;
    /// - **the run arm**, where `run` names the other carrier and the
    ///   window's bend, given only where the polished pair is itself in
    ///   the run: the end's closed-form clearance from the other carrier
    ///   less twice the bend reads past the band
    ///   (`curve_pair_span_run_end`). Any pair of the run beyond the end
    ///   would put the end within the escalation plus twice the bend of
    ///   the other carrier.
    pub fn past_by(
        &self,
        q: Point3<T>,
        near: T,
        (reach, run): (T, Option<(&Curve3<T>, T)>),
        band: Band,
    ) -> PastBy {
        if !self.ended || self.place(q, near, band) != Ok(Place::Past) {
            return PastBy::Not;
        }
        for (_, end) in self.ends() {
            let chord = q - end;
            let lower = decide_reported(
                "curve_pair_span_past_reach",
                Margin::of(chord.norm() - reach),
                band,
            );
            if matches!(
                lower,
                Ok(Decided {
                    sign: Sign::Positive,
                    ..
                })
            ) {
                continue;
            }
            if let Some((other, bend)) = run
                && let Some(clear) = other.clearance(end)
                && k_stats::decide(
                    "curve_pair_span_run_end",
                    Margin::of(clear - bend - bend),
                    band,
                ) == Ok(Sign::Positive)
            {
                continue;
            }
            return match lower {
                Ok(lower) => {
                    // `place` read `q`'s chord to each end from `q`'s foot
                    // on this carrier, this chord from `q` itself: where
                    // the two disagree on the band's side no straddle is
                    // claimed, and the reach clears nothing.
                    match decide_reported("curve_pair_span_end_chord", Margin::norm3(chord), band) {
                        Ok(upper) if lower.sign < upper.sign => PastBy::Near(straddle(
                            "curve_pair_span_past_reach_straddle",
                            lower,
                            upper,
                            band,
                        )),
                        Ok(_) => PastBy::Not,
                        Err(diag) => PastBy::Near(diag),
                    }
                }
                Err(diag) => PastBy::Near(diag),
            };
        }
        PastBy::Clear
    }

    /// **What `q` is against the span**: the two readings at its foot
    /// found from `near`, composed (module docs), deciding nothing of its
    /// own. The carrier is read first, and a point it reads off is off
    /// the span with no place read. Otherwise its reading goes on the
    /// frame after the place's, so a refusal of the carrier's is the last
    /// on the frame; at an end, and past the span beside a carrier that
    /// refused, its escalation is superseded, since neither answer rests
    /// on it.
    ///
    /// # Errors
    ///
    /// Whichever reading the answer rests on refused (module docs), with
    /// which ([`SpanRefusal`]).
    pub fn read(&self, q: Point3<T>, near: T, band: Band) -> Result<Reading<T>, SpanRefusal> {
        let foot = self.carrier.foot(q, near);
        let (on, carrier) = k_stats::detached(|| foot.on_carrier(band));
        let on = match on {
            Ok(OnCarrier::Off) => {
                k_stats::splice(carrier);
                return Ok(Reading::Off);
            }
            Ok(OnCarrier::On { t, margin }) => Ok((t, margin)),
            Err(refused) => Err(refused),
        };
        let place = self.place_at(&foot, band);
        // An end, and a past beside a carrier that refused, are answers
        // the carrier's reading does not decide: its escalation is
        // superseded.
        if matches!(place, Ok(Place::End(_))) || (matches!(place, Ok(Place::Past)) && on.is_err()) {
            k_stats::splice_superseded(carrier);
        } else {
            k_stats::splice(carrier);
        }
        Ok(match place {
            Ok(Place::End(which)) => Reading::End(which),
            Ok(Place::Inside) => {
                let (t, margin) = on.map_err(|refused| SpanRefusal {
                    refused,
                    of: SpanReading::Carrier,
                })?;
                Reading::On {
                    t: self.branch(t),
                    margin,
                }
            }
            Ok(Place::Past) => match on {
                Ok(_) => Reading::Carrier,
                Err(_) => Reading::Off,
            },
            Err(refused) => {
                return Err(SpanRefusal {
                    refused: on.err().unwrap_or(refused),
                    of: SpanReading::Place,
                });
            }
        })
    }

    /// **Whether `q` is on the span**, as a side of a meeting: [`Self::read`]
    /// as a [`Hit`], `None` off the span.
    ///
    /// # Errors
    ///
    /// [`Self::read`]'s.
    pub fn contact(
        &self,
        q: Point3<T>,
        near: T,
        band: Band,
    ) -> Result<Option<Hit<T>>, SpanRefusal> {
        self.read(q, near, band).map(Reading::hit)
    }
}
