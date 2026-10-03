//! What one check finding FOUND, flattened once for the value that
//! carries it.
//!
//! `crate::tags::check_evidence_tag` answers WHICH evidence arm fired.
//! This module answers what the arm CARRIES, as a record whose every
//! field is present on every arm, `None` where the arm does not carry
//! one — so `getattr` never raises on the Python side and a caller
//! reads a payload without first branching on `variant`.
//!
//! # Why it lives outside `py`
//!
//! The match is a DRIFT ALARM: it is exhaustive with no wildcard, so
//! an arm added kernel-side is a compile error here rather than a
//! silently all-`None` payload nobody chose. `crate::py` compiles
//! only under the `python` feature, so a projection sited there is an
//! alarm that does not ring on the row that runs everywhere. Sited
//! here it rings on every build. It is the `crate::mate_payload` and
//! `crate::pick_payload` shape, for the same reasons.
//!
//! One record rather than one match per attribute, and the reason is
//! `pick_payload`'s second one rather than `mate_payload`'s
//! arithmetic: **three of the six arms are unreachable from Python**.
//! `escalated` and `unsupported` need the shell door to escalate on a
//! subject the registry gathered, `separation_unavailable` needs the
//! box builder to refuse over the whole product, and nothing in the
//! authoring surface arranges either. Sited here, `src/tests.rs` can
//! build the arms the façade's types allow and read what each one
//! puts on the wire; sited in `py/`, the projection of those arms
//! would be pinned nowhere at all.
//!
//! # What is not here
//!
//! A NESTED REFUSAL is not flattened into its FIELDS. `Escalated` and
//! `Unsupported` each hold the shell door's own typed refusal and
//! `SeparationUnavailable` the boolean class beside its sentence;
//! which shell, and which face, belong to those types' own
//! vocabularies. What crosses here is the word a caller branches on —
//! `inner_variant` for the shell refusal's discriminant,
//! `boolean_variant` for the boolean class — and the prose those
//! types render, as `reason`.

use std::borrow::Cow;

use pncad::document::{CheckEvidence, RecipeNodeId};

use crate::tags::{
    boolean_error_tag, coherence_condition_tag, shell_classify_error_tag, unexaminable_tag,
};

/// What one [`CheckEvidence`] arm carries, every field present.
///
/// Borrowed from the evidence rather than cloned out of it: the
/// separation arm's sentence is a `String` the kernel already owns,
/// and the shell arms' is rendered here, so the two spellings of
/// `reason` are the two halves of a [`Cow`].
// `PartialEq` without `Eq`: the record carries measured lengths, and
// a float has no total equality to derive.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckEvidencePayload<'a> {
    /// Components actually found.
    pub actual: Option<u32>,
    /// The expectation the subject was held to.
    pub expected: Option<u32>,
    /// The counterpart subject's root, for a pair held apart by no
    /// certificate.
    pub other_root: Option<RecipeNodeId>,
    /// That counterpart's output-body index.
    pub other_output: Option<u32>,
    /// The underlying refusal's own prose, where the arm holds one.
    pub reason: Option<Cow<'a, str>>,
    /// **The shell door's own refusal**, as the branchable word
    /// ([`crate::tags::shell_classify_error_tag`]) — which of its four
    /// ways it refused, on the two arms that carry one.
    pub inner_variant: Option<&'static str>,
    /// **The boolean refusal's own class**, as the branchable word
    /// ([`crate::tags::boolean_error_tag`]) — which kernel refusal
    /// made separation unavailable, on the one arm that carries one.
    ///
    /// A field of its own rather than a second vocabulary under
    /// `inner_variant`: the two alphabets are not disjoint (`band` and
    /// `escalated` are words in both), so one attribute carrying
    /// either would answer a caller that did not first branch on the
    /// arm, and answer it wrong.
    pub boolean_variant: Option<&'static str>,
    /// **The chart-coherence arm's own inner discriminant** — which of
    /// the three conditions was measured
    /// ([`crate::tags::coherence_condition_tag`]), or why one loop was
    /// out of reach ([`crate::tags::unexaminable_tag`]).
    ///
    /// Two alphabets under one attribute, which [`Self::boolean_variant`]
    /// refuses one field over — and the difference is the reason that
    /// field states. The objection there is that `band` and `escalated`
    /// are words in BOTH alphabets, so a caller who did not first branch
    /// on the arm gets an answer that is wrong rather than absent. These
    /// two alphabets are DISJOINT, so the same caller gets an answer that
    /// is right, and the arm remains readable off the word.
    pub chart_variant: Option<&'static str>,
    /// The chart-coherence measurement as a LENGTH, `gap * lever` — the
    /// only unit the band is in, and the only one the finding is judged
    /// in.
    pub metres: Option<f64>,
    /// That measurement's two factors: the disagreement in the chart's
    /// own units (radians of u, or v's units by surface kind)...
    pub gap: Option<f64>,
    /// ...and the lever arm in metres per chart unit at the point the
    /// gap is about. Zero on a chart axis, where an azimuth carries no
    /// length at all.
    pub lever: Option<f64>,
    /// The band the measurement was judged against, per finding.
    ///
    /// It rides the finding rather than the report for the reason the
    /// kernel states at [`pncad::topo::CoherenceFinding::eps`]: a
    /// measurement read without the band it was judged at is a number
    /// without a claim.
    pub eps: Option<f64>,
}

impl CheckEvidencePayload<'_> {
    /// Which attributes this payload CARRIES, in the order the value
    /// publishes them.
    ///
    /// The destructuring is exhaustive with no `..`, so a field added
    /// to the record and not answered here fails to compile — the
    /// same alarm the match over [`CheckEvidence`] is, one level in.
    pub fn presence(&self) -> [(&'static str, bool); 12] {
        let Self {
            actual,
            expected,
            other_root,
            other_output,
            reason,
            inner_variant,
            boolean_variant,
            chart_variant,
            metres,
            gap,
            lever,
            eps,
        } = self;
        [
            ("actual", actual.is_some()),
            ("expected", expected.is_some()),
            ("other_root", other_root.is_some()),
            ("other_output", other_output.is_some()),
            ("reason", reason.is_some()),
            ("inner_variant", inner_variant.is_some()),
            ("boolean_variant", boolean_variant.is_some()),
            ("chart_variant", chart_variant.is_some()),
            ("metres", metres.is_some()),
            ("gap", gap.is_some()),
            ("lever", lever.is_some()),
            ("eps", eps.is_some()),
        ]
    }

    /// The attributes this payload carries, publication order — what
    /// a caller reading this arm finds set rather than `None`.
    pub fn present(&self) -> Vec<&'static str> {
        self.presence()
            .into_iter()
            .filter_map(|(name, set)| set.then_some(name))
            .collect()
    }

    /// The all-`None` record every arm starts from.
    pub const NONE: Self = Self {
        actual: None,
        expected: None,
        other_root: None,
        other_output: None,
        reason: None,
        inner_variant: None,
        boolean_variant: None,
        chart_variant: None,
        metres: None,
        gap: None,
        lever: None,
        eps: None,
    };
}

/// Flatten one finding's evidence into its payload record.
///
/// The match is EXHAUSTIVE with no wildcard arm: an arm added to
/// [`CheckEvidence`] fails this build rather than reaching Python
/// with an all-`None` payload nobody chose for it. An arm that names
/// a counterpart answers `other_root`; an arm that does not answers
/// `None` BY NAME.
pub fn check_payload(evidence: &CheckEvidence) -> CheckEvidencePayload<'_> {
    let none = CheckEvidencePayload::NONE;
    match evidence {
        CheckEvidence::Connectedness { actual, expected } => CheckEvidencePayload {
            actual: Some(*actual),
            expected: Some(*expected),
            ..none
        },
        CheckEvidence::StaleExpectation { expected } => CheckEvidencePayload {
            expected: Some(*expected),
            ..none
        },
        CheckEvidence::NotSeparated {
            other_root,
            other_output,
        } => CheckEvidencePayload {
            other_root: Some(*other_root),
            other_output: Some(*other_output),
            ..none
        },
        // The shell door's refusal is another type's vocabulary, so
        // its arms cross as that vocabulary's own word beside the
        // sentence it renders — the tag is what a caller branches on,
        // the prose is what it reads.
        CheckEvidence::Escalated { source } | CheckEvidence::Unsupported { source } => {
            CheckEvidencePayload {
                reason: Some(Cow::Owned(source.to_string())),
                inner_variant: Some(shell_classify_error_tag(source)),
                ..none
            }
        }
        // The boolean refusal crosses the way the shell door's does:
        // the class is the word a consumer MATCHES on, the kernel's
        // own sentence rides beside it, and neither half is a
        // substring hunt through the other.
        // The measurement, whole: the length it is judged as, the two
        // factors it is the product of, and the band it was judged
        // against. Four numbers rather than one because the finding
        // carries four — `metres` alone cannot be re-derived from, and
        // `metres` without `eps` is a number without a claim.
        CheckEvidence::ChartCoherence { finding } => CheckEvidencePayload {
            chart_variant: Some(coherence_condition_tag(finding.condition)),
            metres: Some(finding.metres),
            gap: Some(finding.gap),
            lever: Some(finding.lever),
            eps: Some(finding.eps),
            ..none
        },
        // The DATA put this loop out of reach; `ChecksReport.skipped`
        // is the other thing, and carries no finding at all.
        CheckEvidence::ChartCoherenceUnexamined { unexamined } => CheckEvidencePayload {
            chart_variant: Some(unexaminable_tag(unexamined.why)),
            ..none
        },
        // The lane has no examination. Nothing was measured, so no
        // attribute is set — the TAG is the whole answer, and it is
        // the answer a caller must not read as a clean body.
        CheckEvidence::ChartCoherenceUnavailable => none,
        CheckEvidence::SeparationUnavailable { kind, reason } => CheckEvidencePayload {
            reason: Some(Cow::Borrowed(reason)),
            boolean_variant: Some(boolean_error_tag(*kind)),
            ..none
        },
    }
}
