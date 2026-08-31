//! **The revolve tool**: a modal layer-3 tool that turns two
//! sequential node picks — a profile, then an axis — into exactly one
//! committed revolve edit (GAUTH-1).
//!
//! # Shape
//!
//! The mate tool's pattern one vocabulary over: single-select stays
//! ruled, so the tool holds its two picks in tool state and consumes
//! the ordinary selection stream — a tree click is a node pick
//! directly, a viewport face pick reaches the FEATURE the face
//! belongs to (`Selection::node`, the one viewport→tree inversion).
//! Everything before the commit is tool state; the document
//! transition is one [`SessionOp::AddRevolve`], which commits one
//! `DocEdit::InsertNode` through the session's ordinary commit door.
//!
//! # The picks are ROLES, not a symmetric pair
//!
//! Unlike the mate tool's interchangeable `a`/`b`, the two picks here
//! mean different things: the first fills the PROFILE seat, the
//! second the AXIS seat, and a third pick replaces the axis (the
//! commit step is still open, and re-picking is how a user corrects
//! it). The tool does not judge node KINDS — that is the session
//! door's job ([`crate::session::Refusal::WrongNodeKind`], one arm
//! for every creation seat), so a wrong-kind pick
//! refuses typed at commit rather than being silently ignored at
//! pick time, and the rule lives in one place.
//!
//! # Survival
//!
//! Tool state survives a held node vanishing (the ratified
//! resolution-failure semantics at node scope):
//! [`RevolveTool::reconcile`] re-reads each seat against the document
//! and drops a pick whose node is gone, each drop a typed
//! [`RevolveToolEvent`] the chrome renders. A dropped profile does
//! NOT promote the held axis into the profile seat — the seats are
//! roles, and the next pick refills the empty one. **This is
//! deliberately DIVERGENT from the mate tool's survival step**, which
//! promotes a surviving second pick into the one-pick position: its
//! picks are an interchangeable pair, so keeping the survivor as "the
//! held pick" is meaningful there, while here it would move a node
//! between seats that mean different things. The mate tool's shipped
//! semantics stand unaltered; both module docs state the divergence.
//!
//! `reconcile` is the consumer's obligation, exactly as the mate
//! tool's is (its module docs carry the argument): the application
//! calls it once per frame. **A consumer that forgets it is NOT
//! reliably caught later.** The session door refuses a stale id that
//! no longer denotes the right kind — but a `RecipeNodeId` is a small
//! per-document counter, so once the document is REPLACED under held
//! picks (a `NewDocument`, an open, an undo past the picks' inserts)
//! fresh inserts re-mint the same small ids and the stale picks
//! silently denote the NEW nodes: a commit then authors a real edit
//! about geometry nobody picked, with no refusal anywhere. Per-frame
//! reconcile guards the deleted-node case only; the id-reuse aliasing
//! is a class hazard of layer-3 state holding `RecipeNodeId`s across
//! history rewinds, tracked as issue #1384.

use pncad::document::{Doc, ProfileProgram, RecipeNodeId};

use crate::session::SessionOp;

/// Which seat of the revolve tool a pick (or a drop) is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevolveSeat {
    /// The profile to revolve.
    Profile,
    /// The axis to revolve about.
    Axis,
}

impl RevolveSeat {
    /// The seat's name, for sentences.
    pub fn name(self) -> &'static str {
        match self {
            Self::Profile => "profile",
            Self::Axis => "axis",
        }
    }
}

/// A typed revolve-tool refusal (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum RevolveToolError {
    /// The tool does not hold both picks yet.
    NotTwoPicks,
}

impl core::fmt::Display for RevolveToolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotTwoPicks => {
                write!(f, "the revolve tool needs a profile pick and an axis pick")
            }
        }
    }
}

impl core::error::Error for RevolveToolError {}

/// A typed tool event the chrome renders — every state change that
/// was not the direct echo of an op.
#[derive(Debug)]
pub enum RevolveToolEvent {
    /// A held pick's node is no longer in the document; the tool
    /// dropped it and the seat is open again.
    PickLost {
        /// Which seat was emptied.
        seat: RevolveSeat,
        /// The node that was held.
        node: RecipeNodeId,
    },
}

impl core::fmt::Display for RevolveToolEvent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::PickLost { seat, node } => write!(
                f,
                "the {} pick (node {}) is no longer in the document; the tool dropped it",
                seat.name(),
                node.0
            ),
        }
    }
}

/// The modal revolve tool. A value: the chrome holds one while the
/// tool is active, a test constructs one and drives the same methods.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct RevolveTool {
    profile: Option<RecipeNodeId>,
    axis: Option<RecipeNodeId>,
}

impl RevolveTool {
    /// A tool holding nothing.
    pub fn new() -> Self {
        Self::default()
    }

    /// The held profile pick.
    pub fn profile(&self) -> Option<RecipeNodeId> {
        self.profile
    }

    /// The held axis pick.
    pub fn axis(&self) -> Option<RecipeNodeId> {
        self.axis
    }

    /// Feed one node pick — the selection vocabulary's node, consumed
    /// into tool state. The first pick fills the profile seat, the
    /// second the axis seat; a third REPLACES the axis (module docs).
    /// After a reconcile drop, the next pick refills whichever seat
    /// is empty, profile first.
    pub fn pick(&mut self, node: RecipeNodeId) {
        if self.profile.is_none() {
            self.profile = Some(node);
        } else {
            self.axis = Some(node);
        }
    }

    /// Empty both seats — the chrome's "start the picks over" door.
    ///
    /// Recorded ergonomic limit of the pick rule (a review
    /// observation, kept rather than fixed): once both seats are
    /// full, a further pick always replaces the AXIS, so a wrong
    /// PROFILE with both seats full is corrected by clearing and
    /// re-picking, not by a third click. A per-seat re-pick
    /// affordance is chrome a later unit can add without touching
    /// this value.
    pub fn clear(&mut self) {
        self.profile = None;
        self.axis = None;
    }

    /// Re-read the held picks against the document, dropping any
    /// whose node is gone (module docs: the survival semantics).
    /// Returns the typed drops.
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<RevolveToolEvent> {
        let mut events = Vec::new();
        for (seat, held) in [
            (RevolveSeat::Profile, &mut self.profile),
            (RevolveSeat::Axis, &mut self.axis),
        ] {
            if let Some(node) = *held
                && doc.node(node).is_none()
            {
                *held = None;
                events.push(RevolveToolEvent::PickLost { seat, node });
            }
        }
        events
    }

    /// **The one committed edit**: the session op that inserts the
    /// revolve node through the ordinary commit door, at `angle`
    /// radians (the chrome's default is a full turn).
    ///
    /// # Errors
    ///
    /// [`RevolveToolError::NotTwoPicks`] until both seats are filled.
    /// Node KINDS are not judged here — the session door refuses a
    /// wrong-kind pick typed (module docs).
    pub fn op(&self, angle: f64) -> Result<SessionOp, RevolveToolError> {
        match (self.profile, self.axis) {
            (Some(profile), Some(axis)) => Ok(SessionOp::AddRevolve {
                profile,
                axis,
                angle,
            }),
            _ => Err(RevolveToolError::NotTwoPicks),
        }
    }
}
