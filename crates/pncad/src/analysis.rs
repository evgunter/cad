//! **The analysis lane** (ERROR-DESIGN E1): the one door that reads a
//! parameter's [`Distribution`](crate::document::Distribution) back.
//!
//! Kept separate from [`crate::document`] because the split is the
//! design's, not the façade's: a distribution is document state that
//! `document`'s doors author and persist, while everything here is
//! DERIVED — an analyzed box and mass columns computed on request from
//! a document plus a policy, never stored, and never seen by
//! evaluation.
//!
//! The three doors: [`analyzed_box`] projects the document to the box
//! the interval and driver lanes work over, [`tail_mass`] reports the
//! mass that box leaves out, and [`box_mass`] prices a sub-interval.
//! The last two refuse typed on a
//! [`Band`](crate::document::Distribution::Band) — limits without a
//! shape price nothing, and no report may quietly promote them to
//! uniform.
//!
//! [`AnalysisPolicy`] is request configuration: the analyzed box is
//! the analysis's knob, not a property of the distribution, and
//! [`DEFAULT_QUANTILE_MASS`] is the ±3σ convention it defaults to.

//! # The certified half
//!
//! Everything above is scalar-free: a box and its masses are `f64`
//! arithmetic over a document. The CERTIFIED half below — the E6
//! driver, the E5 stackup and E10's reports — answers at the certified
//! scalar, and a door that answered `f64` in its place would be a
//! sampler. E11.1's advisory estimator is carried beside it and is not
//! part of it: it is pure `f64` replay, as its entry says. What the
//! façade's own census said while these doors were interior is that
//! the curated face of the analysis lane "is the REPORTING surface — persisted, goldened
//! stackups — which is where the façade row lands"; M10-6 builds that
//! surface, so this is that landing. The alternative was to leave a
//! consumer — the tour's tolerance cell is the first — reaching past
//! the façade into `editor_core`, which is the invariant the whole
//! crate exists to keep.
//!
//! What is NOT carried: the leaf-level clearance engine
//! (`editor_core::clearance`)'s doors. A document-layer consumer asks a
//! clearance as a requirement — a clearance assertion, which
//! [`crate::document`] carries as document state — and reads its
//! verdict here: per certified leaf through [`assertion_at`], or at
//! nominal through [`clearances_at_nominal`].

// `ParamBoxError` and `SeedError` are what `document::NodeErrorKind`'s
// `ParamBox` and `Seed` arms hold: a payload type a consumer can match
// and cannot name is the refusal `crate::document` rules out for
// `NodeRefusal`.
pub use editor_core::{
    AnalysisPolicy, AnalysisPolicyError, AnalyzedBox, AnalyzedParam, DEFAULT_QUANTILE_MASS,
    MeasureUnavailable, OffsetInterval, ParamBoxError, SeedError, analyzed_box, box_mass,
    sample_offset, tail_mass,
};

/// The E6 driver and the box it drives over.
pub use editor_core::{
    BoxAxis, BudgetKind, CertifiedLeaf, DEFAULT_MAX_DEPTH, DEFAULT_MAX_LEAVES, DriveConfig,
    DriveRefusal, LeafResults, MeasureAccounting, ParamBox, ParamBoxVerdict, ReasonClass, Receipt,
    RefusalReason, RefusedLeaf, drive,
};

/// The E4/E5 sensitivity and stackup report — the answer to "does this
/// measurement hold over its tolerances", with its field types, because
/// a report whose fields cannot be named is a report a consumer can
/// print and not read.
pub use editor_core::{
    Chamber, ChamberSpan, DivergedAt, LiftRefusal, PerParam, RecordReading, Rss, Sensitivity,
    SensitivityOutcome, SensitivityRefusal, Stackup, StackupRefusal, Unavailable, WorstCase,
    render_sensitivity, stackup,
};

/// **The recorded requirement, read back over one certified leaf.**
///
/// E10 says an assertion's verdict per certified leaf is what a CI row
/// gates on, and this is the door that answers it. Carried onto the
/// façade because the alternative a consumer reaches for — comparing
/// `Stackup::worst_case.lo` against the bound with `<` — is an `f64`
/// comparison over quantities that can differ by less than the run's
/// coincidence threshold, and it manufactures a certainty the kernel
/// refuses one line away. The tour's tolerance cell made exactly that
/// mistake before M10-6's review.
pub use editor_core::drive::assertion_at;

/// The three-state verdict [`assertion_at`] answers with, and the
/// reasons its third state carries — a consumer that cannot NAME the
/// states cannot keep them three.
pub use editor_core::{AssertionVerdict, UnevaluatedReason};

/// **The E11.1 advisory estimator**: pure `f64` replay, as E11.1 says.
/// [`summarize`] rides here for the reason it is public at all: a
/// consumer that holds its own replay beside a [`McReport`] and
/// requires the two to agree BIT FOR BIT must reduce with the same
/// function the report was reduced with. A transcription of it is a
/// second spelling, and a bitwise comparison of two spellings tests
/// the spellings.
pub use editor_core::mc::{
    DEFAULT_SAMPLES, DEFAULT_SEED, McAssertion, McConfig, McMeasure, McRefusal, McReport, McValue,
    monte_carlo, sample_offsets, summarize,
};

/// The E10/E11.6 reporting layer.
pub use editor_core::report::{
    HistogramRow, LeafHistogram, MassBasis, MassBudget, ReportCache, leaf_histogram, report_key,
};

/// The clearance requirement's vocabulary: the claim a document writes
/// and the report its verdict carries.
pub use editor_core::clearance::{
    ClearanceRefusal, ClearanceReport, ClearanceVerdict, GeometryWitness, ParamWitness, Witness,
};

/// **Every clearance requirement of `doc`, answered at nominal**: the
/// document evaluated at the certifying scalar over no parameter box,
/// and each clearance assertion's verdict read back in plain numbers,
/// in document order. The door an at-rest certify seam submits: it is
/// stopped by `cancel` like any evaluation, and says so.
///
/// # Errors
///
/// [`Canceled`] when `cancel` was set before the evaluation finished.
pub fn clearances_at_nominal(
    doc: &crate::document::ProfileDoc,
    cancel: &crate::document::CancelToken,
    tol: geom_core::Tol,
) -> Result<Vec<ClearanceAnswer>, Canceled> {
    use editor_core::{AssertionVerdict, Claim, EvalOutcome, Node, NodeResult, ValuePayload};
    let ev: editor_core::Evaluation<geom_core::Interval> =
        editor_core::evaluate(doc, None, cancel, &editor_core::EvalOptions::default(), tol);
    if ev.outcome == EvalOutcome::Canceled {
        return Err(Canceled);
    }
    Ok(doc
        .ids()
        .iter()
        .copied()
        .filter(|&id| matches!(doc.node(id), Some(Node::Assertion(Claim::Clearance { .. }))))
        .map(|assertion| {
            let answer = match ev.result(assertion) {
                Some(NodeResult::Ok(value)) => match &value.payload {
                    ValuePayload::Assertion(AssertionVerdict::Clearance(report)) => {
                        ClearanceOutcome::of(report.verdict())
                    }
                    _ => unreachable!("a clearance assertion's value is its report"),
                },
                Some(NodeResult::Failed(_) | NodeResult::Poisoned { .. }) | None => {
                    ClearanceOutcome::Unanswered {
                        sentence: ev.node_error(assertion).map_or_else(
                            || format!("{} did not evaluate", doc.spoken(assertion)),
                            |e| e.kind_spoken(doc),
                        ),
                    }
                }
            };
            ClearanceAnswer { assertion, answer }
        })
        .collect())
}

/// [`clearances_at_nominal`] was stopped by its cancel token.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Canceled;

/// One clearance assertion's answer.
#[derive(Debug, Clone, PartialEq)]
pub struct ClearanceAnswer {
    /// The assertion node.
    pub assertion: crate::document::RecipeNodeId,
    /// What it answered.
    pub answer: ClearanceOutcome,
}

/// **A clearance verdict in plain numbers.**
#[derive(Debug, Clone, PartialEq)]
pub enum ClearanceOutcome {
    /// Holds; a bound with no upper end.
    Holds,
    /// Holds; this pair lies within the upper bound and proves it.
    Within(PairOfPoints),
    /// Violated: this pair is closer than the lower bound.
    Violated(PairOfPoints),
    /// Violated: every pair is certified clear of the upper bound.
    Exceeded,
    /// The third state: the engine's typed refusal, in words with its
    /// recourse, and its stable class name.
    Refused {
        /// The refusal's class ([`ClearanceRefusal::name`]).
        name: &'static str,
        /// The refusal said, with its recourse.
        sentence: String,
    },
    /// The assertion did not evaluate: a seat that does not resolve, or
    /// a node under it that failed.
    Unanswered {
        /// Why, said from the document.
        sentence: String,
    },
}

impl ClearanceOutcome {
    fn of(verdict: &ClearanceVerdict) -> Self {
        match verdict {
            ClearanceVerdict::Holds => Self::Holds,
            ClearanceVerdict::Within(w) => Self::Within(PairOfPoints::of(w)),
            ClearanceVerdict::Violated(w) => Self::Violated(PairOfPoints::of(w)),
            ClearanceVerdict::Exceeded => Self::Exceeded,
            ClearanceVerdict::Refused(r) => Self::Refused {
                name: r.name(),
                sentence: r.to_string(),
            },
        }
    }
}

/// Two face points and their distance, in the frame the pair was
/// decided in, at `f64`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PairOfPoints {
    /// The point on the first seat's face.
    pub a: [f64; 3],
    /// The point on the second seat's face.
    pub b: [f64; 3],
    /// Their distance.
    pub distance: f64,
    /// The first seat's face.
    pub face_a: topo::FaceKey,
    /// The second seat's face.
    pub face_b: topo::FaceKey,
}

impl PairOfPoints {
    fn of(w: &Witness) -> Self {
        let g = &w.geometry;
        let p = |q: geom_core::Point3<f64>| [q.x, q.y, q.z];
        Self {
            a: p(g.a_point),
            b: p(g.b_point),
            distance: g.distance,
            face_a: g.a,
            face_b: g.b,
        }
    }
}
