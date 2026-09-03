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
//!    build. `f64` IS the value channel, so this is DL3's "content key
//!    equality of the value channel", cheap because a fresh handed
//!    evaluation makes the rebuild a memo walk that re-runs no op. A
//!    STALE handed evaluation (the document edited between builds)
//!    differs somewhere in its keys and gets the typed
//!    [`PairingViolation`], never a sensitivity.
//! 2. **The passes.** Each seeded `Dual64` pass is compared against
//!    the anchor node for node on its result ARMS (`Ok`/`Failed`/
//!    `Poisoned`, same poison source) and, at the measure, on the
//!    VALUE CHANNEL by bits. The dual contract makes a pass's value
//!    channel the anchor's bit-identically (pinned over the corpus by
//!    the M10-DI suites); a pass that nonetheless diverges in which
//!    nodes evaluate, or in the number it measured, is a broken
//!    contract surfaced as the same typed error rather than as a
//!    number. A dual's own content keys cannot serve here — they feed
//!    both channels (DL2) and so never equal an f64 key — and the
//!    verdict vector cannot either, because the at-rest gate's
//!    predicates are logged at `f64` and structurally unrun at a dual.
//!
//! # No third state; tangent poison never refuses
//!
//! Every reported derivative carries its [`Chamber`] mark —
//! certified-in-the-nominal's-leaf or [`Chamber::LocalOnly`] — as a
//! field of the SAME enum arm the number lives in, so an unmarked
//! sensitivity is unrepresentable, not discouraged (E4's load-bearing
//! clause: the classic stackup lie is extrapolation across a topology
//! change). And per E9, a degraded tangent (a non-finite tangent under
//! a finite measured value at `Dual64`) is never a refusal: it is the
//! [`SensitivityOutcome::TangentDegraded`] state, which forfeits
//! exactly its uses — the `per_param` contribution and the RSS go
//! [`Unavailable`] — while `worst_case` is untouched, because
//! `worst_case` is a value-channel interval computation that never
//! reads a tangent: its evaluation scalar, [`geom_core::Interval`], has
//! no tangent channel to read.
//!
//! # What decides here
//!
//! Nothing. Every geometric decision happens inside the evaluations
//! this module launches, at their own `Decide` sites; what remains up
//! here is driver and reporting arithmetic — content-key identity,
//! exact offset comparisons on stored leaf boxes, tangent-channel float
//! classification (E9's explicit reading), and `f64` report sums. No ε
//! is consulted and no funnel predicate is minted.
//!
//! Nothing here persists, and the goldening form is M10-6's: it will
//! want a `serialize()` shaped like `ParamBoxVerdict`'s (floats as
//! exact bits, one line per row). The door is visible; nothing is built
//! behind it.

use std::sync::Arc;

use geom_core::interval::Interval;
use geom_core::{CertifiedEnclosure, Dual64, Tol};

use crate::analysis::{AnalyzedBox, BoxAxis, MeasureUnavailable, ParamBox};
use crate::doc::{Doc, DocParam, ParamName};
use crate::drive::{CertifiedLeaf, MeasureAccounting, ParamBoxVerdict, VerdictVectorKey};
use crate::eval::{
    CancelToken, ContentKey, EvalOptions, EvalOutcome, Evaluation, NodeResult, ProfileLift,
    ValuePayload, evaluate,
};
use crate::node::{Node, RecipeNodeId};
use crate::program::ProfileProgram;

/// The E4 semantics-honesty mark: what a reported ∂m/∂pᵢ is valid
/// over. Two variants and no third — a sensitivity is chamber-scoped
/// or it is local, and the consumer holding one cannot fail to know
/// which.
#[derive(Debug, Clone, PartialEq)]
pub enum Chamber {
    /// An E6 certified leaf CONTAINING the nominal, from a drive over
    /// the box asked about: within this leaf the fixed-topology
    /// program provably does not flip, so the derivative describes the
    /// as-built body over the leaf.
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

/// One pass's reading, marked. The derivative and its [`Chamber`] are
/// one arm — no unmarked number exists in this API.
#[derive(Debug, Clone, PartialEq)]
pub enum SensitivityOutcome {
    /// ∂m/∂pᵢ at the nominal, with its validity mark.
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
    /// The measure refused in this pass, through its own typed doors —
    /// a per-entry refusal, not a driver failure. The value channel is
    /// scalar-independent, so a measure that refuses here refuses at
    /// `f64` too; the cause is that door's own prose.
    MeasureRefused {
        /// The measure node's typed error, rendered.
        cause: String,
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
    /// they disagree about which nodes evaluate.
    ResultArm {
        /// The first disagreeing node, in evaluation order.
        node: RecipeNodeId,
        /// The arm in the evaluation being checked.
        found: &'static str,
        /// The arm in the build of record.
        expected: &'static str,
    },
    /// A seeded pass's value channel at the measure is not the
    /// anchor's, bit for bit — the dual contract broken at the one
    /// payload the driver reads.
    ValueChannel {
        /// The measure node.
        node: RecipeNodeId,
        /// The anchor's measured value.
        anchor: f64,
        /// The pass's value channel there.
        pass: f64,
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
                 two runs disagree about which nodes evaluate",
                node.0
            ),
            Self::ValueChannel { node, anchor, pass } => write!(
                f,
                "the seeded pass measured {pass} at node {} where the validated build \
                 measured {anchor} — the dual's value channel is not the f64 run's",
                node.0
            ),
        }
    }
}

impl core::error::Error for PairingViolation {}

/// A sensitivity drive that could not start. Per-entry states (a
/// refused measure, a degraded tangent) are entries, not this.
#[derive(Debug, Clone, PartialEq)]
pub enum SensitivityRefusal {
    /// The named node is not a `Measure` node.
    NotAMeasure {
        /// The node that was asked about.
        node: RecipeNodeId,
    },
    /// The chamber verdict was driven over a different document's
    /// parameters — its root box's axes are not this document's
    /// continuous parameters, so its leaves certify nothing here.
    ForeignVerdict,
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
                 parameters — it was driven over a different document and certifies \
                 nothing here",
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
/// build the record. `chamber` is a drive over the box asked about:
/// with it, a derivative is marked by the nominal's certified leaf;
/// without it (or with the nominal in refused mass) every mark is
/// [`Chamber::LocalOnly`]. `parallel` runs the passes under rayon
/// idiom 1 — the result is bit-identical in either schedule (D9).
///
/// Pure: `doc` is shared, the result is a value, nothing on this path
/// writes.
///
/// # Errors
///
/// [`SensitivityRefusal`] — the node is not a measure, the chamber
/// verdict is foreign, or the pairing gate fired.
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

/// What the driver's shared core hands back: the anchored f64 build
/// plus the entries — [`stackup`] consumes both, [`sensitivities`]
/// keeps the entries.
struct Driven {
    anchor: Evaluation<f64>,
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
    if let Some(verdict) = chamber
        && !box_spans_doc_params(verdict.root(), doc)
    {
        return Err(SensitivityRefusal::ForeignVerdict);
    }

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

    // The mark is a property of the nominal's leaf — one chamber for
    // the whole entry set, computed once.
    let mark = chamber.map_or(Chamber::LocalOnly, nominal_chamber);

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
        // anchor does and measures the anchor's number, or it is not a
        // sensitivity of the anchor's build.
        pair_pass(&anchor, &pass, measure)?;
        Ok(Sensitivity {
            param: name.clone(),
            outcome: read_pass(&pass, measure, &mark),
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

/// One pass's reading at the measure node (E9 lives here): the tangent
/// off the payload's public field; a non-finite tangent under a finite
/// value is the forfeiture state, NEVER a refusal — the only refusals
/// are the measure's own typed doors, carried per entry.
///
/// The finiteness test is tangent-channel float classification in the
/// driver lane — E9's explicit reading of derivative-channel
/// degradation. It consults no ε and decides no topology.
fn read_pass(
    pass: &Evaluation<Dual64>,
    measure: RecipeNodeId,
    mark: &Chamber,
) -> SensitivityOutcome {
    match pass.result(measure) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => {
                let tangent = value.deriv;
                if tangent.is_finite() {
                    SensitivityOutcome::Derivative {
                        value: tangent,
                        chamber: mark.clone(),
                    }
                } else {
                    SensitivityOutcome::TangentDegraded { tangent }
                }
            }
            // Unreachable while the driver's front check holds (the
            // node IS a measure and it evaluated Ok); answered typed
            // rather than panicking.
            other => SensitivityOutcome::MeasureRefused {
                cause: format!("node evaluated to a {}, not a measure", other.kind_name()),
            },
        },
        _ => SensitivityOutcome::MeasureRefused {
            cause: pass
                .node_error(measure)
                .map_or_else(|| "not evaluated".to_owned(), |e| e.kind.to_string()),
        },
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
/// nodes the anchor does — arm for arm, same poison source — and its
/// value channel at the measure is the anchor's number, bit for bit.
fn pair_pass(
    anchor: &Evaluation<f64>,
    pass: &Evaluation<Dual64>,
    measure: RecipeNodeId,
) -> Result<(), PairingViolation> {
    if anchor.order != pass.order {
        return Err(PairingViolation::NodeSet);
    }
    for &id in &anchor.order {
        same_arm(id, anchor.result(id), pass.result(id))?;
    }
    if let (Some(NodeResult::Ok(a)), Some(NodeResult::Ok(p))) =
        (anchor.result(measure), pass.result(measure))
        && let (
            ValuePayload::Measure { value: anchor, .. },
            ValuePayload::Measure { value: pass, .. },
        ) = (&a.payload, &p.payload)
        && anchor.to_bits() != pass.value.to_bits()
    {
        return Err(PairingViolation::ValueChannel {
            node: measure,
            anchor: *anchor,
            pass: pass.value,
        });
    }
    Ok(())
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

/// Whether a verdict's root box spans exactly this document's
/// continuous parameters — the check that keeps a foreign drive from
/// marking this document's sensitivities.
fn box_spans_doc_params(root: &ParamBox, doc: &Doc<ProfileProgram>) -> bool {
    let doc_names: Vec<&ParamName> = continuous_params(doc).collect();
    root.axes().len() == doc_names.len() && doc_names.into_iter().all(|n| root.get(n).is_some())
}

/// The document's continuous parameters, in name order — the entry
/// set of every driver call.
fn continuous_params(doc: &Doc<ProfileProgram>) -> impl Iterator<Item = &ParamName> {
    doc.params()
        .iter()
        .filter(|(_, p)| matches!(p, DocParam::Continuous { .. }))
        .map(|(n, _)| n)
}

/// The certified leaf CONTAINING the nominal, as a [`Chamber`] mark.
///
/// The nominal is offset zero on every axis; a leaf contains it when
/// every axis span holds zero — exact comparisons on stored offsets
/// (leaf boxes are data; no ε anywhere). Where the nominal sits on a
/// split boundary two leaves contain it and the first certified one in
/// the verdict's own order answers: certification is over the leaf's
/// CLOSED box, so either is a true certificate. No certified leaf
/// containing it — the nominal in refused mass, or a drive that never
/// reached it — is [`Chamber::LocalOnly`].
fn nominal_chamber(verdict: &ParamBoxVerdict) -> Chamber {
    verdict
        .certified()
        .iter()
        .find(|leaf| {
            leaf.box_.axes().values().all(|a| {
                let (lo, hi) = a.span();
                lo <= 0.0 && 0.0 <= hi
            })
        })
        .map_or(Chamber::LocalOnly, |leaf| Chamber::ChamberCertified {
            leaf: leaf.box_.clone(),
            verdict_vector_key: leaf.verdict_vector_key,
        })
}

/// Why an ADVISORY column (a `per_param` contribution, an RSS term) is
/// absent, naming the parameter that blocked it. Forfeiture is
/// per-entry and loud (E9); `worst_case` never lands here because it
/// never reads any of the three channels these describe.
#[derive(Debug, Clone, PartialEq)]
pub enum Unavailable {
    /// E9 forfeiture: the parameter's tangent degraded at the nominal.
    TangentDegraded {
        /// The parameter.
        param: ParamName,
    },
    /// The parameter's pass could not read the measure (its typed
    /// doors refused).
    MeasureRefused {
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
            Self::BandHasNoMeasure { param } => write!(
                f,
                "parameter {:?} carries a band: worst-case limits with no shape have \
                 no σ, and a partial RSS is still a lie",
                param.0
            ),
        }
    }
}

/// One row of the advisory `per_param` table.
#[derive(Debug, Clone, PartialEq)]
pub struct PerParam {
    /// The parameter.
    pub param: ParamName,
    /// Its E4-marked sensitivity reading.
    pub sensitivity: SensitivityOutcome,
    /// `|∂m/∂pᵢ| · Δpᵢ`, `Δpᵢ` the analyzed box's half-width on this
    /// axis — the advisory LINEARIZED contribution at the nominal,
    /// labeled as such and never a gate (first-order, silently wrong
    /// under curvature — which is why `worst_case` is not its sum).
    /// Available for every shape that has a support (a band's limits
    /// are real limits); absent exactly when the sensitivity itself
    /// forfeited or refused.
    pub contribution: Result<f64, Unavailable>,
}

/// The advisory RSS column: available only when EVERY contributor
/// carries a measure and a tangent — one blocker refuses the whole
/// column, naming ALL blockers (a partial RSS is still a lie).
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
/// what it does not cover.
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
/// accounting — M10-3's, verbatim.
#[derive(Debug, Clone, PartialEq)]
pub struct Stackup {
    /// The `Measure` node the report is about.
    pub measurement: RecipeNodeId,
    /// The f64 build's measured value — re-derived from the anchored
    /// evaluation, never handed in.
    pub nominal: f64,
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
/// IN the report ([`Unavailable`]); these are the states with no honest
/// report to hand back.
#[derive(Debug, Clone, PartialEq)]
pub enum StackupRefusal {
    /// The sensitivity driver refused (measure/pairing/foreign
    /// verdict).
    Sensitivity(SensitivityRefusal),
    /// The analyzed box is not the box the verdict was driven over —
    /// pairing the two would price one box's leaves with another box's
    /// spreads.
    ForeignBox,
    /// The measure refuses at the nominal build: there is no nominal
    /// and nothing to report. The cause is the measure door's own.
    MeasureRefusedAtNominal {
        /// The measure node's typed error, rendered.
        cause: String,
    },
    /// The drive certified nothing: a worst case over zero certified
    /// leaves is not an enclosure of anything, and a stackup without
    /// its one gating number would be decoration. The verdict's
    /// accounting says where the mass went; refine or widen budgets
    /// and drive again.
    NothingCertified,
    /// A certified leaf's replay refused at the measure — the leaf's
    /// certificate says every node built there, so this is a broken
    /// replay identity (D9) surfaced typed rather than swallowed.
    LeafDiverged {
        /// The leaf's box.
        leaf: ParamBox,
        /// The refusing node's typed error, rendered.
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
            Self::MeasureRefusedAtNominal { cause } => write!(
                f,
                "the measure refuses at the nominal build, so there is no nominal to \
                 report: {cause}"
            ),
            Self::NothingCertified => f.write_str(
                "the drive certified no leaves — a worst case over nothing encloses \
                 nothing, and a stackup without its gating number would be decoration; \
                 the verdict's accounting says where the mass went",
            ),
            Self::LeafDiverged { cause, .. } => write!(
                f,
                "a certified leaf's replay refused at the measure, which its \
                 certificate says cannot happen (D9 replay identity): {cause}"
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
/// root box must be `ParamBox::of(analyzed)` (checked, typed).
/// `paired` is the validated f64 build of record, gated exactly as
/// [`sensitivities`] gates it. `parallel` runs the sensitivity passes
/// and the per-leaf worst-case evaluations under rayon idiom 1; the
/// report is schedule-independent (D9).
///
/// The worst case re-evaluates the measure at `Interval` over each
/// certified leaf's environment (the guided lift on, exactly as the
/// drive replayed it) and hulls the certified brackets. One interval
/// evaluation per certified leaf, bounded above by the drive's own
/// cost — that is the v1 cost decision, taken over a recording dial on
/// `DriveConfig`.
///
/// # Errors
///
/// [`StackupRefusal`] — a driver refusal, a foreign box, a nominal
/// that does not measure, nothing certified, or a broken leaf replay.
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
    let Driven { anchor, entries } = driver(doc, measure, paired, Some(verdict), parallel, tol)
        .map_err(StackupRefusal::Sensitivity)?;

    // The nominal, re-derived from the anchored build.
    let nominal = match anchor.result(measure) {
        Some(NodeResult::Ok(v)) => match &v.payload {
            ValuePayload::Measure { value, .. } => *value,
            other => {
                return Err(StackupRefusal::MeasureRefusedAtNominal {
                    cause: format!("node evaluated to a {}, not a measure", other.kind_name()),
                });
            }
        },
        _ => {
            return Err(StackupRefusal::MeasureRefusedAtNominal {
                cause: anchor
                    .node_error(measure)
                    .map_or_else(|| "not evaluated".to_owned(), |e| e.kind.to_string()),
            });
        }
    };

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
        let half_width = analyzed
            .get(&param)
            .map_or(0.0, |axis| 0.5 * axis.offsets.width());
        let forfeited = match &entry.outcome {
            SensitivityOutcome::Derivative { .. } => None,
            SensitivityOutcome::TangentDegraded { .. } => Some(Unavailable::TangentDegraded {
                param: param.clone(),
            }),
            SensitivityOutcome::MeasureRefused { .. } => Some(Unavailable::MeasureRefused {
                param: param.clone(),
            }),
        };
        // `None` — an axis the analyzed box does not carry — is
        // unreachable past the ForeignBox check; folded into the band
        // arm so a broken pairing surfaces as a named blocker rather
        // than as an invented σ of 0.
        let sigma = analyzed.axis_std_deviation(&param).unwrap_or(Err(
            MeasureUnavailable::BandHasNoMeasure {
                param: param.clone(),
            },
        ));
        let contribution = match (&entry.outcome, &forfeited) {
            (SensitivityOutcome::Derivative { value, .. }, None) => Ok(value.abs() * half_width),
            (_, Some(why)) => Err(why.clone()),
            // A forfeiture is minted for exactly the non-derivative
            // arms above, so this pairing cannot occur; answered as the
            // conservative arm rather than as a number.
            (_, None) => Err(Unavailable::MeasureRefused {
                param: param.clone(),
            }),
        };
        match (&entry.outcome, &forfeited, &sigma) {
            (SensitivityOutcome::Derivative { value, .. }, None, Ok(sigma)) => {
                sum_sq += (value * sigma) * (value * sigma);
            }
            (_, why, sigma) => {
                if let Some(why) = why {
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
        per_param,
        worst_case,
        rss,
        coverage: verdict.accounting().clone(),
    })
}

/// The hull of value-channel interval evaluations of the measure over
/// the certified leaves. Tangent-free BY TYPE: the evaluation scalar is
/// [`Interval`], which has no tangent channel, and the bracket is read
/// through [`CertifiedEnclosure::certified_bracket`] — the
/// domain-honest door, so a poisoned enclosure refuses named instead of
/// hulling a NaN.
fn worst_case(
    doc: &Doc<ProfileProgram>,
    measure: RecipeNodeId,
    verdict: &ParamBoxVerdict,
    parallel: bool,
    tol: Tol,
) -> Result<WorstCase, StackupRefusal> {
    let leaves = verdict.certified();
    if leaves.is_empty() {
        return Err(StackupRefusal::NothingCertified);
    }
    let lane = |box_: ParamBox| EvalOptions {
        profile_lift: ProfileLift::Guided,
        param_box: Some(Arc::new(box_)),
        ..EvalOptions::default()
    };
    // One shared memo prior over the DEGENERATE box — every axis fixed
    // at the nominal, bound through the same door and the same
    // arithmetic a leaf's fixed axes bind through — so the
    // parameter-independent subgraph (and every node downstream only
    // of fixed axes) is evaluated once and served to every leaf.
    // Read-only and shared, so the parallel schedule sees exactly what
    // the sequential one does.
    let nominal_box = ParamBox::from_axes(
        verdict
            .root()
            .axes()
            .keys()
            .map(|n| (n.clone(), BoxAxis::Fixed))
            .collect(),
    );
    let prior: Evaluation<Interval> =
        evaluate(doc, None, &CancelToken::new(), &lane(nominal_box), tol);
    let one = |leaf: &CertifiedLeaf| -> Result<(f64, f64), StackupRefusal> {
        let ev: Evaluation<Interval> = evaluate(
            doc,
            Some(&prior),
            &CancelToken::new(),
            &lane(leaf.box_.clone()),
            tol,
        );
        match ev.result(measure) {
            Some(NodeResult::Ok(v)) => match &v.payload {
                ValuePayload::Measure { value, .. } => {
                    CertifiedEnclosure::certified_bracket(*value).ok_or_else(|| {
                        StackupRefusal::WorstCaseUncertified {
                            leaf: leaf.box_.clone(),
                        }
                    })
                }
                other => Err(StackupRefusal::LeafDiverged {
                    leaf: leaf.box_.clone(),
                    cause: format!("node evaluated to a {}, not a measure", other.kind_name()),
                }),
            },
            _ => Err(StackupRefusal::LeafDiverged {
                leaf: leaf.box_.clone(),
                cause: ev
                    .node_error(measure)
                    .map_or_else(|| "not evaluated".to_owned(), |e| e.kind.to_string()),
            }),
        }
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
