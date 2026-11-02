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
//! # The binding: one build identity, one measured value (DL3)
//!
//! A dual evaluation runs no certified validation
//! (`topo::AtRestPolicy` — its gates are structurally absent at a
//! dual), so the claim that a sensitivity is OF a certified shape
//! belongs to the validated `f64` evaluation the report reads beside
//! the passes; the passes contribute tangents and are never read as a
//! shape. Two checks make that claim a gate rather than a convention:
//!
//! 1. **The build identity.** The report takes the `f64` evaluation and
//!    the seeded `Dual64` passes and refuses typed
//!    ([`BindingViolation`]) unless every pass, and the unseeded dual
//!    run the passes reuse, read exactly the inputs the `f64`
//!    evaluation read: the same document
//!    ([`crate::DocumentId`]) at the same revision
//!    ([`crate::ContentPin`], [`Evaluation::pin`]), the parameter
//!    bindings bit for bit (a dual binding by its value half), the
//!    same tolerance, and the same resolved parts (each reference with
//!    its pin, A4). Under D9 equal inputs are equal value bits, so
//!    nothing pairs the two evaluations node by node. A handed `f64`
//!    evaluation that is stale — the document edited since, a part
//!    file that no longer resolves, another document — differs in its
//!    identity and gets the violation, never a sensitivity.
//! 2. **The measured value.** Where a pass reads the measured value,
//!    its value half is compared bit for bit with the `f64`
//!    evaluation's measured value, and a mismatch refuses
//!    [`SensitivityRefusal::MeasuredValueDrift`]: a D9 break between
//!    the scalars, caught in production at the one number the report
//!    hands out. One number per pass; the refusal names the parameter
//!    whose pass drifted, not the node.
//!
//! **The lift's limits are not drift.** A pass elaborates profiles
//! guided by the `f64` first pass, and a guided elaboration can refuse
//! a decision it cannot re-confirm. That refusal is typed at the node
//! (`NodeErrorKind::ProfileLaneReplay`) and read off the pass's own
//! result: a measured value whose refusal is that kind becomes
//! [`SensitivityOutcome::Unliftable`] — the spec's typed valve, never a
//! finite zero and never a violation.
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
//! here is driver and reporting arithmetic — build-identity and
//! content-key equality, exact offset comparisons on stored leaf boxes,
//! tangent-channel float classification (E9's explicit reading), one
//! bitwise comparison of measured values per pass, and `f64` report
//! sums. No ε is consulted and no funnel predicate is minted.
//!
//! Nothing here persists. The goldening form is [`Stackup::serialize`]
//! (floats as exact bits, one line per row) and [`Stackup::content_key`]
//! keys it, derived on demand; [`Stackup::render`] is the human form.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use geom_core::{Dual64, Readable, Tol};

use crate::analysis::{AnalyzedBox, MeasureUnavailable, ParamBox};
use crate::doc::Doc;
use crate::drive::{CertifiedLeaf, MeasureAccounting, ParamBoxVerdict, Receipt};
use crate::eval::measure::Observed;
use crate::eval::{
    CancelToken, ContentKey, EvalOptions, EvalOutcome, Evaluation, NodeErrorKind, NodeResult,
    evaluate,
};
use crate::node::RecipeNodeId;
use crate::program::ProfileProgram;
use crate::resolve::VerdictVectorKey;
use crate::spoken::SpokenNode;
use crate::spoken::SpokenVar;
use crate::var::VarId;

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
    /// The measured value has no value in this pass — its measure
    /// refused through its own doors, or a node under it did not build
    /// at the pass's scalar — a per-entry refusal, not a driver
    /// failure. Re-evaluating the document at `Dual64` with this
    /// entry's parameter as the seed hands back the typed error itself
    /// (`NodeErrorKind` is neither `Clone` nor `PartialEq`, so this
    /// entry carries its node and its rendering).
    MeasureRefused {
        /// The refusing node — a measure under the value, or the
        /// failed ancestor it was poisoned through; `None` for a value
        /// no measure is under, whose own definition refused.
        node: Option<RecipeNodeId>,
        /// The node error, rendered.
        cause: String,
        /// What the `f64` build of record read for the value.
        /// [`RecordReading::Measured`] says the refusal is the dual's
        /// alone: a door DUAL-DESIGN DL3 leaves unrun at a dual, or a
        /// decision the two scalars took differently (a D9 break).
        record: RecordReading,
    },
    /// The seed could not reach the measure: the lift refused typed at
    /// a node under the measured value (module docs). The derivative is
    /// not zero and is not reported; it is unavailable, and the entry
    /// says why and where.
    Unliftable {
        /// The node the lift refused at.
        node: RecipeNodeId,
        /// The typed limit.
        refusal: LiftRefusal,
    },
}

/// **What the `f64` build of record read for the measured value** —
/// its nominal's three states, kept apart so a pass that disagrees with
/// it says which it disagreed with.
#[derive(Debug, Clone, PartialEq)]
pub enum RecordReading {
    /// The build of record measured the value.
    Measured(f64),
    /// The measure has no value at `f64` by its own door (a
    /// `min_clearance`, answered by an engine that needs an enclosure).
    Unavailable(crate::measure::MeasureUnavailableAt),
    /// The value refused in the build of record: its measure, or a
    /// node under it, did not build.
    Refused {
        /// The refusal, rendered with the node it came from.
        cause: String,
    },
}

impl core::fmt::Display for RecordReading {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Measured(v) => write!(f, "the f64 build of record measured {v:e}"),
            Self::Unavailable(reason) => {
                write!(f, "the f64 build of record has no value: {reason}")
            }
            Self::Refused { cause } => write!(f, "the f64 build of record refused it: {cause}"),
        }
    }
}

/// One continuous parameter's sensitivity entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Sensitivity {
    /// The document this was taken of, the one document its human
    /// form speaks from. Outside the goldening form and its content key.
    pub document: crate::DocumentId,
    /// The variable.
    pub param: VarId,
    /// The pass's reading, marked.
    pub outcome: SensitivityOutcome,
}

/// The binding's typed finding (module docs): the `f64` evaluation and
/// a pass did not read the same inputs, so the pass's tangents are not
/// of the validated build. Never a warning — the driver refuses.
#[derive(Debug, Clone, PartialEq)]
pub enum BindingViolation {
    /// The `f64` evaluation (or a pass) did not run to completion — a
    /// canceled prefix validates nothing.
    Incomplete,
    /// The `f64` evaluation (or a pass) ran out of its member budget
    /// ([`EvalOutcome::Exhausted`]): a prefix validates nothing.
    Exhausted {
        /// The budget it ran out of.
        budget: u64,
    },
    /// The `f64` evaluation (or a pass) refused before building
    /// anything ([`Evaluation::refused_whole`]): it was asked for a
    /// seed or a parameter box its scalar cannot carry, or its
    /// tolerance conflicts with the document's. It reports itself
    /// complete, but it read no inputs — its environment is the
    /// nominal one whatever it was asked to bind — so there is no
    /// build to bind. A build of record is a build-path evaluation.
    NotABuild,
    /// The `f64` evaluation carries no content pin, so which revision
    /// it read cannot be said: it was not asked for one
    /// ([`crate::EvalOptions::pin`]; `refusal` is `None`), or the
    /// document refused to canonicalize (`refusal` says why).
    Unpinned {
        /// The canonicalization's refusal, when there was one.
        refusal: Option<crate::persist::PersistError>,
    },
    /// The `f64` evaluation is of another document (DI3).
    EvaluationOfAnotherDocument {
        /// The document asked about.
        expected: crate::DocumentId,
        /// The document the `f64` evaluation is of.
        found: crate::DocumentId,
    },
    /// The two read different revisions of the document: their content
    /// pins differ.
    Revision,
    /// The two decided at different tolerances.
    Tolerance,
    /// The two bound this variable to different bits (a dual binding
    /// compared by its value half), or only one bound it.
    Binding {
        /// The variable, spoken from the document asked about.
        var: SpokenVar,
    },
    /// The two resolved different parts: a reference one resolved and
    /// the other did not, or at another pin.
    Parts,
}

impl core::fmt::Display for BindingViolation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Incomplete => f.write_str(
                "the f64 build of record is a canceled prefix — an incomplete run \
                 validates nothing, so there is no build of record to differentiate",
            ),
            Self::Exhausted { budget } => write!(
                f,
                "the f64 build of record ran out of its member budget of {budget} listing steps \
                 — a prefix validates nothing; raise `EvalOptions::member_budget` or lower a \
                 count",
            ),
            Self::NotABuild => f.write_str(
                "the f64 build of record refused every node before building anything (a \
                 seed or parameter box its scalar cannot carry, or a tolerance conflict) \
                 — it is no build; hand a build-path evaluation",
            ),
            Self::Unpinned { refusal: None } => f.write_str(
                "the f64 build of record records no content pin, so which revision it \
                 read cannot be said; evaluate it with `EvalOptions::pin`",
            ),
            Self::Unpinned {
                refusal: Some(refusal),
            } => write!(
                f,
                "the document does not canonicalize, so no build of it can be bound by \
                 revision: {refusal}"
            ),
            Self::EvaluationOfAnotherDocument { expected, found } => write!(
                f,
                "the f64 build of record is of document {found}, not of {expected}, the \
                 document asked about"
            ),
            Self::Revision => f.write_str(
                "the f64 build of record read another revision of the document — it \
                 changed since that build, so differentiating now would report a \
                 sensitivity of a build nobody validated",
            ),
            Self::Tolerance => f.write_str(
                "the f64 build of record decided at another tolerance than the \
                 sensitivity passes",
            ),
            Self::Binding { var } => write!(
                f,
                "the f64 build of record bound {var} to other bits than the \
                 sensitivity passes did"
            ),
            Self::Parts => f.write_str(
                "the f64 build of record resolved other parts than the sensitivity \
                 passes did — a part that resolves in one and not the other, or at \
                 another pin",
            ),
        }
    }
}

impl core::error::Error for BindingViolation {}

impl From<crate::Mispaired> for BindingViolation {
    fn from(m: crate::Mispaired) -> Self {
        Self::EvaluationOfAnotherDocument {
            expected: m.expected,
            found: m.found,
        }
    }
}

/// A sensitivity drive that could not start. Per-entry states (a
/// refused measure, a degraded tangent, an unliftable seed) are
/// entries, not this.
#[derive(Debug, Clone, PartialEq)]
pub enum SensitivityRefusal {
    /// The variable asked about is not a live scalar variable.
    NotAScalar {
        /// The variable, spoken from the document asked about.
        var: crate::SpokenVar,
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
        /// The leaf that was replayed, boxed so the refusal stays a
        /// small `Err`.
        leaf: Box<ParamBox>,
        /// The first node whose key differs (or is missing on one
        /// side), in evaluation order; boxed for the same reason.
        node: Box<DivergedAt>,
        /// The drive's recorded key there, if the record has one.
        recorded: Option<ContentKey>,
        /// This document's replay key there, if the replay built it.
        replayed: Option<ContentKey>,
    },
    /// The binding fired (module docs): no sensitivity of an
    /// unvalidated build.
    Binding(BindingViolation),
    /// A pass's measured value is not the `f64` evaluation's, bit for
    /// bit, over inputs the binding found equal — a D9 break between
    /// the scalars (module docs).
    MeasuredValueDrift {
        /// The parameter whose pass drifted.
        param: SpokenVar,
        /// The value half of the pass's measured value.
        pass: f64,
        /// What the `f64` evaluation read for the value: another
        /// number, or no value where the pass has one (and which).
        base: RecordReading,
    },
}

/// **Where a stale verdict's replay first parts from its record**, said
/// by what is known of that node.
#[derive(Debug, Clone, PartialEq)]
pub enum DivergedAt {
    /// A node this document's replay holds, spoken from this document.
    Replayed(SpokenNode),
    /// A node only the drive's record names. The record is spelled in
    /// the document the drive ran on, which may be another one, so the
    /// node is said by tag as the record's, never looked up here.
    Recorded(RecipeNodeId),
}

impl DivergedAt {
    /// The node's full id, whichever side named it.
    #[must_use]
    pub fn id(&self) -> RecipeNodeId {
        match self {
            Self::Replayed(node) => node.id(),
            Self::Recorded(id) => *id,
        }
    }
}

impl core::fmt::Display for DivergedAt {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Replayed(node) => write!(f, "{node}"),
            Self::Recorded(id) => write!(f, "the drive record's node {id}"),
        }
    }
}

impl core::fmt::Display for SensitivityRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotAScalar { var } => write!(
                f,
                "{var} is not a scalar variable of this document: a stackup is over a value, \
                 typically a measure's output or a definition over measures' outputs"
            ),
            Self::ForeignVerdict => f.write_str(
                "the chamber verdict's root box does not span this document's continuous \
                 parameters — it was driven over a different parameter set and certifies \
                 nothing here",
            ),
            Self::VerdictNotOfThisBuild { node, .. } => write!(
                f,
                "the chamber verdict is not of this build: its certified leaf replays with \
                 a different content key at {node} — the document changed since the \
                 drive, or the verdict is another document's; drive again"
            ),
            Self::Binding(v) => write!(f, "binding violation: {v}"),
            Self::MeasuredValueDrift { param, pass, base } => write!(
                f,
                "the sensitivity pass for {param} measured {pass:e} where {base} — the \
                 dual's value channel is not the f64 build's (a D9 break)"
            ),
        }
    }
}

impl core::error::Error for SensitivityRefusal {}

/// **The n-pass E4 driver.** One entry per analysis axis of the
/// document — a toleranced variable ([`crate::analysis::is_axis`]) —
/// in declaration order. A variable with no tolerance is a constant of
/// the analysis and has no entry: nobody asks a sensitivity to a value
/// that carries no tolerance (VR8).
///
/// `of_record` is the caller's validated f64 build of record — a
/// build-path evaluation (`EvalOptions::default()`) of `doc`, asked for
/// its pin ([`EvalOptions::pin`]) — bound to the passes as the module
/// docs describe; `None` makes the driver's own fresh f64 build the
/// record. `chamber` is a drive over this document: its
/// certified leaf holding the nominal, content-tied to this build
/// (module docs), marks every derivative; without it (or with the
/// nominal in refused mass) every mark is [`Chamber::LocalOnly`]. Which
/// box the drive was asked over is not this door's question — a leaf
/// of this build that holds the nominal is a true certificate over
/// itself — and pairing a box's spreads with a verdict's leaves is
/// [`stackup()`]'s, which checks it. `parallel` runs the passes under
/// rayon idiom 1 — the result is bit-identical in either schedule (D9).
/// `resolver` resolves the parts a document instantiates, in every pass
/// and replay, as [`EvalOptions::resolver`] does an evaluation's: an
/// assembly's mates solve at each pass's own scalar (`ASSEMBLY.md` A11
/// (5)), so ∂m/∂pᵢ crosses a mate. An `of_record` build that resolved other
/// parts — over another resolver, or none — fails the binding.
///
/// Pure: `doc` is shared, the result is a value, nothing on this path
/// writes. Cost: one canonicalization of `doc` for its pin, one f64
/// build when no `of_record` is handed, one unseeded and n seeded `Dual64`
/// passes, plus one `Interval` leaf replay when a chamber is handed.
///
/// # Errors
///
/// [`SensitivityRefusal`] — the node is not a measure, the verdict is
/// foreign or not of this build, the binding fired, or a pass's
/// measured value drifted from the build of record's.
pub fn sensitivities(
    doc: &Doc<ProfileProgram>,
    value: VarId,
    of_record: Option<&Evaluation<f64>>,
    chamber: Option<&ParamBoxVerdict>,
    parallel: bool,
    resolver: Option<&Arc<dyn crate::part::PartResolver>>,
    tol: Tol,
) -> Result<Vec<Sensitivity>, SensitivityRefusal> {
    driver(doc, value, of_record, chamber, parallel, resolver, tol).map(|d| d.entries)
}

/// What the driver's shared core hands back: the build of record's
/// nominal reading, the mark, and the entries — [`stackup()`] consumes
/// all three, [`sensitivities`] keeps the entries.
struct Driven {
    nominal:
        Result<Result<f64, crate::measure::MeasureUnavailableAt>, (Option<RecipeNodeId>, String)>,
    chamber: Chamber,
    entries: Vec<Sensitivity>,
}

fn driver(
    doc: &Doc<ProfileProgram>,
    value: VarId,
    of_record: Option<&Evaluation<f64>>,
    chamber: Option<&ParamBoxVerdict>,
    parallel: bool,
    resolver: Option<&Arc<dyn crate::part::PartResolver>>,
    tol: Tol,
) -> Result<Driven, SensitivityRefusal> {
    if doc.var(value).and_then(|v| v.kind().dimension()).is_none() {
        return Err(SensitivityRefusal::NotAScalar {
            var: doc.spoken_var(value),
        });
    }
    // The mark is a property of the nominal's leaf — one chamber for
    // the whole entry set, tied to this build once.
    let chamber = match chamber {
        None => Chamber::LocalOnly,
        Some(verdict) => bind_verdict(doc, verdict, resolver, tol)?,
    };

    // The revision every evaluation below reads, pinned once: the
    // driver's own runs are of `doc` by construction and are stamped
    // with it rather than each canonicalizing `doc` again.
    let pin = crate::persist::content_pin(doc, tol).map_err(|refusal| {
        SensitivityRefusal::Binding(BindingViolation::Unpinned {
            refusal: Some(refusal),
        })
    })?;
    let own: Evaluation<f64>;
    let record = match of_record {
        Some(handed) => handed,
        None => {
            own = stamped(
                evaluate::<f64>(
                    doc,
                    None,
                    &CancelToken::new(),
                    &EvalOptions {
                        resolver: resolver.cloned(),
                        ..EvalOptions::default()
                    },
                    tol,
                ),
                pin,
            );
            &own
        }
    };
    let record = Record::new(doc, record, value).map_err(SensitivityRefusal::Binding)?;

    // The variables, in declaration order (deterministic in both schedules).
    let names: Vec<VarId> = toleranced_params(doc).collect();

    // One UNSEEDED dual base, threaded into every pass as the memo
    // prior: a node outside a pass's seeded cone carries identical
    // value+tangent bits in base and pass (tangent zero in both), so
    // the parameter-independent subgraph is evaluated once — DL2's
    // cross-pass reuse, bought through the front door. Shared
    // read-only, so the parallel schedule sees exactly what the
    // sequential one does. Bound to the record like every pass, so a
    // record that read other inputs is refused even where there is no
    // pass to refuse it — the nominal is read from it.
    let base: Evaluation<Dual64> = stamped(
        evaluate(
            doc,
            None,
            &CancelToken::new(),
            &pass_opts(None, resolver),
            tol,
        ),
        pin,
    );
    BuildIdentity::of(&base)
        .and_then(|identity| record.identity.bind(doc, &identity))
        .map_err(SensitivityRefusal::Binding)?;

    let one = |name: &VarId| -> Result<Sensitivity, SensitivityRefusal> {
        let pass = stamped(
            evaluate(
                doc,
                Some(&base),
                &CancelToken::new(),
                &pass_opts(Some(*name), resolver),
                tol,
            ),
            pin,
        );
        let outcome = record
            .read(doc, &pass, value, &chamber)
            .map_err(|e| match e {
                Unread::Binding(v) => SensitivityRefusal::Binding(v),
                Unread::Drift(Drift { pass, base }) => SensitivityRefusal::MeasuredValueDrift {
                    param: doc.spoken_var(*name),
                    pass,
                    base,
                },
            })?;
        Ok(Sensitivity {
            document: doc.id(),
            param: *name,
            outcome,
        })
    };

    // D9 idiom 1: an indexed map over the name list — results land by
    // position, never accumulated — so the schedule cannot leak. Every
    // pass's result is collected BEFORE any error is chosen: rayon's
    // short-circuiting `collect` into a `Result` hands back whichever
    // error arrives first in time, so the refusal would depend on the
    // schedule. The error reported is the first in parameter order.
    let results: Vec<Result<Sensitivity, SensitivityRefusal>> = if parallel {
        use rayon::prelude::*;
        names.par_iter().map(one).collect()
    } else {
        names.iter().map(one).collect()
    };
    Ok(Driven {
        nominal: record.nominal,
        chamber,
        entries: results.into_iter().collect::<Result<_, _>>()?,
    })
}

/// `ev`, which the driver evaluated from the document `pin` is of,
/// stamped with that pin.
fn stamped<T: geom_core::Decide>(mut ev: Evaluation<T>, pin: crate::ContentPin) -> Evaluation<T> {
    ev.pin = Some(Ok(pin));
    ev
}

/// The pass options: sequential inside (the driver's parallelism is
/// per pass), the seed and the parts' resolver as given. A seed on a
/// profile dimension moves profile geometry, which every evaluation
/// elaborates at its own scalar (`eval::ProfileLane`).
fn pass_opts(
    seed: Option<VarId>,
    resolver: Option<&Arc<dyn crate::part::PartResolver>>,
) -> EvalOptions {
    EvalOptions {
        seed,
        resolver: resolver.cloned(),
        ..EvalOptions::default()
    }
}

/// The options a leaf is replayed with — the drive's own, so the
/// replay's content keys are the drive's record bit for bit — over the
/// parts' resolver.
fn leaf_opts(box_: ParamBox, resolver: Option<&Arc<dyn crate::part::PartResolver>>) -> EvalOptions {
    EvalOptions {
        param_box: Some(Arc::new(box_)),
        resolver: resolver.cloned(),
        ..EvalOptions::default()
    }
}

/// The stackup's NOMINAL column: the f64 value, or the typed reason
/// there is none — distinguished from a measure that genuinely failed,
/// which stays an error.
///
/// [`measure_of`] folds both into one `Err` because its other callers
/// want that; this one is the report's advisory column and has to tell
/// a forfeit from a fault.
fn nominal_of(
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<f64>,
    value: VarId,
) -> Result<Result<f64, crate::measure::MeasureUnavailableAt>, (Option<RecipeNodeId>, String)> {
    match ev.reading_spoken(doc, value)? {
        Observed::Value(v) => Ok(Ok(v)),
        Observed::Unavailable(reason) => Ok(Err(reason)),
    }
}

/// The measured value of `value`, or the refusal rendered with the node
/// it came from ([`Evaluation::reading`]).
///
/// **A measure with no value at this scalar reads as a refusal HERE**,
/// carrying its own reason, and that is the honest shape rather than a
/// gap in this report: E5's nominal is the f64 build's number, its
/// per-param table is a `Dual` pass's, and a `min_clearance` has
/// neither. So a stackup over one refuses `MeasureRefusedAtNominal`
/// naming the scalar and the door, and the certified answer for such a
/// measure is the per-leaf enclosure table
/// ([`crate::report::LeafHistogram`]) rather than a nominal with
/// contributions around it.
fn measure_of<T: geom_core::Decide + Copy>(
    doc: &Doc<ProfileProgram>,
    ev: &Evaluation<T>,
    value: VarId,
) -> Result<T, (Option<RecipeNodeId>, String)> {
    match ev.reading_spoken(doc, value)? {
        Observed::Value(v) => Ok(v),
        Observed::Unavailable(reason) => Err((
            Some(crate::eval::measure::unavailable_at(doc, value)),
            format!("{reason}"),
        )),
    }
}

// ------------------------------------------------------- the binding

/// **What a build read** (DL3): the inputs that decide its every value
/// bit under D9, whatever its scalar. Two evaluations with equal
/// identities are builds of one input.
#[derive(Debug, PartialEq)]
struct BuildIdentity {
    document: crate::DocumentId,
    pin: crate::ContentPin,
    /// `(ε, K)` as bits.
    tolerance: (u64, u64),
    bindings: BTreeMap<VarId, BoundBits>,
    parts: BTreeSet<(crate::DocRef, u64)>,
}

/// One parameter binding as bits: a continuous value by its value half.
#[derive(Debug, PartialEq)]
enum BoundBits {
    Continuous(crate::Dimension, u64),
    Count(i64),
}

/// A scalar's VALUE HALF as exact bits — what D9's "bit-identical"
/// quantifies over: the `f64` itself, a dual's value channel.
trait ValueBits: geom_core::Decide + Copy {
    fn value_bits(self) -> u64;
}

impl ValueBits for f64 {
    fn value_bits(self) -> u64 {
        self.to_bits()
    }
}

impl ValueBits for Dual64 {
    fn value_bits(self) -> u64 {
        self.value.to_bits()
    }
}

impl BuildIdentity {
    /// `ev`'s identity, or why it has none to bind by.
    fn of<T: ValueBits>(ev: &Evaluation<T>) -> Result<Self, BindingViolation> {
        match ev.outcome {
            EvalOutcome::Completed => {}
            EvalOutcome::Canceled => return Err(BindingViolation::Incomplete),
            EvalOutcome::Exhausted { budget, .. } => {
                return Err(BindingViolation::Exhausted { budget });
            }
        }
        if ev.refused_whole() {
            return Err(BindingViolation::NotABuild);
        }
        let pin = match &ev.pin {
            Some(Ok(pin)) => *pin,
            Some(Err(refusal)) => {
                return Err(BindingViolation::Unpinned {
                    refusal: Some(refusal.clone()),
                });
            }
            None => return Err(BindingViolation::Unpinned { refusal: None }),
        };
        let bindings = ev
            .env
            .bindings
            .iter()
            .map(|(&var, binding)| {
                let bits = match binding {
                    crate::expr::ParamValue::Continuous { dim, value } => {
                        BoundBits::Continuous(*dim, value.value_bits())
                    }
                    crate::expr::ParamValue::Count(n) => BoundBits::Count(*n),
                };
                (var, bits)
            })
            .collect();
        Ok(Self {
            document: ev.document,
            pin,
            tolerance: (ev.tolerance.eps.to_bits(), ev.tolerance.k.to_bits()),
            bindings,
            parts: ev.resolved_parts().collect(),
        })
    }

    /// The first input `pass` read differently from `self`, the build
    /// of record's, said from `doc`.
    fn bind(&self, doc: &Doc<ProfileProgram>, pass: &Self) -> Result<(), BindingViolation> {
        if let Some(m) = crate::ident::mispaired(pass.document, self.document) {
            return Err(m.into());
        }
        if self.pin != pass.pin {
            return Err(BindingViolation::Revision);
        }
        if self.tolerance != pass.tolerance {
            return Err(BindingViolation::Tolerance);
        }
        let vars: BTreeSet<&VarId> = self.bindings.keys().chain(pass.bindings.keys()).collect();
        if let Some(var) = vars
            .into_iter()
            .find(|var| self.bindings.get(var) != pass.bindings.get(var))
        {
            return Err(BindingViolation::Binding {
                var: doc.spoken_var(*var),
            });
        }
        if self.parts != pass.parts {
            return Err(BindingViolation::Parts);
        }
        Ok(())
    }
}

/// **The build of record, ready to read passes against**: the
/// validated `f64` evaluation's identity and its measured value, both
/// taken once.
struct Record {
    identity: BuildIdentity,
    /// The measured value's reading in the build of record.
    nominal:
        Result<Result<f64, crate::measure::MeasureUnavailableAt>, (Option<RecipeNodeId>, String)>,
}

/// Why a pass yields no entry: the binding refused it, or its measured
/// value drifted.
#[derive(Debug)]
enum Unread {
    Binding(BindingViolation),
    Drift(Drift),
}

/// A pass's measured value against the build of record's, where the
/// two differ. Naming the node it first differs at would ride here.
#[derive(Debug)]
struct Drift {
    pass: f64,
    base: RecordReading,
}

impl Record {
    /// `ev`, the `f64` build of record, as of `doc`, reading `value`.
    fn new(
        doc: &Doc<ProfileProgram>,
        ev: &Evaluation<f64>,
        value: VarId,
    ) -> Result<Self, BindingViolation> {
        if let Some(m) = crate::ident::mispaired(doc.id(), ev.document) {
            return Err(m.into());
        }
        Ok(Self {
            identity: BuildIdentity::of(ev)?,
            nominal: nominal_of(doc, ev, value),
        })
    }

    /// What the build of record read for the measured value.
    fn reading(&self) -> RecordReading {
        match &self.nominal {
            Ok(Ok(v)) => RecordReading::Measured(*v),
            Ok(Err(reason)) => RecordReading::Unavailable(*reason),
            Err((_, cause)) => RecordReading::Refused {
                cause: cause.clone(),
            },
        }
    }

    /// **The report's reading of one pass** (E9 lives here): bound to
    /// the build of record by identity, its measured value compared
    /// with the record's, then the tangent off the value's public field.
    /// A non-finite tangent under a finite value is the forfeiture
    /// state, NEVER a refusal; a measured value with no value in the
    /// pass is the entry's own refusal, unliftable when the lift
    /// refused under it.
    ///
    /// The finiteness test is tangent-channel float classification in
    /// the driver lane — E9's explicit reading of derivative-channel
    /// degradation. It consults no ε and decides no topology.
    fn read(
        &self,
        doc: &Doc<ProfileProgram>,
        pass: &Evaluation<Dual64>,
        value: VarId,
        chamber: &Chamber,
    ) -> Result<SensitivityOutcome, Unread> {
        BuildIdentity::of(pass)
            .and_then(|identity| self.identity.bind(doc, &identity))
            .map_err(Unread::Binding)?;
        let base = self.reading();
        let measured = match measure_of(doc, pass, value) {
            Ok(measured) => measured,
            Err((node, cause)) => return Ok(unread_measure(pass, node, cause, base)),
        };
        match base {
            RecordReading::Measured(v) if v.to_bits() == measured.value_bits() => {}
            base => {
                return Err(Unread::Drift(Drift {
                    pass: measured.value,
                    base,
                }));
            }
        }
        let tangent = measured.deriv;
        Ok(if tangent.is_finite() {
            SensitivityOutcome::Derivative {
                value: tangent,
                chamber: chamber.clone(),
            }
        } else {
            SensitivityOutcome::TangentDegraded { tangent }
        })
    }
}

/// The entry for a measured value the pass has no value of, refused at
/// `node`: the lift's valve when that node is the guided replay's typed
/// refusal, the measure's own refusal otherwise.
fn unread_measure(
    pass: &Evaluation<Dual64>,
    node: Option<RecipeNodeId>,
    cause: String,
    record: RecordReading,
) -> SensitivityOutcome {
    if let Some(id) = node
        && let Some(NodeResult::Failed(e)) = pass.result(id)
        && let NodeErrorKind::ProfileLaneReplay { loop_, step, .. } = &e.kind
    {
        return SensitivityOutcome::Unliftable {
            node: id,
            refusal: LiftRefusal::GuidedReplay {
                loop_: *loop_,
                step: *step,
            },
        };
    }
    SensitivityOutcome::MeasureRefused {
        node,
        cause,
        record,
    }
}

// ------------------------------------------------- the verdict's tie

/// The document's analysis axes ([`crate::analysis::is_axis`]: its
/// toleranced variables), in declaration order — the entry set of
/// every driver call.
fn toleranced_params(doc: &Doc<ProfileProgram>) -> impl Iterator<Item = VarId> + '_ {
    doc.free_vars()
        .filter(|(_, free)| crate::analysis::is_axis(free))
        .map(|(id, _)| id)
}

/// Whether a verdict's root box spans exactly this document's
/// analysis axes ([`toleranced_params`]) — the cheap pre-check before
/// the content tie.
fn box_spans_doc_params(root: &ParamBox, doc: &Doc<ProfileProgram>) -> bool {
    let doc_names: Vec<VarId> = toleranced_params(doc).collect();
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
    resolver: Option<&Arc<dyn crate::part::PartResolver>>,
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
    // At the tier the drive ran (`ParamBoxVerdict::symbolic`): a leaf
    // certified with the symbolic tier on can carry a node a
    // numeric-only replay refuses, and the tie would then report a
    // perfectly good build as "not of this build".
    let readback = crate::eval::replay_leaf(
        doc,
        &leaf_opts(tied.box_.clone(), resolver),
        verdict.lane(),
        &crate::eval::LeafPrior::None,
        crate::eval::LeafRequest {
            keys: true,
            ..crate::eval::LeafRequest::default()
        },
        tol,
    );
    tie(doc, tied, &readback.keys)?;
    Ok(
        chamber.map_or(Chamber::LocalOnly, |leaf| Chamber::ChamberCertified {
            leaf: leaf.box_.clone(),
            verdict_vector_key: leaf.verdict_vector_key,
        }),
    )
}

/// The tie itself: the leaf's recorded per-node keys against its
/// replay's, in evaluation order, the first difference named.
fn tie(
    doc: &Doc<ProfileProgram>,
    leaf: &CertifiedLeaf,
    replayed: &[(RecipeNodeId, ContentKey)],
) -> Result<(), SensitivityRefusal> {
    let recorded = &leaf.results.node_keys;
    let n = recorded.len().max(replayed.len());
    for i in 0..n {
        let r = recorded.get(i).copied();
        let p = replayed.get(i).copied();
        if r != p {
            // The replay is of `doc`; the record is of whichever
            // document the drive ran on.
            let node = match (r, p) {
                (Some((recorded, _)), Some((replayed, _))) if recorded == replayed => {
                    DivergedAt::Replayed(doc.spoken(replayed))
                }
                (None, Some((replayed, _))) => DivergedAt::Replayed(doc.spoken(replayed)),
                (Some((recorded, _)), _) => DivergedAt::Recorded(recorded),
                (None, None) => unreachable!("two absent keys are equal"),
            };
            return Err(SensitivityRefusal::VerdictNotOfThisBuild {
                leaf: Box::new(leaf.box_.clone()),
                node: Box::new(node),
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
        /// The variable.
        var: SpokenVar,
    },
    /// The parameter's pass could not read the measure (its doors
    /// refused) — [`SensitivityOutcome::MeasureRefused`], carried.
    MeasureRefused {
        /// The variable.
        var: SpokenVar,
        /// The refusing node, spoken from the document asked about;
        /// `None` for a value no measure is under.
        node: Option<SpokenNode>,
        /// The node error, rendered.
        cause: String,
        /// What the `f64` build of record read for the value —
        /// [`RecordReading::Measured`] marks a refusal the dual alone
        /// made.
        record: RecordReading,
    },
    /// The parameter's seed could not reach the measure — the lift's
    /// typed limit ([`SensitivityOutcome::Unliftable`]).
    Unliftable {
        /// The variable.
        var: SpokenVar,
    },
    /// The parameter carries a [`crate::Distribution::Band`]: limits
    /// without a shape have no σ, and a partial RSS is still a lie
    /// (E5) — so the RSS names it and refuses whole.
    BandHasNoMeasure {
        /// The variable.
        var: SpokenVar,
    },
}

impl Unavailable {
    /// The blocked variable.
    pub fn var(&self) -> &SpokenVar {
        match self {
            Self::TangentDegraded { var }
            | Self::MeasureRefused { var, .. }
            | Self::Unliftable { var }
            | Self::BandHasNoMeasure { var } => var,
        }
    }

    /// This blocker with its variable spoken again from `doc`, a later
    /// version of the document the stackup was taken of
    /// ([`SpokenVar::respoken`]).
    #[must_use]
    pub fn respoken<P>(&self, doc: &Doc<P>) -> Self {
        let mut again = self.clone();
        match &mut again {
            Self::TangentDegraded { var }
            | Self::Unliftable { var }
            | Self::BandHasNoMeasure { var } => *var = var.respoken(doc),
            Self::MeasureRefused { var, node, .. } => {
                *var = var.respoken(doc);
                if let Some(node) = node {
                    *node = node.respoken(doc);
                }
            }
        }
        again
    }
}

impl core::fmt::Display for Unavailable {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TangentDegraded { var } => write!(
                f,
                "parameter {var}'s tangent degraded at the nominal (E9: forfeits its \
                 advisory uses, refuses nothing)"
            ),
            Self::MeasureRefused {
                var,
                node,
                cause,
                record,
            } => {
                write!(f, "parameter {var}'s pass could not read the measure")?;
                if let Some(node) = node {
                    write!(f, " at {node}")?;
                }
                write!(f, ": {cause}")?;
                match record {
                    RecordReading::Measured(_) => write!(
                        f,
                        " — but {record}, so the refusal is the dual's alone: a door DL3 \
                         leaves unrun at a dual, or a decision the two scalars took \
                         differently (a D9 break)"
                    ),
                    RecordReading::Unavailable(_) | RecordReading::Refused { .. } => {
                        write!(f, " ({record})")
                    }
                }
            }
            Self::Unliftable { var } => write!(
                f,
                "parameter {var}'s seed could not reach the measure: the lift refused typed"
            ),
            Self::BandHasNoMeasure { var } => write!(
                f,
                "parameter {var} carries a band: worst-case limits with no shape have \
                 no σ, and a partial RSS is still a lie"
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
    /// The variable.
    pub param: VarId,
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
    /// The document this was taken of, the one document its human
    /// form speaks from. Outside the goldening form and its content key.
    pub document: crate::DocumentId,
    /// The measured value the report is about ([`crate::Doc::observed`]).
    pub measurement: VarId,
    /// The f64 build of record's measured value — read from the
    /// evaluation the passes were bound to (the handed one, or the
    /// driver's own build) — or the typed reason there is none.
    ///
    /// **Absent rather than missing** (M10-6, R2's MINOR-3). The
    /// nominal is an ADVISORY column: E5 puts the certified worst case
    /// first and calls it the only gating number, and E9's rule is that
    /// a degraded advisory column forfeits while the gate stays. A
    /// `min_clearance` measure has no f64 value by construction — it is
    /// answered by an engine that needs an enclosure — and refusing the
    /// whole report for it inverted that rule: the worst case IS
    /// computable from the certified leaves' enclosures, and it was
    /// being withheld because an advisory number beside it was not.
    ///
    /// So this forfeits, `per_param` and `rss` forfeit with it (they
    /// are a `Dual` pass's and there is no tangent either), and
    /// `worst_case` is built and gates.
    pub nominal: Result<f64, crate::measure::MeasureUnavailableAt>,
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
    /// **What the masses in `coverage` MEAN** — priced under a stated
    /// measure, or set-theoretically forced by a Band contributor
    /// (M10-1's obligation, R2's MINOR-7).
    ///
    /// A field rather than a rendering-time computation, because it is
    /// a property of the report and not of one way of printing it:
    /// without it [`Self::serialize`] golden-compares a Band document
    /// and a Uniform one as identical, and the goldening form is where
    /// the priced/forced distinction most needs to be visible.
    pub basis: crate::report::MassBasis,
}

impl Stackup {
    /// **The goldening form** (E10's "serializable for CI goldening"):
    /// deterministic text with every float as its exact bits.
    ///
    /// The driver's own idiom, so a CI row that golden-compares a
    /// stackup and one that golden-compares a verdict read the same
    /// way. Nothing here rounds: a rounded golden is a golden that
    /// stops moving when the report does.
    pub fn serialize(&self) -> String {
        use core::fmt::Write as _;
        let mut s = String::new();
        let _ = writeln!(
            s,
            "stackup measure={} nominal={} chamber={}",
            self.measurement.full(),
            match &self.nominal {
                Ok(v) => format!("{:016x}", v.to_bits()),
                Err(why) => format!("unavailable:{}", why.verb()),
            },
            match &self.chamber {
                Chamber::ChamberCertified {
                    verdict_vector_key, ..
                } => format!("certified:{verdict_vector_key:032x}"),
                Chamber::LocalOnly => "local_only".to_owned(),
            }
        );
        let _ = writeln!(
            s,
            "worst_case lo={:016x} hi={:016x} leaves={}",
            self.worst_case.lo.to_bits(),
            self.worst_case.hi.to_bits(),
            self.worst_case.leaves
        );
        for row in &self.per_param {
            let _ = writeln!(
                s,
                "param {} sensitivity={} contribution={} chamber_span={}",
                row.param.full(),
                sensitivity_text(&row.sensitivity, |id| format!("node {}", id.full()),),
                match &row.contribution {
                    Ok(v) => format!("{:016x}", v.to_bits()),
                    Err(u) => format!("unavailable:{}", u.var().id().full()),
                },
                match &row.chamber_span {
                    Some(c) => format!(
                        "{:016x},{:016x}",
                        c.half_width.to_bits(),
                        c.contribution.to_bits()
                    ),
                    None => "none".to_owned(),
                }
            );
        }
        let _ = writeln!(
            s,
            "rss {}",
            match &self.rss {
                Rss::Advisory { sigma } => format!("advisory:{:016x}", sigma.to_bits()),
                Rss::UnavailableBecause { blockers } => format!(
                    "unavailable:{}",
                    blockers
                        .iter()
                        .map(|b| b.var().id().full().to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                ),
            }
        );
        // The BASIS before the masses it qualifies: a reader who diffs
        // two goldens and sees only this line has still learned the
        // thing that changed.
        let _ = writeln!(s, "basis {}", self.basis.word());
        if let crate::report::MassBasis::Forced { by } = &self.basis {
            for p in by {
                let _ = writeln!(s, "  forced_by {}", p.full());
            }
        }
        let _ = write!(s, "{}", coverage_bits(&self.coverage));
        s
    }

    /// **The content key** of everything [`Self::serialize`] renders —
    /// D9's proof, taken over the report's own bits.
    ///
    /// Derived on demand, never persisted (E10's "derived, never
    /// persisted"), and it moves exactly when the serialized form
    /// moves: two runs of one (recipe slice, box, ε, K) produce equal
    /// keys, and any bit that differs anywhere in the report produces
    /// different ones.
    pub fn content_key(&self) -> ContentKey {
        crate::eval::key_of(0xE5, &self.serialize())
    }

    /// **The human form**: the gating number first, the advisory table
    /// after it and labeled, the coverage with its tail on every line.
    ///
    /// The ORDER is the E5 rule ("labeling and ordering, not
    /// omission"): an engineer who reads only the first two lines has
    /// read the certified answer, and the RSS cannot be met before the
    /// number that gates.
    ///
    /// Each node it names is spoken from `doc`, the document the
    /// stackup was taken of.
    ///
    /// # Panics
    ///
    /// When `doc` is not the document the stackup was taken of.
    pub fn render<P: crate::ProfilePayload>(
        &self,
        doc: &Doc<P>,
        analyzed: &crate::analysis::AnalyzedBox,
    ) -> String {
        use core::fmt::Write as _;
        crate::spoken::assert_taken_of("this stackup", self.document, doc);
        let mut s = String::new();
        let _ = writeln!(s, "stackup of {}", doc.spoken_value(self.measurement));
        let _ = writeln!(
            s,
            "  CERTIFIED WORST CASE (the only gating number): [{}, {}] over {} certified \
             leaf/leaves",
            Readable(self.worst_case.lo),
            Readable(self.worst_case.hi),
            self.worst_case.leaves
        );
        match &self.nominal {
            Ok(v) => {
                let _ = writeln!(
                    s,
                    "  nominal (f64 build): {}  [{}]",
                    Readable(*v),
                    match &self.chamber {
                        Chamber::ChamberCertified { .. } =>
                            "the nominal sits in a certified chamber",
                        Chamber::LocalOnly => "LOCAL ONLY: the nominal's chamber was not certified",
                    }
                );
            }
            // The advisory column forfeits and SAYS SO — E9's rule,
            // and the gating line above it is unaffected.
            Err(why) => {
                let _ = writeln!(
                    s,
                    "  nominal (f64 build): UNAVAILABLE (advisory column forfeited, the \
                     worst case above still gates) — {why}"
                );
            }
        }
        let _ = writeln!(s, "  ADVISORY, never gating:");
        let _ = write!(s, "{}", render_rss(&self.rss, doc));
        for row in &self.per_param {
            let _ = writeln!(
                s,
                "    ∂m/∂{}: {}   contribution {}",
                doc.spoken_var(row.param),
                sensitivity_text(&row.sensitivity, |id| doc.spoken(id).to_string(),),
                match &row.contribution {
                    Ok(v) => Readable(*v).to_string(),
                    Err(u) => format!("[{}]", u.respoken(doc)),
                }
            );
        }
        let _ = write!(
            s,
            "{}",
            crate::report::MassBudget::of(&self.coverage, analyzed).render(doc)
        );
        s
    }
}

/// The human form's rss row; under a refused column, a count and then
/// one line per blocker, each blocker's variable spoken from `doc`.
///
/// The boundary between blockers is the line break, not punctuation a
/// blocker's sentence may itself write. No [`Unavailable`] arm writes
/// a line break (`every_blocker_renders_on_one_line`, over every arm),
/// so the lines under the count are exactly the blockers
/// (`a_refused_rss_splits_back_into_its_blockers`). The parameter name
/// a blocker frames is the document's, not the arm's; what a name may
/// contain is the declaration door's to decide.
fn render_rss<P>(rss: &Rss, doc: &Doc<P>) -> String {
    use core::fmt::Write as _;
    let mut s = String::new();
    match rss {
        Rss::Advisory { sigma } => {
            let _ = writeln!(
                s,
                "    rss σ ≈ {} (linearized, first-order)",
                Readable(*sigma)
            );
        }
        Rss::UnavailableBecause { blockers } => {
            let _ = writeln!(
                s,
                "    rss UNAVAILABLE — {} {}:",
                blockers.len(),
                if blockers.len() == 1 {
                    "blocker"
                } else {
                    "blockers"
                }
            );
            for b in blockers {
                let _ = writeln!(s, "{RSS_BLOCKER_LEAD}{}", b.respoken(doc));
            }
        }
    }
    s
}

/// What opens each blocker's line under a refused rss row.
const RSS_BLOCKER_LEAD: &str = "      - ";

/// One sensitivity reading as a person reads it — the number and its
/// E4 mark, never the number alone. The goldening form writes the same
/// sentence with each node's full id.
///
/// **Public since M10-6's review**: a `NothingCertified` refusal hands
/// a consumer its `sensitivities` and nothing to print them with, so
/// the tour's stop 1 was `Debug`-printing `Derivative { value: 2.0,
/// chamber: LocalOnly }` at a reader. The E4 mark is the load-bearing
/// half of that reading — a derivative certified only in the nominal's
/// own chamber is not a derivative over the box — and it deserves
/// better than a struct dump, in ONE spelling rather than a second one
/// per consumer.
///
/// A node it names is spoken from `doc`, the document the entry was
/// taken of.
///
/// # Panics
///
/// When `doc` is not the document the entry was taken of.
pub fn render_sensitivity<P>(entry: &Sensitivity, doc: &Doc<P>) -> String {
    crate::spoken::assert_taken_of("this sensitivity", entry.document, doc);
    sensitivity_text(&entry.outcome, |id| doc.spoken(id).to_string())
}

/// One sensitivity reading, each node it names written by `node`:
/// spoken in the human form, the full id in the goldening form.
fn sensitivity_text(outcome: &SensitivityOutcome, node: impl Fn(RecipeNodeId) -> String) -> String {
    match outcome {
        SensitivityOutcome::Derivative { value, chamber } => format!(
            "{value} ({})",
            match chamber {
                Chamber::ChamberCertified { .. } => "chamber-certified",
                Chamber::LocalOnly => "LOCAL ONLY",
            }
        ),
        SensitivityOutcome::TangentDegraded { tangent } => {
            format!("degraded tangent ({tangent})")
        }
        SensitivityOutcome::MeasureRefused {
            node: refusing,
            cause,
            record,
        } => {
            let at = refusing.map_or(String::new(), |id| format!(" at {}", node(id)));
            let alone = match record {
                RecordReading::Measured(v) => {
                    format!(" (the f64 build of record measured {v:e}: the dual's refusal alone)")
                }
                RecordReading::Unavailable(_) | RecordReading::Refused { .. } => String::new(),
            };
            format!("refused{at}: {cause}{alone}")
        }
        // Spelled out rather than `Debug`-printed: this string is read
        // by a person in `render` and compared by a golden in
        // `serialize`, and `Debug` is a form neither of those wants.
        SensitivityOutcome::Unliftable { node: id, refusal } => format!(
            "unliftable at {}: {}",
            node(*id),
            match refusal {
                LiftRefusal::GuidedReplay { loop_, step } => format!(
                    "the guided elaboration could not re-confirm loop {loop_} step {step} \
                     at this pass's scalar"
                ),
            }
        ),
    }
}

/// The coverage block of the goldening form — the accounting's own
/// bits, through the reporting layer's one spelling.
fn coverage_bits(coverage: &MeasureAccounting) -> String {
    use core::fmt::Write as _;
    let mut s = String::new();
    let _ = writeln!(
        s,
        "coverage certified={}",
        crate::report::mass_bits(&coverage.certified)
    );
    for (class, m) in &coverage.refused {
        let _ = writeln!(
            s,
            "coverage refused {} {}",
            class.name(),
            crate::report::mass_bits(m)
        );
    }
    let _ = writeln!(
        s,
        "coverage tail={} containment={}",
        crate::report::mass_bits(&coverage.unanalyzed),
        coverage.containment
    );
    s
}

/// A stackup that could not be built. Advisory degradation is carried
/// IN the report ([`Unavailable`]); these are the states with no
/// gating number to hand back — and the one that spent a drive to
/// learn it carries what the drive and the driver did produce.
#[derive(Debug, Clone, PartialEq)]
pub enum StackupRefusal {
    /// The sensitivity driver refused (measure, binding, or a verdict
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
        /// The refusing node (a measure under the value, or the
        /// ancestor it was poisoned through), spoken from the document
        /// asked about; `None` for a value no measure is under.
        node: Option<SpokenNode>,
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
        /// The f64 build's measured value, or the typed reason there
        /// is none (M10-6; see [`Stackup::nominal`]).
        nominal: Result<f64, crate::measure::MeasureUnavailableAt>,
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
        /// The leaf's box, boxed so the refusal stays a small `Err`.
        leaf: Box<ParamBox>,
        /// The refusing node, spoken from the document asked about;
        /// `None` for a value no measure is under.
        node: Option<SpokenNode>,
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
                "{} refuses at the nominal build, so there is no nominal to report: {cause}",
                node.as_ref()
                    .map_or_else(|| "the value".to_owned(), ToString::to_string)
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
                "a certified leaf tied to this build by its content keys refused at {} on \
                 replay — same inputs, a different result (a D9 replay-identity break): \
                 {cause}",
                node.as_ref()
                    .map_or_else(|| "the value".to_owned(), ToString::to_string)
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
/// `of_record` is the validated f64 build of record, bound exactly as
/// [`sensitivities`] binds it. `parallel` runs the sensitivity passes
/// and the per-leaf worst-case evaluations under rayon idiom 1; the
/// report is schedule-independent (D9).
///
/// The worst case re-evaluates the measure over each certified leaf's
/// environment ON THE LANE THE DRIVE RAN — the symbolic tier on or off
/// exactly as `ParamBoxVerdict::symbolic` says,
/// because a leaf the tier certified can carry a node a numeric-only
/// replay refuses. The numeric channel is the same either way, so the
/// hulled brackets are the same bits; what the lane decides is which
/// leaves have one. It ties every leaf's keys to the drive's record on
/// the way and hulls the certified brackets. One evaluation per
/// certified leaf, bounded above by the drive's own cost — the v1 cost
/// decision, taken over a recording dial on `DriveConfig`.
///
/// # Errors
///
/// [`StackupRefusal`] — a driver refusal, a foreign box, a nominal
/// that does not measure, nothing certified (with everything the run
/// did produce), or a broken leaf replay.
// Eight inputs, each read and none written: the box, its verdict and
// the record build are three distinct pairings the door checks, so a
// bundle would be a struct that exists to satisfy a count.
#[allow(clippy::too_many_arguments)]
pub fn stackup(
    doc: &Doc<ProfileProgram>,
    value: VarId,
    analyzed: &AnalyzedBox,
    verdict: &ParamBoxVerdict,
    of_record: Option<&Evaluation<f64>>,
    parallel: bool,
    resolver: Option<&Arc<dyn crate::part::PartResolver>>,
    tol: Tol,
) -> Result<Stackup, StackupRefusal> {
    if ParamBox::of(analyzed) != *verdict.root() {
        return Err(StackupRefusal::ForeignBox);
    }
    let Driven {
        nominal,
        chamber,
        entries,
    } = driver(
        doc,
        value,
        of_record,
        Some(verdict),
        parallel,
        resolver,
        tol,
    )
    .map_err(StackupRefusal::Sensitivity)?;

    // **The nominal, read from the build of record — and
    // FORFEITED rather than fatal when the measure has no f64 value**
    // (M10-6, R2's MINOR-3). E5 makes the certified worst case the only
    // gating number and E9 says a degraded advisory column forfeits
    // while the gate stays; refusing the whole report because an
    // advisory number is missing inverted both. A `min_clearance` is
    // exactly that case, by construction and at every f64 build.
    //
    // A measure that FAILED for any other reason is still fatal here:
    // `measure_of`'s other error arms mean the node did not evaluate,
    // which is a broken report and not a forfeited column.
    let nominal = match nominal {
        Ok(n) => n,
        Err((node, cause)) => {
            return Err(StackupRefusal::MeasureRefusedAtNominal {
                node: node.map(|node| doc.spoken(node)),
                cause,
            });
        }
    };

    if verdict.certified().is_empty() {
        return Err(StackupRefusal::NothingCertified {
            nominal,
            sensitivities: entries,
            coverage: Box::new(verdict.accounting().clone()),
            receipt: verdict.receipt(),
        });
    }
    let worst_case = worst_case(doc, value, verdict, parallel, resolver, tol)?;

    // The advisory columns. Half-widths and σ come off the analyzed
    // box's own axes (the pairing-safe doors), derivatives off the
    // entries; a forfeited entry forfeits exactly its own row and its
    // RSS term (E9), and EVERY band names itself whatever its tangent
    // did (E5: all band contributors, not the first).
    let mut per_param = Vec::with_capacity(entries.len());
    let mut blockers: Vec<Unavailable> = Vec::new();
    let mut sum_sq = 0.0_f64;
    for entry in entries {
        let param = entry.param;
        // Spoken from `doc`, not the box: a box taken before a rename
        // compares equal after it, and the row says the name `doc`
        // holds now.
        let spoken = doc.spoken_var(param);
        // Every entry names a continuous parameter of `doc`, and the
        // ForeignBox check above made `analyzed` span exactly those;
        // an axis missing here is a broken pairing, refused as one.
        let axis = analyzed.get(param).ok_or(StackupRefusal::ForeignBox)?;
        let half_width = 0.5 * axis.offsets.width();
        let sigma = analyzed
            .axis_std_deviation(param)
            .ok_or(StackupRefusal::ForeignBox)?;
        let derivative = match &entry.outcome {
            SensitivityOutcome::Derivative { value, chamber } => Ok((*value, chamber)),
            SensitivityOutcome::TangentDegraded { .. } => Err(Unavailable::TangentDegraded {
                var: spoken.clone(),
            }),
            SensitivityOutcome::MeasureRefused {
                node,
                cause,
                record,
            } => Err(Unavailable::MeasureRefused {
                var: spoken.clone(),
                node: node.map(|node| doc.spoken(node)),
                cause: cause.clone(),
                record: record.clone(),
            }),
            SensitivityOutcome::Unliftable { .. } => Err(Unavailable::Unliftable {
                var: spoken.clone(),
            }),
        };
        let contribution = derivative
            .as_ref()
            .map(|(value, _)| value.abs() * half_width)
            .map_err(Clone::clone);
        let chamber_span = match &derivative {
            Ok((value, Chamber::ChamberCertified { leaf, .. })) => leaf.get(param).map(|axis| {
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
                if let Err(MeasureUnavailable::BandHasNoMeasure { .. }) = sigma {
                    blockers.push(Unavailable::BandHasNoMeasure { var: spoken });
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
        document: doc.id(),
        measurement: value,
        nominal,
        chamber,
        per_param,
        worst_case,
        rss,
        coverage: verdict.accounting().clone(),
        basis: crate::report::MassBasis::of(analyzed),
    })
}

/// The hull of value-channel interval evaluations of the measure over
/// the certified leaves, every leaf tied to the drive's record on the
/// way. Tangent-free BY TYPE: the evaluation scalar is
/// [`geom_core::Interval`], which has no tangent channel, and the
/// bracket is read through
/// [`geom_core::CertifiedEnclosure::certified_bracket`] — the
/// domain-honest door, so a refused enclosure refuses named instead of
/// hulling a NaN.
fn worst_case(
    doc: &Doc<ProfileProgram>,
    value: VarId,
    verdict: &ParamBoxVerdict,
    parallel: bool,
    resolver: Option<&Arc<dyn crate::part::PartResolver>>,
    tol: Tol,
) -> Result<WorstCase, StackupRefusal> {
    let leaves = verdict.certified();
    let lane = verdict.lane();
    // One shared memo prior over the DEGENERATE box — every axis fixed
    // at the nominal, bound through the same door and the same
    // arithmetic a leaf's fixed axes bind through — so the
    // parameter-independent subgraph (and every node downstream only
    // of fixed axes) is evaluated once and served to every leaf.
    // Read-only and shared, so the parallel schedule sees exactly what
    // the sequential one does; the memo serves bit-equal inputs only,
    // so the hull is the same with or without it.
    // The root collapsed to the nominal (`ParamBox::at_nominal`): it
    // varies the leaves' axes, so its profile lift is theirs (guided) and
    // a node outside every axis's cone keys alike in prior and leaf.
    let nominal_box = verdict.root().at_nominal();
    // The hull runs on the lane the drive ran on
    // (`ParamBoxVerdict::symbolic`), for the tie's reason: a leaf the
    // symbolic tier certified may carry a node a numeric-only replay
    // refuses. The numeric channel of the wrapped scalar is the plain
    // one's, verbatim, so the BRACKETS are the same bits either way —
    // only which leaves have one changes. The prior is the numeric
    // lane's alone; `LeafPrior` states why the symbolic lane has none.
    let prior = crate::eval::LeafPrior::of(doc, &leaf_opts(nominal_box, resolver), lane, tol);
    let one = |leaf: &CertifiedLeaf| -> Result<(f64, f64), StackupRefusal> {
        let readback = crate::eval::replay_leaf(
            doc,
            &leaf_opts(leaf.box_.clone(), resolver),
            lane,
            &prior,
            crate::eval::LeafRequest {
                keys: true,
                measure: Some(value),
                ..crate::eval::LeafRequest::default()
            },
            tol,
        );
        tie(doc, leaf, &readback.keys).map_err(StackupRefusal::Sensitivity)?;
        readback
            .measure
            .unwrap_or(Ok(None))
            .map_err(|(node, cause)| StackupRefusal::LeafDiverged {
                leaf: Box::new(leaf.box_.clone()),
                node: node.map(|node| doc.spoken(node)),
                cause,
            })?
            .ok_or_else(|| StackupRefusal::WorstCaseUncertified {
                leaf: leaf.box_.clone(),
            })
    };
    // D9 idiom 1 again: an indexed map, then one sequential fold over
    // the collected brackets, so the hull is the same bits in either
    // schedule. The brackets are certified (no NaN reaches the fold),
    // so min/max are exact lattice operations here.
    // Every leaf's result is collected before an error is chosen, so
    // the refusal is the first in the verdict's leaf order whatever the
    // schedule (the driver's own collection says why).
    let results: Vec<Result<(f64, f64), StackupRefusal>> = if parallel {
        use rayon::prelude::*;
        leaves.par_iter().map(one).collect()
    } else {
        leaves.iter().map(one).collect()
    };
    let brackets = results.into_iter().collect::<Result<Vec<_>, _>>()?;
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{
        BindingViolation, BuildIdentity, Chamber, Drift, RSS_BLOCKER_LEAD, Record, RecordReading,
        Rss, SensitivityOutcome, Unavailable, Unread, pass_opts, render_rss, stamped,
    };
    use crate::eval::{CancelToken, Evaluation, NodeResult, evaluate};
    use crate::program::{ProfileDoc, ProfileProgram};
    use crate::{VarId, VarName};
    use geom_core::{Dual64, Tol};

    test_utils::f6_variants! {
        /// Every [`Unavailable`] arm, welded to the enum by the match
        /// the macro writes; [`every_arm`] must produce each of them.
        const UNAVAILABLE: Unavailable = [
            TangentDegraded,
            MeasureRefused,
            Unliftable,
            BandHasNoMeasure,
        ];
    }

    /// One blocker of every arm, for `param` — checked against the
    /// weld, so an arm with no example here fails every row that reads
    /// this.
    fn every_arm(param: &'static str) -> Vec<Unavailable> {
        let param =
            crate::SpokenVar::new(crate::VarId::new(0, 1), Some(VarName::from_static(param)));
        let all = vec![
            Unavailable::TangentDegraded { var: param.clone() },
            Unavailable::MeasureRefused {
                var: param.clone(),
                node: None,
                cause: "a refusal".to_owned(),
                record: RecordReading::Measured(1.0),
            },
            Unavailable::Unliftable { var: param.clone() },
            Unavailable::BandHasNoMeasure { var: param },
        ];
        let made: Vec<String> = all.iter().map(test_utils::f6::variant_identifier).collect();
        let made: Vec<&str> = made.iter().map(String::as_str).collect();
        if let Some(report) = test_utils::census::set_difference(
            UNAVAILABLE.identifiers(),
            &made,
            "`every_arm`'s examples and the `Unavailable` roster disagree",
            "made here and absent from the roster: weld the arm into `UNAVAILABLE`",
            "in the roster with no example here: add one to `every_arm`",
        ) {
            panic!("{report}");
        }
        assert_eq!(
            made.len(),
            UNAVAILABLE.identifiers().len(),
            "one example per Unavailable arm, no more: an arm is made twice in {made:?}"
        );
        all
    }

    /// The claim the rss row's line-per-blocker layout rests on: no
    /// arm's sentence writes a line break of its own.
    #[test]
    fn every_blocker_renders_on_one_line() {
        for b in every_arm("width") {
            let line = b.to_string();
            assert!(!line.contains(['\n', '\r']), "{line:?}");
        }
    }

    /// A refused rss row splits back into exactly the blockers it was
    /// made from, in order — including blockers whose sentences carry
    /// the punctuation a flat join would have split on, which a
    /// sentence is free to write. A NAME cannot carry it: a `VarName`
    /// is one identifier by construction, so the second batch is a
    /// second identifier and the sentences alone carry the separators.
    #[test]
    fn a_refused_rss_splits_back_into_its_blockers() {
        let mut blockers = every_arm("width");
        blockers.extend(every_arm("depth"));
        let rendered = render_rss(
            &Rss::UnavailableBecause {
                blockers: blockers.clone(),
            },
            &no_vars(),
        );
        let mut lines = rendered.lines();
        assert_eq!(
            lines.next(),
            Some(format!("    rss UNAVAILABLE — {} blockers:", blockers.len()).as_str()),
            "{rendered}"
        );
        let items: Vec<&str> = lines
            .map(|l| {
                l.strip_prefix(RSS_BLOCKER_LEAD)
                    .unwrap_or_else(|| panic!("a blocker line opens with the lead: {l:?}"))
            })
            .collect();
        let want: Vec<String> = blockers.iter().map(ToString::to_string).collect();
        assert_eq!(items, want, "{rendered}");
    }

    #[test]
    fn a_single_blocker_is_counted_in_the_singular() {
        let rendered = render_rss(
            &Rss::UnavailableBecause {
                blockers: vec![Unavailable::Unliftable {
                    var: crate::SpokenVar::new(
                        crate::VarId::new(0, 1),
                        Some(VarName::from_static("w")),
                    ),
                }],
            },
            &no_vars(),
        );
        assert_eq!(
            rendered,
            "    rss UNAVAILABLE — 1 blocker:\n      - parameter w's seed could not reach the \
             measure: the lift refused typed\n"
        );
    }

    /// **The slab**: a unit square extruded by `depth`, measured cap to
    /// cap — the thickness, whose derivative in `depth` is 1. The
    /// document, `depth`, and the thickness.
    fn slab(depth: f64) -> (ProfileDoc, VarId, VarId) {
        use crate::{CapEnd, DocEdit, Formula, Node, RoleSeg, SitedRef, StableName};
        let tol = Tol::witness();
        let push = |doc: &ProfileDoc, edit: DocEdit<ProfileProgram>| {
            let applied = crate::apply(doc, &edit, tol).expect("the edit applies");
            (applied.doc, applied.record.minted)
        };
        let doc = crate::doc::Doc::empty_derived("stackup-binding-slab", tol);
        let (doc, _) = push(
            &doc,
            DocEdit::DeclareVar {
                name: VarName::from_static("depth"),
                def: crate::VarDecl::Free(crate::FreeVar::Continuous {
                    dim: crate::Dimension::Length,
                    value: depth,
                    display_unit: crate::UnitSym::canonical_for(crate::Dimension::Length),
                    distribution: Some(crate::Distribution::Uniform {
                        lo: -1e-3,
                        hi: 1e-3,
                    }),
                }),
            },
        );
        let depth_var = doc.var_named("depth").expect("declared");
        let (doc, profile) = push(
            &doc,
            DocEdit::InsertNode {
                node: Box::new(Node::Profile(ProfileProgram {
                    loops: vec![
                        crate::LoopProgram::polygon([
                            (0.0, 0.0),
                            (1.0, 0.0),
                            (1.0, 1.0),
                            (0.0, 1.0),
                        ])
                        .expect("finite corners"),
                    ],
                    ids: Vec::new(),
                    on: None,
                })),
                fresh: Vec::new(),
            },
        );
        let (doc, block) = push(
            &doc,
            DocEdit::InsertNode {
                node: Box::new(Node::Extrude {
                    profile: profile.expect("minted").into(),
                    distance: Formula::named(
                        VarName::from_static("depth"),
                        crate::Dimension::Length,
                    ),
                }),
                fresh: Vec::new(),
            },
        );
        let block = block.expect("minted");
        let cap = |end| {
            SitedRef::new(
                block,
                StableName {
                    kind: crate::EntityKind::Face,
                    node: block,
                    path: vec![RoleSeg::Cap(end)],
                },
            )
        };
        let measured = crate::measure(
            &doc,
            &[crate::MeasurePrimitive::Distance {
                a: cap(CapEnd::Start),
                b: cap(CapEnd::End),
            }],
            tol,
        )
        .expect("the measure inserts");
        (measured.doc, depth_var, measured.measured.outputs[0])
    }

    /// `doc`'s build-path `f64` evaluation, stamped with `doc`'s pin as
    /// the driver stamps its own runs.
    fn record_run(doc: &ProfileDoc) -> Evaluation<f64> {
        let ev = evaluate(
            doc,
            None,
            &CancelToken::new(),
            &pass_opts(None, None),
            Tol::witness(),
        );
        stamped(ev, pin(doc))
    }

    /// `doc`'s `Dual64` pass seeded at `seed`, stamped likewise.
    fn pass_run(doc: &ProfileDoc, seed: VarId) -> Evaluation<Dual64> {
        let opts = pass_opts(Some(seed), None);
        stamped(
            evaluate(doc, None, &CancelToken::new(), &opts, Tol::witness()),
            pin(doc),
        )
    }

    fn pin(doc: &ProfileDoc) -> crate::ContentPin {
        crate::persist::content_pin(doc, Tol::witness()).expect("canonicalizes")
    }

    /// **A pass whose measured value is perturbed refuses as drift**,
    /// over inputs the binding finds equal: one ulp on the value half,
    /// the tangent untouched. Unperturbed, the same pass reads ∂/∂depth
    /// = 1, so the refusal is the perturbation's.
    #[test]
    fn a_pass_whose_measured_value_is_perturbed_refuses_as_drift() {
        let (doc, depth, thickness) = slab(1.0);
        let record = Record::new(&doc, &record_run(&doc), thickness).expect("binds");
        let mut pass = pass_run(&doc, depth);
        match record.read(&doc, &pass, thickness, &Chamber::LocalOnly) {
            Ok(SensitivityOutcome::Derivative { value, .. }) => assert_eq!(value, 1.0),
            other => panic!("the unperturbed pass reads its derivative: {other:?}"),
        }
        let (node, _) = pass.read_port(thickness).expect("a measure's output");
        let Some(NodeResult::Ok(v)) = pass.nodes.get_mut(&node) else {
            panic!("the measure built");
        };
        let crate::eval::ValuePayload::Measure { value, .. } = &mut v.payload else {
            panic!("a measure's payload");
        };
        let base = value.value;
        value.value = f64::from_bits(base.to_bits() + 1);
        let perturbed = value.value;
        match record.read(&doc, &pass, thickness, &Chamber::LocalOnly) {
            Err(Unread::Drift(Drift { pass, base: got })) => {
                assert_eq!(pass.to_bits(), perturbed.to_bits(), "the pass's value half");
                match got {
                    RecordReading::Measured(got) => {
                        assert_eq!(got.to_bits(), base.to_bits(), "the record's");
                    }
                    other => panic!("the record measured: {other:?}"),
                }
            }
            other => panic!("a perturbed measured value must refuse as drift: {other:?}"),
        }
    }

    /// **A refusal only the dual made says so.** The pass's measure is
    /// planted as refused while the build of record measured it: the
    /// entry carries the node, the rendered cause and the record's
    /// value, and the stackup's blocker built from it names the
    /// refusal as the dual's alone — the shape a D9 break in a decision
    /// takes when it surfaces as a refusal.
    #[test]
    fn a_refusal_only_the_dual_made_carries_the_records_value() {
        let (doc, depth, thickness) = slab(1.0);
        let built = record_run(&doc);
        let record = Record::new(&doc, &built, thickness).expect("binds");
        let RecordReading::Measured(base) = record.reading() else {
            panic!("the record measures the thickness");
        };
        let mut pass = pass_run(&doc, depth);
        let (node, _) = pass.read_port(thickness).expect("a measure's output");
        pass.nodes.insert(
            node,
            NodeResult::Failed(crate::eval::NodeError {
                node,
                kind: crate::eval::NodeErrorKind::ToleranceConflict {
                    document_eps: 1.0,
                    process_eps: 2.0,
                },
                escalations: std::sync::Arc::new(Vec::new()),
            }),
        );
        let outcome = record
            .read(&doc, &pass, thickness, &Chamber::LocalOnly)
            .expect("a refused measure is an entry, not a refusal");
        let SensitivityOutcome::MeasureRefused {
            node: Some(at),
            cause,
            record: RecordReading::Measured(measured),
        } = &outcome
        else {
            panic!("the dual's refusal, with the record's value: {outcome:?}");
        };
        assert_eq!(*at, node);
        assert_eq!(measured.to_bits(), base.to_bits());
        assert!(!cause.is_empty());
        let blocker = Unavailable::MeasureRefused {
            var: doc.spoken_var(depth),
            node: Some(doc.spoken(node)),
            cause: cause.clone(),
            record: RecordReading::Measured(*measured),
        };
        let text = blocker.to_string();
        assert!(text.contains(cause.as_str()), "{text}");
        assert!(text.contains("the dual's alone"), "{text}");
    }

    /// **A pass from another revision, tolerance or parts set refuses
    /// at the binding**, before its measured value is read.
    #[test]
    fn a_pass_of_other_inputs_refuses_at_the_binding() {
        let (doc, depth, thickness) = slab(1.0);
        let record = Record::new(&doc, &record_run(&doc), thickness).expect("binds");
        let refused = |pass: &Evaluation<Dual64>| match record.read(
            &doc,
            pass,
            thickness,
            &Chamber::LocalOnly,
        ) {
            Err(Unread::Binding(v)) => v,
            other => panic!("the binding must refuse: {other:?}"),
        };

        // Another revision: the same recipe at another depth.
        let (other, _, _) = slab(1.5);
        assert_eq!(
            refused(&pass_run(&other, depth)),
            BindingViolation::Revision
        );

        // Another tolerance, restamped (one process holds one ε).
        let mut pass = pass_run(&doc, depth);
        pass.tolerance.k *= 2.0;
        assert_eq!(refused(&pass), BindingViolation::Tolerance);

        // Another parts set: a reference the pass resolved and the
        // record did not, written into the pass's identity (a document
        // that instantiates nothing resolves no part;
        // `msolve14_run_scalar::a5_sensitivities_cross_a_seated_mate`
        // has the real case, over an assembly).
        let mut identity = BuildIdentity::of(&pass_run(&doc, depth)).expect("an identity");
        identity.parts.insert((
            crate::DocRef {
                id: crate::DocumentId::derive("stackup-binding-part"),
                pin: crate::ContentPin([7; 32]),
            },
            Tol::witness().eps().to_bits(),
        ));
        assert_eq!(
            record.identity.bind(&doc, &identity),
            Err(BindingViolation::Parts)
        );

        // An unpinned pass cannot be bound.
        let mut pass = pass_run(&doc, depth);
        pass.pin = None;
        assert_eq!(refused(&pass), BindingViolation::Unpinned { refusal: None });
    }

    /// A document holding no variable, so a blocker is said as it was
    /// spoken.
    fn no_vars() -> crate::doc::Doc<crate::program::ProfileProgram> {
        crate::doc::Doc::empty_derived("stackup-render-rows", geom_core::Tol::witness())
    }
}
