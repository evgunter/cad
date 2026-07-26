//! The persistence doors' walks (spec D2/D6.3):
//!
//! - [`first_non_finite`] — the SAVE door: every float the format
//!   would write, checked finite, with a typed site name. Expression
//!   literals are finite BY CONSTRUCTION (`Expr::literal` refuses
//!   non-finite — ruled door 1), so the walk covers the float carriers
//!   outside that door: profile payloads, continuous doc params, the
//!   recorded ε, and D7 metadata trees (in the snapshot AND in the
//!   edit log). A NEW float-carrying field must join this walk — the
//!   D2 round-trip property tests are the tripwire.
//! - [`validate_snapshot`] — the LOAD door: a parsed snapshot is not
//!   trusted; the document invariants `apply` maintains are re-checked
//!   structurally before replay.

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

/// The first non-finite float the format would write, or `None`.
pub(crate) fn first_non_finite(
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

/// Re-checks the document invariants on a parsed snapshot.
pub(crate) fn validate_snapshot(doc: &ProfileDoc) -> Result<(), SnapshotError> {
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

/// Where an out-of-range declared-tangent joint sits (save door,
/// review MAJOR-DELTA-1 — the `Index` channel's twin of
/// [`NonFiniteSite`]): the profile payload is `pub`, so a stale joint
/// (e.g. after a vertex deletion in the payload) is API-reachable
/// without passing any edit door.
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

/// The first out-of-range declared-tangent joint the format would
/// write, as `(site, loop_index, joint, vertex_count)` — the save
/// door's twin of the LOAD door's bounds check in `wire.rs`: a file
/// carrying such a joint refuses on load, so save refuses FIRST with
/// the same diagnostics (no unloadable file is ever produced).
/// Non-canonical (unsorted/duplicated) lists are NOT refused here:
/// they are set-semantic in memory and the wire canonicalizes them.
pub(crate) fn first_bad_joint(
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
