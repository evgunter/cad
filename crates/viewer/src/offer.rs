//! **Where a live editor's draft stands with the document** — the one
//! state machine the profile editor (`crate::drafts::ProfileEdit`) and
//! the placement editor ([`crate::matetool::MateTool`]) hold their draft
//! in (`crates/viewer/README.md`, *A create gesture commits the node*).
//! Each change is offered to the document's door; taken, it lands as one
//! op, and the changes made while it lands wait on it; refused, the
//! draft is held with its reason until a later change makes it pass.
//!
//! The editors differ in what a change is (`C`), why a draft is held
//! (`P`), and what a landed op wrote (`W`); the transitions are these.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use crate::session::SessionOp;

/// **One state**, so no two flags can disagree about it.
#[derive(Debug, Clone, PartialEq)]
pub enum Offer<P, C, W> {
    /// Nothing to offer: the draft is as loaded, or its last offer
    /// formed nothing (held back, or nothing moved).
    Quiet,
    /// Changed since it was last offered ([`Self::changed`]); the
    /// refusal of the offer before, if there was one, still said.
    Unsettled(Option<P>),
    /// Offered and refused: the draft is *pending*.
    Pending(P),
    /// Offered and taken: `op` formed and is still to land, and the
    /// changes made meanwhile wait on it ([`Self::defer`]).
    Landing {
        /// The op that lands the draft.
        op: SessionOp,
        /// The changes that wait on it.
        deferred: Vec<C>,
    },
    /// `op` landed, writing `written`: a reload over what it wrote makes
    /// the waiting changes, and over anything else drops them, said.
    Landed {
        /// What the op wrote.
        written: W,
        /// The changes that wait on it.
        deferred: Vec<C>,
    },
    /// Changes made over the document as it now is, still to be made
    /// ([`Self::take_deferred`]).
    Deferred(Vec<C>),
}

impl<P, C, W> Offer<P, C, W> {
    /// **A change was made to the draft**: it waits to be offered,
    /// still saying the refusal it was made over.
    pub fn changed(&mut self) {
        let was = core::mem::replace(self, Self::Quiet);
        *self = Self::Unsettled(match was {
            Self::Pending(refused) | Self::Unsettled(Some(refused)) => Some(refused),
            _ => None,
        });
    }

    /// **`change` waits on the op landing**, to be made over what it
    /// writes. Answers `change` back when no op is landing: it is the
    /// caller's to make now.
    #[must_use = "a change no op waits for is the caller's to make"]
    pub fn defer(&mut self, change: C) -> Option<C> {
        match self {
            Self::Landing { deferred, .. } => {
                deferred.push(change);
                None
            }
            _ => Some(change),
        }
    }

    /// The changes that waited on an op which has landed, to be made
    /// now; taken.
    pub fn take_deferred(&mut self) -> Vec<C> {
        match core::mem::replace(self, Self::Quiet) {
            Self::Deferred(deferred) => deferred,
            other => {
                *self = other;
                Vec::new()
            }
        }
    }

    /// **Why the draft is held**: the refusal of its last offer, while
    /// a change made over it is still to be offered or it was refused.
    pub fn refused(&self) -> Option<&P> {
        match self {
            Self::Pending(refused) | Self::Unsettled(Some(refused)) => Some(refused),
            _ => None,
        }
    }

    /// The op the draft waits on to land, if one formed and has not
    /// been answered.
    pub fn landing(&self) -> Option<&SessionOp> {
        match self {
            Self::Landing { op, .. } => Some(op),
            _ => None,
        }
    }

    /// **Whether letting the draft go loses anything** the document
    /// does not hold, `differs` saying whether the draft itself differs
    /// from what it was loaded from: an op still landing, changes
    /// waiting, or a changed draft.
    pub fn holds_changes(&self, differs: bool) -> bool {
        match self {
            Self::Landing { .. } => true,
            Self::Landed { deferred, .. } => !deferred.is_empty(),
            Self::Deferred(deferred) => !deferred.is_empty() || differs,
            Self::Quiet | Self::Unsettled(_) | Self::Pending(_) => differs,
        }
    }

    /// Whether a change was made since the last offer.
    pub fn unsettled(&self) -> bool {
        matches!(self, Self::Unsettled(_))
    }
}
