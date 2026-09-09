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
//! A NESTED REFUSAL is not flattened. `Escalated` and `Unsupported`
//! each hold the shell door's own typed refusal and
//! `SeparationUnavailable` the boolean class beside its sentence;
//! those belong to those types' own vocabularies, and what crosses
//! here is the prose they render, as `reason`.

use std::borrow::Cow;

use pncad::document::{CheckEvidence, RecipeNodeId};

/// What one [`CheckEvidence`] arm carries, every field present.
///
/// Borrowed from the evidence rather than cloned out of it: the
/// separation arm's sentence is a `String` the kernel already owns,
/// and the shell arms' is rendered here, so the two spellings of
/// `reason` are the two halves of a [`Cow`].
#[derive(Debug, Clone, PartialEq, Eq)]
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
}

impl CheckEvidencePayload<'_> {
    /// Which attributes this payload CARRIES, in the order the value
    /// publishes them.
    ///
    /// The destructuring is exhaustive with no `..`, so a field added
    /// to the record and not answered here fails to compile — the
    /// same alarm the match over [`CheckEvidence`] is, one level in.
    pub fn presence(&self) -> [(&'static str, bool); 5] {
        let Self {
            actual,
            expected,
            other_root,
            other_output,
            reason,
        } = self;
        [
            ("actual", actual.is_some()),
            ("expected", expected.is_some()),
            ("other_root", other_root.is_some()),
            ("other_output", other_output.is_some()),
            ("reason", reason.is_some()),
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
        // The shell door's refusal is another type's vocabulary; what
        // crosses is the sentence it renders.
        CheckEvidence::Escalated { source } | CheckEvidence::Unsupported { source } => {
            CheckEvidencePayload {
                reason: Some(Cow::Owned(source.to_string())),
                ..none
            }
        }
        // `kind` is the boolean class a consumer MATCHES on and has
        // no attribute of its own here; `reason` is the kernel's own
        // sentence beside it, which is what this record carries.
        CheckEvidence::SeparationUnavailable { kind: _, reason } => CheckEvidencePayload {
            reason: Some(Cow::Borrowed(reason)),
            ..none
        },
    }
}
