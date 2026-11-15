//! **How far a ray passes an edge, sorted against the band once for
//! every arm** (D10, Booleans). A ray within the band of an edge meets
//! it, half a crossing for each face the edge bounds
//! ([`super::ray_crossings`]); one definitely past the band passes it
//! clear. Each reading of such a distance (a line edge's gap between
//! the two lines, a curved edge's carrier reading at the ray's closest
//! approach, the offset of a carrier crossing from a passage) is one of
//! three ([`Reach`]):
//!
//! - **past** the band: decided Positive;
//! - **within** it ([`Gap`]): decided Zero, or an escalation that lies
//!   in the band ([`geom_core::ladder::in_band`]: a point margin in the
//!   sliver, or an enclosure wholly inside it);
//! - **unread**: any other escalation. An enclosure reaching past the
//!   band's edge holds members that pass the edge and members that meet
//!   it; a carrier's two bounds on one distance that straddle the band
//!   say neither; a poisoned margin says nothing. None is a reading, so
//!   none stands for a crossing: the ray is set aside where the edge
//!   could be met ([`unread`]).
//!
//! [`InBand`] is built only here, so no arm reads an escalation as an
//! edge met without asking whether it lies in the band.

use geom_core::Indeterminate;
use geom_core::k_stats::Magnitude;

use super::{Mark, PointInSolidError};
use crate::entity::FaceKey;
use crate::ray_walk::RayFault;

/// How far the ray passes an edge it meets, as a wedge's mark reads it.
#[derive(Clone, Copy)]
pub(super) enum Gap {
    /// Decided Zero.
    Zero,
    /// In the band's gap.
    InBand(InBand),
}

/// An escalation that lies in the band ([`geom_core::ladder::in_band`]).
#[derive(Clone, Copy)]
pub(super) struct InBand(Indeterminate);

impl Gap {
    /// **An escalated distance as a gap**: in the band where the whole
    /// escalation lies in it, and unread (`Err`) otherwise.
    ///
    /// # Errors
    ///
    /// The escalation, where it does not lie in the band.
    pub(super) fn escalated(diag: Indeterminate) -> Result<Self, Indeterminate> {
        if geom_core::ladder::in_band(&diag) {
            Ok(Self::InBand(InBand(diag)))
        } else {
            Err(diag)
        }
    }

    /// The mark two met edges of one face read: a wedge where either gap
    /// is in band, else a graze.
    pub(super) const fn mark(self, other: Self) -> Mark {
        match (self, other) {
            (Self::InBand(InBand(gap)), _) | (Self::Zero, Self::InBand(InBand(gap))) => {
                Mark::Wedge(gap)
            }
            (Self::Zero, Self::Zero) => Mark::Graze,
        }
    }
}

/// What one reading of a distance says against the band (module docs).
pub(super) enum Reach {
    /// Definitely past the band.
    Past,
    /// Within it.
    Within(Gap),
    /// No reading.
    Unread(Indeterminate),
}

impl Reach {
    /// The reach a magnitude's decision states.
    pub(super) fn of(read: Result<Magnitude, Indeterminate>) -> Self {
        match read {
            Ok(Magnitude::Positive) => Self::Past,
            Ok(Magnitude::Zero) => Self::Within(Gap::Zero),
            Err(diag) => Gap::escalated(diag).map_or_else(Self::Unread, Self::Within),
        }
    }

    /// Whether the distance is read within the band.
    pub(super) const fn within(&self) -> bool {
        matches!(self, Self::Within(_))
    }
}

/// **The ray's refusal where it passes an edge of `face` by a distance
/// nothing read**: a met edge stands for its faces' crossings, so one
/// met on such a distance would count for every member of a box what
/// only some cross.
pub(super) const fn unread(face: FaceKey, diag: Indeterminate) -> RayFault<PointInSolidError> {
    RayFault::InBand(PointInSolidError::Escalated { face, diag })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod rows {
    use super::{Gap, Reach};
    use geom_core::k_stats::Magnitude;
    use geom_core::{Band, Indeterminate, MarginDiag};

    const ZERO: f64 = 1e-9;

    /// **Only an escalation that lies in the band is a gap in band.** A
    /// point margin in the sliver and an enclosure wholly inside it (a
    /// terminal sliver) are within the band. An enclosure from on the
    /// edge to far past it (a box some of whose members meet the edge
    /// and others pass it), one from inside the zero band to past the
    /// escalation (a carrier's straddle) and a poisoned margin are
    /// unread. Red under `Gap::escalated` returning
    /// `Ok(Self::InBand(InBand(diag)))` for every escalation, where all
    /// three are within.
    #[test]
    fn an_escalation_reaching_past_the_band_is_unread() {
        let band = Band::new(ZERO, 10.0 * ZERO).unwrap();
        let escalated = |margin: MarginDiag, terminal_sliver: bool| {
            Reach::of(Err(Indeterminate {
                margin,
                band,
                predicate: None,
                terminal_sliver,
            }))
        };
        assert!(matches!(
            Reach::of(Ok(Magnitude::Zero)),
            Reach::Within(Gap::Zero)
        ));
        assert!(matches!(Reach::of(Ok(Magnitude::Positive)), Reach::Past));
        assert!(
            matches!(
                escalated(MarginDiag::value(5.0 * ZERO), false),
                Reach::Within(Gap::InBand(_))
            ),
            "a point margin in the sliver"
        );
        assert!(
            matches!(
                escalated(MarginDiag::enclosure(2.0 * ZERO, 5.0 * ZERO), true),
                Reach::Within(Gap::InBand(_))
            ),
            "an enclosure wholly inside the sliver"
        );
        for (what, margin) in [
            (
                "a box from on the edge to far past it",
                MarginDiag::enclosure(0.0, 0.4),
            ),
            (
                "a straddle's bracket",
                MarginDiag::enclosure(0.6 * ZERO, 20.0 * ZERO),
            ),
            ("a poisoned margin", MarginDiag::INVALID),
        ] {
            let reach = escalated(margin, false);
            assert!(matches!(reach, Reach::Unread(_)), "{what}");
            assert!(!reach.within(), "{what}");
        }
    }
}
