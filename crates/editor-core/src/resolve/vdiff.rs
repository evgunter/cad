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
//! decision sequence a pure function of its inputs.
//!
//! # Alignment: per-predicate sign populations, not positions
//!
//! Two runs of a node are compared PER PREDICATE by their sign
//! POPULATIONS (how many Negative/Zero/Positive verdicts each run
//! recorded), never by log position. Positional pairing — global or
//! per-predicate — is unsound here by the kernel's own rules: the
//! construction order inside an op is itself steered by recorded
//! exact-order predicates, so a legitimate flip can permute the
//! entire remaining decision sequence, and positional pairing then
//! reports pure permutation noise as flips. Populations are
//! permutation-invariant: matched signs cancel, and the residual —
//! signs present in one run but not the other — is exactly the NET
//! verdict change at that node. Residuals pair canonically
//! (ascending sign order, old against new) into [`VerdictFlip`]s
//! with multiplicity; an instance-count change is additionally
//! recorded as a [`PredicateDivergence`] (the predicate ran a
//! different number of times — a structural change, reported, never
//! guessed about).
//!
//! When the decision structure is stable (the ε-audit's common case:
//! same recipe, same construction, a handful of margins re-classified)
//! the residual IS the exact flip list. The documented blind spot:
//! two instances of one predicate trading opposite signs in one node
//! cancel — a pure exchange has no net population change. WITHIN one
//! qualifier group such an exchange also swaps no name STRINGS (order
//! qualifiers keep their rank vocabulary); ACROSS groups it CAN
//! re-qualify names while this engine reports no flip (two fragments'
//! `name_frag_side_of` probes flipping in opposite directions against
//! different partners cancel exactly, yet both names change). A
//! `Vanished` attribution then sees an empty [`FlipSet`] and rests on
//! the later ladder rungs — the recorded-qualifier delta, or the
//! cause-not-in-evidence fallback (`super` module docs, "Low-evidence
//! diagnosis"). The PR 6 audit inherits the caveat with this
//! paragraph as its record.
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

/// One net verdict flip at a node: `count` instances of `predicate`
/// decided `from` in the old run where the new run decided `to`
/// (population residuals, canonically paired — module docs). The
/// pillar's recorded change site, N5's `PredicateFlip` payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerdictFlip {
    /// The predicate that flipped (k_stats static name).
    pub predicate: &'static str,
    /// Its net old-run sign.
    pub from: Sign,
    /// Its net new-run sign.
    pub to: Sign,
    /// How many instances made this transition (net).
    pub count: u32,
}

/// A predicate whose instance count differs between the two runs at
/// one node: the decision structure changed there (the predicate ran
/// a different number of times). Reported alongside any pairable
/// net flips, never silently absorbed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PredicateDivergence {
    /// The structurally-diverged predicate.
    pub predicate: &'static str,
    /// Its instance count in the old run.
    pub old_count: u32,
    /// Its instance count in the new run.
    pub new_count: u32,
}

/// One node's verdict delta between two runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeVerdictDelta {
    /// The node's standing in the old run.
    pub old_status: RunStatus,
    /// The node's standing in the new run.
    pub new_status: RunStatus,
    /// Net sign changes, ordered by predicate name then (from, to)
    /// (deterministic).
    pub flips: Vec<VerdictFlip>,
    /// Count-changed predicates, ordered by name.
    pub diverged: Vec<PredicateDivergence>,
}

impl NodeVerdictDelta {
    /// Whether this delta records any difference at all.
    pub fn is_empty(&self) -> bool {
        self.old_status == self.new_status && self.flips.is_empty() && self.diverged.is_empty()
    }
}

/// The diff engine's output: per-node verdict deltas, only for nodes
/// where SOMETHING differs (statuses, flips, or divergence).
/// Deterministic by construction (`BTreeMap`, canonical order within
/// nodes).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FlipSet {
    /// The differing nodes' deltas, ascending by node id.
    pub nodes: BTreeMap<RecipeNodeId, NodeVerdictDelta>,
}

impl FlipSet {
    /// True when the two runs' verdict populations (and node
    /// standings) are identical — the no-flip certificate.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// The flips recorded at one node (empty when none).
    pub fn flips_at(&self, node: RecipeNodeId) -> &[VerdictFlip] {
        self.nodes.get(&node).map_or(&[], |d| d.flips.as_slice())
    }

    /// Every flip, deterministically ordered (node id ascending, then
    /// the per-node canonical order) — the `SetTolerance` audit's
    /// "exactly the flipped predicates" report (PR 6 wires the
    /// recorded-ε edit to this; the ambient-ε mechanism feeds it now).
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

const SIGNS: [Sign; 3] = [Sign::Negative, Sign::Zero, Sign::Positive];

fn sign_ix(s: Sign) -> usize {
    match s {
        Sign::Negative => 0,
        Sign::Zero => 1,
        Sign::Positive => 2,
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
            diverged: Vec::new(),
        };
        if let (Some(a), Some(b)) = (old.value(id), new.value(id))
            && a.verdicts != b.verdicts
        {
            // Per-predicate sign populations (module docs).
            let populate = |log: &[geom_core::k_stats::Verdict]| {
                let mut m: BTreeMap<&'static str, [u32; 3]> = BTreeMap::new();
                for v in log {
                    m.entry(v.predicate).or_default()[sign_ix(v.sign)] += 1;
                }
                m
            };
            let (pa, pb) = (populate(&a.verdicts), populate(&b.verdicts));
            let predicates: BTreeSet<&'static str> = pa.keys().chain(pb.keys()).copied().collect();
            for predicate in predicates {
                let ca = pa.get(predicate).copied().unwrap_or_default();
                let cb = pb.get(predicate).copied().unwrap_or_default();
                if ca == cb {
                    continue;
                }
                // Residuals after matched signs cancel.
                let mut old_surplus = Vec::new();
                let mut new_surplus = Vec::new();
                for s in SIGNS {
                    let (x, y) = (ca[sign_ix(s)], cb[sign_ix(s)]);
                    if x > y {
                        old_surplus.push((s, x - y));
                    } else if y > x {
                        new_surplus.push((s, y - x));
                    }
                }
                // Canonical pairing: ascending sign order both sides;
                // grouped runs zip into net flips with multiplicity.
                let mut oi = old_surplus.into_iter();
                let mut ni = new_surplus.into_iter();
                let (mut o, mut n) = (oi.next(), ni.next());
                while let (Some((fs, fc)), Some((ts, tc))) = (o, n) {
                    let take = fc.min(tc);
                    delta.flips.push(VerdictFlip {
                        predicate,
                        from: fs,
                        to: ts,
                        count: take,
                    });
                    o = if fc > take {
                        Some((fs, fc - take))
                    } else {
                        oi.next()
                    };
                    n = if tc > take {
                        Some((ts, tc - take))
                    } else {
                        ni.next()
                    };
                }
                // Unbalanced totals = an instance-count change.
                let (ta, tb) = (ca.iter().sum::<u32>(), cb.iter().sum::<u32>());
                if ta != tb {
                    delta.diverged.push(PredicateDivergence {
                        predicate,
                        old_count: ta,
                        new_count: tb,
                    });
                }
            }
        }
        if !delta.is_empty() {
            out.insert(id, delta);
        }
    }
    FlipSet { nodes: out }
}
