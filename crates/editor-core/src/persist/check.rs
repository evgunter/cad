//! The persistence doors' ONE shared validator (spec D2/D6.3; DESIGN
//! engineering convention 2, discharged M5 S4):
//!
//! [`validate_document`] holds every direction-independent document
//! check and is invoked by BOTH doors — at save on the in-memory
//! document before a byte is written, at load on the parsed document
//! before replay. A document that would refuse to load is therefore
//! impossible to save by construction: not two mirrored door sets
//! kept in sync by a sweep, but code that is literally the same and
//! cannot drift. Its walks:
//!
//! - [`first_non_finite`] — every float the format would write,
//!   checked finite, with a typed site name. Expression literals are
//!   finite BY CONSTRUCTION (`Expr::literal` refuses non-finite —
//!   ruled door 1; the load side re-runs the same constructors), so
//!   the walk covers the float carriers outside that door: profile
//!   payloads, continuous doc params, the recorded ε, and D7 metadata
//!   trees (in the snapshot AND in the edit log). A NEW float-carrying
//!   field must join this walk — the D2 round-trip property tests are
//!   the tripwire. (Post-parse this walk cannot fire — JSON has no
//!   non-finite tokens — which is the asymmetry being BYTE-level, not
//!   a reason to fork the validator.)
//! - [`first_bad_joint`] — declared-tangent joints in range, snapshot
//!   and edit log (the `Index` channel's twin of the float walk).
//! - [`validate_snapshot`] — the document invariants `apply`
//!   maintains, re-checked structurally (a parsed snapshot is not
//!   trusted; an in-memory one can be corrupted through the `pub`
//!   payload or an in-crate bug).
//!
//! The genuinely asymmetric residue stays at its door and is the
//! symmetry sweep's whole remit now: header/parse/position errors and
//! the wire-only canonical-set rule ([`super::wire`]) are load-only by
//! nature; serializer failure is save-only; ε reconciliation is
//! process state, not a document property. Log replayability is
//! shared STRUCTURALLY instead — both doors replay through
//! [`crate::edit::apply`].

use crate::appearance::AppearanceRecord;
use crate::doc::{DocParam, ParamName};
use crate::edit::DocEdit;
use crate::expr::Dimension;
use crate::meta::MetaVersionError;
use crate::names::StableName;
use crate::node::{Node, RecipeNodeId};
use crate::profile_desc::{DescToken, ProfileDesc, ProfileDoc};
use crate::resolve::derivation_nodes;

/// Where a non-finite float sits (the D2 refusal's typed site).
#[derive(Debug, Clone, PartialEq)]
pub enum NonFiniteSite {
    /// The recorded ε.
    Epsilon,
    /// A continuous document parameter (snapshot).
    DocParam {
        /// The parameter.
        name: ParamName,
    },
    /// A float of a profile node's payload (snapshot), by position
    /// among the REAL floats of the canonical traversal (the `Float`
    /// tokens of [`ProfileDesc::tokens`]: placement columns, then
    /// vertices in loop order — structural tokens do not count).
    Profile {
        /// The profile node.
        node: RecipeNodeId,
        /// Index in the canonical float traversal.
        index: usize,
    },
    /// A float inside an appearance record's metadata (snapshot).
    Metadata {
        /// The attributed name.
        name: StableName,
        /// The metadata key.
        key: String,
        /// Path within the value tree (dot/index notation).
        path: String,
    },
    /// A float of a profile payload carried by an `InsertNode` edit
    /// (the node id is minted only at replay, so the site is the
    /// float's traversal index alone).
    InsertedProfile {
        /// Index in the canonical float traversal.
        index: usize,
    },
    /// A float carried by an edit in the log; `index` is the edit's
    /// position, `inner` the site within that edit's payload.
    Edit {
        /// The edit's index in the log.
        index: usize,
        /// The site within the edit.
        inner: Box<NonFiniteSite>,
    },
}

/// The shared validator (module docs): every direction-independent
/// document check, in one place, invoked by both doors. Check order
/// is float walk → joint walk → structural invariants (the save
/// door's historical precedence, pinned by the refusal suite).
pub(crate) fn validate_document(
    snapshot: &ProfileDoc,
    edits: &[DocEdit<ProfileDesc>],
) -> Result<(), super::PersistError> {
    if let Some(site) = first_non_finite(snapshot, edits) {
        return Err(super::PersistError::NonFinite { site });
    }
    if let Some((site, loop_index, joint, vertex_count)) = first_bad_joint(snapshot, edits) {
        return Err(super::PersistError::TangentJointOutOfRange {
            site,
            loop_index,
            joint,
            vertex_count,
        });
    }
    validate_snapshot(snapshot).map_err(super::PersistError::Snapshot)
}

/// The first non-finite float the format would write, or `None`.
fn first_non_finite(
    snapshot: &ProfileDoc,
    edits: &[DocEdit<ProfileDesc>],
) -> Option<NonFiniteSite> {
    if !snapshot.epsilon.is_finite() {
        return Some(NonFiniteSite::Epsilon);
    }
    for (name, p) in &snapshot.params {
        if let Some(site) = param_site(name, p) {
            return Some(site);
        }
    }
    for (&id, node) in &snapshot.nodes {
        if let Node::Profile(desc) = node
            && let Some(index) = profile_non_finite(desc)
        {
            return Some(NonFiniteSite::Profile { node: id, index });
        }
    }
    for (name, rec) in &snapshot.appearance {
        if let Some((key, path)) = record_non_finite(rec) {
            return Some(NonFiniteSite::Metadata {
                name: name.clone(),
                key,
                path,
            });
        }
    }
    for (index, edit) in edits.iter().enumerate() {
        if let Some(inner) = edit_non_finite(edit) {
            return Some(NonFiniteSite::Edit {
                index,
                inner: Box::new(inner),
            });
        }
    }
    None
}

fn param_site(name: &ParamName, p: &DocParam) -> Option<NonFiniteSite> {
    match p {
        DocParam::Continuous { value, .. } if !value.is_finite() => {
            Some(NonFiniteSite::DocParam { name: name.clone() })
        }
        _ => None,
    }
}

/// Walks every `Float` token of the description — keyed off token
/// TAGS, so the door skips NOTHING and shares no blind spot with any
/// encoding detail (review MAJOR-1: the former bit-stream walk
/// skipped the `u64::MAX` in-band marker, which is itself a real NaN
/// pattern). The returned index counts REAL floats only (placement
/// columns first, then vertices in loop order).
fn profile_non_finite(desc: &ProfileDesc) -> Option<usize> {
    let mut float_index = 0usize;
    for tok in desc.tokens() {
        if let DescToken::Float(bits) = tok {
            if !f64::from_bits(bits).is_finite() {
                return Some(float_index);
            }
            float_index += 1;
        }
    }
    None
}

fn record_non_finite(rec: &AppearanceRecord) -> Option<(String, String)> {
    rec.metadata
        .iter()
        .find_map(|(key, value)| value.first_non_finite().map(|path| (key.clone(), path)))
}

/// The float carriers an edit can smuggle past `apply` (a saved log
/// is DATA — it has not necessarily been applied by this process).
fn edit_non_finite(edit: &DocEdit<ProfileDesc>) -> Option<NonFiniteSite> {
    match edit {
        DocEdit::InsertNode {
            node: Node::Profile(desc),
        } => profile_non_finite(desc).map(|index| NonFiniteSite::InsertedProfile { index }),
        DocEdit::SetDocParam { name, value } => param_site(name, value),
        DocEdit::SetAppearanceMeta { name, key, value } => {
            value
                .first_non_finite()
                .map(|path| NonFiniteSite::Metadata {
                    name: name.clone(),
                    key: key.clone(),
                    path,
                })
        }
        DocEdit::SetTolerance { eps } if !eps.is_finite() => Some(NonFiniteSite::Epsilon),
        _ => None,
    }
}

/// A structural invariant violation in a parsed snapshot (load door).
#[derive(Debug, Clone, PartialEq)]
pub enum SnapshotError {
    /// `order` and the node map disagree (missing, extra, or
    /// duplicated ids).
    OrderMismatch,
    /// A `Node::Fillet` selection is not in canonical form (sorted
    /// and deduplicated) — a corrupt file, refused rather than
    /// repaired (M6-5).
    FilletSelectionNotCanonical {
        /// The offending fillet node.
        node: RecipeNodeId,
    },
    /// An id at or beyond the mint counter appears in the document.
    IdBeyondCounter {
        /// The offending id.
        id: RecipeNodeId,
        /// The counter.
        next_id: u64,
    },
    /// A node's input ref does not name a live node.
    DanglingInput {
        /// The referring node.
        node: RecipeNodeId,
        /// The missing input.
        input: RecipeNodeId,
    },
    /// A node's input ref does not precede it in `order` (insertion
    /// order is topological by construction — a forward ref means a
    /// tampered file, and possibly a cycle).
    ForwardInput {
        /// The referring node.
        node: RecipeNodeId,
        /// The forward input.
        input: RecipeNodeId,
    },
    /// A witness attached to a missing or non-sketch-bearing node.
    WitnessSite {
        /// The offending node id.
        node: RecipeNodeId,
    },
    /// A continuous parameter declared with the Count dimension
    /// (`apply` refuses this; a file must not smuggle it).
    CountContinuous {
        /// The parameter.
        name: ParamName,
    },
    /// The recorded ε is not finite and strictly positive.
    EpsilonInvalid {
        /// The recorded value.
        value: f64,
    },
    /// An appearance metadata value violating the D7 producer
    /// convention (map with an integer `"v"`).
    MetadataUnversioned {
        /// The attributed name.
        name: StableName,
        /// The metadata key.
        key: String,
        /// The typed shape refusal.
        error: MetaVersionError,
    },
}

/// Re-checks the document invariants `apply` maintains — on a parsed
/// snapshot (load) and on the in-memory snapshot (save) alike.
fn validate_snapshot(doc: &ProfileDoc) -> Result<(), SnapshotError> {
    // order ↔ nodes agreement (and no duplicates: equal lengths plus
    // every order id resolving implies a bijection on a BTreeMap).
    let mut position = std::collections::BTreeMap::new();
    for (i, &id) in doc.order.iter().enumerate() {
        if !doc.nodes.contains_key(&id) || position.insert(id, i).is_some() {
            return Err(SnapshotError::OrderMismatch);
        }
    }
    if position.len() != doc.nodes.len() {
        return Err(SnapshotError::OrderMismatch);
    }
    if !(doc.epsilon.is_finite() && doc.epsilon > 0.0) {
        return Err(SnapshotError::EpsilonInvalid { value: doc.epsilon });
    }
    for (name, p) in &doc.params {
        if matches!(
            p,
            DocParam::Continuous {
                dim: Dimension::Count,
                ..
            }
        ) {
            return Err(SnapshotError::CountContinuous { name: name.clone() });
        }
    }
    // Every id in the document stays below the mint counter — replay
    // after load must never re-mint a referenced id.
    let check_id = |id: RecipeNodeId| -> Result<(), SnapshotError> {
        if id.0 >= doc.next_id {
            Err(SnapshotError::IdBeyondCounter {
                id,
                next_id: doc.next_id,
            })
        } else {
            Ok(())
        }
    };
    for (&id, node) in &doc.nodes {
        check_id(id)?;
        for input in node.inputs() {
            check_id(input)?;
            if !doc.nodes.contains_key(&input) {
                return Err(SnapshotError::DanglingInput { node: id, input });
            }
            if position.get(&input) >= position.get(&id) {
                return Err(SnapshotError::ForwardInput { node: id, input });
            }
        }
        if let Node::Declare { pairs } = node {
            for (a, b) in pairs {
                for n in derivation_nodes(a).iter().chain(derivation_nodes(b).iter()) {
                    check_id(*n)?;
                }
            }
        }
        // The fillet selection (M6-5): the same name-reference id
        // check, PLUS the canonical-form assertion. `Node::fillet` is
        // the only construction door and it canonicalizes, so a
        // non-canonical selection on the wire is a CORRUPT file — it
        // is refused, never quietly re-sorted (a repair would change
        // the node's content key behind the caller's back).
        if let Node::Fillet { selection, .. } = node {
            for name in selection {
                for n in derivation_nodes(name) {
                    check_id(n)?;
                }
            }
            if selection.windows(2).any(|w| w[0] >= w[1]) {
                return Err(SnapshotError::FilletSelectionNotCanonical { node: id });
            }
        }
    }
    for name in doc.appearance.keys() {
        for n in derivation_nodes(name) {
            check_id(n)?;
        }
    }
    for &node in doc.witnesses.keys() {
        check_id(node)?;
        if !matches!(doc.nodes.get(&node), Some(Node::Profile(_))) {
            return Err(SnapshotError::WitnessSite { node });
        }
    }
    for (name, rec) in &doc.appearance {
        for (key, value) in &rec.metadata {
            if let Err(error) = value.require_versioned() {
                return Err(SnapshotError::MetadataUnversioned {
                    name: name.clone(),
                    key: key.clone(),
                    error,
                });
            }
        }
    }
    Ok(())
}

/// Where an out-of-range declared-tangent joint sits (review
/// MAJOR-DELTA-1 — the `Index` channel's twin of [`NonFiniteSite`]):
/// the profile payload is `pub`, so a stale joint (e.g. after a
/// vertex deletion in the payload) is API-reachable without passing
/// any edit door; a parsed file can carry the same corruption.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JointSite {
    /// A profile node in the snapshot.
    Profile {
        /// The profile node.
        node: RecipeNodeId,
    },
    /// A profile payload carried by an `InsertNode` edit in the log
    /// (the node id is minted only at replay).
    InsertedProfile {
        /// The edit's index in the log.
        index: usize,
    },
}

/// The first out-of-range declared-tangent joint, as
/// `(site, loop_index, joint, vertex_count)` — the ONE bounds check
/// for both doors (the mirrored `wire.rs` twin was retired at the
/// convention-2 consolidation): save refuses before a byte is
/// written, load refuses the parsed document with the SAME
/// diagnostics (no unloadable file is ever produced, and no file
/// with a stale joint ever replays). Non-canonical
/// (unsorted/duplicated) lists are NOT refused here: they are
/// set-semantic in memory and the wire canonicalizes them; the
/// canonical-set rule on the wire is `wire.rs`'s load-only residue.
fn first_bad_joint(
    snapshot: &ProfileDoc,
    edits: &[DocEdit<ProfileDesc>],
) -> Option<(JointSite, usize, u64, usize)> {
    for (&id, node) in &snapshot.nodes {
        if let Node::Profile(desc) = node
            && let Some((loop_index, joint, vertex_count)) = desc_bad_joint(desc)
        {
            return Some((
                JointSite::Profile { node: id },
                loop_index,
                joint,
                vertex_count,
            ));
        }
    }
    for (index, edit) in edits.iter().enumerate() {
        if let DocEdit::InsertNode {
            node: Node::Profile(desc),
        } = edit
            && let Some((loop_index, joint, vertex_count)) = desc_bad_joint(desc)
        {
            return Some((
                JointSite::InsertedProfile { index },
                loop_index,
                joint,
                vertex_count,
            ));
        }
    }
    None
}

fn desc_bad_joint(desc: &ProfileDesc) -> Option<(usize, u64, usize)> {
    for (loop_index, lp) in desc.0.loops.iter().enumerate() {
        let vertex_count = lp.vertices.len();
        for &j in &lp.tangent_joints {
            if j >= vertex_count {
                return Some((loop_index, j as u64, vertex_count));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]

    use crate::node::RecipeNodeId;
    use crate::persist::{PersistError, SnapshotError, save};
    use crate::profile_desc::ProfileDoc;

    /// Convention 2's point, pinned at the unit level: a document
    /// that would refuse to load cannot be saved. Both corruptions
    /// need `pub(crate)` access — no API door reaches them, only an
    /// in-crate bug would — and before the consolidation both SAVED
    /// fine, producing a file the load door refuses.
    #[test]
    fn structurally_invalid_documents_refuse_at_save() {
        // ε = 0.0 is finite (past the float walk) but invalid — the
        // load door's EpsilonInvalid, now at save too.
        let mut doc = ProfileDoc::empty();
        doc.epsilon = 0.0;
        match save(&doc, &[]) {
            Err(PersistError::Snapshot(SnapshotError::EpsilonInvalid { value })) => {
                assert_eq!(value, 0.0);
            }
            other => panic!("non-positive ε must refuse at save, got {other:?}"),
        }
        // order naming a node the map does not hold.
        let mut doc = ProfileDoc::empty();
        doc.order.push(RecipeNodeId(7));
        match save(&doc, &[]) {
            Err(PersistError::Snapshot(SnapshotError::OrderMismatch)) => {}
            other => panic!("order mismatch must refuse at save, got {other:?}"),
        }
    }
}
