//! Structural document diff (spec D7): node-granular adds/removes/
//! changes plus doc-param and metadata deltas — the primitive PR 6's
//! `SetTolerance` audit and the naming layer's edit diagnosis will
//! consume. Deliberately NO expression-level cleverness yet.

use crate::doc::{Doc, ParamName};
use crate::node::RecipeNodeId;

/// One node-level difference (spec D7).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeChange {
    /// Present in `other`, absent in `self`.
    Added(RecipeNodeId),
    /// Present in `self`, absent in `other`.
    Removed(RecipeNodeId),
    /// Present in both with unequal payloads.
    Changed(RecipeNodeId),
}

/// The structural difference between two documents (spec D7).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocDiff {
    /// Node-level changes, ascending by id (deterministic).
    pub nodes: Vec<NodeChange>,
    /// Doc-param names added, removed, or changed (value or
    /// dimension), ascending.
    pub params: Vec<ParamName>,
    /// Whether the two insertion orders differ (reorder is not an
    /// edit in v1, but the diff reports it rather than assuming).
    pub order_changed: bool,
    /// Whether recorded ε differs (bit comparison; ε edits are PR 6).
    pub epsilon_changed: bool,
    /// Nodes whose recorded witness datum was added, removed, or
    /// changed (M4 PR 4; witness bytes are exact data), ascending.
    pub witnesses: Vec<RecipeNodeId>,
    /// Whether the metadata maps differ.
    pub metadata_changed: bool,
    /// Whether the appearance stores differ (attribute values are
    /// float-free, so structural comparison is bit comparison).
    pub appearance_changed: bool,
}

impl DocDiff {
    /// True when the documents are equal at this diff's granularity.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
            && self.params.is_empty()
            && !self.order_changed
            && !self.epsilon_changed
            && self.witnesses.is_empty()
            && !self.metadata_changed
            && !self.appearance_changed
    }
}

impl<P: PartialEq> Doc<P> {
    /// Node-granular structural diff, `self` → `other` (spec D7).
    pub fn diff(&self, other: &Doc<P>) -> DocDiff {
        let mut nodes = Vec::new();
        for (&id, node) in &self.nodes {
            match other.nodes.get(&id) {
                None => nodes.push(NodeChange::Removed(id)),
                // BIT comparison (review non-blocker): diff is the
                // future SetTolerance-audit substrate and must not be
                // bit-blind — a 0.0 → -0.0 payload change is Changed.
                Some(theirs) if !theirs.bit_eq(node) => nodes.push(NodeChange::Changed(id)),
                Some(_) => {}
            }
        }
        for &id in other.nodes.keys() {
            if !self.nodes.contains_key(&id) {
                nodes.push(NodeChange::Added(id));
            }
        }
        nodes.sort_by_key(|c| match *c {
            NodeChange::Added(id) | NodeChange::Removed(id) | NodeChange::Changed(id) => id,
        });
        let mut params = Vec::new();
        for (name, p) in &self.params {
            if !other
                .params
                .get(name)
                .is_some_and(|theirs| theirs.bit_eq(p))
            {
                params.push(name.clone());
            }
        }
        for name in other.params.keys() {
            if !self.params.contains_key(name) {
                params.push(name.clone());
            }
        }
        params.sort();
        params.dedup();
        let mut witnesses = Vec::new();
        for (&id, w) in &self.witnesses {
            if other.witnesses.get(&id) != Some(w) {
                witnesses.push(id);
            }
        }
        for &id in other.witnesses.keys() {
            if !self.witnesses.contains_key(&id) {
                witnesses.push(id);
            }
        }
        witnesses.sort_unstable();
        witnesses.dedup();
        DocDiff {
            nodes,
            params,
            order_changed: self.order != other.order,
            epsilon_changed: self.epsilon.to_bits() != other.epsilon.to_bits(),
            witnesses,
            metadata_changed: self.metadata != other.metadata,
            appearance_changed: self.appearance != other.appearance,
        }
    }
}
