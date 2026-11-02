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
//! arithmetic: **several arms are unreachable from Python**.
//! `escalated` and `unsupported` need the shell door to escalate on a
//! subject the registry gathered, and most of the at-rest check's
//! could-not-look reasons need a census verdict nothing in the
//! authoring surface arranges. Sited here, `src/tests.rs` can build
//! the arms the façade's types allow and read what each one puts on
//! the wire; sited in `py/`, the projection of those arms would be
//! pinned nowhere at all.
//!
//! # What is not here
//!
//! A NESTED REFUSAL is not flattened into its FIELDS. `Escalated` and
//! `Unsupported` each hold the shell door's own typed refusal, and the
//! at-rest arm the validator's finding or the gather's refusal, shared
//! whole; which shell, and which face, belong to those types' own
//! vocabularies. What crosses here is the word a caller branches on —
//! `inner_variant` for the inner refusal's discriminant — and the
//! prose those types render, as `reason`. A finding the evidence
//! carries whole (an interference, a carried row) crosses on the
//! Python value, as a coincidence's row does.

use std::borrow::Cow;

use pncad::document::{AtRestEvidence, CheckEvidence};

use crate::tags::{
    at_rest_evidence_tag, coherence_condition_tag, coincidence_relation_tag, decision_site_tag,
    product_error_tag, shell_classify_error_tag, unexaminable_tag, validation_error_tag,
};

/// What one [`CheckEvidence`] arm carries, every field present.
///
/// `reason` is a [`Cow`] so an arm whose sentence the kernel already
/// owns can lend it; every arm today renders its own.
// `PartialEq` without `Eq`: the record carries measured lengths, and
// a float has no total equality to derive.
#[derive(Debug, Clone, PartialEq)]
pub struct CheckEvidencePayload<'a> {
    /// Components actually found.
    pub actual: Option<u32>,
    /// The expectation the subject was held to.
    pub expected: Option<u32>,
    /// The underlying refusal's own prose, where the arm holds one.
    pub reason: Option<Cow<'a, str>>,
    /// **The shell door's own refusal**, as the branchable word
    /// ([`crate::tags::shell_classify_error_tag`]) — which of its
    /// ways it refused, on the two arms that carry one; the
    /// validator's finding's word on `not_at_rest`; and on `at_rest`,
    /// the inner validator finding's or gather refusal's word
    /// ([`AtRestPayload::inner_variant`]).
    pub inner_variant: Option<&'static str>,
    /// **Why the at-rest check could not look**, as the branchable word
    /// ([`crate::tags::at_rest_evidence_tag`]), on `at_rest` alone.
    pub at_rest_variant: Option<&'static str>,
    /// **The chart-coherence arm's own inner discriminant** — which of
    /// the three conditions was measured
    /// ([`crate::tags::coherence_condition_tag`]), or why one loop was
    /// out of reach ([`crate::tags::unexaminable_tag`]).
    ///
    /// Two alphabets under one attribute, and they are DISJOINT, so a
    /// caller who did not first branch on the arm gets an answer that
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
    /// What an unproven coincidence decided between its cells
    /// ([`crate::tags::coincidence_relation_tag`]).
    pub relation: Option<&'static str>,
    /// Where that coincidence was decided
    /// ([`crate::tags::decision_site_tag`]).
    pub site: Option<&'static str>,
    /// How many fold steps' rows an unproven coincidence reports with
    /// it, each decided at a vertex it names (the rows themselves cross
    /// on the Python value).
    pub restated: Option<usize>,
}

impl CheckEvidencePayload<'_> {
    /// Which attributes this payload CARRIES, in the order the value
    /// publishes them.
    ///
    /// The destructuring is exhaustive with no `..`, so a field added
    /// to the record and not answered here fails to compile — the
    /// same alarm the match over [`CheckEvidence`] is, one level in.
    pub fn presence(&self) -> [(&'static str, bool); 13] {
        let Self {
            actual,
            expected,
            reason,
            inner_variant,
            at_rest_variant,
            chart_variant,
            metres,
            gap,
            lever,
            eps,
            relation,
            site,
            restated,
        } = self;
        [
            ("actual", actual.is_some()),
            ("expected", expected.is_some()),
            ("reason", reason.is_some()),
            ("inner_variant", inner_variant.is_some()),
            ("at_rest_variant", at_rest_variant.is_some()),
            ("chart_variant", chart_variant.is_some()),
            ("metres", metres.is_some()),
            ("gap", gap.is_some()),
            ("lever", lever.is_some()),
            ("eps", eps.is_some()),
            ("relation", relation.is_some()),
            ("site", site.is_some()),
            ("restated", restated.is_some()),
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
        reason: None,
        inner_variant: None,
        at_rest_variant: None,
        chart_variant: None,
        metres: None,
        gap: None,
        lever: None,
        eps: None,
        relation: None,
        site: None,
        restated: None,
    };
}

/// Flatten one finding's evidence into its payload record.
///
/// The match is EXHAUSTIVE with no wildcard arm: an arm added to
/// [`CheckEvidence`] fails this build rather than reaching Python
/// with an all-`None` payload nobody chose for it.
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
        // The validator's first finding crosses as its own word and
        // sentence, the way the shell door's refusal does.
        CheckEvidence::NotAtRest { errors } => CheckEvidencePayload {
            reason: errors.first().map(|e| Cow::Owned(e.to_string())),
            inner_variant: errors.first().map(validation_error_tag),
            ..none
        },
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
        // The tag is the whole answer: the subject is what was not used.
        CheckEvidence::Unused { .. } => none,
        // The row's two words, the residual's sentence as the reason
        // (what separates the cells' constructions), and how many fold
        // rows it reports with it. The cells and those rows themselves
        // cross on the Python value.
        CheckEvidence::UnprovenCoincidence {
            row,
            residual,
            restated,
            ..
        } => CheckEvidencePayload {
            reason: Some(Cow::Owned(residual.to_string())),
            relation: Some(coincidence_relation_tag(row.relation)),
            site: Some(decision_site_tag(row.site)),
            restated: Some(restated.len()),
            ..none
        },
        // The finding crosses whole on the Python value; nothing of it
        // is flattened here.
        CheckEvidence::Interference { .. }
        | CheckEvidence::CarriedInterference { .. }
        | CheckEvidence::CarriedCoincidence { .. } => none,
        CheckEvidence::AtRest { evidence } => {
            let AtRestPayload {
                variant,
                reason,
                inner_variant,
            } = at_rest_payload(evidence);
            CheckEvidencePayload {
                reason: Some(Cow::Owned(reason)),
                inner_variant,
                at_rest_variant: Some(variant),
                ..none
            }
        }
    }
}

/// What one [`AtRestEvidence`] carries, flattened: its own word, its
/// sentence, and the inner refusal's word where it holds one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtRestPayload {
    /// Why the check could not look ([`at_rest_evidence_tag`]).
    pub variant: &'static str,
    /// The evidence's own sentence, its nodes said by their tags.
    pub reason: String,
    /// The validator finding's word on `no_lane`, `malformed` and
    /// `inside_arrival`, and the gather refusal's on `not_gathered`;
    /// `None` on `not_run` and `carried`, which hold no refusal of
    /// their own (a carried row's own evidence crosses whole).
    pub inner_variant: Option<&'static str>,
}

/// Flatten one at-rest evidence. Exhaustive, no wildcard, for the
/// reason [`check_payload`] is.
pub fn at_rest_payload(evidence: &AtRestEvidence) -> AtRestPayload {
    let inner_variant = match evidence {
        AtRestEvidence::NoLane(error)
        | AtRestEvidence::Malformed(error)
        | AtRestEvidence::InsideArrival(error) => Some(validation_error_tag(error)),
        AtRestEvidence::NotGathered(refusal) => Some(product_error_tag(refusal.error())),
        AtRestEvidence::NotRun | AtRestEvidence::Carried(_) => None,
    };
    AtRestPayload {
        variant: at_rest_evidence_tag(evidence),
        reason: evidence.to_string(),
        inner_variant,
    }
}
