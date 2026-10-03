//! The pin-update door's two non-primitive halves (ASSEMBLY-DESIGN
//! A13, clauses 2 and 3; ASM-UPD D-2 and D-3).
//!
//! The primitive itself is [`crate::DocEdit::UpdateReference`], one
//! reference at a time. What lives here is everything A13 builds on
//! top of it:
//!
//! - [`update_references`] — "update document X everywhere", which is
//!   an ELABORATION and never a second primitive. It returns the
//!   ordinary edits that do the job and applies none of them; the
//!   caller applies the list, and purity is what makes that atomic
//!   (the ASM-4 precedent: `split` hands back values and edits, not a
//!   mutated document).
//! - [`mixed_pins`] — the multiplicity LINT. Two pins of one document
//!   id in one assembly is legal, sometimes-intended state (a staged
//!   migration), so it is reported and never refused. A13 is explicit
//!   about both directions: refusal would make staged updates
//!   unauthorable, silence would hide the most common mistake.
//!
//! **Nothing here resolves a pin.** This layer has no store; whether
//! `new_pin` names content that exists is evaluation's question,
//! answered through the seam that already names both pins. The
//! workspace-layer convenience that COMPUTES a pin from a store
//! (`pncad::workspace::update_to_store`) sits one layer up, where the
//! store actually is.

use std::collections::BTreeMap;

use crate::doc::Doc;
use crate::edit::DocEdit;
use crate::ident::{ContentPin, DocumentId};
use crate::node::{Node, RecipeNodeId};

/// Why a whole-document update produced no edit list (ASM-UPD D-2).
/// Both arms name the document id, because "which part did you mean"
/// is the only question an author can act on here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateError {
    /// No live node in this document instantiates that id. The
    /// elaboration has nothing to elaborate INTO, which is a typo or a
    /// stale id — never a silent success.
    NoSuchReference {
        /// The id nothing references.
        id: DocumentId,
    },
    /// Every reference to the id already names `pin`, so the whole
    /// elaboration is semantically empty. The per-reference primitive
    /// refuses that state one site at a time
    /// ([`crate::EditError::PinUnchanged`]); this is the same rule at
    /// document scale, so an update-all can never report success while
    /// recording nothing.
    AlreadyPinned {
        /// The id every site already pins.
        id: DocumentId,
        /// The pin they all already name.
        pin: ContentPin,
    },
}

impl core::fmt::Display for UpdateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoSuchReference { id } => {
                write!(f, "update: no node in this document instantiates {id}")
            }
            Self::AlreadyPinned { id, pin } => write!(
                f,
                "update: every reference to {id} already pins {pin}, so this update moves nothing"
            ),
        }
    }
}

impl core::error::Error for UpdateError {}

/// The edits that move EVERY reference to `id` onto `new_pin` (A13
/// clause 2; ASM-UPD D-2) — in document order, one per site whose pin
/// actually moves.
///
/// Pure: `doc` is untouched and nothing is applied. The caller applies
/// the whole list or none of it, and that all-or-nothing is what
/// "atomically grouped" means at this layer — there is no partially
/// applied state to roll back from, because applying is the caller's
/// single step (the ASM-4 precedent).
///
/// A site already pinning `new_pin` contributes NO edit: A13 blesses
/// mixed-pin state as authorable, so "update everywhere" must stay
/// usable from the staged state where some sites already moved.
/// Emitting an edit there would hand back a list the first
/// [`crate::EditError::PinUnchanged`] refuses — an un-appliable
/// group, which is the one thing an atomic list must never be.
///
/// # Errors
///
/// [`UpdateError::NoSuchReference`] when no live node instantiates the
/// id, and [`UpdateError::AlreadyPinned`] when every site that does
/// already names `new_pin`. The two are separate because the recourses
/// differ: one is a wrong id, the other is a completed update.
pub fn update_references<P>(
    doc: &Doc<P>,
    id: DocumentId,
    new_pin: ContentPin,
) -> Result<Vec<DocEdit<P>>, UpdateError> {
    let mut referenced = false;
    let mut edits = Vec::new();
    for &node in doc.order() {
        let Some(Node::InstantiatePart { doc_ref, .. }) = doc.node(node) else {
            continue;
        };
        if doc_ref.id != id {
            continue;
        }
        referenced = true;
        if doc_ref.pin != new_pin {
            edits.push(DocEdit::UpdateReference { node, new_pin });
        }
    }
    if !referenced {
        return Err(UpdateError::NoSuchReference { id });
    }
    if edits.is_empty() {
        return Err(UpdateError::AlreadyPinned { id, pin: new_pin });
    }
    Ok(edits)
}

/// One referenced document id carrying more than one pin, with the
/// sites holding each (A13 clause 3; ASM-UPD D-3).
///
/// Deliberately carries no `Display` yet: the report is typed data
/// with no user-facing rendering site. When one appears, this is the
/// finding sink's next resident — it composes through
/// `crate::finding::Finding` (subject: the id and its pin spread;
/// story: the sites; recourse: `update_references`), never a fourth
/// hand-rolled renderer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinMultiplicity {
    /// The document referenced at two or more versions.
    pub id: DocumentId,
    /// Its pins, ascending, each with the nodes naming it. Length is
    /// ≥ 2 by construction — a single-pin id is not a multiplicity and
    /// is not reported.
    pub pins: Vec<PinSites>,
}

/// One pin of one id, and the instantiate nodes that name it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinSites {
    /// The version.
    pub pin: ContentPin,
    /// The referencing nodes, in document order. Non-empty by
    /// construction — a pin is listed because a site holds it.
    pub nodes: Vec<RecipeNodeId>,
}

/// The mixed-pin lint: every referenced id whose pin multiplicity
/// exceeds one, listing each pin and its referencing nodes (A13 clause
/// 3; ASM-UPD D-3).
///
/// **Reports, never gates.** Nothing calls this from `apply`, from the
/// load door, or from evaluation — a document in mixed-pin state is
/// valid at every one of those doors, and stays that way, because a
/// staged migration IS this state. A clean document returns an empty
/// vector, which is the whole difference between "checked and fine"
/// and "not checked".
///
/// Deterministic: ids ascending, pins ascending within an id, nodes in
/// document order — a report that changes only when the document does.
pub fn mixed_pins<P>(doc: &Doc<P>) -> Vec<PinMultiplicity> {
    let mut by_id: BTreeMap<DocumentId, BTreeMap<ContentPin, Vec<RecipeNodeId>>> = BTreeMap::new();
    for &node in doc.order() {
        if let Some(Node::InstantiatePart { doc_ref, .. }) = doc.node(node) {
            by_id
                .entry(doc_ref.id)
                .or_default()
                .entry(doc_ref.pin)
                .or_default()
                .push(node);
        }
    }
    by_id
        .into_iter()
        .filter(|(_, pins)| pins.len() > 1)
        .map(|(id, pins)| PinMultiplicity {
            id,
            pins: pins
                .into_iter()
                .map(|(pin, nodes)| PinSites { pin, nodes })
                .collect(),
        })
        .collect()
}
