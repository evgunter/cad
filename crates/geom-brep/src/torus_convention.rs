//! **The ring-torus convention `R > r > 0` as a decision** (D3, D4 ¶1
//! (i)/(iv)): its two halves' subjects, levers and decided facts, in
//! the lowest crate that can name a [`SizedDecision`], so every door
//! that refuses a torus outside the convention reads one table:
//! `topo`'s pierce normal and tier 3 today, and the doors below `topo`
//! ([`crate::intersect`]'s plane×torus section, [`crate::offset`]'s
//! realized ring) when they carry the decision. The margins themselves
//! are decided in `geom` ([`geom::torus_tube`], [`geom::ring_torus`]),
//! beside the type whose convention they state.

use crate::recourse::{Refused, SizedDecision, SizedPass, StoredDefinite};

/// **A half of the ring-torus convention `R > r > 0`**, as a decision.
/// A torus outside it (no tube, a horn, a spindle) has no
/// representation, so every arm that refuses one refuses the torus's
/// own shape and ends in the half's one lever, whether its margin fell
/// in band or was decided.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "test-support", derive(strum::EnumIter))]
pub enum TorusConvention {
    /// Whether the tube radius is positive at the band
    /// (`torus_tube_positive`).
    ///
    /// Tier 3 reads the same datum under another question: whether it
    /// lies inside `Surface::Torus`'s range at all, an unbanded
    /// representability read ending in "give the tube radius a value
    /// inside its range" (`topo`'s `UnrepresentableSurfaceDatum`). A tube
    /// thinner than the band passes that read and refuses this one, so
    /// the two are different decisions with different levers, not two
    /// stories of one.
    Tube,
    /// Whether the tube radius is smaller than the ring radius
    /// (`ring_torus_convention`), the half tier 3 decides on the same
    /// band.
    Ring,
}

impl TorusConvention {
    /// What the half decides, as a clause with no colon or dash of its
    /// own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::Tube => "whether a torus's tube radius is positive",
            Self::Ring => "whether a torus's tube radius is smaller than its ring radius",
        }
    }

    /// The half's lever and the size its margin measures, at every door
    /// that reads it. The lever edits the stored radii, so a definite
    /// refusal read at rest ends in it too.
    #[must_use]
    pub const fn sized(self) -> SizedDecision {
        let (lever, size) = match self {
            Self::Tube => (
                "reshape the torus so its tube is clearly thicker than the tolerance",
                "tube radius",
            ),
            Self::Ring => (
                "make the tube radius clearly smaller than the ring radius",
                "difference between the radii",
            ),
        };
        SizedDecision {
            lever,
            size,
            passes: SizedPass::Positive,
            stored: StoredDefinite::Lever,
            at_zero: None,
        }
    }

    /// The decided refusal as a clause about `whose` torus, with no
    /// colon or dash of its own.
    #[must_use]
    pub fn refused(self, whose: &str, verdict: Refused) -> String {
        let fact = match (self, verdict) {
            (Self::Tube, Refused::Zero(_)) => "tube radius is zero at this tolerance",
            (Self::Tube, Refused::Negative { .. }) => "tube radius is negative",
            (Self::Ring, Refused::Zero(_)) => {
                "tube radius equals its ring radius at this tolerance (a horn torus)"
            }
            (Self::Ring, Refused::Negative { .. }) => {
                "tube radius is larger than its ring radius (a spindle torus)"
            }
        };
        format!("{whose} {fact}")
    }
}
