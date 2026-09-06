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
//! # The two derived forms, in one module
//!
//! A node's verdict log is the substrate, and this module holds BOTH
//! shapes derived from it: the per-predicate sign POPULATIONS above
//! ([`NodeVerdicts`], [`VerdictSummary`]) and the ordered STRICT form
//! at the foot of this file ([`VerdictVector`], keyed by
//! [`VerdictVectorKey`]). They answer different questions and neither
//! subsumes the other — [`VerdictVector`]'s docs argue the split — but
//! they share an outcome tag ([`RunStatus`]) and they are read off the
//! same runs, so they are argued against each other here rather than
//! drifting apart in two modules. A third shape over this substrate
//! belongs in this file too, or not at all.
//!
//! Only the population form is persisted ([`VerdictSummary`], the ε
//! audit's cross-process seam); the strict form is derived at every
//! use and stored nowhere (E10). A node's escalations
//! ([`crate::eval::NodeValue::escalations`], the frame's other channel)
//! are not persisted either and have no population form: an escalation
//! is a fact about one run at one box, read in hand by the
//! subdivision driver, never a population to compare across runs.
//!
//! [`SetTolerance`]: crate::edit::DocEdit

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Decide, Sign};

use crate::eval::{Evaluation, KeyHasher, NodeResult};
use crate::names::StableName;
use crate::node::RecipeNodeId;

use super::derivation_nodes;

/// A node's standing in one run — the outcome tag BOTH derived forms
/// carry: the population form's [`NodeVerdicts::status`] and the strict
/// form's [`VerdictRow::outcome`]. One enum, read off the `NodeResult`
/// discriminants by [`status`], so the two forms cannot disagree about
/// a node's standing.
///
/// Serializable: it rides in [`VerdictSummary`], the cross-process ε
/// audit's persist-grade seam. Its key tag bytes are chosen at
/// [`VerdictVector::key`], not derived from this enum's shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
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

/// THE sign ladder, in both places a sign is turned into a number: the
/// column a verdict is counted in
/// ([`NodeVerdicts::populations`], indexed Negative/Zero/Positive) and,
/// plus one, the tag byte [`VerdictVector::key`] writes for it. The
/// offset is what the minted keys carry, so it is fixed rather than
/// free; the ladder itself is written once so the two cannot drift.
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
            let (flips, diverged) = diff_populations(&pa, &pb);
            delta.flips = flips
                .into_iter()
                .map(|(predicate, from, to, count)| VerdictFlip {
                    predicate,
                    from,
                    to,
                    count,
                })
                .collect();
            delta.diverged = diverged
                .into_iter()
                .map(|(predicate, old_count, new_count)| PredicateDivergence {
                    predicate,
                    old_count,
                    new_count,
                })
                .collect();
        }
        if !delta.is_empty() {
            out.insert(id, delta);
        }
    }
    FlipSet { nodes: out }
}

/// THE population-diff core (module docs): per-predicate sign
/// populations cancel matched signs; residuals pair canonically
/// (ascending sign order, old against new) into net flips with
/// multiplicity; unbalanced totals report as divergence. Generic over
/// the predicate-name type so the in-process engine (`&'static str`,
/// [`diff_verdicts`]) and the persist-grade cross-process audit
/// (`String`, [`diff_summaries`]) are the SAME math — built once,
/// spec D2.
#[allow(clippy::type_complexity)]
fn diff_populations<N: Ord + Clone>(
    pa: &BTreeMap<N, [u32; 3]>,
    pb: &BTreeMap<N, [u32; 3]>,
) -> (Vec<(N, Sign, Sign, u32)>, Vec<(N, u32, u32)>) {
    let mut flips = Vec::new();
    let mut diverged = Vec::new();
    let predicates: BTreeSet<&N> = pa.keys().chain(pb.keys()).collect();
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
        // Canonical pairing: ascending sign order both sides; grouped
        // runs zip into net flips with multiplicity.
        let mut oi = old_surplus.into_iter();
        let mut ni = new_surplus.into_iter();
        let (mut o, mut n) = (oi.next(), ni.next());
        while let (Some((fs, fc)), Some((ts, tc))) = (o, n) {
            let take = fc.min(tc);
            flips.push((predicate.clone(), fs, ts, take));
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
            diverged.push((predicate.clone(), ta, tb));
        }
    }
    (flips, diverged)
}

/// One node's verdict POPULATIONS in one run — status plus, for each
/// predicate name, its (Negative, Zero, Positive) instance counts.
/// The persist-grade projection of a run's verdict log: everything
/// the population diff needs, nothing else (no values — N2).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeVerdicts {
    /// The node's standing.
    pub status: RunStatus,
    /// Per-predicate sign populations, indexed Negative/Zero/Positive.
    #[serde(with = "crate::persist::strict::populations")]
    pub populations: BTreeMap<String, [u32; 3]>,
}

/// A whole run's verdict populations by node (M4 PR 6 spec D4): the
/// SERIALIZABLE run summary the SetTolerance audit ships between
/// processes. One process hosts one ε, so the two runs of an ε audit
/// can never share an address space; each run summarizes itself with
/// [`verdict_summary`], and [`diff_summaries`] — the same population
/// math as [`diff_verdicts`], via one shared core — reports exactly
/// the flipped predicates.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerdictSummary {
    /// Per-node populations (every node in the run's order).
    #[serde(with = "crate::persist::strict::summary_nodes")]
    pub nodes: BTreeMap<RecipeNodeId, NodeVerdicts>,
}

/// Projects an evaluation onto its serializable verdict populations.
pub fn verdict_summary<T: Decide>(run: &Evaluation<T>) -> VerdictSummary {
    let mut nodes = BTreeMap::new();
    for &id in &run.order {
        let mut populations: BTreeMap<String, [u32; 3]> = BTreeMap::new();
        if let Some(v) = run.value(id) {
            for verdict in v.verdicts.iter() {
                populations.entry(verdict.predicate.to_owned()).or_default()
                    [sign_ix(verdict.sign)] += 1;
            }
        }
        nodes.insert(
            id,
            NodeVerdicts {
                status: status(run, id),
                populations,
            },
        );
    }
    VerdictSummary { nodes }
}

/// One net verdict flip in a summary diff ([`VerdictFlip`]'s
/// owned-name twin; the audit report's row).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryFlip {
    /// The predicate that flipped.
    pub predicate: String,
    /// Its net old-run sign.
    pub from: Sign,
    /// Its net new-run sign.
    pub to: Sign,
    /// How many instances made this transition (net).
    pub count: u32,
}

/// A count-diverged predicate in a summary diff
/// ([`PredicateDivergence`]'s owned-name twin).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryDivergence {
    /// The structurally-diverged predicate.
    pub predicate: String,
    /// Its instance count in the old run.
    pub old_count: u32,
    /// Its instance count in the new run.
    pub new_count: u32,
}

/// One node's delta between two summarized runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryDelta {
    /// The node's standing in the old run (`Absent` when the node has
    /// no row there).
    pub old_status: RunStatus,
    /// The node's standing in the new run.
    pub new_status: RunStatus,
    /// Net sign changes, canonical order.
    pub flips: Vec<SummaryFlip>,
    /// Count-changed predicates, ordered by name.
    pub diverged: Vec<SummaryDivergence>,
}

/// The summary diff's output — [`FlipSet`]'s cross-process twin.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SummaryFlipSet {
    /// The differing nodes' deltas, ascending by node id.
    pub nodes: BTreeMap<RecipeNodeId, SummaryDelta>,
}

impl SummaryFlipSet {
    /// True when nothing differs (the no-flip certificate).
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Every flip, deterministically ordered — "exactly the flipped
    /// predicates", the SetTolerance audit's report (spec D4/D6.2).
    pub fn report(&self) -> Vec<(RecipeNodeId, SummaryFlip)> {
        self.nodes
            .iter()
            .flat_map(|(&id, d)| d.flips.iter().map(move |f| (id, f.clone())))
            .collect()
    }
}

/// Diffs two runs' serialized verdict summaries — the SetTolerance
/// audit's engine call (M4 PR 6 spec D4, discharging PR 4 review
/// Finding 6's wait). Same population math as [`diff_verdicts`]
/// (one shared core, [`diff_populations`]); the summaries typically
/// come from two processes committed to different ε values.
pub fn diff_summaries(old: &VerdictSummary, new: &VerdictSummary) -> SummaryFlipSet {
    let ids: BTreeSet<RecipeNodeId> = old.nodes.keys().chain(new.nodes.keys()).copied().collect();
    let mut out = BTreeMap::new();
    let empty: BTreeMap<String, [u32; 3]> = BTreeMap::new();
    for id in ids {
        let (old_status, pa) = old
            .nodes
            .get(&id)
            .map_or((RunStatus::Absent, &empty), |n| (n.status, &n.populations));
        let (new_status, pb) = new
            .nodes
            .get(&id)
            .map_or((RunStatus::Absent, &empty), |n| (n.status, &n.populations));
        let (flips, diverged) = if old_status == RunStatus::Ok && new_status == RunStatus::Ok {
            diff_populations(pa, pb)
        } else {
            (Vec::new(), Vec::new())
        };
        let delta = SummaryDelta {
            old_status,
            new_status,
            flips: flips
                .into_iter()
                .map(|(predicate, from, to, count)| SummaryFlip {
                    predicate,
                    from,
                    to,
                    count,
                })
                .collect(),
            diverged: diverged
                .into_iter()
                .map(|(predicate, old_count, new_count)| SummaryDivergence {
                    predicate,
                    old_count,
                    new_count,
                })
                .collect(),
        };
        if delta.old_status != delta.new_status
            || !delta.flips.is_empty()
            || !delta.diverged.is_empty()
        {
            out.insert(id, delta);
        }
    }
    SummaryFlipSet { nodes: out }
}

// ---------------------------------------------------------------
// The STRICT form of the same substrate.
// ---------------------------------------------------------------

/// One node's row of a [`VerdictVector`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerdictRow {
    /// The node.
    pub node: RecipeNodeId,
    /// Its standing in the run.
    pub outcome: RunStatus,
    /// Every definite decision it made, in decision order.
    pub verdicts: Vec<geom_core::k_stats::Verdict>,
}

/// An evaluation's verdict vector: one [`VerdictRow`] per node, in the
/// evaluation's deterministic order.
///
/// Float-free, so equality is exact and means what it says. This is the
/// object leaf certification compares, and the ONLY thing it compares.
///
/// # Why this is not [`NodeVerdicts`], one module over
///
/// Two shapes, two questions, over one substrate — a node's verdict
/// log ([`crate::eval::NodeValue::verdicts`]).
///
/// - **"Is this leaf the witness build?"** wants the STRICTEST
///   available test, because a false yes is a false certificate. Order
///   included, outcome tags included, no cancellation anywhere: that is
///   this type, and it is what `drive` gates on.
/// - **"What differs, and what should the report call it?"** wants a
///   test that survives permutation, because construction order inside
///   an op is itself predicate-steered (module docs above) — this
///   module's populations, which `drive::FlipEvidence` carries
///   verbatim.
///
/// The population form is deliberately WEAKER (a pure sign exchange in
/// one node nets to nothing), so it can name but must not gate; this
/// form cannot explain a difference it detects, so it gates but does
/// not name. Neither subsumes the other, and neither is a rounding of
/// the other: `crates/editor-core/tests/props_verdict_shapes.rs` is the
/// asymmetry as two rows — logs the engine calls identical whose keys
/// here differ.
///
/// **Not the persisted shape.** [`VerdictSummary`] is: it is the one
/// form with a serde derive and the one the strict codecs in
/// `crate::persist::strict` carry, because a cross-process ε audit
/// ships populations. This vector is DERIVED at every use, from a run
/// in hand, and stored nowhere (E10); a certified leaf keeps only its
/// [`VerdictVectorKey`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct VerdictVector {
    /// The rows, in evaluation order.
    pub rows: Vec<VerdictRow>,
}

impl VerdictVector {
    /// The vector of an evaluation, in its own `order`.
    ///
    /// Every node of the order gets a row, tagged with [`status`]' four-
    /// way standing — the same standing the population form records.
    /// The verdict rows alone do not separate "this node built and
    /// decided nothing" from "this node failed before deciding
    /// anything", and a certificate that could not tell those apart
    /// would certify a leaf whose build refused; the tag closes that.
    pub fn of<T: Decide>(ev: &Evaluation<T>) -> Self {
        let rows = ev
            .order
            .iter()
            .map(|&node| VerdictRow {
                node,
                outcome: status(ev, node),
                verdicts: ev
                    .value(node)
                    .map_or_else(Vec::new, |v| v.verdicts.to_vec()),
            })
            .collect();
        Self { rows }
    }

    /// The vector's content key — the `verdict_vector_key` a certified
    /// leaf carries. Derived, never persisted (E10).
    pub fn key(&self) -> VerdictVectorKey {
        let mut h = KeyHasher::new();
        h.write_tag(0xE6);
        h.write_u64(self.rows.len() as u64);
        for row in &self.rows {
            h.write_u64(row.node.0);
            // The tag alphabet is hand-written literals, so the enum's
            // declaration order decides nothing: reordering
            // [`RunStatus`] moves no key. `Ok`/`Failed`/`Poisoned` carry
            // the bytes every key already minted was built from, and
            // `Absent` is appended rather than inserted. The match is
            // exhaustive, which is what forces a new variant to be given
            // a byte here before it can reach a key.
            h.write_tag(match row.outcome {
                RunStatus::Ok => 1,
                RunStatus::Failed => 2,
                RunStatus::Poisoned => 3,
                RunStatus::Absent => 4,
            });
            h.write_u64(row.verdicts.len() as u64);
            for v in &row.verdicts {
                h.write_str(v.predicate);
                h.write_tag(sign_tag(v.sign));
            }
        }
        VerdictVectorKey(h.finish().0)
    }
}

/// The identity of a [`VerdictVector`] — what a certified leaf carries
/// instead of a copy of the vector, since every certified leaf's vector
/// is the witness's by definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VerdictVectorKey(pub u128);

fn sign_tag(s: Sign) -> u8 {
    sign_ix(s) as u8 + 1
}
