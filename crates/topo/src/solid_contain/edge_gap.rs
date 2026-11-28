//! **How far a ray passes an edge, sorted against the band once for
//! every arm** (D10, Booleans). A ray within the band of an edge meets
//! it, half a crossing for each face the edge bounds
//! ([`super::ray_crossings`]); one definitely past the band passes it
//! clear. Each reading of such a distance (a line edge's gap between
//! the two lines, a curved edge's carrier reading at the ray's closest
//! approach, the offset of a carrier crossing from a passage, a root's
//! reach margin) is one of three ([`Reach`]):
//!
//! - **past** the band: decided Positive;
//! - **within** it ([`Gap`]): decided Zero, or an escalation in the
//!   band;
//! - **unplaced**: any other escalation. An enclosure reaching past the
//!   band's edge holds members that pass the edge and members that meet
//!   it; a carrier's two bounds on one distance that straddle the band
//!   say neither; a poisoned margin says nothing. None is a reading, so
//!   none stands for a crossing: the ray reads nothing where the edge
//!   could be met, a mark at a line edge's passage
//!   ([`super::RayHit::mark`]) and the ray's refusal at a curved one
//!   ([`set_aside`]).
//!
//! **Which escalation is in the band** is the classifier's word, read
//! here and nowhere else in the ray's edge arms
//! ([`crate::validate::in_band`]): a point margin is (at `f64` the
//! classifier escalates only inside the sliver), an enclosure is where
//! the classifier flagged it a terminal sliver (it set the flag on an
//! enclosure wholly inside the sliver; the bounds are not read again
//! here), and a poisoned margin is not.
//!
//! **What the types hold.** A [`Gap`] has no constructor outside this
//! file: an arm gets one only as this file's sorting of a decision's
//! result ([`Reach::of`]) or of a carrier's reading
//! ([`Reach::on_carrier`]). So an arm cannot read an escalation as an
//! edge met by writing a gap of its own. It can still forge the result
//! it hands the door (`Reach::of(Ok(Magnitude::Zero))`); no type here
//! forbids that, and the rows that go red under it are what hold it
//! (`curved_edges::straddle_rows`, `topo`'s
//! `an_aimed_ray_beside_a_boxed_cylinders_moving_rim`).

use geom::CarrierRefusal;
use geom_core::k_stats::Magnitude;
use geom_core::{Band, Decide, Indeterminate, MarginDiag, Sign};

use super::{Mark, PointInSolidError, RayRows};
use crate::entity::FaceKey;
use crate::ray_walk::RayFault;

/// How far the ray passes an edge it meets, as a wedge's mark reads it:
/// decided Zero, or in the band's gap.
#[derive(Clone, Copy)]
pub(super) struct Gap(Within);

#[derive(Clone, Copy)]
enum Within {
    Zero,
    InBand(Indeterminate),
}

impl Gap {
    /// The mark two met edges of one face read: a wedge where either gap
    /// is in band, else a graze.
    pub(super) const fn mark(self, other: Self) -> Mark {
        match (self.0, other.0) {
            (Within::InBand(gap), _) | (Within::Zero, Within::InBand(gap)) => {
                Mark::SetAside(crate::side_reader::MarkKind::Wedge, gap)
            }
            (Within::Zero, Within::Zero) => Mark::Graze,
        }
    }

    /// Whether the gap is decided Zero.
    pub(super) const fn is_zero(self) -> bool {
        matches!(self.0, Within::Zero)
    }
}

/// What one reading of a distance says against the band (module docs).
pub(super) enum Reach {
    /// Definitely past the band.
    Past,
    /// Within it.
    Within(Gap),
    /// Not placed against it: no reading.
    Unplaced(Indeterminate),
}

impl Reach {
    /// The reach a magnitude's decision states.
    pub(super) fn of(read: Result<Magnitude, Indeterminate>) -> Self {
        match read {
            Ok(Magnitude::Positive) => Self::Past,
            Ok(Magnitude::Zero) => Self::Within(Gap(Within::Zero)),
            Err(diag) => Self::escalated(diag),
        }
    }

    /// **The gap a carrier's reading of a point states**, the point
    /// inside the edge's span: on the carrier (`Ok`, decided Zero), in
    /// the band, or unplaced (`Err`); `None` where the refusal is no
    /// escalation (a spline's walk that could not run), which sorts
    /// nothing.
    pub(super) fn on_carrier(
        read: Result<MarginDiag, CarrierRefusal>,
    ) -> Option<Result<Gap, Indeterminate>> {
        match read {
            Ok(_) => Some(Ok(Gap(Within::Zero))),
            Err(refused) => refused
                .escalation()
                .ok()
                .and_then(|diag| Self::escalated(diag).met()),
        }
    }

    /// The one sorting of an escalation: within the band where the
    /// classifier says it lies in it, unplaced otherwise.
    fn escalated(diag: Indeterminate) -> Self {
        if crate::validate::in_band(&diag) {
            Self::Within(Gap(Within::InBand(diag)))
        } else {
            Self::Unplaced(diag)
        }
    }

    /// **The gap the ray meets the edge by**, where this is the ray's
    /// distance from it: `None` past the band, and `Err` where nothing
    /// placed it, which the arm raises where the edge could be met
    /// ([`set_aside`]).
    pub(super) fn met(self) -> Option<Result<Gap, Indeterminate>> {
        match self {
            Self::Past => None,
            Self::Within(gap) => Some(Ok(gap)),
            Self::Unplaced(diag) => Some(Err(diag)),
        }
    }

    /// Whether the distance is read within the band.
    pub(super) const fn within(&self) -> bool {
        matches!(self, Self::Within(_))
    }
}

/// **Whether a signed margin shows its claim**, the claim standing on
/// Negative or Zero (a root reached in band of its carrier): shown,
/// not shown (decided Positive, or in band, which decides nothing), or
/// unplaced (`Err`), which sets the ray aside as an unplaced gap does.
///
/// # Errors
///
/// The escalation, where it is not in the band.
pub(super) fn shown(read: Result<Sign, Indeterminate>) -> Result<bool, Indeterminate> {
    match read {
        Ok(sign) => Ok(sign != Sign::Positive),
        Err(diag) => match Reach::escalated(diag) {
            Reach::Unplaced(diag) => Err(diag),
            Reach::Past | Reach::Within(_) => Ok(false),
        },
    }
}

/// **The ray's refusal where it passes an edge of `face` by a distance
/// nothing placed**: a met edge stands for its faces' crossings, so one
/// met on such a distance would count for every member of a box what
/// only some cross. The fault is the ray's own escalation
/// ([`RayFault::InBand`]), its diagnostics saying which kind.
pub(super) const fn set_aside(face: FaceKey, diag: Indeterminate) -> RayFault<PointInSolidError> {
    RayFault::InBand(PointInSolidError::Escalated { face, diag })
}

/// Where a passage of the ray lies against its start ([`start`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Start {
    /// Wholly behind it, decided: the ray is nearest the edge at its
    /// start, which lies clear of both the edge's faces (point
    /// containment's by its boundary pre-pass, an aimed ray's by lying
    /// beyond every face), so the passage reads nothing.
    Behind,
    /// Its near end decided behind and its far end not: read nowhere.
    Straddles,
    /// Not decided behind at either end.
    Ahead,
}

/// **Where the passage spanning `(lo, hi)` along `ray` lies against the
/// ray's start**: the one statement of the rule for every edge arm.
pub(super) fn start<T: Decide>(
    ray: &geom::Ray<T>,
    (lo, hi): (T, T),
    rows: RayRows,
    band: Band,
) -> Start {
    let behind = |s: T| ray.ahead(s, rows.advance(), band) == Ok(Sign::Negative);
    if behind(hi) {
        Start::Behind
    } else if behind(lo) {
        Start::Straddles
    } else {
        Start::Ahead
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod rows {
    use super::{Reach, shown};
    use geom_core::k_stats::Magnitude;
    use geom_core::{Band, Indeterminate, MarginDiag, Sign};

    const ZERO: f64 = 1e-9;

    fn escalation(margin: MarginDiag, terminal_sliver: bool) -> Indeterminate {
        Indeterminate {
            margin,
            band: Band::new(ZERO, 10.0 * ZERO).unwrap(),
            predicate: None,
            terminal_sliver,
        }
    }

    /// **Only an escalation the classifier placed in the band is a gap
    /// in band.** A point margin in the sliver is within the band, and
    /// so is an enclosure the classifier flagged a terminal sliver. The
    /// same enclosure without the flag is unplaced: the door reads the
    /// classifier's flag, never the bounds. An enclosure from on the
    /// edge to far past it (a box some of whose members meet the edge
    /// and others pass it), one from inside the zero band to past the
    /// escalation (a carrier's straddle) and a poisoned margin are
    /// unplaced. Red under `Reach::escalated` sorting every escalation
    /// within the band, where all four are within.
    #[test]
    fn an_escalation_reaching_past_the_band_is_unplaced() {
        let sort = |margin, flag| Reach::of(Err(escalation(margin, flag)));
        assert!(
            Reach::of(Ok(Magnitude::Zero))
                .met()
                .unwrap()
                .unwrap()
                .is_zero()
        );
        assert!(matches!(Reach::of(Ok(Magnitude::Positive)), Reach::Past));
        let sliver = sort(MarginDiag::value(5.0 * ZERO), false);
        assert!(
            matches!(sliver.met(), Some(Ok(gap)) if !gap.is_zero()),
            "a point margin in the sliver"
        );
        let inside = MarginDiag::enclosure(2.0 * ZERO, 5.0 * ZERO);
        assert!(sort(inside, true).within(), "flagged a terminal sliver");
        for (what, margin) in [
            ("the same bounds, not flagged", inside),
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
            let reach = sort(margin, false);
            assert!(matches!(reach, Reach::Unplaced(_)), "{what}");
            assert!(!reach.within(), "{what}");
        }
    }

    /// **A signed margin shows its claim on Negative or Zero alone**:
    /// Positive and an escalation in the band show nothing, and an
    /// escalation past the band's edge is unplaced. Red under
    /// `Reach::escalated` sorting every escalation within the band (the
    /// last reads "not shown" where it refuses).
    #[test]
    fn a_signed_margin_is_shown_not_shown_or_unplaced() {
        assert_eq!(shown(Ok(Sign::Negative)), Ok(true));
        assert_eq!(shown(Ok(Sign::Zero)), Ok(true));
        assert_eq!(shown(Ok(Sign::Positive)), Ok(false));
        assert_eq!(
            shown(Err(escalation(MarginDiag::value(5.0 * ZERO), false))),
            Ok(false)
        );
        let wide = escalation(MarginDiag::enclosure(-0.1, 0.3), false);
        assert_eq!(shown(Err(wide)), Err(wide));
    }
}
