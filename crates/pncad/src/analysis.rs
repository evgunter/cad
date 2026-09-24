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
//! (`editor_core::clearance`). Its consumer vocabulary is a selection
//! and a certified leaf, and the answer a document-layer consumer wants
//! from it comes through the `min_clearance` MEASURE, which is document
//! state and is carried by [`crate::document`].

// `ParamBoxError` and `SeedError` are what `document::NodeErrorKind`'s
// `ParamBox` and `Seed` arms hold: a payload type a consumer can match
// and cannot name is the refusal `crate::document` rules out for
// `VerbKind` and `NodeRefusal`.
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
    Chamber, ChamberSpan, LiftRefusal, PerParam, Rss, Sensitivity, SensitivityOutcome,
    SensitivityRefusal, Stackup, StackupRefusal, Unavailable, WorstCase, render_sensitivity,
    stackup,
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
pub use editor_core::{AssertionVerdict, Certified, UnevaluatedReason, WINDOW_TIGHTENING};

/// **The E11.1 advisory estimator**: pure `f64` replay, as E11.1 says.
/// [`summarize`] rides here for the reason it is public at all: a
/// consumer that holds its own replay beside a [`McReport`] and
/// requires the two to agree BIT FOR BIT must reduce with the same
/// function the report was reduced with. A transcription of it is a
/// second spelling, and a bitwise comparison of two spellings tests
/// the spellings.
pub use editor_core::mc::{
    DEFAULT_SAMPLES, DEFAULT_SEED, McAssertion, McConfig, McMeasure, McRefusal, McReport,
    monte_carlo, sample_offsets, summarize,
};

/// The E10/E11.6 reporting layer.
pub use editor_core::report::{
    HistogramRow, LeafHistogram, MassBasis, MassBudget, ReportCache, leaf_histogram, report_key,
};
