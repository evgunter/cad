//! **The refusals a chain's step order earns at every value**: each is
//! a predicate over a structural fact (a fillet left open, a first leg
//! drawn, the first segment's kind, a written count), asked by the
//! replay where it holds the fact and by [`super::shape::chain_shape`]
//! where it tracks the same fact with values erased. One predicate per
//! refusal, so the two cannot disagree on its condition or its words.

use super::PathError;
use crate::{PathErrorKind, Real};

/// **A refusal the step order alone earns**: the replay's own
/// [`PathError`], read off structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderFault {
    /// An arc leg authored from a fillet's open arrival
    /// ([`PathError::ArcLegOnOpenFillet`]).
    ArcLegOnOpenFillet {
        /// What was reached, and what binds it instead.
        site: &'static str,
    },
    /// A far-end anchor with no fillet open
    /// ([`PathError::FarEndAnchorWithoutFillet`]).
    FarEndAnchorWithoutFillet,
    /// A seam fillet over a chain whose first side is not straight
    /// ([`PathError::SeamRetrimsArcFirstSide`]).
    SeamRetrimsArcFirstSide,
    /// A `circle_split` count outside `1..=u32::MAX`
    /// ([`PathError::CircleSplitCount`]).
    CircleSplitCount {
        /// The written count.
        n: usize,
    },
    /// A seam classified on a chain whose closing leg is its first
    /// ([`PathError::UnderdeterminedLeg`]).
    NoFirstLeg,
    /// An arc closing at the start from the entry itself, no leg drawn
    /// before it: its chord is the zero one
    /// ([`PathError::DegenerateArcChord`]).
    ClosesOnItsEntry,
}

/// The sites [`OrderFault::ArcLegOnOpenFillet`] names.
pub(crate) mod site {
    /// `arc_to` from the arrival anchor to a point.
    pub(crate) const ARC_TO: &str = "an arc arrival is authored WITH the fillet that trims it — \
        fillet_arc(r, spec) / arc_fillet_arc(spec, r, spec2) — not by an arc LEG from an \
        already-positioned arrival point";
    /// `arc_to` from the arrival anchor to the start.
    pub(crate) const ARC_TO_START: &str = "an arc arrival that CLOSES is authored with the \
        fillet that trims it — fillet_arc(r, Center { c, winding, p: Start }) — not by an arc \
        LEG from an already-positioned arrival point";
    /// A fused verb's incoming arc from the arrival anchor.
    pub(crate) const ARC_FILLET: &str = "an arc incoming to a second fillet from a fillet's \
        arrival anchor would claim that arrival a second time — author the first fillet's \
        arrival arc with it (fillet_arc(r, spec) / arc_fillet_arc(spec, r, spec2)), or bind its \
        direction before the next fillet";
    /// `arc_onto` from the arrival anchor.
    pub(crate) const ARC_ONTO: &str = "an arc that arrives along a direction from a fillet's \
        arrival anchor is the fillet's ARC ARRIVAL — fillet_arc(r, spec) / \
        arc_fillet_arc(spec, r, spec2) — not an arc_onto leg from an already-positioned arrival \
        point";
}

impl OrderFault {
    /// The refusal's class.
    #[must_use]
    pub fn kind(self) -> PathErrorKind {
        PathError::<f64>::from(self).kind()
    }
}

impl core::fmt::Display for OrderFault {
    /// The replay's own sentence for it.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        PathError::<f64>::from(*self).fmt(f)
    }
}

impl<T: Real> From<OrderFault> for PathError<T> {
    fn from(fault: OrderFault) -> Self {
        match fault {
            OrderFault::ArcLegOnOpenFillet { site } => PathError::ArcLegOnOpenFillet { site },
            OrderFault::FarEndAnchorWithoutFillet => PathError::FarEndAnchorWithoutFillet,
            OrderFault::SeamRetrimsArcFirstSide => PathError::SeamRetrimsArcFirstSide,
            OrderFault::CircleSplitCount { n } => PathError::CircleSplitCount { n },
            OrderFault::NoFirstLeg => PathError::UnderdeterminedLeg {
                site: "a seam junction on a chain with no first leg",
            },
            OrderFault::ClosesOnItsEntry => PathError::DegenerateArcChord {
                chord: super::length(T::zero()),
            },
        }
    }
}

/// An arc leg (`arc_to`, `arc_onto`) may not leave a fillet's open
/// arrival: that arrival is authored with the fillet.
pub(crate) fn arc_leg_leaves_no_open_fillet(
    open: bool,
    site: &'static str,
) -> Result<(), OrderFault> {
    if open {
        return Err(OrderFault::ArcLegOnOpenFillet { site });
    }
    Ok(())
}

/// A far-end anchor ends a fillet's arrival side, so one must be open.
pub(crate) fn far_end_ends_an_open_fillet(open: bool) -> Result<(), OrderFault> {
    if !open {
        return Err(OrderFault::FarEndAnchorWithoutFillet);
    }
    Ok(())
}

/// A seam fillet retrims the entry vertex, so the first side must be
/// straight.
pub(crate) fn seam_fillet_retrims_a_straight_side(first_is_line: bool) -> Result<(), OrderFault> {
    if !first_is_line {
        return Err(OrderFault::SeamRetrimsArcFirstSide);
    }
    Ok(())
}

/// A classified seam reads the first leg's arm, so one must be drawn
/// before the closing leg.
pub(crate) fn seam_reads_a_first_leg(drawn: bool) -> Result<(), OrderFault> {
    if !drawn {
        return Err(OrderFault::NoFirstLeg);
    }
    Ok(())
}

/// An arc closing at the start spans a chord from the tip to the
/// entry, so a leg must be drawn before it: with none, the tip is the
/// entry and the chord is the zero one at every value.
pub(crate) fn a_closing_arc_leaves_its_entry(drawn: bool) -> Result<(), OrderFault> {
    if !drawn {
        return Err(OrderFault::ClosesOnItsEntry);
    }
    Ok(())
}

/// A `circle_split` draws `n` arcs, one at least, each named by a
/// `u32`.
pub(crate) fn circle_split_count(n: usize) -> Result<(), OrderFault> {
    if n == 0 || u32::try_from(n).is_err() {
        return Err(OrderFault::CircleSplitCount { n });
    }
    Ok(())
}
