//! Canonical semantic bytes and the content pin (ASM-1 D-2/D-3).
//!
//! [`canonical_bytes`] serializes exactly the SEMANTIC projection of
//! a document — what makes two documents the same VERSION of a part
//! (ASSEMBLY-DESIGN A4):
//!
//! - INCLUDED: nodes (with their structural params and expressions),
//!   insertion order, document params, the recorded ε (semantic: two
//!   documents differing only in ε are different documents — the A2
//!   seam rule), and witnesses (branch selection is geometry).
//! - EXCLUDED: the edit log (two edit histories reaching one snapshot
//!   pin identically; undo history must not move pins), the monotone
//!   `next_id` counter (history residue of deletions — it constrains
//!   future minting, not present content), metadata and appearance
//!   (affordance, never semantics), and the [`DocumentId`] itself
//!   (the pin answers "which version", the id "which part" — a
//!   retargeted id with unchanged content keeps its pin).
//!
//! The bytes are the deterministic JSON the persistence layer already
//! speaks (BTreeMaps, pair-list structural keys, ryu
//! shortest-round-trip floats), over the projection struct below —
//! deliberately NOT the save bytes, which carry the excluded fields.
//! Callers holding a (snapshot, edit-log) pair pin the REPLAYED
//! document (the same discipline as [`super::save`]/[`super::load`],
//! which replay before trusting): [`super::Loaded::doc`] is the value
//! to pin, so pin stability across no-op saves holds by construction.

use crate::ident::ContentPin;
use crate::program::ProfileDoc;

use super::{PersistError, check};

/// The serialize-only semantic projection. Field order is the wire
/// order; changing it (or any included field's serialization) moves
/// every pin — a format change on A4's version-identity currency,
/// ratify like a schema bump.
#[derive(serde::Serialize)]
struct CanonicalDoc<'a> {
    /// The nodes, by stable id.
    nodes: &'a std::collections::BTreeMap<crate::node::RecipeNodeId, crate::node::Node<crate::program::ProfileProgram>>,
    /// Insertion order of the live nodes.
    order: &'a [crate::node::RecipeNodeId],
    /// Document-level named parameters.
    params: &'a std::collections::BTreeMap<crate::doc::ParamName, crate::doc::DocParam>,
    /// The recorded modeling tolerance ε.
    epsilon: f64,
    /// Per-node witness data (branch selection is geometry).
    witnesses:
        &'a std::collections::BTreeMap<crate::node::RecipeNodeId, crate::witness::WitnessDatum>,
}

/// The canonical semantic bytes of the CURRENT document state (module
/// docs: included/excluded fields). Runs the shared validator first
/// (with an empty log — the projection is of a current state, not a
/// history), so a document that could not be saved cannot be pinned.
///
/// # Errors
///
/// The shared validator's arms ([`PersistError::NonFinite`],
/// [`PersistError::ProfileProgram`], [`PersistError::Snapshot`]), and
/// [`PersistError::Serialize`] if the JSON writer itself fails.
pub fn canonical_bytes(doc: &ProfileDoc) -> Result<Vec<u8>, PersistError> {
    check::validate_document(doc, &[])?;
    let projection = CanonicalDoc {
        nodes: &doc.nodes,
        order: &doc.order,
        params: &doc.params,
        epsilon: doc.epsilon,
        witnesses: &doc.witnesses,
    };
    let json = serde_json::to_string(&projection).map_err(|e| PersistError::Serialize {
        message: e.to_string(),
    })?;
    Ok(json.into_bytes())
}

/// The document's content pin: SHA-256 of [`canonical_bytes`] (ASM-1
/// D-2; ASSEMBLY-DESIGN A4 — the pin IS version identity).
///
/// # Errors
///
/// Exactly [`canonical_bytes`]'s.
pub fn content_pin(doc: &ProfileDoc) -> Result<ContentPin, PersistError> {
    Ok(ContentPin::of_bytes(&canonical_bytes(doc)?))
}
