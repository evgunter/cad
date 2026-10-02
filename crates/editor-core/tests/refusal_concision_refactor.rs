//! **A split or inline refusal states one recourse.**
//!
//! Every `SplitError` and `InlineError` arm, rendered on a
//! representative payload and held to [`test_utils::refusal::problems`]
//! as `EditError`'s arms are (`viewer/tests/refusal_concision_edits.rs`):
//! the budget, no `Debug` struct, no arena key, one recourse. The
//! rosters are wildcard-free, so an arm added to either enum fails to
//! compile here until it is rendered.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    CapEnd, DocumentId, EditError, EntityKind, InlineError, MateSide, ParamName, PersistError,
    RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, SplitError, SpokenName, SpokenNode,
    StableName, StepId, Unplaced,
};
use test_utils::refusal::Admission;

fn n(id: u64) -> RecipeNodeId {
    RecipeNodeId(test_utils::refusal::tagged(id))
}

/// Node `id` as a refusal raised over a document holding it as a
/// `kind` speaks it.
fn s(id: u64, kind: &'static str) -> SpokenNode {
    editor_core::test_support::spoken(n(id), Some(kind))
}

fn name() -> SpokenName {
    editor_core::test_support::spoken_name(
        StableName {
            kind: EntityKind::Face,
            node: n(3),
            path: vec![RoleSeg::Cap(CapEnd::End)],
        },
        s(3, "Extrude"),
    )
}

fn param() -> ParamName {
    ParamName::from_static("width")
}

test_utils::f6_variants! {
    const SPLIT: SplitError = [
        EmptyCut, UnknownCutNode, PartIdCollides, SeveredEdge, OperandSeveredFromMate,
        TornGroup, CutHoldsGauge, TwoAnchors, PlacingMateLeft, DeadGaugeReference,
        UnplacedAlone, WouldStartPlacing, MateFrameCrosses, MateFaceFrameCrosses,
        HoistedMemberOffset, UncutParamReference, PartNameReachesRemainder,
        NameStraddlesCut, NameOnDroppedStep, BodyNameCrossesCut, Pin, PartEdit,
        RemainderEdit,
    ];
}

test_utils::f6_variants! {
    const INLINE: InlineError = [
        UnknownNode, NotAnInstance, InstanceConsumed, Unresolved, EpsilonSeam,
        PartCarriesMetadata, ParamConflict, UnplaceableFrame, MatePlaced, Unplaced,
        NeedsAGauge, PartDeadGauge, MateFrameCrosses, MateFaceFrameCrosses, MatePairSplits,
        InstanceBodyNameReferenced, ForeignInstanceName, NameOnDroppedStep,
        StrandedPartName, Edit,
    ];
}

/// Every `SplitError` arm, on a representative payload.
fn split_refusals() -> Vec<SplitError> {
    vec![
        SplitError::EmptyCut,
        SplitError::UnknownCutNode {
            id: SpokenNode::absent(n(9)),
        },
        SplitError::PartIdCollides {
            id: DocumentId::derive("refusal-concision-refactor"),
        },
        SplitError::SeveredEdge {
            consumer: s(5, "Extrude"),
            input: s(3, "Profile"),
            consumer_is_cut: true,
        },
        SplitError::OperandSeveredFromMate {
            mate: s(7, "Mate"),
            side: MateSide::A,
            operand: s(3, "InstantiatePart"),
            mate_is_cut: false,
        },
        SplitError::TornGroup {
            root: s(2, "InstantiatePart"),
            instance: s(4, "InstantiatePart"),
            root_is_cut: true,
        },
        SplitError::CutHoldsGauge {
            gauge: s(1, "Gauge"),
        },
        SplitError::TwoAnchors {
            node: s(4, "Extrude"),
            first: Some(s(1, "Gauge")),
            second: None,
        },
        SplitError::PlacingMateLeft { mate: s(7, "Mate") },
        SplitError::DeadGaugeReference {
            instance: s(4, "InstantiatePart"),
            gauge: s(1, "Gauge"),
        },
        SplitError::UnplacedAlone {
            group: s(2, "InstantiatePart"),
        },
        SplitError::WouldStartPlacing { mate: s(7, "Mate") },
        SplitError::MateFrameCrosses {
            mate: s(7, "Mate"),
            side: MateSide::B,
        },
        SplitError::MateFaceFrameCrosses {
            mate: s(7, "Mate"),
            side: MateSide::B,
        },
        SplitError::HoistedMemberOffset {
            instance: s(4, "InstantiatePart"),
        },
        SplitError::UncutParamReference {
            param: param(),
            cut_node: s(4, "Extrude"),
            kept_node: s(6, "Extrude"),
        },
        SplitError::PartNameReachesRemainder {
            node: s(5, "Extrude"),
            name: name(),
            missing: s(6, "Extrude"),
        },
        SplitError::NameStraddlesCut {
            name: name(),
            missing: Some(s(6, "Extrude")),
        },
        SplitError::NameOnDroppedStep {
            name: name(),
            step: StepId(4),
        },
        SplitError::BodyNameCrossesCut { name: name() },
        SplitError::Pin {
            error: Box::new(PersistError::Serialize {
                message: "the writer refused".to_owned(),
            }),
        },
        SplitError::PartEdit {
            error: Box::new(EditError::UnknownNode {
                id: SpokenNode::absent(n(9)),
            }),
        },
        SplitError::RemainderEdit {
            error: Box::new(EditError::UnknownNode {
                id: SpokenNode::absent(n(9)),
            }),
        },
    ]
}

/// Every `InlineError` arm, on a representative payload.
fn inline_refusals() -> Vec<InlineError> {
    vec![
        InlineError::UnknownNode {
            id: SpokenNode::absent(n(9)),
        },
        InlineError::NotAnInstance {
            node: s(5, "Extrude"),
        },
        InlineError::InstanceConsumed {
            node: s(4, "InstantiatePart"),
            by: s(5, "Union"),
        },
        InlineError::Unresolved {
            failure: ResolveFailure::new(ResolveFault::Unresolved, "no such document"),
        },
        InlineError::EpsilonSeam {
            host_eps: 1e-9,
            part_eps: 1e-6,
        },
        InlineError::PartCarriesMetadata {
            key: "author".to_owned(),
        },
        InlineError::ParamConflict { param: param() },
        InlineError::UnplaceableFrame {
            root: s(3, "Extrude"),
        },
        InlineError::MatePlaced {
            instance: s(4, "InstantiatePart"),
            root: s(2, "InstantiatePart"),
            mates: vec![s(7, "Mate")],
        },
        InlineError::Unplaced {
            instance: s(4, "InstantiatePart"),
            cause: Unplaced::NoOffset,
        },
        InlineError::NeedsAGauge {
            instance: s(4, "InstantiatePart"),
        },
        InlineError::PartDeadGauge {
            node: s(3, "Extrude"),
        },
        InlineError::MateFrameCrosses {
            mate: s(7, "Mate"),
            side: MateSide::A,
        },
        InlineError::MateFaceFrameCrosses {
            mate: s(7, "Mate"),
            side: MateSide::A,
        },
        InlineError::MatePairSplits {
            first: s(7, "Mate"),
            second: s(8, "Mate"),
        },
        InlineError::InstanceBodyNameReferenced { name: name() },
        InlineError::ForeignInstanceName { name: name() },
        InlineError::NameOnDroppedStep {
            name: name(),
            step: StepId(4),
        },
        InlineError::StrandedPartName {
            name: name(),
            missing: s(6, "Extrude"),
        },
        InlineError::Edit {
            error: Box::new(EditError::UnknownNode {
                id: SpokenNode::absent(n(9)),
            }),
        },
    ]
}

/// The stage words a refactor refusal opens with, each on the row
/// namespace that writes it.
const LABELS: &[(&str, &str)] = &[
    ("Split/", "split"),
    ("Inline/", "inline"),
    // The pin's own refusal, forwarded under its stage word.
    ("Split/Pin", "persist"),
];

/// The rows that state no recourse, by exact row id, filed with their
/// owner.
const FILED_NO_RECOURSE: &[&str] = &[
    // work/place/split-and-inline-refusals-short-of-the-shape-guard.md
    "Split/EmptyCut",
    "Split/UnknownCutNode",
    "Split/PartIdCollides",
    "Split/SeveredEdge",
    "Split/OperandSeveredFromMate",
    "Split/UncutParamReference",
    "Split/PartNameReachesRemainder",
    "Split/NameStraddlesCut",
    "Split/NameOnDroppedStep",
    "Split/BodyNameCrossesCut",
    "Split/Pin",
    "Inline/UnknownNode",
    "Inline/NotAnInstance",
    "Inline/InstanceConsumed",
    "Inline/Unresolved",
    "Inline/EpsilonSeam",
    "Inline/PartCarriesMetadata",
    "Inline/ParamConflict",
    "Inline/InstanceBodyNameReferenced",
    "Inline/ForeignInstanceName",
    "Inline/NameOnDroppedStep",
    "Inline/StrandedPartName",
];

/// The rows that name a document by its hex id, filed with their owner.
const ADMISSIONS: &[Admission<'static>] = &[Admission {
    row: "Split/PartIdCollides",
    span: "b1f205f718a5f31e2d727b6a7572a3e7",
    filed: "work/doctail/part-refusals-name-documents-by-hex-id.md",
}];

/// **Every split and inline refusal states exactly one recourse**, and
/// the rosters reach every arm.
#[test]
fn every_split_and_inline_refusal_states_one_recourse() {
    let mut problems = Vec::new();
    let mut names = Vec::new();
    let mut used = std::collections::BTreeSet::new();
    let split: Vec<(String, String)> = split_refusals()
        .iter()
        .map(|e| {
            (
                format!("Split/{}", test_utils::f6::variant_identifier(e)),
                e.to_string(),
            )
        })
        .collect();
    let inline: Vec<(String, String)> = inline_refusals()
        .iter()
        .map(|e| {
            (
                format!("Inline/{}", test_utils::f6::variant_identifier(e)),
                e.to_string(),
            )
        })
        .collect();
    for (census, rows, prefix) in [
        (SPLIT.identifiers(), &split, "Split/"),
        (INLINE.identifiers(), &inline, "Inline/"),
    ] {
        let read: Vec<&str> = rows
            .iter()
            .map(|(name, _)| name.trim_start_matches(prefix))
            .collect();
        if let Some(report) = test_utils::census::set_difference(
            census,
            &read,
            "the roster and its samples disagree",
            "sampled here and absent from the roster",
            "in the roster with no sample here — add one",
        ) {
            problems.push(report);
        }
    }
    for (name, text) in split.iter().chain(&inline) {
        eprintln!("MEASURE {} {name}: {text}", text.split_whitespace().count());
        let no_recourse = format!("{name} states no recourse");
        let allowed: Vec<&str> = LABELS
            .iter()
            .filter(|(ns, _)| name.starts_with(ns))
            .map(|(_, l)| *l)
            .collect();
        for problem in
            test_utils::refusal::problems_admitting(name, text, &allowed, false, ADMISSIONS)
        {
            if FILED_NO_RECOURSE.contains(&name.as_str()) && problem.starts_with(&no_recourse) {
                used.insert(name.clone());
            } else {
                problems.push(problem);
            }
        }
        names.push(name.clone());
    }
    for filed in FILED_NO_RECOURSE {
        if !used.contains(*filed) {
            problems.push(format!(
                "the admission {filed} admits nothing a row renders"
            ));
        }
    }
    problems.extend(test_utils::refusal::unclaimed_admissions(
        ADMISSIONS,
        names.iter().map(String::as_str),
    ));
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
