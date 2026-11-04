//! **Where a crossing lands along its edge**: the words and the
//! decision every door that splits an edge at a crossing states (a
//! curve × face door, the split, the blend, the Boolean), whichever
//! door found the crossing.

use geom_core::Indeterminate;

use crate::recourse::{RefusedArm, RefusedSide, SizedDecision, SizedPass};

/// Where a crossing lands along its edge, in words: the subject every
/// decision on it states, wherever an edge is split at a crossing.
pub const CROSSING_INTERIOR: &str = "whether a crossing lands strictly inside its edge";

/// The lever every door that splits an edge at a crossing has (the
/// split, the blend, the Boolean): the geometry.
pub const CROSSING_LEVER: &str =
    "move the geometry so the crossing lands clearly away from the edge's ends";

/// What a crossing's interiority margin measures.
pub const CROSSING_SIZE: &str = "distance from the edge's end";

/// A decision on where a crossing lands along its edge, where every
/// definite answer passes and only an undecided one refuses. Its ending
/// is the one every door that splits an edge at a crossing states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(any(test, feature = "test-support"), derive(strum::EnumIter))]
pub enum CrossingDecision {
    /// Whether a crossing lands strictly inside its edge (inside, at an
    /// end, and outside all pass).
    OnEdge,
    /// Which of two crossings on an edge comes first (either order, or
    /// one crossing, all pass).
    Order,
}

impl CrossingDecision {
    /// What the decision decides, as a clause with no colon or dash of
    /// its own.
    #[must_use]
    pub const fn subject(self) -> &'static str {
        match self {
            Self::OnEdge => CROSSING_INTERIOR,
            Self::Order => "which of two crossings on an edge comes first",
        }
    }

    /// The decision's table row: a size the user may intend, passing on
    /// every definite sign.
    #[must_use]
    pub const fn sized(self) -> SizedDecision {
        let (lever, size) = match self {
            Self::OnEdge => (CROSSING_LEVER, CROSSING_SIZE),
            Self::Order => (
                "move the geometry so the two crossings on that edge lie clearly apart",
                "distance between the crossings",
            ),
        };
        SizedDecision {
            lever,
            size,
            passes: SizedPass::AnySign,
            side: RefusedSide::Lever,
            no_size: None,
        }
    }

    /// The one ending `diag`'s escalation carries, read at `at`.
    #[must_use]
    pub fn ending_of(self, diag: &Indeterminate, at: impl Into<crate::recourse::ReadAt>) -> String {
        self.sized().recourse(RefusedArm::Undecided(diag), at)
    }
}
