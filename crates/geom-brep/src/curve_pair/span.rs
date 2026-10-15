//! **A carrier span, its reading of a point against its ends, and the
//! point-on-span reading that composes it with the carrier's.**
//!
//! A [`Span`] is a carrier curve and a parameter interval on it, with no
//! topology attached: the stretch of the carrier an edge covers.
//!
//! **Two readings, one composition.** Whether a point is on a span is two
//! questions. Whether it lies on the carrier is the carrier's own
//! reading, [`geom::Curve3::on_carrier`], at the point's foot
//! ([`geom::Curve3::foot`]): on by the foot's gap, off only by a lower
//! bound on the distance from the whole carrier. Where the foot sits
//! against the span's ends is this module's reading, [`Span::place`],
//! which decides no carrier membership. [`Span::contact`] composes the
//! two and decides nothing of its own (C5, `crates/geom-brep/README.md`).
//!
//! **The place.** On a closed carrier (a circle, an ellipse, a spiric
//! oval) the foot's parameter is first read on the branch within half a
//! turn of the span's middle. Each end is read by the point's raw chord
//! to the end's own point (`curve_pair_span_end_chord`): zero puts the
//! point at that end ([`Place::End`]), whatever the foot. Past the band,
//! the side of the end is the sign of the foot's parameter distance `m`
//! from it, inside the span positive, metered by the carrier's greatest
//! speed (`curve_pair_span_end_side`). The arc from the foot's point to
//! the end is at least the chord less the foot's gap `g`, a value, so a
//! chord past `K·ε` with `g` inside `ε` leaves `|m|·speed` past
//! `(K − 1)·ε`: definite once that clears the band's gap, and an
//! escalation where it falls in `(ε, K·ε]`. A side that reads zero beside
//! a chord past the band escalates as their straddle
//! (`curve_pair_span_end_straddle`): only a band narrower than twice its
//! zero, or a foot far from the point, allows it.
//!
//! A span whose ends are no vertex — a closed edge whose start is not a
//! vertex of its own, [`Span::endless`] — has no end to be near: every
//! foot is inside it.
//!
//! **The composition** ([`Span::contact`]): at an end is a [`Hit::End`],
//! past the span none; inside, the carrier's reading decides, on (a
//! [`Hit::On`]), off (none) or undecided (an escalation). A place the band
//! cannot read is none where the carrier reads the point off, and an
//! escalation otherwise.

use geom::{CarrierRefusal, Curve3, Foot, OnCarrier};
use geom_core::k_stats::{decide, decide_reported, straddle};
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

    /// The carrier.
    pub fn carrier(&self) -> &'a Curve3<T> {
        self.carrier
    }

    /// The parameter interval.
    pub fn params(&self) -> (T, T) {
        (self.t0, self.t1)
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
                let inside = Band::new(f64::from_bits(1), f64::from_bits(2)).is_ok_and(|exact| {
                    decide("curve_pair_span_branch", Margin::of(r - T::pi()), exact)
                        == Ok(Sign::Negative)
                        && decide("curve_pair_span_branch", Margin::of(r + T::pi()), exact)
                            == Ok(Sign::Positive)
                });
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

    /// **Where `q`'s foot sits against the span's ends** (module docs).
    /// It decides nothing about whether `q` lies on the carrier.
    ///
    /// # Errors
    ///
    /// The band's escalation when a decision the answer rests on falls in
    /// its gap: the chord to an end, or the side of an end.
    pub fn place(
        &self,
        q: Point3<T>,
        foot: &Foot<T>,
        band: Band,
    ) -> Result<Place<T>, Indeterminate> {
        let t = self.branch(foot.t());
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
        for (side, ((_, m, _), read)) in sides.iter_mut().zip(ends.into_iter().zip(chords)) {
            *side = self.side(m, read?, band)?;
        }
        Ok(match sides {
            [Side::Past, _] | [_, Side::Past] => Place::Past,
            [Side::Clear, Side::Clear] => Place::Inside { t },
        })
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

    /// **Whether `q` is on the span**, read at its foot: the composition
    /// of the carrier's reading ([`geom::Curve3::on_carrier`]) and the
    /// span's ([`Self::place`]), deciding nothing of its own (module
    /// docs). `None` where it is not.
    ///
    /// # Errors
    ///
    /// Whichever reading the answer rests on refused.
    pub fn contact(
        &self,
        q: Point3<T>,
        foot: &Foot<T>,
        band: Band,
    ) -> Result<Option<Hit<T>>, CarrierRefusal> {
        match self.place(q, foot, band) {
            Ok(Place::End(which)) => Ok(Some(Hit::End(which))),
            Ok(Place::Past) => Ok(None),
            Ok(Place::Inside { t }) => Ok(match self.carrier.on_carrier(q, foot, band)? {
                OnCarrier::On { margin } => Some(Hit::On { t, margin }),
                OnCarrier::Off => None,
            }),
            Err(diag) => match self.carrier.on_carrier(q, foot, band) {
                Ok(OnCarrier::Off) => Ok(None),
                Ok(OnCarrier::On { .. }) | Err(_) => Err(CarrierRefusal::Escalated(diag)),
            },
        }
    }
}
