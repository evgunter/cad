//! **Role-typed pick seats**: the state every modal tool that consumes
//! node picks is built out of, and the one place their pick rule,
//! survival step and refusal sentence live.
//!
//! # Why one value and not one per tool
//!
//! The revolve tool and the four combining tools differ in exactly two
//! things: how many seats they have and what those seats MEAN. Everything
//! else — fill the first empty seat, replace the last when both are
//! full, drop a pick whose node left the document, refuse typed until a
//! needed seat is filled — is one behaviour, and a second copy of it is
//! a second place for it to drift.
//!
//! # The picks are ROLES
//!
//! A seat means a particular thing, and no pair here is symmetric — not
//! even the boolean's two operands, whose order is the difference
//! between `A ∖ B` and `B ∖ A`. So seats fill in order, a further pick
//! replaces the LAST one, and a drop empties the seat it was in without
//! promoting the survivor: moving a node between seats that mean
//! different things is not a correction, it is a different edit.
//!
//! **The mate tool is the deliberate exception and stays outside this
//! module**: its two picks are an interchangeable pair (`a`/`b` of one
//! mate), so its survival step promotes the survivor into the one-pick
//! step — meaningful there, wrong here. Its picks are also faces rather
//! than nodes, so it shares neither the state nor the rule; the
//! divergence is stated in both module docs and neither is the other's
//! accident.
//!
//! # Kinds are not judged here
//!
//! No tool checks what KIND of node filled a seat. That is the session
//! door's job ([`crate::session::Refusal::WrongNodeKind`], one arm for
//! every creation seat), so a wrong-kind pick refuses typed at the
//! commit rather than being silently ignored at pick time, and the rule
//! lives in one place.
//!
//! # Survival, and the hazard it does not cover
//!
//! [`Seats::reconcile`] is the consumer's obligation — the application
//! calls it once per frame ([`crate::tools::Tools::reconcile`] is where
//! that happens) — and a consumer that forgets it is not reliably
//! caught later. It guards the DELETED-node case only: a `RecipeNodeId`
//! is a small per-document counter, so once the document is REPLACED
//! under held picks (a `NewDocument`, an open, an undo past the picks'
//! inserts) fresh inserts re-mint the same small ids and stale picks
//! silently denote the NEW nodes. That aliasing is a class hazard of
//! layer-3 state holding `RecipeNodeId`s across history rewinds,
//! tracked as issue #1384.

use pncad::document::{Doc, ProfileProgram, RecipeNodeId};

/// Which seat of a tool a pick (or a drop) is about — one vocabulary of
/// roles for every seated tool, so a sentence about a held pick is
/// composed the same way wherever it appears.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seat {
    /// The profile a revolve sweeps.
    RevolveProfile,
    /// The axis a revolve sweeps about.
    RevolveAxis,
    /// The boolean's first operand — the body `A ∖ B` KEEPS.
    OperandA,
    /// The boolean's second operand — the body `A ∖ B` REMOVES.
    OperandB,
    /// The body a split cuts.
    SplitTarget,
    /// The datum plane a split cuts with.
    SplitPlane,
    /// The body a transform places.
    TransformBody,
    /// The body a pattern replicates.
    PatternBody,
    /// The datum axis a circular pattern steps around.
    PatternAxis,
}

impl Seat {
    /// The seat's name, for sentences.
    pub fn name(self) -> &'static str {
        match self {
            Self::RevolveProfile => "profile",
            Self::RevolveAxis => "axis",
            Self::OperandA => "first operand",
            Self::OperandB => "second operand",
            Self::SplitTarget => "split target",
            Self::SplitPlane => "split plane",
            Self::TransformBody => "transformed body",
            Self::PatternBody => "patterned body",
            Self::PatternAxis => "pattern axis",
        }
    }
}

/// A typed seated-tool refusal (closed enum, D4 ¶3).
#[derive(Debug)]
pub enum SeatError {
    /// A seat the commit needs is still empty.
    Empty {
        /// Which one.
        seat: Seat,
    },
}

impl core::fmt::Display for SeatError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Empty { seat } => write!(f, "no {} picked yet", seat.name()),
        }
    }
}

impl core::error::Error for SeatError {}

/// A typed tool event the chrome renders — every state change that was
/// not the direct echo of an op.
#[derive(Debug, PartialEq, Eq)]
pub enum SeatEvent {
    /// A held pick's node is no longer in the document; the tool
    /// dropped it and the seat is open again.
    PickLost {
        /// Which seat was emptied.
        seat: Seat,
        /// The node that was held.
        node: RecipeNodeId,
    },
}

impl core::fmt::Display for SeatEvent {
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

/// Two role-typed seats filled in order.
///
/// The roles travel WITH the seats rather than being re-supplied per
/// call: a drop's event has to name the role, and a value that knew its
/// picks but not what they were for could not compose that sentence.
///
/// A one-seat tool is this value with its second role unused — see
/// [`Seats::one`], which names the same seat twice so that the pick
/// rule ("fill the first empty, else replace the last") degenerates to
/// "replace", with no arm anywhere that has to know the arity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seats {
    roles: [Seat; 2],
    held: [Option<RecipeNodeId>; 2],
}

impl Seats {
    /// Two empty seats in their roles.
    pub const fn new(roles: [Seat; 2]) -> Self {
        Self {
            roles,
            held: [None, None],
        }
    }

    /// One empty seat: the role twice, so a pick always lands in it and
    /// a reconcile can only ever drop it once (both slots hold the same
    /// `Option`'s worth of state because only the first is ever set).
    pub const fn one(role: Seat) -> Self {
        Self::new([role, role])
    }

    /// The pick in seat `i` (0 or 1).
    pub fn held(&self, i: usize) -> Option<RecipeNodeId> {
        self.held.get(i).copied().flatten()
    }

    /// Whether any seat holds a pick.
    pub fn is_empty(&self) -> bool {
        self.held.iter().all(Option::is_none)
    }

    /// Fill the first empty seat; with both full, REPLACE the second
    /// (the module docs' pick rule).
    pub fn pick(&mut self, node: RecipeNodeId) {
        if self.held[0].is_none() {
            self.held[0] = Some(node);
        } else {
            self.held[1] = Some(node);
        }
    }

    /// Empty every seat — the chrome's "start the picks over" door.
    ///
    /// Recorded ergonomic limit of the pick rule (a review observation,
    /// kept rather than fixed): once both seats are full, a further pick
    /// always replaces the SECOND, so a wrong FIRST pick is corrected by
    /// clearing and re-picking, not by a third click. A per-seat re-pick
    /// affordance is chrome a later unit can add without touching this
    /// value.
    pub fn clear(&mut self) {
        self.held = [None, None];
    }

    /// Re-read the held picks against the document, dropping any whose
    /// node is gone (module docs: the survival semantics). Returns the
    /// typed drops.
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        let mut events = Vec::new();
        for (i, held) in self.held.iter_mut().enumerate() {
            if let Some(node) = *held
                && doc.node(node).is_none()
            {
                *held = None;
                events.push(SeatEvent::PickLost {
                    seat: self.roles[i],
                    node,
                });
            }
        }
        events
    }

    /// Seat `i`'s pick, or the typed "still empty" refusal naming it.
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] when that seat holds nothing.
    pub fn require(&self, i: usize) -> Result<RecipeNodeId, SeatError> {
        self.held(i).ok_or(SeatError::Empty {
            seat: self.roles[i],
        })
    }
}

/// **The one sentence a tool panel shows for its held picks**, so the
/// seats read the same way in every panel and a reader can tell which
/// pick is in which role — the fact that decides what a subtraction
/// removes.
///
/// Composed here rather than in the widgets because it is the same
/// vocabulary a lost-pick notice is composed from, and two copies is
/// how the two drift.
pub fn seat_line(seats: &[(Seat, Option<RecipeNodeId>)]) -> String {
    if seats.iter().all(|(_, held)| held.is_none()) {
        return "no picks yet".to_owned();
    }
    seats
        .iter()
        .map(|(seat, held)| match held {
            Some(node) => format!("{}: feature {}", seat.name(), node.0),
            None => format!("{}: —", seat.name()),
        })
        .collect::<Vec<_>>()
        .join("; ")
}
