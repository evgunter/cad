//! Structural document diff (spec D7): node-granular adds/removes/
//! changes plus variable and metadata deltas — the primitive PR 6's
//! `SetTolerance` audit and the naming layer's edit diagnosis will
//! consume. Deliberately NO expression-level cleverness yet.

use crate::doc::Doc;
use crate::node::RecipeNodeId;
use crate::var::VarId;

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
    /// Node-level changes in document order: `self`'s nodes as `self`
    /// orders them, then the added ones as `other` orders them.
    pub nodes: Vec<NodeChange>,
    /// Variables added, removed, or whose definition changed — `self`'s
    /// as `self` declared them, then the added ones as `other` declared
    /// them, as [`Self::nodes`] is ordered. A name is not a definition:
    /// a variable whose name alone moved is not here (VR2).
    pub vars: Vec<VarId>,
    /// Whether the two insertion orders differ (reorder is not an
    /// edit in v1, but the diff reports it rather than assuming).
    pub order_changed: bool,
    /// Whether recorded ε differs (bit comparison; ε edits are PR 6).
    pub epsilon_changed: bool,
    /// Nodes whose recorded witness datum was added, removed, or
    /// changed (M4 PR 4; witness bytes are exact data), in the order
    /// [`Self::nodes`] uses.
    pub witnesses: Vec<RecipeNodeId>,
    /// Whether the metadata maps differ.
    pub metadata_changed: bool,
    /// Whether the appearance stores differ (attribute values are
    /// float-free, so structural comparison is bit comparison).
    pub appearance_changed: bool,
    /// Nodes whose label was added, removed, or changed, in the order
    /// [`Self::nodes`] uses.
    pub labels: Vec<RecipeNodeId>,
}

impl DocDiff {
    /// True when the documents are equal at this diff's granularity.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
            && self.vars.is_empty()
            && !self.order_changed
            && !self.epsilon_changed
            && self.witnesses.is_empty()
            && !self.metadata_changed
            && !self.appearance_changed
            && self.labels.is_empty()
    }
}

impl<P: PartialEq + crate::ProfilePayload> Doc<P> {
    /// Node-granular structural diff, `self` → `other` (spec D7).
    pub fn diff(&self, other: &Doc<P>) -> DocDiff {
        let mut nodes = Vec::new();
        for &id in &self.order {
            let Some(node) = self.nodes.get(&id) else {
                continue;
            };
            match other.nodes.get(&id) {
                None => nodes.push(NodeChange::Removed(id)),
                // BIT comparison (review non-blocker): diff is the
                // future SetTolerance-audit substrate and must not be
                // bit-blind — a 0.0 → -0.0 payload change is Changed.
                Some(theirs) if !theirs.bit_eq(node) => nodes.push(NodeChange::Changed(id)),
                Some(_) => {}
            }
        }
        for &id in &other.order {
            if !self.nodes.contains_key(&id) {
                nodes.push(NodeChange::Added(id));
            }
        }
        let vars: Vec<VarId> = self
            .var_order
            .iter()
            .filter(|id| {
                let ours = self.vars.get(id);
                !other
                    .vars
                    .get(id)
                    .is_some_and(|theirs| ours.is_some_and(|var| theirs.bit_eq(var)))
            })
            .chain(
                other
                    .var_order
                    .iter()
                    .filter(|id| !self.vars.contains_key(id)),
            )
            .copied()
            .collect();
        let witness_moved = |id: &RecipeNodeId| self.witnesses.get(id) != other.witnesses.get(id);
        let label_moved = |id: &RecipeNodeId| self.labels.get(id) != other.labels.get(id);
        let mut witnesses: Vec<RecipeNodeId> = Vec::new();
        let mut labels: Vec<RecipeNodeId> = Vec::new();
        for &id in self.order.iter().chain(&other.order) {
            if witness_moved(&id) && !witnesses.contains(&id) {
                witnesses.push(id);
            }
            if label_moved(&id) && !labels.contains(&id) {
                labels.push(id);
            }
        }
        DocDiff {
            nodes,
            vars,
            order_changed: self.order != other.order,
            epsilon_changed: self.epsilon.to_bits() != other.epsilon.to_bits(),
            witnesses,
            metadata_changed: self.metadata != other.metadata,
            appearance_changed: self.appearance != other.appearance,
            labels,
        }
    }
}
