//! **The revolve tool**: a modal layer-3 tool that turns two sequential
//! node picks — a profile, then an axis — into exactly one committed
//! revolve edit (GAUTH-1).
//!
//! # Shape
//!
//! The mate tool's pattern one vocabulary over: single-select stays
//! ruled, so the tool holds its two picks in tool state and consumes
//! the ordinary selection stream — a tree click is a node pick
//! directly, a viewport face or edge pick reaches the FEATURE it
//! belongs to (`Selection::node`, the one viewport→tree inversion).
//! Everything before the commit is tool state; the document transition
//! is one [`SessionOp::AddRevolve`], which commits one
//! `DocEdit::InsertNode` through the session's ordinary commit door.
//!
//! The seats, their pick rule, the survival step, the divergence from
//! the mate tool's promoting one, and the id-reuse hazard reconcile
//! does not cover (issue #1384) are all [`crate::seats`]'s — this tool
//! is that value with two roles and a commit door, exactly as the four
//! combining tools are.

use pncad::document::{Doc, ProfileProgram, RecipeNodeId};

use crate::seats::{Seat, SeatError, SeatEvent, Seats};
use crate::session::SessionOp;

/// The modal revolve tool. A value: the chrome holds one while the tool
/// is active, a test constructs one and drives the same methods.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RevolveTool {
    seats: Seats,
}

impl Default for RevolveTool {
    fn default() -> Self {
        Self::new()
    }
}

impl RevolveTool {
    /// A tool holding nothing.
    pub const fn new() -> Self {
        Self {
            seats: Seats::new([Seat::RevolveProfile, Seat::RevolveAxis]),
        }
    }

    /// The held profile pick.
    pub fn profile(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// The held axis pick.
    pub fn axis(&self) -> Option<RecipeNodeId> {
        self.seats.held(1)
    }

    /// Feed one node pick — the selection vocabulary's node, consumed
    /// into tool state. `doc` routes it, and does not judge it.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// Empty both seats — the chrome's "start the picks over" door.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// **The one committed edit**: the session op that inserts the
    /// revolve node through the ordinary commit door, at `angle`
    /// radians (the chrome's default is a full turn).
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until both seats are filled. Node KINDS are
    /// not judged here — the session door refuses a wrong-kind pick
    /// typed.
    pub fn op(&self, angle: f64) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddRevolve {
            profile: self.seats.require(0)?,
            axis: self.seats.require(1)?,
            angle,
        })
    }
}
