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
    CapEnd, ConstraintId, DocumentId, EditError, EntityKind, InlineError, MateSide, PersistError,
    RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, SplitError, SpokenName, SpokenNode,
    StableName, StepId, Uncarried, VarName,
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

/// A pair boolean's ranked `EdgeCrossing` vertex over two lateral
/// edges of one extrude, as a refusal naming a ranked run speaks it.
fn ranked() -> SpokenName {
    let edge = |step| {
        editor_core::NameRef::new(StableName {
            kind: EntityKind::Edge,
            node: n(3),
            path: vec![RoleSeg::LateralEdge(editor_core::ProfileVertexRef::Piece {
                step: StepId::new(0, test_utils::refusal::tagged(step)),
                role: editor_core::PieceRole::Leg,
            })],
        })
    };
    editor_core::test_support::spoken_name(
        StableName {
            kind: EntityKind::Vertex,
            node: n(5),
            path: vec![
                RoleSeg::EdgeCrossing(
                    [1, 2]
                        .into_iter()
                        .map(|step| editor_core::RunLine {
                            line: edge(step),
                            sense: Some(editor_core::Sense::Outside),
                        })
                        .collect(),
                ),
                RoleSeg::Fragment(editor_core::Qualifier::OrderAlong { rank: 0, of: 2 }),
            ],
        },
        s(5, "Subtract"),
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
        SeveredEdge, NoMaterial, MateFrameCrosses, UncutVarReference, DefinitionStraddlesCut,
        UnresolvedVarCrossesCut, PartNameReachesRemainder, NameStraddlesCut, NameOnDroppedStep, RankedRunReordered,
        BodyNameCrossesCut, NameOutsidePartWorld, Pin, PartEdit, RemainderEdit,
    ];
}

test_utils::f6_variants! {
    const INLINE: InlineError = [
        UnknownNode, NotAnInstance, UnplaceablePart, InstanceReadUncarried, Unresolved,
        EpsilonSeam, PartCarriesMetadata, VarNameConflict, InstanceOutputUncarried,
        UnresolvedVarCrossesCut, ConstructionReadsCopy, CopyPinnedToConstruction, MateFrameCrosses,
        SideLeavesAnInstance,
        InstanceBodyNameReferenced, ForeignInstanceName, NameOnDroppedStep, RankedRunReordered,
        StrandedPartName,
        StrandedPartRead, Edit,
    ];
}

/// A constraint id, as a refusal names one in its placement.
fn constraint() -> ConstraintId {
    ConstraintId(editor_core::MintId::new(1, test_utils::refusal::tagged(2)))
}

/// Every [`Uncarried`] reason, on a representative payload.
fn uncarried() -> Vec<Uncarried> {
    vec![
        Uncarried::Bodies { count: 2 },
        Uncarried::HeirNamed { held: param() },
        Uncarried::Posed {
            placement: s(4, "Place"),
        },
        Uncarried::Loose {
            placement: s(4, "InstantiatePart"),
        },
        Uncarried::NotAnInstance {
            placement: s(4, "Place"),
        },
        Uncarried::NoSuchCopy {
            key: Box::new(editor_core::CopyKey::part(
                n(4),
                editor_core::CopyKey::Read(editor_core::VarId::new(
                    0,
                    test_utils::refusal::tagged(8),
                )),
            )),
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
        SplitError::MateFrameCrosses {
            placement: s(7, "InstantiatePart"),
            constraint: constraint(),
            position: 2,
            side: MateSide::B,
        },
        SplitError::UncutVarReference {
            var: Box::new(var()),
            cut_node: s(4, "Extrude"),
            kept_node: s(6, "Extrude"),
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
        SplitError::RankedRunReordered { name: ranked() },
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
    out.extend(uncarried().into_iter().flat_map(|why| {
        [
            InlineError::UnplaceablePart {
                instance: s(4, "InstantiatePart"),
                why: why.clone(),
            },
            InlineError::InstanceReadUncarried {
                reader: s(5, "Union"),
                why,
            },
        ]
    }));
    out.extend([
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
        InlineError::InstanceOutputUncarried {
            name: param(),
            why: editor_core::Uncarried::Bodies { count: 2 },
        },
        InlineError::UnresolvedVarCrossesCut {
            var: editor_core::SpokenVar::new(
                editor_core::VarId::new(0, test_utils::refusal::tagged(9)),
                None,
            ),
            node: s(4, "Extrude"),
        },
        InlineError::ConstructionReadsCopy {
            node: s(3, "Union"),
        },
        InlineError::CopyPinnedToConstruction {
            placement: s(8, "Place"),
            construction: s(3, "Union"),
        },
        InlineError::MateFrameCrosses {
            placement: s(7, "InstantiatePart"),
            constraint: constraint(),
            position: 2,
            side: MateSide::A,
        },
        InlineError::SideLeavesAnInstance {
            placement: s(7, "InstantiatePart"),
            constraint: constraint(),
            side: MateSide::B,
        },
        InlineError::InstanceBodyNameReferenced { name: name() },
        InlineError::ForeignInstanceName { name: name() },
        InlineError::NameOnDroppedStep {
            name: name(),
            step: StepId::new(0, 4),
        },
        InlineError::RankedRunReordered { name: ranked() },
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
