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

pub use editor_core::{
    AnalysisPolicy, AnalysisPolicyError, AnalyzedBox, AnalyzedParam, DEFAULT_QUANTILE_MASS,
    MeasureUnavailable, OffsetInterval, analyzed_box, box_mass, tail_mass,
};
