//! **E4 sensitivities and the E5 stackup** (`docs/ERROR-DESIGN.md`
//! E4/E5/E9, `docs/DUAL-DESIGN.md` DL1–DL3).
//!
//! Two doors, both analysis-lane services on the one evaluation
//! service (no parallel path):
//!
//! - [`sensitivities`] — the n-pass forward driver: one `Dual64`
//!   evaluation per continuous document parameter with that parameter
//!   seeded ([`crate::eval::EvalOptions::seed`]), ∂m/∂pᵢ read off the
//!   measure payload's PUBLIC tangent field — never off `Bounds` (E9).
//! - [`stackup()`] — the E5 typed report over a driven box:
//!   `worst_case` gates, everything else is advisory and forfeits
//!   loudly.
//!
//! # The pairing hook (DL3's sentence made a mechanism)
//!
//! A dual evaluation runs no certified validation
//! (`topo::AtRestPolicy` — its gates are structurally absent at a
//! dual), which is sound exactly when the sensitivity is OF the build
//! the f64 run validated. This driver makes that a GATE rather than a
//! convention, in two halves that together leave no path to a
//! sensitivity of an unvalidated build:
//!
//! 1. **The anchor.** Every driver call builds the document's f64
//!    evaluation on the build path (`EvalOptions::default()` — the run
//!    that validates), threading the caller's `paired` evaluation, the
//!    validated build of record, as the memo prior. The anchor's
//!    per-node content keys are then compared against the handed
//!    one's, at every node: same key ⇒ same input bits ⇒ (D9) same
//!    build. A fresh handed evaluation makes the rebuild a memo walk
//!    that re-runs no op; a STALE one (the document edited between
//!    builds) differs somewhere in its keys and gets the typed
//!    [`PairingViolation`], never a sensitivity. A handed build made
//!    with the guided lift keys differently at every profile node and
//!    is refused the same way — over-strict for identical geometry,
//!    and safe: the build of record is the build path.
//! 2. **The passes.** Each seeded `Dual64` pass is compared against
//!    the anchor at EVERY node on its result arm (`Ok`/`Failed`/
//!    `Poisoned`, same poison source) and, where both built, on a
//!    digest of the VALUE CHANNEL of the whole payload — every body
//!    point, datum frame, profile vertex, measured value and verdict,
//!    read through each scalar's own value bracket. That is DL3's
//!    "value-channel equality" taken literally and per node; the dual
//!    contract says the two are bit-identical, and the driver checks
//!    it rather than trusting it. A dual's own content keys cannot
//!    serve here — they feed both channels (DL2) and so never equal an
//!    f64 key — and the verdict vector cannot either, because the
//!    at-rest gate's predicates are logged at `f64` and structurally
//!    unrun at a dual.
//!
//! **The lift's limits are not staleness.** A pass runs the guided
//! lift and the anchor runs the build path, and there are nodes the
//! lift refuses that the build path builds: a loft's or a sweep's
//! section is C6/D9-pinned to `f64` (the skinned surface's structure
//! must be lane-identical), so a seed on a parameter that section
//! reads has nowhere to go — and a guided elaboration can refuse a
//! decision it cannot re-confirm. Those refusals are typed at the
//! node (`NodeErrorKind::SeedPinnedSection`, `ProfileLaneReplay`) and
//! the pairing recognises them BY KIND: the entry becomes
//! [`SensitivityOutcome::Unliftable`] — the spec's typed valve, never a
//! finite zero and never a `PairingViolation` — and every node poisoned
//! through that refusal is accounted to it. Any other disagreement
//! between the two runs is a pairing violation, as before.
//!
//! # The chamber verdict is tied to the build (§5's "unwritable" lie)
//!
//! A handed [`ParamBoxVerdict`] is not trusted by name: the mark it
//! mints is CONTENT-tied to this document. Every certified leaf
//! carries the drive's own per-node content-key record of its replay
//! (`CertifiedLeaf::results`), and the driver replays the leaf it is
//! about to cite — the certified leaf holding the nominal — at
//! `Interval`, over the leaf's own box, exactly as the drive did, and
//! compares the keys node for node. A value edit, a retargeted slot, a
//! different document with the same parameter names: each re-keys some
//! node and is refused [`SensitivityRefusal::VerdictNotOfThisBuild`]
//! before any mark is written. The stackup ties EVERY certified leaf
//! the same way, for free, on the replays its worst case runs anyway.
//! A verdict-vector comparison would not do this (a pure value edit
//! keeps the vector); a key comparison does.
//!
//! # No third state; tangent poison never refuses
//!
//! Every reported derivative carries its [`Chamber`] mark —
//! certified-in-the-nominal's-leaf or [`Chamber::LocalOnly`] — as a
//! field of the SAME enum arm the number lives in, and the report
//! carries the mark once more at the top ([`Stackup::chamber`]), so an
//! unmarked sensitivity is unrepresentable and a nominal sitting in
//! refused mass is visible without walking the rows. Per E9, a
//! degraded tangent (a non-finite tangent under a finite measured
//! value at `Dual64`) is never a refusal: it is the
//! [`SensitivityOutcome::TangentDegraded`] state, which forfeits
//! exactly its uses — the `per_param` contribution and the RSS go
//! [`Unavailable`] — while `worst_case` is untouched, because
//! `worst_case` is a value-channel interval computation that never
//! reads a tangent: its evaluation scalar, [`geom_core::Interval`], has
//! no tangent channel to read.
//!
//! # What a `Derivative` at a kink is
//!
//! The dual's kink conventions are ratified per scalar
//! (`geom_core::dual`): at `Dual64`, `abs`, `min` and `max` at a tie
//! take a FINITE one-sided derivative (+1 at `|0|`), so a measure
//! sitting exactly at such a kink reports a `Derivative` — with the
//! same mark and the same confidence a smooth one gets. That is
//! subgradient honesty by convention, not a claim of smoothness; a
//! consumer who needs the difference reads the fixed-topology program
//! at the nominal, and a kink-aware mark is M10-6's report shape. The
//! loud arm is the domain-edge kink: `sqrt` at exactly zero poisons
//! its tangent (`0/0` through a norm, a vertical tangent at a
//! coincidence), and it poisons it for EVERY seeded parameter, those
//! that feed the coincidence and those that do not — a measure of the
//! angle between two parallel caps forfeits its whole advisory table.
//! The report says which entries forfeited; it cannot say why the
//! tangent poisoned, because the tangent does not carry its site.
//!
//! # What decides here
//!
//! Nothing. Every geometric decision happens inside the evaluations
//! this module launches, at their own `Decide` sites; what remains up
//! here is driver and reporting arithmetic — content-key identity,
//! exact offset comparisons on stored leaf boxes, tangent-channel float
//! classification (E9's explicit reading), value-channel digests, and
//! `f64` report sums. No ε is consulted and no funnel predicate is
//! minted.
//!
//! Nothing here persists, and the goldening form is M10-6's: it will
//! want a `serialize()` shaped like `ParamBoxVerdict`'s (floats as
//! exact bits, one line per row). The door is visible; nothing is built
//! behind it.

use std::collections::BTreeSet;
use std::sync::Arc;

use geom_core::interval::Interval;
use geom_core::{CertifiedEnclosure, Dual64, Tol};
use topo::Body;

use crate::analysis::{AnalyzedBox, BoxAxis, MeasureUnavailable, ParamBox};
use crate::doc::{Doc, DocParam, ParamName};
use crate::drive::{CertifiedLeaf, MeasureAccounting, ParamBoxVerdict, Receipt, VerdictVectorKey};
use crate::eval::{
    BooleanValue, CancelToken, ContentKey, DatumValue, EvalOptions, EvalOutcome, Evaluation,
    NodeErrorKind, NodeResult, ProfileLift, SplitSide, ValuePayload, evaluate,
};
use crate::measure::AssertionVerdict;
use crate::node::{Node, RecipeNodeId};
use crate::program::ProfileProgram;

/// The E4 semantics-honesty mark: what a reported ∂m/∂pᵢ is valid
/// over. Two variants and no third — a sensitivity is chamber-scoped
/// or it is local, and the consumer holding one cannot fail to know
/// which.
#[derive(Debug, Clone, PartialEq)]
pub enum Chamber {
    /// An E6 certified leaf CONTAINING the nominal, from a drive whose
    /// leaves are content-tied to this build (module docs): within
    /// this leaf the fixed-topology program provably does not flip, so
    /// the derivative describes the as-built body over the leaf.
    ChamberCertified {
        /// The certified leaf's box (offsets around the nominal).
        leaf: ParamBox,
        /// The leaf's verdict-vector identity — the witness build's,
        /// which is what certification means.
        verdict_vector_key: VerdictVectorKey,
    },
    /// No drive was run, or the nominal's leaf is not certified: the
    /// derivative is the fixed-topology program's at the nominal and
    /// may jump at the nearest predicate flip. Explicitly marked,
    /// never defaulted.
    LocalOnly,
}

/// Why a seeded pass could not carry its seed to the measure — the
/// lift's typed limits (module docs), never a finite wrong number.
#[derive(Debug, Clone, PartialEq)]
pub enum LiftRefusal {
    /// The seeded parameter feeds a loft's or a sweep's SECTION, which
    /// stays `f64` by C6/D9 (the skinned surface's knots, degrees and
    /// control bits must be identical in every lane), so the seed has
    /// no lane to ride in. The valve the spec's grounding names.
    PinnedSection {
        /// The section profile node the seed reaches and stops at.
        section: RecipeNodeId,
        /// The seeded parameter.
        param: ParamName,
    },
    /// The guided elaboration at the pass's scalar could not
    /// re-confirm a structure decision the build path made (a lift
    /// limitation, not a build difference: the value channel is the
    /// nominal's).
    GuidedReplay {
        /// The refusing loop (program order).
        loop_: u32,
        /// The step it refused at.
        step: usize,
    },
}

/// One pass's reading, marked. The derivative and its [`Chamber`] are
/// one arm — no unmarked number exists in this API.
#[derive(Debug, Clone, PartialEq)]
pub enum SensitivityOutcome {
    /// ∂m/∂pᵢ at the nominal, with its validity mark. At a kink this is
    /// the ratified one-sided derivative (module docs).
    Derivative {
        /// The tangent read off the measure payload's public field.
        value: f64,
        /// What the number is valid over.
        chamber: Chamber,
    },
    /// E9's forfeiture state, never a refusal: the measured VALUE is
    /// finite (the value channel is fine) and the tangent is not — a
    /// domain-edge kink at the operating point, a `0/0` through a
    /// norm. The entry's advisory uses are forfeited loudly;
    /// `worst_case` never consulted this channel.
    TangentDegraded {
        /// The degraded tangent, as evidence (NaN or ±∞).
        tangent: f64,
    },
    /// The measure refused in this pass, through its own doors — a
    /// per-entry refusal, not a driver failure. The value channel is
    /// scalar-independent, so a measure that refuses here refuses at
    /// `f64` too; re-evaluating the document hands back the typed
    /// error itself (`NodeErrorKind` is neither `Clone` nor
    /// `PartialEq`, so this entry carries its node and its rendering).
    MeasureRefused {
        /// The refusing node — the measure, or the failed ancestor it
        /// was poisoned through.
        node: RecipeNodeId,
        /// The node error, rendered.
        cause: String,
    },
    /// The seed could not reach the measure: the lift refused typed at
    /// a node the build path built (module docs). The derivative is
    /// not zero and is not reported; it is unavailable, and the entry
    /// says why and where.
    Unliftable {
        /// The node the lift refused at.
        node: RecipeNodeId,
        /// The typed limit.
        refusal: LiftRefusal,
    },
}

/// One continuous parameter's sensitivity entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Sensitivity {
    /// The parameter.
    pub param: ParamName,
    /// The pass's reading, marked.
    pub outcome: SensitivityOutcome,
}

/// The pairing gate's typed finding: the driver was asked to
/// differentiate a build that is not the validated one, or a pass
/// diverged from the anchor. Never a warning — the driver refuses.
#[derive(Debug, Clone, PartialEq)]
pub enum PairingViolation {
    /// The handed f64 evaluation did not run to completion — a
    /// canceled prefix validates nothing.
    Incomplete,
    /// The handed evaluation's node set (or order) is not the
    /// document's — nodes were added, removed, or re-scheduled since
    /// it was built.
    NodeSet,
    /// Content keys differ at this node: the handed evaluation is of
    /// different input bits than the document's build — the exact
    /// silent state DL3's availability argument leans on excluding.
    ContentKey {
        /// The first differing node, in evaluation order.
        node: RecipeNodeId,
        /// The handed evaluation's key there.
        handed: ContentKey,
        /// The document's own build's key there.
        rebuilt: ContentKey,
    },
    /// A node's result arm differs between the two runs being paired —
    /// they disagree about which nodes evaluate — and the difference
    /// is not one of the lift's typed limits (module docs).
    ResultArm {
        /// The first disagreeing node, in evaluation order.
        node: RecipeNodeId,
        /// The arm in the evaluation being checked.
        found: &'static str,
        /// The arm in the build of record.
        expected: &'static str,
    },
    /// A seeded pass's value channel at this node is not the anchor's,
    /// bit for bit, over the whole payload — the dual contract broken.
    ValueChannel {
        /// The first diverging node, in evaluation order.
        node: RecipeNodeId,
    },
}

impl core::fmt::Display for PairingViolation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Incomplete => f.write_str(
                "the paired f64 evaluation is a canceled prefix — an incomplete run \
                 validates nothing, so there is no build of record to differentiate",
            ),
            Self::NodeSet => f.write_str(
                "the paired f64 evaluation's node set is not this document's — the \
                 document changed since that build, so the sensitivity would not be \
                 of the validated build",
            ),
            Self::ContentKey { node, .. } => write!(
                f,
                "the paired f64 evaluation is STALE at node {}: its content key is not \
                 the document's own build's, so differentiating now would report a \
                 sensitivity of a build nobody validated",
                node.0
            ),
            Self::ResultArm {
                node,
                found,
                expected,
            } => write!(
                f,
                "node {} is {found} where the build of record has it {expected} — the \
                 two runs disagree about which nodes evaluate, and not by one of the \
                 lift's typed limits",
                node.0
            ),
            Self::ValueChannel { node } => write!(
                f,
                "the seeded pass's value channel at node {} is not the validated \
                 build's, bit for bit — the dual contract is broken there",
                node.0
            ),
        }
    }
}

impl core::error::Error for PairingViolation {}

/// A sensitivity drive that could not start. Per-entry states (a
/// refused measure, a degraded tangent, an unliftable seed) are
/// entries, not this.
#[derive(Debug, Clone, PartialEq)]
pub enum SensitivityRefusal {
    /// The named node is not a `Measure` node.
    NotAMeasure {
        /// The node that was asked about.
        node: RecipeNodeId,
    },
    /// The chamber verdict's root box does not even name this
    /// document's continuous parameters — driven over a different
    /// parameter set, so its leaves certify nothing here.
    ForeignVerdict,
    /// The chamber verdict's certified leaf is not of THIS build: its
    /// recorded per-node content keys differ from the leaf's replay
    /// over this document (module docs — the document was edited
    /// since the drive, or the verdict is another document's).
    VerdictNotOfThisBuild {
        /// The leaf that was replayed.
        leaf: ParamBox,
        /// The first node whose key differs (or is missing on one
        /// side), in evaluation order.
        node: RecipeNodeId,
        /// The drive's recorded key there, if the record has one.
        recorded: Option<ContentKey>,
        /// This document's replay key there, if the replay built it.
        replayed: Option<ContentKey>,
    },
    /// The pairing gate fired (module docs): no sensitivity of an
    /// unvalidated build.
    Pairing(PairingViolation),
}

impl core::fmt::Display for SensitivityRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotAMeasure { node } => write!(f, "node {} is not a Measure node", node.0),
            Self::ForeignVerdict => f.write_str(
                "the chamber verdict's root box does not span this document's continuous \
                 parameters — it was driven over a different parameter set and certifies \
                 nothing here",
            ),
            Self::VerdictNotOfThisBuild { node, .. } => write!(
                f,
                "the chamber verdict is not of this build: its certified leaf replays with \
                 a different content key at node {} — the document changed since the \
                 drive, or the verdict is another document's; drive again",
                node.0
            ),
            Self::Pairing(v) => write!(f, "pairing violation: {v}"),
        }
    }
}

impl core::error::Error for SensitivityRefusal {}

/// **The n-pass E4 driver.** One entry per continuous document
/// parameter, in name order — a parameter without a distribution still
/// gets its ∂m/∂pᵢ (the fixed-parameter typed spelling: distributions
/// matter to the report's mass and spread columns, not to the
/// derivative).
///
/// `paired` is the caller's validated f64 build of record — a
/// build-path evaluation (`EvalOptions::default()`) of `doc` — gated as
/// the module docs describe; `None` makes the driver's own fresh f64
/// build the record. `chamber` is a drive over this document: its
/// certified leaf holding the nominal, content-tied to this build
/// (module docs), marks every derivative; without it (or with the
/// nominal in refused mass) every mark is [`Chamber::LocalOnly`]. Which
/// box the drive was asked over is not this door's question — a leaf
/// of this build that holds the nominal is a true certificate over
/// itself — and pairing a box's spreads with a verdict's leaves is
/// [`stackup()`]'s, which checks it. `parallel` runs the passes under
/// rayon idiom 1 — the result is bit-identical in either schedule (D9).
///
/// Pure: `doc` is shared, the result is a value, nothing on this path
/// writes. Cost: one f64 anchor, one unseeded and n seeded `Dual64`
/// passes, plus one `Interval` leaf replay when a chamber is handed.
///
/// # Errors
///
/// [`SensitivityRefusal`] — the node is not a measure, the verdict is
/// foreign or not of this build, or the pairing gate fired.
pub fn sensitivities(
    doc: &Doc<ProfileProgram>,
    measure: RecipeNodeId,
    paired: Option<&Evaluation<f64>>,
    chamber: Option<&ParamBoxVerdict>,
    parallel: bool,
    tol: Tol,
) -> Result<Vec<Sensitivity>, SensitivityRefusal> {
    driver(doc, measure, paired, chamber, parallel, tol).map(|d| d.entries)
}

/// What the driver's shared core hands back: the anchored f64 build,
/// the mark, and the entries — [`stackup()`] consumes all three,
/// [`sensitivities`] keeps the entries.
struct Driven {
    anchor: Evaluation<f64>,
    chamber: Chamber,
    entries: Vec<Sensitivity>,
}

fn driver(
    doc: &Doc<ProfileProgram>,
    measure: RecipeNodeId,
    paired: Option<&Evaluation<f64>>,
    chamber: Option<&ParamBoxVerdict>,
    parallel: bool,
    tol: Tol,
) -> Result<Driven, SensitivityRefusal> {
    if !matches!(doc.node(measure), Some(Node::Measure { .. })) {
        return Err(SensitivityRefusal::NotAMeasure { node: measure });
    }
    // The mark is a property of the nominal's leaf — one chamber for
    // the whole entry set, tied to this build once.
    let chamber = match chamber {
        None => Chamber::LocalOnly,
        Some(verdict) => bind_verdict(doc, verdict, tol)?,
    };

    // The anchor: the document's own build-path f64 evaluation,
    // threaded from the handed prior so agreement is cheap (a full memo
    // walk, zero ops re-run) and staleness is loud (the edited cone
    // re-keys and the comparison names the first difference).
    let anchor = evaluate::<f64>(
        doc,
        paired,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    if let Some(handed) = paired {
        pair_record(handed, &anchor).map_err(SensitivityRefusal::Pairing)?;
    }

    // The names, in name order (deterministic in both schedules).
    let names: Vec<ParamName> = continuous_params(doc).cloned().collect();

    // One UNSEEDED dual base, threaded into every pass as the memo
    // prior: a node outside a pass's seeded cone carries identical
    // value+tangent bits in base and pass (tangent zero in both), so
    // the parameter-independent subgraph is evaluated once — DL2's
    // cross-pass reuse, bought through the front door. Shared
    // read-only, so the parallel schedule sees exactly what the
    // sequential one does.
    let base: Evaluation<Dual64> = evaluate(doc, None, &CancelToken::new(), &pass_opts(None), tol);

    let one = |name: &ParamName| -> Result<Sensitivity, PairingViolation> {
        let pass: Evaluation<Dual64> = evaluate(
            doc,
            Some(&base),
            &CancelToken::new(),
            &pass_opts(Some(name.clone())),
            tol,
        );
        // DL3, per pass: the pass evaluates exactly the nodes the
        // anchor does with the anchor's value channel, or it is not a
        // sensitivity of the anchor's build — unless the lift refused
        // typed, which is the entry's own state.
        let outcome = match pair_pass(&anchor, &pass)? {
            Some((node, refusal)) => SensitivityOutcome::Unliftable { node, refusal },
            None => read_pass(&pass, measure, &chamber),
        };
        Ok(Sensitivity {
            param: name.clone(),
            outcome,
        })
    };

    // D9 idiom 1: an indexed map over the name list — results land by
    // position, never accumulated — so the schedule cannot leak.
    let entries: Result<Vec<Sensitivity>, PairingViolation> = if parallel {
        use rayon::prelude::*;
        names.par_iter().map(one).collect()
    } else {
        names.iter().map(one).collect()
    };
    Ok(Driven {
        anchor,
        chamber,
        entries: entries.map_err(SensitivityRefusal::Pairing)?,
    })
}

/// The pass options: the profile lift GUIDED (a seed on a profile
/// dimension must move profile geometry — the exact silent zero
/// `ProfileLift`'s docs warn about), sequential inside (the driver's
/// parallelism is per pass), the seed as given.
fn pass_opts(seed: Option<ParamName>) -> EvalOptions {
    EvalOptions {
        profile_lift: ProfileLift::Guided,
        seed,
        ..EvalOptions::default()
    }
}

/// The options a leaf is replayed with — the drive's own, so the
/// replay's content keys are the drive's record bit for bit.
fn leaf_opts(box_: ParamBox) -> EvalOptions {
    EvalOptions {
        profile_lift: ProfileLift::Guided,
        param_box: Some(Arc::new(box_)),
        ..EvalOptions::default()
    }
}

/// The measured value at `id`, or the refusal rendered with the node
/// it came from (the measure itself, or the failed ancestor a poisoned
/// measure names) — the one ladder every reader of a measure payload
/// takes.
fn measure_of<T: geom_core::Decide + Copy>(
    ev: &Evaluation<T>,
    id: RecipeNodeId,
) -> Result<T, (RecipeNodeId, String)> {
    match ev.result(id) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => Ok(*value),
            // Unreachable while the driver's front check holds (the
            // node IS a measure and it evaluated Ok); answered as the
            // node's own refusal rather than panicking.
            other => Err((
                id,
                format!("node evaluated to a {}, not a measure", other.kind_name()),
            )),
        },
        _ => Err(ev.node_error(id).map_or_else(
            || (id, "not evaluated".to_owned()),
            |e| (e.node, e.kind.to_string()),
        )),
    }
}

/// One pass's reading at the measure node (E9 lives here): the tangent
/// off the payload's public field; a non-finite tangent under a finite
/// value is the forfeiture state, NEVER a refusal — the only refusals
/// are the measure's own doors, carried per entry.
///
/// The finiteness test is tangent-channel float classification in the
/// driver lane — E9's explicit reading of derivative-channel
/// degradation. It consults no ε and decides no topology.
fn read_pass(
    pass: &Evaluation<Dual64>,
    measure: RecipeNodeId,
    chamber: &Chamber,
) -> SensitivityOutcome {
    match measure_of(pass, measure) {
        Ok(value) => {
            let tangent = value.deriv;
            if tangent.is_finite() {
                SensitivityOutcome::Derivative {
                    value: tangent,
                    chamber: chamber.clone(),
                }
            } else {
                SensitivityOutcome::TangentDegraded { tangent }
            }
        }
        Err((node, cause)) => SensitivityOutcome::MeasureRefused { node, cause },
    }
}

/// Pairing half 1 (module docs): the handed evaluation IS the rebuild,
/// by per-node content keys — the value channel's identity, since `f64`
/// is the value channel. `Failed`/`Poisoned` nodes carry no key; for
/// them arm agreement is the whole check, which is honest: no
/// sensitivity is read from a failed subgraph, and every `Ok` node's
/// inputs are certified transitively by its own key.
fn pair_record(
    handed: &Evaluation<f64>,
    rebuilt: &Evaluation<f64>,
) -> Result<(), PairingViolation> {
    if handed.outcome != EvalOutcome::Completed {
        return Err(PairingViolation::Incomplete);
    }
    if handed.order != rebuilt.order {
        return Err(PairingViolation::NodeSet);
    }
    for &id in &rebuilt.order {
        match (handed.result(id), rebuilt.result(id)) {
            (Some(NodeResult::Ok(h)), Some(NodeResult::Ok(r))) => {
                if h.content_key != r.content_key {
                    return Err(PairingViolation::ContentKey {
                        node: id,
                        handed: h.content_key,
                        rebuilt: r.content_key,
                    });
                }
            }
            (h, r) => same_arm(id, h, r)?,
        }
    }
    Ok(())
}

/// Pairing half 2 (module docs): a seeded pass evaluates exactly the
/// nodes the anchor does — arm for arm, same poison source — with the
/// anchor's value channel at every node that built, EXCEPT where the
/// lift refused typed: that node and everything poisoned through it
/// are the entry's own [`LiftRefusal`], returned rather than raised.
/// Total: every node is compared, and a node both runs built is
/// compared on its whole payload.
fn pair_pass(
    anchor: &Evaluation<f64>,
    pass: &Evaluation<Dual64>,
) -> Result<Option<(RecipeNodeId, LiftRefusal)>, PairingViolation> {
    if anchor.order != pass.order {
        return Err(PairingViolation::NodeSet);
    }
    let mut valve: Option<(RecipeNodeId, LiftRefusal)> = None;
    let mut unlifted: BTreeSet<RecipeNodeId> = BTreeSet::new();
    for &id in &anchor.order {
        match (anchor.result(id), pass.result(id)) {
            (Some(NodeResult::Ok(a)), Some(NodeResult::Ok(p))) => {
                if payload_digest(&a.payload) != payload_digest(&p.payload) {
                    return Err(PairingViolation::ValueChannel { node: id });
                }
            }
            (Some(NodeResult::Ok(_)), Some(NodeResult::Failed(e))) => match &e.kind {
                NodeErrorKind::SeedPinnedSection { section, param } => {
                    unlifted.insert(id);
                    valve.get_or_insert((
                        id,
                        LiftRefusal::PinnedSection {
                            section: *section,
                            param: param.clone(),
                        },
                    ));
                }
                NodeErrorKind::ProfileLaneReplay { loop_, step, .. } => {
                    unlifted.insert(id);
                    valve.get_or_insert((
                        id,
                        LiftRefusal::GuidedReplay {
                            loop_: *loop_,
                            step: *step,
                        },
                    ));
                }
                _ => same_arm(id, anchor.result(id), pass.result(id))?,
            },
            (Some(NodeResult::Ok(_)), Some(NodeResult::Poisoned { through }))
                if unlifted.contains(through) =>
            {
                unlifted.insert(id);
            }
            (a, p) => same_arm(id, a, p)?,
        }
    }
    Ok(valve)
}

/// The arm comparison both halves share: `Ok`/`Failed`/`Poisoned` (same
/// poison source)/absent, the RECORD's arm being the expected one.
fn same_arm<T: geom_core::Decide, U: geom_core::Decide>(
    id: RecipeNodeId,
    record: Option<&NodeResult<T>>,
    checked: Option<&NodeResult<U>>,
) -> Result<(), PairingViolation> {
    let same = match (record, checked) {
        (Some(NodeResult::Poisoned { through: a }), Some(NodeResult::Poisoned { through: b })) => {
            a == b
        }
        _ => arm(record) == arm(checked),
    };
    if same {
        Ok(())
    } else {
        Err(PairingViolation::ResultArm {
            node: id,
            found: arm(checked),
            expected: arm(record),
        })
    }
}

/// A result arm's name, for the pairing comparison and its prose.
fn arm<T: geom_core::Decide>(r: Option<&NodeResult<T>>) -> &'static str {
    match r {
        Some(NodeResult::Ok(_)) => "Ok",
        Some(NodeResult::Failed(_)) => "Failed",
        Some(NodeResult::Poisoned { .. }) => "Poisoned",
        None => "absent",
    }
}

// ---------------------------------------------- the value-channel digest

/// A scalar's VALUE CHANNEL as exact bits — what the dual contract's
/// "bit-identical" quantifies over, read through each scalar's own
/// bracket: the `f64` itself, a dual's value channel.
trait ValueChannel: geom_core::Decide + Copy {
    fn bits(self) -> u64;
}

impl ValueChannel for f64 {
    fn bits(self) -> u64 {
        self.to_bits()
    }
}

impl ValueChannel for Dual64 {
    fn bits(self) -> u64 {
        self.value.bits()
    }
}

/// FNV-1a 64 over value-channel bits — a comparison digest for the
/// pairing, never a content key (it hashes OUTPUTS, keyed by nothing).
struct Digest(u64);

impl Digest {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }

    fn u64(&mut self, x: u64) {
        for b in x.to_le_bytes() {
            self.0 ^= u64::from(b);
            self.0 = self.0.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }

    fn scalar<T: ValueChannel>(&mut self, x: T) {
        self.u64(x.bits());
    }

    fn point3<T: ValueChannel>(&mut self, p: geom_core::Point3<T>) {
        self.scalar(p.x);
        self.scalar(p.y);
        self.scalar(p.z);
    }

    fn vec3<T: ValueChannel>(&mut self, v: geom_core::Vec3<T>) {
        self.scalar(v.x);
        self.scalar(v.y);
        self.scalar(v.z);
    }

    fn body<T: ValueChannel>(&mut self, body: &Body<T>) {
        self.u64(body.solids().count() as u64);
        self.u64(body.faces().count() as u64);
        self.u64(body.edges().count() as u64);
        self.u64(body.vertices().count() as u64);
        for (_k, p) in body.points() {
            self.point3(*p);
        }
    }
}

/// The value-channel digest of one payload: its arm, its counts, and
/// every scalar it stores — body points, datum frames, profile
/// vertices and bulges, measured values, verdict numbers — through the
/// scalar's own value bracket, so an `f64` build and a `Dual64` pass
/// digest identically exactly when their value channels agree.
fn payload_digest<T: ValueChannel>(payload: &ValuePayload<T>) -> u64 {
    let mut d = Digest::new();
    match payload {
        ValuePayload::Datum(DatumValue::Plane { origin, normal }) => {
            d.u64(10);
            d.point3(*origin);
            d.vec3(normal.get());
        }
        ValuePayload::Datum(DatumValue::Axis { origin, dir }) => {
            d.u64(11);
            d.point3(*origin);
            d.vec3(dir.get());
        }
        ValuePayload::Datum(DatumValue::Point { position }) => {
            d.u64(12);
            d.point3(*position);
        }
        ValuePayload::Datum(DatumValue::Frame { origin, u, v }) => {
            d.u64(13);
            d.point3(*origin);
            d.vec3(u.get());
            d.vec3(v.get());
        }
        ValuePayload::Datum(DatumValue::AxisInPlane {
            plane_origin,
            plane_dir,
            origin,
            dir,
        }) => {
            // 24, not a number among the datum arms above: the tags
            // are the digest's wire and renumbering one would move
            // every pinned digest that carries a later arm.
            d.u64(24);
            // All four fields, not just the world pair. The authored
            // 2-D numbers are what a revolve actually reads, and this
            // digest's contract is every scalar the payload stores —
            // so the lift being derived from them is a reason they
            // agree, not a reason to digest only one of the two.
            d.scalar(plane_origin.x);
            d.scalar(plane_origin.y);
            d.scalar(plane_dir.x);
            d.scalar(plane_dir.y);
            d.point3(*origin);
            d.vec3(dir.get());
        }
        ValuePayload::Profile(p) => {
            d.u64(14);
            for lp in p.validated.loops() {
                d.u64(lp.vertices().len() as u64);
                for v in lp.vertices() {
                    d.scalar(v.pos().x);
                    d.scalar(v.pos().y);
                    d.scalar(v.bulge());
                }
            }
        }
        ValuePayload::Body(b) => {
            d.u64(15);
            d.body(b);
        }
        ValuePayload::Boolean(BooleanValue::Empty) => d.u64(16),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            d.u64(17);
            d.body(body);
        }
        ValuePayload::Split { above, below } => {
            d.u64(18);
            for side in [above, below] {
                match side {
                    SplitSide::Empty => d.u64(0),
                    SplitSide::Body(b) => {
                        d.u64(1);
                        d.body(b);
                    }
                }
            }
        }
        ValuePayload::Instances(bodies) => {
            d.u64(19);
            d.u64(bodies.len() as u64);
            for b in bodies {
                d.body(b);
            }
        }
        ValuePayload::Declarations(pairs) => {
            d.u64(20);
            d.u64(pairs.len() as u64);
        }
        ValuePayload::Mate(_) => d.u64(21),
        ValuePayload::Measure { value, .. } => {
            d.u64(22);
            d.scalar(*value);
        }
        ValuePayload::Assertion(verdict) => {
            d.u64(23);
            match verdict {
                AssertionVerdict::Holds { measured, bound } => {
                    d.u64(1);
                    d.scalar(*measured);
                    d.scalar(*bound);
                }
                AssertionVerdict::Violated { measured, bound } => {
                    d.u64(2);
                    d.scalar(*measured);
                    d.scalar(*bound);
                }
                AssertionVerdict::Unevaluated { .. } => d.u64(3),
            }
        }
    }
    d.0
}

// ------------------------------------------------- the verdict's tie

/// The document's continuous parameters, in name order — the entry
/// set of every driver call.
fn continuous_params(doc: &Doc<ProfileProgram>) -> impl Iterator<Item = &ParamName> {
    doc.params()
        .iter()
        .filter(|(_, p)| matches!(p, DocParam::Continuous { .. }))
        .map(|(n, _)| n)
}

/// Whether a verdict's root box spans exactly this document's
/// continuous parameters — the cheap pre-check before the content tie.
fn box_spans_doc_params(root: &ParamBox, doc: &Doc<ProfileProgram>) -> bool {
    let doc_names: Vec<&ParamName> = continuous_params(doc).collect();
    root.axes().len() == doc_names.len() && doc_names.into_iter().all(|n| root.get(n).is_some())
}

/// The handed verdict's mark for this build (module docs): the first
/// certified leaf holding the nominal, content-tied to the document by
/// replaying it and comparing keys. Where the nominal sits on a split
/// boundary two leaves contain it and the first in the verdict's own
/// order answers: certification is over the leaf's CLOSED box, so
/// either is a true certificate. No certified leaf holding the nominal
/// — the nominal in refused mass, or a drive that never reached it —
/// is [`Chamber::LocalOnly`], tied through the first certified leaf
/// instead; a verdict with no certified leaf at all has no content to
/// tie and marks nothing.
fn bind_verdict(
    doc: &Doc<ProfileProgram>,
    verdict: &ParamBoxVerdict,
    tol: Tol,
) -> Result<Chamber, SensitivityRefusal> {
    if !box_spans_doc_params(verdict.root(), doc) {
        return Err(SensitivityRefusal::ForeignVerdict);
    }
    let chamber = verdict
        .certified()
        .iter()
        .find(|leaf| leaf.box_.contains_nominal());
    let Some(tied) = chamber.or_else(|| verdict.certified().first()) else {
        return Ok(Chamber::LocalOnly);
    };
    let replay: Evaluation<Interval> = evaluate(
        doc,
        None,
        &CancelToken::new(),
        &leaf_opts(tied.box_.clone()),
        tol,
    );
    tie(tied, &replay)?;
    Ok(
        chamber.map_or(Chamber::LocalOnly, |leaf| Chamber::ChamberCertified {
            leaf: leaf.box_.clone(),
            verdict_vector_key: leaf.verdict_vector_key,
        }),
    )
}

/// The tie itself: the leaf's recorded per-node keys against its
/// replay's, in evaluation order, the first difference named.
fn tie(leaf: &CertifiedLeaf, replay: &Evaluation<Interval>) -> Result<(), SensitivityRefusal> {
    let replayed: Vec<(RecipeNodeId, ContentKey)> = replay
        .order
        .iter()
        .filter_map(|&id| replay.value(id).map(|v| (id, v.content_key)))
        .collect();
    let recorded = &leaf.results.node_keys;
    let n = recorded.len().max(replayed.len());
    for i in 0..n {
        let r = recorded.get(i).copied();
        let p = replayed.get(i).copied();
        if r != p {
            let node = r.or(p).map_or(RecipeNodeId(0), |(id, _)| id);
            return Err(SensitivityRefusal::VerdictNotOfThisBuild {
                leaf: leaf.box_.clone(),
                node,
                recorded: r.map(|(_, k)| k),
                replayed: p.map(|(_, k)| k),
            });
        }
    }
    Ok(())
}

// ------------------------------------------------------- the report

/// Why an ADVISORY column (a `per_param` contribution, an RSS term) is
/// absent, naming the parameter that blocked it. Forfeiture is
/// per-entry and loud (E9); `worst_case` never lands here because it
/// never reads any of the channels these describe.
#[derive(Debug, Clone, PartialEq)]
pub enum Unavailable {
    /// E9 forfeiture: the parameter's tangent degraded at the nominal.
    TangentDegraded {
        /// The parameter.
        param: ParamName,
    },
    /// The parameter's pass could not read the measure (its doors
    /// refused).
    MeasureRefused {
        /// The parameter.
        param: ParamName,
    },
    /// The parameter's seed could not reach the measure — the lift's
    /// typed limit ([`SensitivityOutcome::Unliftable`]).
    Unliftable {
        /// The parameter.
        param: ParamName,
    },
    /// The parameter carries a [`crate::Distribution::Band`]: limits
    /// without a shape have no σ, and a partial RSS is still a lie
    /// (E5) — so the RSS names it and refuses whole.
    BandHasNoMeasure {
        /// The parameter.
        param: ParamName,
    },
}

impl Unavailable {
    /// The blocked parameter.
    pub fn param(&self) -> &ParamName {
        match self {
            Self::TangentDegraded { param }
            | Self::MeasureRefused { param }
            | Self::Unliftable { param }
            | Self::BandHasNoMeasure { param } => param,
        }
    }
}

impl core::fmt::Display for Unavailable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TangentDegraded { param } => write!(
                f,
                "parameter {:?}'s tangent degraded at the nominal (E9: forfeits its \
                 advisory uses, refuses nothing)",
                param.0
            ),
            Self::MeasureRefused { param } => write!(
                f,
                "parameter {:?}'s pass could not read the measure",
                param.0
            ),
            Self::Unliftable { param } => write!(
                f,
                "parameter {:?}'s seed could not reach the measure: the lift refused typed",
                param.0
            ),
            Self::BandHasNoMeasure { param } => write!(
                f,
                "parameter {:?} carries a band: worst-case limits with no shape have \
                 no σ, and a partial RSS is still a lie",
                param.0
            ),
        }
    }
}

/// The advisory contribution RESTRICTED to the certified chamber: the
/// same derivative times the certified leaf's half-width on this axis
/// — the span the mark actually covers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChamberSpan {
    /// The certified leaf's half-width on this axis.
    pub half_width: f64,
    /// `|∂m/∂pᵢ| · half_width` — the linearized contribution over the
    /// span the derivative is marked valid for.
    pub contribution: f64,
}

/// One row of the advisory `per_param` table.
#[derive(Debug, Clone, PartialEq)]
pub struct PerParam {
    /// The parameter.
    pub param: ParamName,
    /// Its E4-marked sensitivity reading.
    pub sensitivity: SensitivityOutcome,
    /// `|∂m/∂pᵢ| · Δpᵢ`, `Δpᵢ` the ANALYZED box's half-width on this
    /// axis — the advisory LINEARIZED contribution at the nominal,
    /// labeled as such and never a gate (first-order, silently wrong
    /// under curvature — which is why `worst_case` is not its sum).
    /// **Chamber-exceeding by construction**: the derivative is marked
    /// valid over ONE certified leaf and this multiplies it by the
    /// whole box's span, which on a drive that split is many leaves
    /// wide; [`Self::chamber_span`] is the same product over the span
    /// the mark covers. Available for every shape that has a support (a
    /// band's limits are real limits); absent exactly when the
    /// sensitivity itself forfeited, refused or could not be lifted.
    pub contribution: Result<f64, Unavailable>,
    /// The contribution over the certified chamber itself — present
    /// exactly when the sensitivity is a [`Chamber::ChamberCertified`]
    /// derivative.
    pub chamber_span: Option<ChamberSpan>,
}

/// The advisory RSS column: available only when EVERY contributor
/// carries a measure and a tangent — one blocker refuses the whole
/// column, naming ALL blockers (a partial RSS is still a lie).
///
/// A parameter with NO distribution carries a measure — the point mass
/// at its nominal, σ = 0 — and contributes a zero term rather than
/// blocking: E2's opt-in rule read literally (fixed is a modelling
/// statement, not a missing spread).
#[derive(Debug, Clone, PartialEq)]
pub enum Rss {
    /// `√Σ(∂m/∂pᵢ·σᵢ)²`, linearized, ADVISORY — never a gate.
    Advisory {
        /// The linearized output standard deviation.
        sigma: f64,
    },
    /// The column is refused whole; every blocking parameter is named
    /// (per-param order; a parameter blocking twice — a band whose
    /// tangent also degraded — appears once per reason).
    UnavailableBecause {
        /// Every blocker, in per-param order.
        blockers: Vec<Unavailable>,
    },
}

/// The gating enclosure: the hull of value-channel INTERVAL evaluations
/// of the measure over the drive's certified leaves — never the
/// linearized sum. Refused + tail mass in [`Stackup::coverage`] say
/// what it does not cover. The pair IS a certified bracket (every leaf
/// contributed through `certified_bracket`), which is the mark it
/// carries: there is no unmarked number here because the type is the
/// enclosure.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorstCase {
    /// The hull's lower end.
    pub lo: f64,
    /// The hull's upper end.
    pub hi: f64,
    /// How many certified leaves the hull is over.
    pub leaves: usize,
}

/// **The E5 stackup report.** Field for field E5's block: the
/// measurement, its nominal, the advisory per-parameter table, the
/// gating certified worst case, the advisory RSS, and the coverage
/// accounting — M10-3's, verbatim — plus the mark, once, at the top.
#[derive(Debug, Clone, PartialEq)]
pub struct Stackup {
    /// The `Measure` node the report is about.
    pub measurement: RecipeNodeId,
    /// The f64 build's measured value — re-derived from the anchored
    /// evaluation, never handed in.
    pub nominal: f64,
    /// The nominal's chamber — the one mark every derivative row
    /// carries, stated once so a nominal in refused mass is visible
    /// without walking the rows.
    pub chamber: Chamber,
    /// The advisory per-parameter table (sensitivity + contribution),
    /// labeled advisory and forfeitable per entry.
    pub per_param: Vec<PerParam>,
    /// The ONLY gating number (E5).
    pub worst_case: WorstCase,
    /// The advisory RSS, whole or refused whole.
    pub rss: Rss,
    /// The drive's mass accounting, verbatim (certified + per-reason
    /// refused + tail; sums to 1 where it prices — E2/E6). Not
    /// recomputed here.
    pub coverage: MeasureAccounting,
}

/// A stackup that could not be built. Advisory degradation is carried
/// IN the report ([`Unavailable`]); these are the states with no
/// gating number to hand back — and the one that spent a drive to
/// learn it carries what the drive and the driver did produce.
#[derive(Debug, Clone, PartialEq)]
pub enum StackupRefusal {
    /// The sensitivity driver refused (measure, pairing, or a verdict
    /// that is foreign or not of this build).
    Sensitivity(SensitivityRefusal),
    /// The analyzed box is not the box the verdict was driven over —
    /// pairing the two would price one box's leaves with another box's
    /// spreads.
    ForeignBox,
    /// The measure refuses at the nominal build: there is no nominal
    /// and nothing to report. The cause is the measure door's own,
    /// rendered.
    MeasureRefusedAtNominal {
        /// The refusing node (the measure, or the ancestor it was
        /// poisoned through).
        node: RecipeNodeId,
        /// The node error, rendered.
        cause: String,
    },
    /// The drive certified nothing: a worst case over zero certified
    /// leaves is not an enclosure of anything, and a stackup without
    /// its gating number would be decoration. What the run DID
    /// produce rides here — the nominal, the `LocalOnly` sensitivities,
    /// and the drive's accounting and receipt saying where the mass
    /// went — so a real study's answer is legible from the refusal.
    NothingCertified {
        /// The f64 build's measured value.
        nominal: f64,
        /// The driver's entries, every derivative marked
        /// [`Chamber::LocalOnly`].
        sensitivities: Vec<Sensitivity>,
        /// The drive's accounting, verbatim: which refusal class holds
        /// the mass, and the tail. Boxed so the refusal stays a small
        /// `Err` on every result path that carries it.
        coverage: Box<MeasureAccounting>,
        /// The drive's counting receipt.
        receipt: Receipt,
    },
    /// A certified leaf's replay refused at the measure AFTER its keys
    /// tied to the drive's record — same inputs, a different result:
    /// a D9 replay-identity break, surfaced typed rather than
    /// swallowed. (A foreign or edited document is caught before this
    /// by the tie, as [`SensitivityRefusal::VerdictNotOfThisBuild`].)
    LeafDiverged {
        /// The leaf's box.
        leaf: ParamBox,
        /// The refusing node.
        node: RecipeNodeId,
        /// The node error, rendered.
        cause: String,
    },
    /// A certified leaf's measure enclosure carries a domain violation
    /// (no certified bracket): a non-real entered a certified lane's
    /// report path — DL6's bug-shaped state, refused with the leaf
    /// named rather than hulled around.
    WorstCaseUncertified {
        /// The leaf's box.
        leaf: ParamBox,
    },
}

impl core::fmt::Display for StackupRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sensitivity(r) => write!(f, "{r}"),
            Self::ForeignBox => f.write_str(
                "the analyzed box is not the box the verdict was driven over — the two \
                 name different axes or spans, and pairing them would price one box's \
                 leaves with another box's spreads",
            ),
            Self::MeasureRefusedAtNominal { node, cause } => write!(
                f,
                "the measure refuses at the nominal build (node {}), so there is no \
                 nominal to report: {cause}",
                node.0
            ),
            Self::NothingCertified { receipt, .. } => write!(
                f,
                "the drive certified no leaves ({} refused) — a worst case over nothing \
                 encloses nothing, and a stackup without its gating number would be \
                 decoration; the accounting carried here says where the mass went",
                receipt.refused
            ),
            Self::LeafDiverged { node, cause, .. } => write!(
                f,
                "a certified leaf tied to this build by its content keys refused at node {} \
                 on replay — same inputs, a different result (a D9 replay-identity \
                 break): {cause}",
                node.0
            ),
            Self::WorstCaseUncertified { .. } => f.write_str(
                "a certified leaf's measure enclosure carries a domain violation — a \
                 non-real entered the certified report path (the DL6 class); the \
                 refusal names the leaf rather than hulling around it",
            ),
        }
    }
}

impl core::error::Error for StackupRefusal {}

/// **The E5 stackup** over a driven box.
///
/// `analyzed` and `verdict` must be the same analysis: the verdict's
/// root box must be `ParamBox::of(analyzed)` (checked, typed), and the
/// verdict's leaves must be of THIS build (content-tied, module docs).
/// `paired` is the validated f64 build of record, gated exactly as
/// [`sensitivities`] gates it. `parallel` runs the sensitivity passes
/// and the per-leaf worst-case evaluations under rayon idiom 1; the
/// report is schedule-independent (D9).
///
/// The worst case re-evaluates the measure at `Interval` over each
/// certified leaf's environment (the guided lift on, exactly as the
/// drive replayed it), ties every leaf's keys to the drive's record
/// on the way, and hulls the certified brackets. One interval
/// evaluation per certified leaf, bounded above by the drive's own
/// cost — that is the v1 cost decision, taken over a recording dial on
/// `DriveConfig`.
///
/// # Errors
///
/// [`StackupRefusal`] — a driver refusal, a foreign box, a nominal
/// that does not measure, nothing certified (with everything the run
/// did produce), or a broken leaf replay.
pub fn stackup(
    doc: &Doc<ProfileProgram>,
    measure: RecipeNodeId,
    analyzed: &AnalyzedBox,
    verdict: &ParamBoxVerdict,
    paired: Option<&Evaluation<f64>>,
    parallel: bool,
    tol: Tol,
) -> Result<Stackup, StackupRefusal> {
    if ParamBox::of(analyzed) != *verdict.root() {
        return Err(StackupRefusal::ForeignBox);
    }
    let Driven {
        anchor,
        chamber,
        entries,
    } = driver(doc, measure, paired, Some(verdict), parallel, tol)
        .map_err(StackupRefusal::Sensitivity)?;

    // The nominal, re-derived from the anchored build.
    let nominal = measure_of(&anchor, measure)
        .map_err(|(node, cause)| StackupRefusal::MeasureRefusedAtNominal { node, cause })?;

    if verdict.certified().is_empty() {
        return Err(StackupRefusal::NothingCertified {
            nominal,
            sensitivities: entries,
            coverage: Box::new(verdict.accounting().clone()),
            receipt: verdict.receipt(),
        });
    }
    let worst_case = worst_case(doc, measure, verdict, parallel, tol)?;

    // The advisory columns. Half-widths and σ come off the analyzed
    // box's own axes (the pairing-safe doors), derivatives off the
    // entries; a forfeited entry forfeits exactly its own row and its
    // RSS term (E9), and EVERY band names itself whatever its tangent
    // did (E5: all band contributors, not the first).
    let mut per_param = Vec::with_capacity(entries.len());
    let mut blockers: Vec<Unavailable> = Vec::new();
    let mut sum_sq = 0.0_f64;
    for entry in entries {
        let param = entry.param.clone();
        // Every entry names a continuous parameter of `doc`, and the
        // ForeignBox check above made `analyzed` span exactly those;
        // an axis missing here is a broken pairing, refused as one.
        let axis = analyzed.get(&param).ok_or(StackupRefusal::ForeignBox)?;
        let half_width = 0.5 * axis.offsets.width();
        let sigma = analyzed
            .axis_std_deviation(&param)
            .ok_or(StackupRefusal::ForeignBox)?;
        let derivative = match &entry.outcome {
            SensitivityOutcome::Derivative { value, chamber } => Ok((*value, chamber)),
            SensitivityOutcome::TangentDegraded { .. } => Err(Unavailable::TangentDegraded {
                param: param.clone(),
            }),
            SensitivityOutcome::MeasureRefused { .. } => Err(Unavailable::MeasureRefused {
                param: param.clone(),
            }),
            SensitivityOutcome::Unliftable { .. } => Err(Unavailable::Unliftable {
                param: param.clone(),
            }),
        };
        let contribution = derivative
            .as_ref()
            .map(|(value, _)| value.abs() * half_width)
            .map_err(Clone::clone);
        let chamber_span = match &derivative {
            Ok((value, Chamber::ChamberCertified { leaf, .. })) => leaf.get(&param).map(|axis| {
                let (lo, hi) = axis.span();
                let half = 0.5 * (hi - lo);
                ChamberSpan {
                    half_width: half,
                    contribution: value.abs() * half,
                }
            }),
            Ok((_, Chamber::LocalOnly)) | Err(_) => None,
        };
        match (&derivative, &sigma) {
            (Ok((value, _)), Ok(sigma)) => sum_sq += (value * sigma) * (value * sigma),
            (derivative, sigma) => {
                if let Err(why) = derivative {
                    blockers.push(why.clone());
                }
                if let Err(MeasureUnavailable::BandHasNoMeasure { param }) = sigma {
                    blockers.push(Unavailable::BandHasNoMeasure {
                        param: param.clone(),
                    });
                }
            }
        }
        per_param.push(PerParam {
            param,
            sensitivity: entry.outcome,
            contribution,
            chamber_span,
        });
    }
    let rss = if blockers.is_empty() {
        Rss::Advisory {
            sigma: sum_sq.sqrt(),
        }
    } else {
        Rss::UnavailableBecause { blockers }
    };

    Ok(Stackup {
        measurement: measure,
        nominal,
        chamber,
        per_param,
        worst_case,
        rss,
        coverage: verdict.accounting().clone(),
    })
}

/// The hull of value-channel interval evaluations of the measure over
/// the certified leaves, every leaf tied to the drive's record on the
/// way. Tangent-free BY TYPE: the evaluation scalar is [`Interval`],
/// which has no tangent channel, and the bracket is read through
/// [`CertifiedEnclosure::certified_bracket`] — the domain-honest door,
/// so a poisoned enclosure refuses named instead of hulling a NaN.
fn worst_case(
    doc: &Doc<ProfileProgram>,
    measure: RecipeNodeId,
    verdict: &ParamBoxVerdict,
    parallel: bool,
    tol: Tol,
) -> Result<WorstCase, StackupRefusal> {
    let leaves = verdict.certified();
    // One shared memo prior over the DEGENERATE box — every axis fixed
    // at the nominal, bound through the same door and the same
    // arithmetic a leaf's fixed axes bind through — so the
    // parameter-independent subgraph (and every node downstream only
    // of fixed axes) is evaluated once and served to every leaf.
    // Read-only and shared, so the parallel schedule sees exactly what
    // the sequential one does; the memo serves bit-equal inputs only,
    // so the hull is the same with or without it.
    let nominal_box = ParamBox::from_axes(
        verdict
            .root()
            .axes()
            .keys()
            .map(|n| (n.clone(), BoxAxis::Fixed))
            .collect(),
    );
    let prior: Evaluation<Interval> =
        evaluate(doc, None, &CancelToken::new(), &leaf_opts(nominal_box), tol);
    let one = |leaf: &CertifiedLeaf| -> Result<(f64, f64), StackupRefusal> {
        let ev: Evaluation<Interval> = evaluate(
            doc,
            Some(&prior),
            &CancelToken::new(),
            &leaf_opts(leaf.box_.clone()),
            tol,
        );
        tie(leaf, &ev).map_err(StackupRefusal::Sensitivity)?;
        let value =
            measure_of(&ev, measure).map_err(|(node, cause)| StackupRefusal::LeafDiverged {
                leaf: leaf.box_.clone(),
                node,
                cause,
            })?;
        CertifiedEnclosure::certified_bracket(value).ok_or_else(|| {
            StackupRefusal::WorstCaseUncertified {
                leaf: leaf.box_.clone(),
            }
        })
    };
    // D9 idiom 1 again: an indexed map, then one sequential fold over
    // the collected brackets, so the hull is the same bits in either
    // schedule. The brackets are certified (no NaN reaches the fold),
    // so min/max are exact lattice operations here.
    let brackets: Result<Vec<(f64, f64)>, StackupRefusal> = if parallel {
        use rayon::prelude::*;
        leaves.par_iter().map(one).collect()
    } else {
        leaves.iter().map(one).collect()
    };
    let brackets = brackets?;
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for (l, h) in &brackets {
        lo = lo.min(*l);
        hi = hi.max(*h);
    }
    Ok(WorstCase {
        lo,
        hi,
        leaves: brackets.len(),
    })
}
