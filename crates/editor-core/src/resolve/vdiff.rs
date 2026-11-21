//! The verdict-vector diff engine (M4 PR 4 spec D2) — built **once**.
//!
//! One call signature: [`diff_verdicts`]`(old-run, new-run)` → the
//! [`FlipSet`], localizable to derivation paths. The engine is
//! deliberately cause-agnostic: the same diff runs whether the two
//! runs differ by a parameter edit, an ε change, or a recipe edit. Its
//! consumers are `Diagnosis::PredicateFlip` attribution for `Vanished`
//! names, and the subdivision driver's and the range reading's
//! `FlipCrossing` evidence.
//!
//! Why the diff is well-defined (N5): both evaluations carry their
//! verdict logs ([`crate::eval::NodeValue::verdicts`] — k_stats names,
//! definite signs and roles), and D9 replay determinism makes each
//! node's decision sequence a pure function of its inputs.
//!
//! # The first turn
//!
//! Every verdict says, from where it was taken, whether it is a
//! **choice** (another sign can change what the node builds or names)
//! or a **check** (it can only make the node refuse):
//! [`geom_core::k_stats::Role`]. Checks are not read. A node's two runs
//! are read along their choices in decision order, and the flip is the
//! FIRST choice they take differently: after an identical stretch of
//! choices, the same predicate decided with the other sign
//! ([`first_turn`]). Up to that choice the two runs took one path, so
//! the turn is where they parted, and everything after it — however it
//! is permuted, whatever it counts — is downstream of it and is not
//! read. Two runs whose choices part any other way (another predicate
//! at the first difference, or one log ending before the other) name
//! no flip: their paths parted at something the log does not state.
//!
//! **One recorded path, and what could fake one.** The stretch before
//! the turn is identical choice for choice, so both runs reached the
//! turn by the same logged decisions, provided no unlogged branch
//! steered what was enumerated in between. A pruning query does exactly
//! that: the boolean's sweep and the census prune pairs by box overlap
//! and decide nothing, so two runs can decide one predicate at one place
//! in their logs about two different pairs. A slid block's union decides
//! a plane parallelism first in both runs, `Zero` overlapping and
//! `Positive` disjoint: no slide turns a plane, so those are two pairs,
//! and read as a turn they would name a flip no decision made. So each
//! `bvh` query marks the candidates it hands on
//! ([`geom_core::k_stats::mark_candidates`], [`crate::eval::NodeValue::marks`]),
//! and the walk reads a choice's candidates beside the choices: two runs
//! whose queries handed on other candidates part there, before any
//! turn, and name no flip.
//!
//! Three pruning tests written at their call sites mark their outcome
//! the same way ([`geom_core::k_stats::mark_outcome`]): the section
//! certificate's face-pair box test (`topo::boolean::ops`' `walk_pairs`),
//! the n-ary union's hull test (`judged_pairs`), and every `escalate`
//! threshold the blend branches on, each through one home in
//! `sweep::blend::reach` (`sure`, `sure_below`, `fine`). The residue is
//! the box-overlap tests and thresholds still written unmarked at their call sites
//! (`work/doctail/hand-written-box-filters-steer-the-first-turn-unmarked.md`
//! lists them), a heuristic's [`geom_core::k_stats::choose`], and an
//! [`geom_core::k_stats::unlogged`] reading. Where one of those steers
//! what follows while every logged decision and mark agrees, a first
//! difference past it can still sit on a different candidate than the
//! one its twin decided.

//! # The two derived forms, in one module
//!
//! A node's verdict log is the substrate, and this module holds BOTH
//! shapes derived from it: the first-turn reading above ([`FlipSet`])
//! and the ordered STRICT form at the foot of this file
//! ([`VerdictVector`], keyed by [`VerdictVectorKey`]). They answer
//! different questions and neither subsumes the other —
//! [`VerdictVector`]'s docs argue the split — but they share an outcome
//! tag ([`RunStatus`]) and they are read off the same runs, so they are
//! argued against each other here rather than drifting apart in two
//! modules.
//!
//! The ε audit's cross-process seam is a third projection, the
//! per-predicate sign POPULATIONS ([`VerdictSummary`]), persisted
//! because one process hosts one ε. Its diff ([`diff_summaries`])
//! reports where counts moved between signs ([`SummaryShift`]), which
//! is not a flip: instances of one predicate can trade signs (a through
//! slot becoming a blind pocket trades convex corners for concave ones)
//! with no decision turning. A node's escalations
//! ([`crate::eval::NodeValue::escalations`], the frame's other channel)
//! are not persisted either and have no population form: an escalation
//! is a fact about one run at one box, read in hand by the subdivision
//! driver, never a population to compare across runs.

use std::collections::{BTreeMap, BTreeSet};

use geom_core::k_stats::{Mark, MarkItem, Role, Verdict};
use geom_core::{Decide, Sign};

use crate::eval::{Evaluation, KeyHasher, NodeStanding};
use crate::names::StableName;
use crate::node::RecipeNodeId;

use super::derivation_nodes;

/// A node's outcome in one run — the tag every derived form carries:
/// [`NodeVerdictDelta`]'s, the population form's
/// [`NodeVerdicts::status`] and the strict form's
/// [`VerdictRow::outcome`]. `Ok`, or the kind of the node's
/// [`NodeStanding`] (`Absent` is its `NotEvaluated` and its
/// `NotInDocument`), read off [`Evaluation::usable`] by [`status`], so
/// the forms cannot disagree about a node's standing.
///
/// Serializable: it rides in [`VerdictSummary`], the cross-process ε
/// audit's persist-grade seam. Its key tag bytes are chosen at
/// [`VerdictVector::key`], not derived from this enum's shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

/// **A node's flip**: the first choice its two runs take differently,
/// where they part on one predicate's sign ([`first_turn`]). The
/// pillar's recorded change site, N5's `PredicateFlip` payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerdictFlip {
    /// The predicate that turned (k_stats static name).
    pub predicate: &'static str,
    /// The sign the old run decided.
    pub from: Sign,
    /// The sign the new run decided.
    pub to: Sign,
}

/// **One run's log as the first-turn reading walks it**: a node's
/// verdicts and the candidate marks placed among them
/// ([`crate::eval::NodeValue::verdicts`], [`crate::eval::NodeValue::marks`]).
#[derive(Debug, Clone, Copy)]
pub struct Path<'a> {
    /// The verdicts, in decision order.
    pub verdicts: &'a [Verdict],
    /// The candidates the run's pruning queries handed on.
    pub marks: &'a [Mark],
}

/// One step along a [`Path`]'s choices: a choice, or a candidate a
/// choice-feeding query handed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Choice(&'static str, Sign),
    Candidate(&'static str, MarkItem),
}

impl Path<'_> {
    /// The choices and the choices' candidates, in the order the run met
    /// them. Checks, and the candidates checks examined, are not read.
    fn steps(self) -> Vec<Step> {
        let mut marks = self
            .marks
            .iter()
            .filter(|m| m.role == Role::Choice)
            .peekable();
        let mut out = Vec::new();
        for (k, v) in self.verdicts.iter().enumerate() {
            while let Some(m) = marks.next_if(|m| m.before <= k) {
                out.push(Step::Candidate(m.site, m.item));
            }
            if v.role == Role::Choice {
                out.push(Step::Choice(v.predicate, v.sign));
            }
        }
        out.extend(marks.map(|m| Step::Candidate(m.site, m.item)));
        out
    }
}

/// **How two runs' paths compare** ([`first_turn`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirstTurn {
    /// The choices and their candidates agree throughout.
    Agree,
    /// The first difference is one choice decided with the other sign:
    /// the flip.
    Turned(VerdictFlip),
    /// The paths part at something that is no turn of a sign: another
    /// predicate, a candidate a pruning test handed on or did not, or one
    /// path ending first. The log shows where; no flip is named.
    Parted,
}

impl FirstTurn {
    /// The flip, where the paths turned.
    #[must_use]
    pub fn flip(self) -> Option<VerdictFlip> {
        match self {
            Self::Turned(flip) => Some(flip),
            Self::Agree | Self::Parted => None,
        }
    }
}

/// **The first turn between two runs' logs** (module docs): the first
/// step the two take differently, read along their choices and their
/// choices' candidates in the order each run met them. It is a flip
/// where it is the same predicate decided with the other sign, and a
/// parting otherwise ([`FirstTurn`]). Checks are not read.
#[must_use]
pub fn first_turn(old: Path<'_>, new: Path<'_>) -> FirstTurn {
    let (old, new) = (old.steps(), new.steps());
    match old.iter().zip(&new).find(|(a, b)| a != b) {
        Some((&Step::Choice(predicate, from), &Step::Choice(other, to))) if predicate == other => {
            FirstTurn::Turned(VerdictFlip {
                predicate,
                from,
                to,
            })
        }
        Some(_) => FirstTurn::Parted,
        None if old.len() == new.len() => FirstTurn::Agree,
        None => FirstTurn::Parted,
    }
}

/// One node's verdict delta between two runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeVerdictDelta {
    /// The node's standing in the old run.
    pub old_status: RunStatus,
    /// The node's standing in the new run.
    pub new_status: RunStatus,
    /// The node's flip, where both runs hold its log and their choices
    /// part on a sign ([`first_turn`]).
    pub flip: Option<VerdictFlip>,
}

/// The diff engine's output: per-node deltas, only for nodes whose
/// standing differs or that flipped. Deterministic by construction
/// (`BTreeMap`).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FlipSet {
    /// The differing nodes' deltas, ascending by node id.
    pub nodes: BTreeMap<RecipeNodeId, NodeVerdictDelta>,
}

impl FlipSet {
    /// True when no node's standing differs and none flipped.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// The flip at one node, if it flipped.
    pub fn flip_at(&self, node: RecipeNodeId) -> Option<VerdictFlip> {
        self.nodes.get(&node).and_then(|d| d.flip)
    }

    /// Every flip, in node id order.
    pub fn report(&self) -> Vec<(RecipeNodeId, VerdictFlip)> {
        self.nodes
            .iter()
            .filter_map(|(&id, d)| d.flip.map(|f| (id, f)))
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
            .filter_map(|(&id, d)| d.flip.map(|f| (id, f)))
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

/// A node value's [`Path`].
fn path<T: topo::AtRestPolicy + Decide>(value: &crate::eval::NodeValue<T>) -> Path<'_> {
    Path {
        verdicts: &value.verdicts,
        marks: &value.marks,
    }
}

fn status<T: topo::AtRestPolicy + Decide>(run: &Evaluation<T>, id: RecipeNodeId) -> RunStatus {
    match run.usable(id) {
        Ok(_) => RunStatus::Ok,
        Err(NodeStanding::Failed { .. }) => RunStatus::Failed,
        Err(NodeStanding::Poisoned { .. }) => RunStatus::Poisoned,
        Err(NodeStanding::NotEvaluated { .. } | NodeStanding::NotInDocument { .. }) => {
            RunStatus::Absent
        }
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
/// once): each node's standing in the two runs and, where both hold its
/// log, its [`first_turn`]. Scalar-generic on BOTH sides independently,
/// and the caller chooses what the two runs differ by (parameter edit, ε
/// change, recipe edit). The verdicts are float-free and a node logs
/// the same passes at every scalar (a profile's `f64` lane logs the
/// second pass it skips, [`crate::ProfileLane`]), but the candidate
/// marks a path carries are NOT scalar-independent: an Interval run's
/// boxes cover its whole parameter box, so its pruning queries can hand
/// on candidates an f64 run at one point of that box prunes. An f64 run
/// diffed against an Interval one can therefore part on a mark where
/// their choices alone would agree, and names no flip there
/// ([`FirstTurn::Parted`]); a mixed diff that finds a flip found one on
/// one path.
///
/// **It keeps only the turns.** A node whose paths part at a mark or at
/// another predicate ([`FirstTurn::Parted`]) records no flip here, the
/// same as one whose paths agree: the resolver's lanes and the drive's
/// evidence read flips, and whether to name the parting site itself (the
/// query and the candidate where two runs parted) as a diagnosis is a
/// question raised to Ev on PR 7114 and not yet ruled. Until it is, a
/// reader that needs the parting calls [`first_turn`].
pub fn diff_verdicts<T: topo::AtRestPolicy + Decide, U: topo::AtRestPolicy + Decide>(
    old: &Evaluation<T>,
    new: &Evaluation<U>,
) -> FlipSet {
    let ids: BTreeSet<RecipeNodeId> = old.nodes.keys().chain(new.nodes.keys()).copied().collect();
    let mut out = BTreeMap::new();
    for id in ids {
        let (old_status, new_status) = (status(old, id), status(new, id));
        let flip = match (old.value(id), new.value(id)) {
            (Some(a), Some(b)) => first_turn(path(a), path(b)).flip(),
            _ => None,
        };
        if old_status != new_status || flip.is_some() {
            out.insert(
                id,
                NodeVerdictDelta {
                    old_status,
                    new_status,
                    flip,
                },
            );
        }
    }
    FlipSet { nodes: out }
}

/// A verdict log's per-predicate sign populations, checks and choices
/// alike — the projection [`verdict_summary`] persists and
/// [`diff_populations`] compares.
fn populations(log: &[Verdict]) -> BTreeMap<&'static str, [u32; 3]> {
    let mut m: BTreeMap<&'static str, [u32; 3]> = BTreeMap::new();
    for v in log {
        m.entry(v.predicate).or_default()[sign_ix(v.sign)] += 1;
    }
    m
}

/// The population diff ([`diff_summaries`]): per-predicate sign
/// populations cancel matched signs; residuals pair canonically
/// (ascending sign order, old against new) into shifts with
/// multiplicity; unbalanced totals report as divergence. A shift is
/// where counts moved, not a decision that turned (module docs).
#[allow(clippy::type_complexity)]
fn diff_populations(
    pa: &BTreeMap<String, [u32; 3]>,
    pb: &BTreeMap<String, [u32; 3]>,
) -> (Vec<(String, Sign, Sign, u32)>, Vec<(String, u32, u32)>) {
    let mut shifts = Vec::new();
    let mut diverged = Vec::new();
    let predicates: BTreeSet<&String> = pa.keys().chain(pb.keys()).collect();
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
        // runs zip into shifts with multiplicity.
        let mut oi = old_surplus.into_iter();
        let mut ni = new_surplus.into_iter();
        let (mut o, mut n) = (oi.next(), ni.next());
        while let (Some((fs, fc)), Some((ts, tc))) = (o, n) {
            let take = fc.min(tc);
            shifts.push((predicate.clone(), fs, ts, take));
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
    (shifts, diverged)
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
/// [`verdict_summary`], and [`diff_summaries`] reports where each
/// predicate's sign counts moved.
#[derive(Debug, Clone, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerdictSummary {
    /// Per-node populations (every node in the run's order).
    #[serde(with = "crate::persist::strict::summary_nodes")]
    pub nodes: BTreeMap<RecipeNodeId, NodeVerdicts>,
}

/// Projects an evaluation onto its serializable verdict populations.
pub fn verdict_summary<T: topo::AtRestPolicy + Decide>(run: &Evaluation<T>) -> VerdictSummary {
    let mut nodes = BTreeMap::new();
    for &id in &run.order {
        let populations: BTreeMap<String, [u32; 3]> = run
            .value(id)
            .map(|v| {
                populations(&v.verdicts)
                    .into_iter()
                    .map(|(name, counts)| (name.to_owned(), counts))
                    .collect()
            })
            .unwrap_or_default();
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

/// **Where one predicate's counts moved between signs** in a summary
/// diff: `count` more instances decided `to` in the new run, and that
/// many fewer `from`, net (the audit report's row). Not a flip: the
/// instances may be different decisions, one gone and another of the
/// other sign come (module docs).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SummaryShift {
    /// The predicate whose counts moved.
    pub predicate: String,
    /// The sign the old run decided more often.
    pub from: Sign,
    /// The sign the new run decided more often.
    pub to: Sign,
    /// How many instances moved (net).
    pub count: u32,
}

/// A predicate whose instance count differs between two summarized
/// runs at one node: it ran a different number of times.
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
    /// Net count moves between signs, canonical order.
    pub shifts: Vec<SummaryShift>,
    /// Count-changed predicates, ordered by name.
    pub diverged: Vec<SummaryDivergence>,
}

/// The summary diff's output: per node, where its counts moved.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SummaryDiff {
    /// The differing nodes' deltas, ascending by node id.
    pub nodes: BTreeMap<RecipeNodeId, SummaryDelta>,
}

impl SummaryDiff {
    /// True when nothing differs: no standing, no count.
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Every shift, deterministically ordered — the SetTolerance audit's
    /// report (spec D4/D6.2).
    pub fn report(&self) -> Vec<(RecipeNodeId, SummaryShift)> {
        self.nodes
            .iter()
            .flat_map(|(&id, d)| d.shifts.iter().map(move |f| (id, f.clone())))
            .collect()
    }
}

/// Diffs two runs' serialized verdict summaries — the SetTolerance
/// audit's engine call (M4 PR 6 spec D4): where each node's counts
/// moved ([`diff_populations`]). The summaries typically come from two
/// processes committed to different ε values.
pub fn diff_summaries(old: &VerdictSummary, new: &VerdictSummary) -> SummaryDiff {
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
        let (shifts, diverged) = if old_status == RunStatus::Ok && new_status == RunStatus::Ok {
            diff_populations(pa, pb)
        } else {
            (Vec::new(), Vec::new())
        };
        let delta = SummaryDelta {
            old_status,
            new_status,
            shifts: shifts
                .into_iter()
                .map(|(predicate, from, to, count)| SummaryShift {
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
            || !delta.shifts.is_empty()
            || !delta.diverged.is_empty()
        {
            out.insert(id, delta);
        }
    }
    SummaryDiff { nodes: out }
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
    /// Every definite decision it made, choices and checks, in decision
    /// order.
    pub verdicts: Vec<Verdict>,
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
///   included, roles included, checks included, outcome tags included,
///   no cancellation anywhere: that is this type, and it is what `drive`
///   gates on.
/// - **"What differs, and what should the report call it?"** wants the
///   first decision that steered the two runs apart — this module's
///   first turn ([`first_turn`]), which `drive::FlipEvidence` carries
///   verbatim.
///
/// The first-turn reading is deliberately WEAKER (it reads no check, and
/// nothing past the first difference), so it can name but must not
/// gate; this form cannot explain a difference it detects, so it gates
/// but does not name. Neither subsumes the other, and neither is a
/// rounding of the other: `crates/editor-core/tests/props_verdict_shapes.rs`
/// is the asymmetry as rows — logs the engine names no flip in whose
/// keys here differ.
///
/// **Why the certificate omits the candidate marks.** A leaf is the
/// witness build when its decisions are the witness's, row for row:
/// every decision a pruning query steered is among them, so pruning that
/// differs shows here as rows that differ, and a mark would add nothing
/// a false yes could slip past. A mark is not scalar-independent either
/// (an Interval leaf's boxes cover its box, so its queries can hand on
/// candidates the f64 witness prunes, then decide nothing about them),
/// and hashing marks would refuse leaves that decided exactly what the
/// witness decided. The marks are the naming reading's: they keep the
/// first turn from naming a flip between two different candidates, which
/// a gate that compares whole logs never does.
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
    pub fn of<T: topo::AtRestPolicy + Decide>(ev: &Evaluation<T>) -> Self {
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
            h.write_id(row.node.0);
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
                h.write_tag(match v.role {
                    Role::Choice => 1,
                    Role::Check => 2,
                });
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

/// The key's bits in hex, as a goldened report spells it.
impl core::fmt::LowerHex for VerdictVectorKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.0, f)
    }
}

fn sign_tag(s: Sign) -> u8 {
    sign_ix(s) as u8 + 1
}
