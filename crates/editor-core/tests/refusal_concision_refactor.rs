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
    CapEnd, DocumentId, EditError, EntityKind, InlineError, PersistError, RecipeNodeId,
    ResolveFailure, ResolveFault, RoleSeg, SplitError, SpokenName, SpokenNode, StableName, StepId,
    Uncarried, VarName,
};
use test_utils::refusal::Admission;

fn n(id: u64) -> RecipeNodeId {
    RecipeNodeId::new(0, test_utils::refusal::tagged(id))
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

fn param() -> VarName {
    VarName::from_static("width")
}

fn var() -> editor_core::SpokenVar {
    editor_core::SpokenVar::new(
        editor_core::VarId::new(0, test_utils::refusal::tagged(8)),
        Some(param()),
    )
}

test_utils::f6_variants! {
    const SPLIT: SplitError = [
        EmptyCut, UnknownCutNode, CutsTheWorld, PartIdCollides, RemainderReadUncarried,
        SeveredEdge, NoMaterial, UncutVarReference, SpaceStraddlesCut, DefinitionStraddlesCut,
        UnresolvedVarCrossesCut, PartNameReachesRemainder, NameStraddlesCut, NameOnDroppedStep,
        BodyNameCrossesCut, NameOutsidePartWorld, Pin, PartEdit, RemainderEdit,
    ];
}

test_utils::f6_variants! {
    const INLINE: InlineError = [
        UnknownNode, NotAnInstance, UnplaceablePart, Unresolved,
        EpsilonSeam, PartCarriesMetadata, VarNameConflict, UnreadPose,
        UnresolvedVarCrossesCut, InstanceBodyNameReferenced, NameOnDroppedStep,
        StrandedPartName,
        StrandedPartRead, Edit,
    ];
}

/// Every [`Uncarried`] reason, on a representative payload.
fn uncarried() -> Vec<Uncarried> {
    vec![
        Uncarried::Bodies { count: 2 },
        Uncarried::Posed {
            placement: s(4, "Place"),
        },
        Uncarried::OtherSpace {
            placement: s(4, "InstantiatePart"),
        },
        Uncarried::NoSuchPiece {
            of: vec![editor_core::VarId::new(0, test_utils::refusal::tagged(8))],
        },
    ]
}

/// Every `SplitError` arm, on a representative payload.
fn split_refusals() -> Vec<SplitError> {
    let mut out = vec![
        SplitError::EmptyCut,
        SplitError::UnknownCutNode {
            id: SpokenNode::absent(n(9)),
        },
        SplitError::CutsTheWorld,
        SplitError::PartIdCollides {
            id: DocumentId::derive("refusal-concision-refactor"),
        },
        SplitError::SeveredEdge {
            consumer: s(5, "Extrude"),
            slot: Some(editor_core::OperandSlot::Profile),
            input: s(3, "Profile"),
            consumer_is_cut: true,
        },
    ];
    out.extend(
        uncarried()
            .into_iter()
            .map(|why| SplitError::RemainderReadUncarried {
                reader: s(6, "Union"),
                why,
            }),
    );
    out.extend([
        SplitError::NoMaterial {
            node: s(1, "Profile"),
        },
        SplitError::UncutVarReference {
            var: Box::new(var()),
            cut_node: s(4, "Extrude"),
            kept_node: s(6, "Extrude"),
        },
        SplitError::SpaceStraddlesCut {
            space: Box::new(s(3, "Space")),
            cut_node: s(4, "Place"),
            kept_node: s(6, "Place"),
        },
        SplitError::DefinitionStraddlesCut {
            var: Box::new(var()),
            moving: editor_core::SpokenVar::new(
                editor_core::VarId::new(0, test_utils::refusal::tagged(7)),
                Some(VarName::from_static("depth")),
            ),
            staying: editor_core::SpokenVar::new(
                editor_core::VarId::new(0, test_utils::refusal::tagged(6)),
                None,
            ),
            staying_held: true,
        },
        SplitError::DefinitionStraddlesCut {
            var: Box::new(var()),
            moving: editor_core::SpokenVar::new(
                editor_core::VarId::new(0, test_utils::refusal::tagged(7)),
                Some(VarName::from_static("depth")),
            ),
            staying: editor_core::SpokenVar::new(
                editor_core::VarId::new(0, test_utils::refusal::tagged(6)),
                None,
            ),
            staying_held: false,
        },
        SplitError::UnresolvedVarCrossesCut {
            var: editor_core::SpokenVar::new(
                editor_core::VarId::new(0, test_utils::refusal::tagged(9)),
                None,
            ),
            node: s(4, "Extrude"),
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
            step: StepId::new(0, 4),
        },
        SplitError::BodyNameCrossesCut { name: name() },
        SplitError::NameOutsidePartWorld { name: name() },
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
    ]);
    out
}

/// Every `InlineError` arm, on a representative payload.
fn inline_refusals() -> Vec<InlineError> {
    let mut out = vec![
        InlineError::UnknownNode {
            id: SpokenNode::absent(n(9)),
        },
        InlineError::NotAnInstance {
            node: s(5, "Extrude"),
        },
    ];
    out.extend([
        InlineError::UnplaceablePart {
            instance: s(4, "InstantiatePart"),
        },
        InlineError::Unresolved {
            failure: ResolveFailure::new(
                ResolveFault::EpsilonSeam,
                "document ε 1e-6 conflicts with the process ε 1e-9 (one process, one ε)",
            ),
        },
        InlineError::EpsilonSeam {
            host_eps: 1e-9,
            part_eps: 1e-6,
        },
        InlineError::PartCarriesMetadata {
            key: "author".to_owned(),
        },
        InlineError::VarNameConflict { name: param() },
        InlineError::UnreadPose {
            var: editor_core::SpokenVar::new(
                editor_core::VarId::new(0, test_utils::refusal::tagged(9)),
                Some(param()),
            ),
        },
        InlineError::UnresolvedVarCrossesCut {
            var: editor_core::SpokenVar::new(
                editor_core::VarId::new(0, test_utils::refusal::tagged(9)),
                None,
            ),
            node: s(4, "Extrude"),
        },
        InlineError::InstanceBodyNameReferenced { name: name() },
        InlineError::NameOnDroppedStep {
            name: name(),
            step: StepId::new(0, 4),
        },
        InlineError::StrandedPartName {
            name: name(),
            missing: s(6, "Extrude"),
        },
        InlineError::StrandedPartRead {
            name: name(),
            read: editor_core::SpokenVar::new(editor_core::VarId::new(0, 6), None),
        },
        InlineError::Edit {
            error: Box::new(EditError::UnknownNode {
                id: SpokenNode::absent(n(9)),
            }),
        },
    ]);
    out
}

/// The stage words a refactor refusal opens with, each on the row
/// namespace that writes it.
const LABELS: &[(&str, &str)] = &[("Split/", "split"), ("Inline/", "inline")];

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
        let allowed: Vec<&str> = LABELS
            .iter()
            .filter(|(ns, _)| name.starts_with(ns))
            .map(|(_, l)| *l)
            .collect();
        problems.extend(test_utils::refusal::problems_admitting(
            name, text, &allowed, false, ADMISSIONS,
        ));
        names.push(name.clone());
    }
    problems.extend(test_utils::refusal::unclaimed_admissions(
        ADMISSIONS,
        names.iter().map(String::as_str),
    ));
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
