//! The verdict-vector diff engine (M4 PR 4 spec D2) — built **once**.
//!
//! One call signature: [`diff_verdicts`]`(old-run, new-run)` → the
//! [`FlipSet`], localizable to derivation paths. The engine is
//! deliberately cause-agnostic: the same diff runs whether the two
//! runs differ by a parameter edit, an ε change ([`SetTolerance`]'s
//! audit — PR 6 reuses this untouched), or a recipe edit. Its
//! consumers in this PR are (a) `Diagnosis::PredicateFlip`
//! attribution for `Vanished` names and (b) the `SetTolerance`
//! apply/report semantics ([`FlipSet::report`]).
//!
//! Why the diff is well-defined (N5): both evaluations carry their
//! verdict logs ([`crate::eval::NodeValue::verdicts`] — k_stats names
//! and definite signs), and D9 replay determinism makes each node's
//! decision sequence a pure function of its inputs. Two runs of the
//! same node therefore produce logs that are positionally comparable
//! until the first structural divergence; a sign change at a matched
//! position IS a predicate flip, the pillar's recorded change site.
//!
//! [`SetTolerance`]: crate::edit::DocEdit

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Decide, Sign};

use crate::eval::{Evaluation, NodeResult};
use crate::names::StableName;
use crate::node::RecipeNodeId;

use super::derivation_nodes;

/// A node's standing in one run, as the diff engine sees it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    /// Evaluated to a value (has a verdict log).
    Ok,
    /// Failed typed.
    Failed,
    /// Poisoned by an upstream failure.
    Poisoned,
    /// No result in the run (canceled suffix, or the node does not
    /// exist in that run's document).
    Absent,
}

/// One verdict flip: the same decision site (same position, same
/// predicate name) classified to different signs in the two runs —
/// the pillar's recorded change site, N5's `PredicateFlip` payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerdictFlip {
    /// Position in the node's decision order (both logs).
    pub index: u32,
    /// The predicate that flipped (k_stats static name).
    pub predicate: &'static str,
    /// Its sign in the old run.
    pub from: Sign,
    /// Its sign in the new run.
    pub to: Sign,
}

/// One node's verdict delta between two runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeVerdictDelta {
    /// The node's standing in the old run.
    pub old_status: RunStatus,
    /// The node's standing in the new run.
    pub new_status: RunStatus,
    /// Sign changes at matched decision sites, in decision order.
    pub flips: Vec<VerdictFlip>,
    /// The first log position where the two decision sequences stop
    /// being positionally comparable (different predicate name, or
    /// one log ends): the node's decision STRUCTURE changed there —
    /// downstream positions are not compared (they would pair
    /// unrelated decisions). `None` when the sequences align end to
    /// end.
    pub diverged: Option<u32>,
}

impl NodeVerdictDelta {
    /// Whether this delta records any difference at all.
    pub fn is_empty(&self) -> bool {
        self.old_status == self.new_status && self.flips.is_empty() && self.diverged.is_none()
    }
}

/// The diff engine's output: per-node verdict deltas, only for nodes
/// where SOMETHING differs (statuses, flips, or divergence).
/// Deterministic by construction (`BTreeMap`, decision order within
/// nodes).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FlipSet {
    /// The differing nodes' deltas, ascending by node id.
    pub nodes: BTreeMap<RecipeNodeId, NodeVerdictDelta>,
}

impl FlipSet {
    /// True when the two runs' verdict vectors (and node standings)
    /// are identical — the no-flip certificate.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// The flips recorded at one node (empty when none).
    pub fn flips_at(&self, node: RecipeNodeId) -> &[VerdictFlip] {
        self.nodes.get(&node).map_or(&[], |d| d.flips.as_slice())
    }

    /// Every flip, deterministically ordered (node id ascending, then
    /// decision order) — the `SetTolerance` audit's "exactly the
    /// flipped predicates" report (PR 6 wires the recorded-ε edit to
    /// this; the ambient-ε mechanism feeds it now).
    pub fn report(&self) -> Vec<(RecipeNodeId, VerdictFlip)> {
        self.nodes
            .iter()
            .flat_map(|(&id, d)| d.flips.iter().map(move |f| (id, *f)))
            .collect()
    }

    /// The flips at a restricted node set (localization primitive).
    pub fn flips_on_nodes(
        &self,
        nodes: &BTreeSet<RecipeNodeId>,
    ) -> Vec<(RecipeNodeId, VerdictFlip)> {
        self.nodes
            .iter()
            .filter(|(id, _)| nodes.contains(id))
            .flat_map(|(&id, d)| d.flips.iter().map(move |f| (id, *f)))
            .collect()
    }

    /// The flips localized to a name's derivation path (spec D2:
    /// "localizable to derivation paths"): flips at the name's
    /// minting node, at every node an embedded operand name derives
    /// from, and at every discriminator partner's node.
    pub fn flips_on_path(&self, name: &StableName) -> Vec<(RecipeNodeId, VerdictFlip)> {
        self.flips_on_nodes(&derivation_nodes(name))
    }
}

fn status<T: Decide>(run: &Evaluation<T>, id: RecipeNodeId) -> RunStatus {
    match run.nodes.get(&id) {
        Some(NodeResult::Ok(_)) => RunStatus::Ok,
        Some(NodeResult::Failed(_)) => RunStatus::Failed,
        Some(NodeResult::Poisoned { .. }) => RunStatus::Poisoned,
        None => RunStatus::Absent,
    }
}

/// Diffs two evaluations' verdict logs (spec D2 — THE engine, built
/// once). Scalar-generic on BOTH sides independently: verdict logs
/// are scalar-independent data, so f64 runs, Interval runs, and
/// mixtures diff identically; the caller chooses what the two runs
/// differ by (parameter edit, ε change, recipe edit) and this
/// function does not care.
pub fn diff_verdicts<T: Decide, U: Decide>(old: &Evaluation<T>, new: &Evaluation<U>) -> FlipSet {
    let ids: BTreeSet<RecipeNodeId> = old.nodes.keys().chain(new.nodes.keys()).copied().collect();
    let mut out = BTreeMap::new();
    for id in ids {
        let (old_status, new_status) = (status(old, id), status(new, id));
        let mut delta = NodeVerdictDelta {
            old_status,
            new_status,
            flips: Vec::new(),
            diverged: None,
        };
        if let (Some(a), Some(b)) = (old.value(id), new.value(id)) {
            let (la, lb) = (a.verdicts.as_slice(), b.verdicts.as_slice());
            let n = la.len().min(lb.len());
            for (i, (va, vb)) in la.iter().zip(lb.iter()).enumerate() {
                if va.predicate != vb.predicate {
                    delta.diverged = Some(i as u32);
                    break;
                }
                if va.sign != vb.sign {
                    delta.flips.push(VerdictFlip {
                        index: i as u32,
                        predicate: va.predicate,
                        from: va.sign,
                        to: vb.sign,
                    });
                }
            }
            if delta.diverged.is_none() && la.len() != lb.len() {
                delta.diverged = Some(n as u32);
            }
        }
        if !delta.is_empty() {
            out.insert(id, delta);
        }
    }
    FlipSet { nodes: out }
}
