//! **The revolve tool**: a modal layer-3 tool that turns a profile pick
//! and an axis line written in the profile's own plane into exactly one
//! committed revolve edit (GAUTH-1).
//!
//! # Shape
//!
//! The mate tool's pattern one vocabulary over: single-select stays
//! ruled, so the tool holds its pick in tool state and consumes the
//! ordinary selection stream — a tree click is a node pick directly, a
//! viewport face or edge pick reaches the node whose drawn body the ray
//! met (`Selection::seat_node`). The seat is not a body, so in practice
//! it is filled from the tree: a profile is not a drawn body a ray can
//! meet. The axis is no pick: it is two coordinate pairs in the
//! profile's own plane, which cannot leave it.
//! Everything before the commit is tool state; the document transition
//! is one [`SessionOp::AddRevolve`], which commits one
//! `DocEdit::InsertNode` through the session's ordinary commit door.
//!
//! The seats, their pick rule, the survival step, the divergence from
//! the mate tool's promoting one, and the id-reuse hazard reconcile
//! does not cover (issue #1384) are all [`crate::seats`]'s — this tool
//! is that value with one role and a commit door, exactly as the
//! combining tools are.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::document::{Doc, Formula, ProfileProgram, RecipeNodeId};

use crate::seats::{Seat, SeatError, SeatEvent, Seats};
use crate::session::SessionOp;

/// The modal revolve tool. A value: the chrome holds one while the tool
/// is active, a test constructs one and drives the same methods.
#[derive(Debug, Clone, PartialEq, Eq)]
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
            seats: Seats::new([Seat::RevolveProfile]),
        }
    }

    /// The seats, roles and picks together — what the panel's line is
    /// composed from ([`crate::seats::seat_line`]).
    pub fn seats(&self) -> &Seats {
        &self.seats
    }

    /// The held profile pick.
    pub fn profile(&self) -> Option<RecipeNodeId> {
        self.seats.held(0)
    }

    /// Feed one node pick — the selection vocabulary's node, consumed
    /// into tool state. `doc` routes it, and does not judge it.
    pub fn pick(&mut self, doc: &Doc<ProfileProgram>, node: RecipeNodeId) {
        self.seats.pick(doc, node);
    }

    /// Empty the seat — the chrome's "start the picks over" door.
    pub fn clear(&mut self) {
        self.seats.clear();
    }

    /// The survival step ([`crate::seats`]).
    pub fn reconcile(&mut self, doc: &Doc<ProfileProgram>) -> Vec<SeatEvent> {
        self.seats.reconcile(doc)
    }

    /// The held picks spoken again from `doc` ([`Seats::respeak`]).
    pub fn respeak(&mut self, doc: &Doc<ProfileProgram>) {
        self.seats.respeak(doc);
    }

    /// **The one committed edit**: the session op that inserts the
    /// revolve node through the ordinary commit door, about the axis
    /// line through `axis_origin` along `axis_direction` in the
    /// profile's own coordinates, by `angle` (the chrome's default is a
    /// full turn about the profile's +y).
    ///
    /// # Errors
    ///
    /// [`SeatError::Empty`] until the seat is filled. Node KINDS are
    /// not judged here — the session door refuses a wrong-kind pick
    /// typed.
    pub fn op(
        &self,
        axis_origin: [Formula; 2],
        axis_direction: [Formula; 2],
        angle: Formula,
    ) -> Result<SessionOp, SeatError> {
        Ok(SessionOp::AddRevolve {
            profile: self.seats.require(0)?,
            axis_origin,
            axis_direction,
            angle,
        })
    }
}
