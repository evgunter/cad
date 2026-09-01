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
//! # Kinds ROUTE a pick; they still do not judge one
//!
//! No tool decides whether a seat's contents are legal — that is the
//! session door's job ([`crate::session::Refusal::WrongNodeKind`], one
//! arm for every creation seat), so a wrong-kind pick still refuses
//! typed at the commit rather than being silently ignored at pick
//! time, and the verdict lives in one place.
//!
//! What the kinds do decide here is WHICH SEAT a pick lands in. The
//! seats of a two-seat tool have different kinds — a split cuts a
//! BODY with a PLANE, a pattern steps a BODY around an AXIS — and
//! under the plain fill-then-replace rule, a user with both seats
//! full who clicked a second body got that body in the datum seat:
//! a pick that can only ever refuse, silently replacing one that was
//! fine. There is exactly one seat such a pick can have been meant
//! for, so it goes there. [`Seats::pick`] states the rule; the kinds
//! themselves come from [`Seat::wants`], and the classification is
//! [`crate::session::admits`] — the SAME one the commit door refuses
//! by, so a seat can never steer a pick into a seat the door would
//! then reject.
//!
//! The routing is deliberately narrow: it moves a pick only when the
//! seat it would otherwise land in cannot hold it AND another seat
//! can. A pick that fits nowhere lands where the plain rule puts it
//! and refuses at the commit, which is the case the typed refusal
//! exists for.
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

use crate::session::{NodeKindWanted, admits};

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
    /// **What kind of node this seat is for.**
    ///
    /// The same vocabulary the commit door refuses by
    /// ([`crate::session::NodeKindWanted`]) rather than a second one:
    /// this table and the door's are two readings of one fact, and
    /// two spellings of it would let a tool route a pick into a seat
    /// the door rejects.
    ///
    /// Exhaustive on purpose — a seventh seat has to say what it is
    /// for before it can be routed to.
    pub fn wants(self) -> NodeKindWanted {
        match self {
            Self::RevolveProfile => NodeKindWanted::Profile,
            Self::RevolveAxis | Self::PatternAxis => NodeKindWanted::Axis,
            Self::SplitPlane => NodeKindWanted::Plane,
            Self::OperandA
            | Self::OperandB
            | Self::SplitTarget
            | Self::TransformBody
            | Self::PatternBody => NodeKindWanted::Body,
        }
    }

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
    /// (the module docs' pick rule) — **unless the seat that would
    /// take the pick cannot hold this KIND of node and the other seat
    /// can**, in which case it lands in the seat it can only have
    /// been meant for.
    ///
    /// The narrow condition is the point (module docs): a pick that
    /// fits both seats, or neither, follows the plain rule, so the
    /// roles still fill in order and a re-pick still replaces the
    /// last. What the routing removes is the one move that could only
    /// ever refuse — a second body clicked into a datum seat — where
    /// wanting to re-target the body is the only thing the click can
    /// have meant.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        // **A one-seat tool has one seat, and its role names itself
        // twice to say so** ([`Seats::one`]) — so a pick can only
        // land in the first slot, and the second is not a seat to
        // route to or to reconcile.
        //
        // Stated here rather than assumed: the rule below is written
        // over the first EMPTY slot, and on a one-seat tool that made
        // a second pick land in a slot nothing reads. The transform
        // tool's own door says "a second pick replaces the first",
        // and until this arm existed it did not — re-picking the body
        // silently kept the old one, which is the shape a user
        // reports as the tool ignoring their click.
        if self.roles[0] == self.roles[1] {
            self.held[0] = Some(node);
            return;
        }
        let plain = usize::from(self.held[0].is_some());
        let held = doc.node(node);
        let other = 1 - plain;
        let seat = if admits(held, self.roles[other].wants())
            && !admits(held, self.roles[plain].wants())
        {
            other
        } else {
            plain
        };
        self.held[seat] = Some(node);
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
