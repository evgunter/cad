//! **The coincidence record** (D10): every coincidence an operation
//! decides from values, one row per decision, in decision order.
//!
//! A row says which two cells were decided one, what relation was
//! decided between them, at which decision, and the margin that
//! decision read. It records; it proves nothing. Whether a row holds
//! across the family is decided at the document's one door
//! (`editor-core`'s `coincide::prove`), which the `unproven-coincidence`
//! lint reads.
//!
//! A row's cells are keyed in the deciding operation's INPUTS, the
//! arenas the decision read, so the document layer names each by its
//! input's table and the row survives whatever the operation later
//! does to those cells (a merge, a split into fragments).
//!
//! What is recorded is what a ladder decides from a margin. A pair a
//! ladder settles before any margin (the same key, the same
//! [`crate::GeomSource`]) is structure and is not recorded, and an ON
//! verdict that only places topology is not a coincidence (D1).

use geom_core::MarginDiag;

use crate::boolean::{Cell, Operand};

/// One cell of a row.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RowCell {
    /// A cell of one of the deciding operation's input bodies, keyed in
    /// that body's arena: a boolean's operand, or [`Operand::A`] for the
    /// input of an operation that reads one body.
    Input {
        /// Which input body.
        input: Operand,
        /// The cell, in that body's keys.
        cell: Cell,
    },
    /// The plane the operation cuts with (a split's tool): a carrier
    /// the operation reads, not a cell of any body.
    Tool,
}

impl RowCell {
    /// A face of input `input`.
    #[must_use]
    pub const fn face(input: Operand, face: crate::FaceKey) -> Self {
        Self::Input {
            input,
            cell: Cell::Face(face),
        }
    }
}

/// What was decided between a row's two cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Relation {
    /// One carrier, the two outward sides aligned: a continuation.
    SameOriented,
    /// One carrier, the two outward sides opposed: a contact.
    SameOpposite,
    /// The first cell lies on the second's carrier.
    OnCarrier,
    /// The two cells make equal angles with a third (an isosceles
    /// turn, whose mitre lands on that third edge).
    EqualAngles,
}

/// The decision a row was recorded at: a closed set, one per site
/// that decides a coincidence from values.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DecisionSite {
    /// The plane ladder's declared rung: a declared pair of planes read
    /// as one displacement over its consumed extent.
    PlaneLadder,
    /// The carrier ladder's declared rung, for a curved pair.
    CarrierLadder,
    /// A split's ON verdict at a vertex whose neighbourhood leaves it
    /// with two or more runs on one side, so pieces of one side touch
    /// there.
    SplitOn,
    /// The blend battery's isosceles turn (`fillet3_turn_isosceles`).
    BatteryTurn,
}

/// **One coincidence an operation decided from values.**
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coincidence {
    /// The two cells decided one, in the order the decision read them.
    pub cells: [RowCell; 2],
    /// What was decided between them.
    pub relation: Relation,
    /// Where it was decided.
    pub site: DecisionSite,
    /// The margin that decision read: Zero, or in band where a
    /// declaration bridged it.
    pub margin: MarginDiag,
}
