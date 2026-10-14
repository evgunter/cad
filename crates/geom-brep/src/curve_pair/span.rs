//! **A carrier span, and the one reading of a point against it.**
//!
//! A [`Span`] is a carrier curve and a parameter interval on it, with no
//! topology attached: the stretch of the carrier an edge covers. Its one
//! reading, [`Span::contact`], places a point read at a parameter of the
//! carrier in one of four places ([`Contact`]): off the carrier, on the
//! carrier but past the span and clear of its ends, on the span's
//! interior, or within the band of one of its ends.
//!
//! **On the carrier** is the point's distance from the carrier's point at
//! that parameter, decided zero (`curve_pair_span_gap`). The parameter is
//! the caller's foot of the point; a parameter that is not the point's
//! foot only reads the point farther off, so it can lose a contact but
//! never invent one. On a closed carrier (a circle, an ellipse, a spiric
//! oval) the parameter is first read on the branch within half a turn of
//! the span's middle.
//!
//! **Each end** is then read by the point's chord to the end's own point
//! (`curve_pair_span_end_chord`): zero puts the point at that end. A chord
//! definitely past the band leaves only the side of the end the point is
//! on, the sign of the parameter's distance `m` from the end, inside the
//! span positive, metered by the carrier's greatest speed
//! (`curve_pair_span_end_side`). The arc from the point to the end is at
//! least the chord less the gap, so a chord past `K·ε` with the gap inside
//! `ε` leaves `|m|·speed` past `(K − 1)·ε`: definite once that clears the
//! band's gap, and an escalation where it falls in `(ε, K·ε]`. It cannot
//! read zero there, since the chord would then be within `2ε`.
//!
//! A span whose ends are no vertex — a closed edge whose start is not a
//! vertex of its own, [`Span::endless`] — has no end to be near: a point
//! on its carrier is on it.

use geom::Curve3;
use geom_core::k_stats::{decide, decide_reported};
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

/// Where [`Span::contact`] places a point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Contact<T> {
    /// Definitely off the carrier at the parameter read.
    Off,
    /// On the carrier, definitely past the span and clear of both its
    /// ends.
    Carrier,
    /// On the span's interior at `t`, clear of both its ends; `margin` is
    /// the reading that put the point on the carrier.
    On {
        /// The parameter, on the branch about the span's middle.
        t: T,
        /// The carrier reading's margin.
        margin: MarginDiag,
    },
    /// On the carrier within the band of one of the span's ends.
    End,
}

/// A [`Contact`] that places the point on the span: what a meeting
/// holds on each side.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Hit<T> {
    /// [`Contact::On`].
    On {
        /// The parameter, on the branch about the span's middle.
        t: T,
        /// The carrier reading's margin.
        margin: MarginDiag,
    },
    /// [`Contact::End`].
    End,
}

impl<T> Contact<T> {
    /// The contact as a [`Hit`], or `None` where it places the point off
    /// the span.
    pub fn hit(self) -> Option<Hit<T>> {
        match self {
            Self::On { t, margin } => Some(Hit::On { t, margin }),
            Self::End => Some(Hit::End),
            Self::Off | Self::Carrier => None,
        }
    }
}

/// One end of a span against a point on its carrier.
#[derive(Clone, Copy, PartialEq, Eq)]
enum EndSide {
    /// Inside the span, clear of this end.
    Clear,
    /// Within the band of this end.
    At,
    /// Outside the span past this end, clear of it.
    Past,
}

impl<'a, T: Decide> Span<'a, T> {
    /// The span of `carrier` over `(t0, t1)`, its rate bounds read once
    /// ([`crate::carrier_speed::speed_range`]).
    pub fn new(carrier: &'a Curve3<T>, (t0, t1): (T, T)) -> Self {
        let (lo, hi) = crate::carrier_speed::speed_range(carrier);
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
                mid + (t - mid).reduce_periodic_centred(T::tau())
            }
            Curve3::Line { .. } | Curve3::Nurbs(_) => t,
        }
    }

    /// **Where `q`, read at the carrier's parameter `t`, sits against the
    /// span** (module docs).
    ///
    /// # Errors
    ///
    /// The band's escalation when a decision the answer rests on falls in
    /// its gap: the gap to the carrier, the chord to an end, or the side
    /// of an end.
    pub fn contact(&self, q: Point3<T>, t: T, band: Band) -> Result<Contact<T>, Indeterminate> {
        let t = self.branch(t);
        let Decided { sign, margin } = decide_reported(
            "curve_pair_span_gap",
            Margin::norm3(q - self.carrier.eval(t)),
            band,
        )?;
        match sign {
            Sign::Zero => {}
            Sign::Positive | Sign::Negative => return Ok(Contact::Off),
        }
        if !self.ended {
            return Ok(Contact::On { t, margin });
        }
        let from_start = self.end_side(q, t - self.t0, self.t0, band)?;
        let from_end = self.end_side(q, self.t1 - t, self.t1, band)?;
        Ok(match (from_start, from_end) {
            (EndSide::At, _) | (_, EndSide::At) => Contact::End,
            (EndSide::Past, _) | (_, EndSide::Past) => Contact::Carrier,
            (EndSide::Clear, EndSide::Clear) => Contact::On { t, margin },
        })
    }

    /// `q`, on the carrier at the parameter distance `m` inside the span
    /// from its end at `end`, against that end (module docs).
    fn end_side(&self, q: Point3<T>, m: T, end: T, band: Band) -> Result<EndSide, Indeterminate> {
        let chord = Margin::norm3(q - self.carrier.eval(end));
        match decide("curve_pair_span_end_chord", chord, band)? {
            Sign::Zero => return Ok(EndSide::At),
            Sign::Positive | Sign::Negative => {}
        }
        Ok(
            match decide(
                "curve_pair_span_end_side",
                Margin::metered_sup(m, self.hi),
                band,
            )? {
                Sign::Positive => EndSide::Clear,
                Sign::Negative => EndSide::Past,
                Sign::Zero => EndSide::At,
            },
        )
    }
}
