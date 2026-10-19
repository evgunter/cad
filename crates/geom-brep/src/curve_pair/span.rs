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
//! ([`geom::Foot::is_nearest`]). On an ellipse, a spiric oval or a spline
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
//! reading decides, on ([`Reading::On`]) or undecided (its refusal);
//! past, the point is off the span, on the carrier
//! ([`Reading::Carrier`]) or not ([`Reading::Off`], an undecided carrier
//! included, since no answer to it puts the point on the span). A place
//! the band cannot read is a refusal: the carrier's where it refused
//! too, else the place's.

use geom::{CarrierRefusal, Clearance, Curve3, Foot, OnCarrier};
use geom_core::k_stats::{self, decide_reported, straddle};
use geom_core::{Band, Decide, Indeterminate, InfSpeed, Margin, MarginDiag, Point3, Real, Sign};
use geom_core::{Decided, SupSpeed};

/// A carrier and a parameter interval on it (module docs).
#[derive(Clone, Copy)]
pub struct Span<'a, T: Real> {
    carrier: &'a Curve3<T>,
    t0: T,
    t1: T,
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
pub enum Place<T> {
    /// Inside the span at `t`, clear of both its ends.
    Inside {
        /// The parameter, on the branch about the span's middle.
        t: T,
    },
    /// The point within the band of this end's point.
    End(SpanEnd),
    /// Past the span, clear of both its ends.
    Past,
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
    /// The span of `carrier` over `(t0, t1)`, its rate bounds read once
    /// ([`geom::carrier_speed::speed_range`]).
    pub fn new(carrier: &'a Curve3<T>, (t0, t1): (T, T)) -> Self {
        let (lo, hi) = geom::carrier_speed::speed_range(carrier);
        Self {
            carrier,
            t0,
            t1,
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

    /// The parameter interval.
    pub fn params(&self) -> (T, T) {
        (self.t0, self.t1)
    }

    /// The parameter of `which` end.
    pub fn end(&self, which: SpanEnd) -> T {
        match which {
            SpanEnd::Start => self.t0,
            SpanEnd::Stop => self.t1,
        }
    }

    /// Where `hit` sits on the span's carrier: its parameter, or its
    /// end's.
    pub fn at(&self, hit: Hit<T>) -> T {
        match hit {
            Hit::On { t, .. } => t,
            Hit::End(which) => self.end(which),
        }
    }

    /// The carrier's least and greatest speed.
    pub fn speeds(&self) -> (InfSpeed<T>, SupSpeed<T>) {
        (self.lo, self.hi)
    }

    /// `t` on the branch within half a turn of the span's middle, on a
    /// closed carrier; `t` itself on an open one.
    pub fn branch(&self, t: T) -> T {
        match self.carrier {
            Curve3::Circle { .. } | Curve3::Ellipse { .. } | Curve3::Spiric { .. } => {
                let mid = geom::mid_param(self.t0, self.t1);
                let d = t - mid;
                let r = d.reduce_periodic_centred(T::tau());
                // An enclosure of `t` about the antipode of the middle
                // would wrap the whole turn: there either branch is as
                // near, so it is read on the one whose cut lies at the
                // middle instead.
                let row = "curve_pair_span_branch";
                let inside = super::compare(row, r, T::pi()) == Some(Sign::Negative)
                    && super::compare(row, r, T::zero() - T::pi()) == Some(Sign::Positive);
                if inside {
                    mid + r
                } else {
                    mid + (d + T::pi()).reduce_periodic_centred(T::tau()) - T::pi()
                }
            }
            Curve3::Line { .. } | Curve3::Nurbs(_) => t,
        }
    }

    /// **The parameters a shared run of this span and another is read
    /// at**: this span's middle, and halfway between each of its ends and
    /// each of `feet`, the feet on this carrier of the other span's ends.
    /// A positive-length overlap of the two spans holds one of them.
    pub fn witness_params(&self, feet: impl IntoIterator<Item = T>) -> Vec<T> {
        let mut ts = vec![geom::mid_param(self.t0, self.t1)];
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
    pub fn place(&self, q: Point3<T>, near: T, band: Band) -> Result<Place<T>, CarrierRefusal> {
        self.place_at(&self.carrier.foot(q, near), band)
    }

    /// [`Self::place`] at a foot already found on this span's carrier.
    fn place_at(&self, foot: &Foot<'_, T>, band: Band) -> Result<Place<T>, CarrierRefusal> {
        let (q, t) = (foot.point(), self.branch(foot.t()));
        if !self.ended {
            return Ok(Place::Inside { t });
        }
        let ends = [
            (SpanEnd::Start, t - self.t0, self.t0),
            (SpanEnd::Stop, self.t1 - t, self.t1),
        ];
        // An end at the point first, on its raw chord: no side matters
        // beside it.
        let mut chords = Vec::with_capacity(2);
        for (which, _, end) in ends {
            let chord = q - self.carrier.eval(end);
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
        for (side, ((_, m, _), read)) in sides.iter_mut().zip(ends.into_iter().zip(chords)) {
            let chord = read?;
            *side = self.side(m, chord, band)?;
            if *side == Side::Past {
                past = past.or(Some(chord));
            }
        }
        let Some(chord) = past else {
            return Ok(Place::Inside { t });
        };
        if foot.is_nearest() {
            return Ok(Place::Past);
        }
        match self.carrier.clear_of(q, (self.t0, self.t1), band)? {
            Clearance::Clear => Ok(Place::Past),
            // A kept piece reading zero is no `On`: its reading is the
            // lower bound `|C(m) − q| − ℓ` on `q`'s distance from the
            // piece, so zero puts `q` within `z + 2ℓ` of it, past the
            // zero band `z` the composition's `On` claims, and the place
            // decides no carrier membership of its own (module docs).
            Clearance::Near(Ok(near)) => Err(CarrierRefusal::Straddle(straddle(
                "curve_pair_span_past_straddle",
                near,
                chord,
                band,
            ))),
            Clearance::Near(Err(diag)) => Err(CarrierRefusal::Escalated(diag)),
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
    /// Whichever reading the answer rests on refused (module docs).
    pub fn read(&self, q: Point3<T>, near: T, band: Band) -> Result<Reading<T>, CarrierRefusal> {
        let foot = self.carrier.foot(q, near);
        let (on, carrier) = k_stats::detached(|| foot.on_carrier(band));
        let on = match on {
            Ok(OnCarrier::Off) => {
                k_stats::splice(carrier);
                return Ok(Reading::Off);
            }
            Ok(OnCarrier::On { margin }) => Ok(margin),
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
            Ok(Place::Inside { t }) => Reading::On { t, margin: on? },
            Ok(Place::Past) => match on {
                Ok(_) => Reading::Carrier,
                Err(_) => Reading::Off,
            },
            Err(refused) => return Err(on.err().unwrap_or(refused)),
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
    ) -> Result<Option<Hit<T>>, CarrierRefusal> {
        self.read(q, near, band).map(Reading::hit)
    }
}
