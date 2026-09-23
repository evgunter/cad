//! **An edit refusal fits the status line it is drawn on.**
//!
//! A refused edit reaches the person holding the mouse as
//! [`Refusal::Edit`]'s sentence on the status line — the viewer's
//! "the edit was refused:" wrapper, then `EditError`'s own `Display`.
//! These rows render every `EditError` arm that way on a representative
//! payload and hold each to the 75-word budget
//! `editor-core/tests/refusal_concision.rs` states (the one statement
//! of the number; this is the same number for the edit chain).
//!
//! The forwarding arms are rendered over what they forward: every
//! `MateFault` arm inside `MaintenanceRefused` and `MateRefused`, and
//! the longest path refusals inside `ProfileProgramRefused` (the
//! feature tree's rows in `editor-core/tests/refusal_concision_chains.rs`
//! render every `PathError` arm).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::program::ProgramRefusal;
use editor_core::{
    AttrKind, ContentPin, Dimension, DimensionError, DistributionFault, DistributionField,
    DocumentId, EditError, EntityKind, EvalError, ExprPath, MateFault, MeasureNodeFault,
    MetaVersionError, NodeErrorKind, ParamName, RecipeNodeId, RootFault, SlotId, StableName,
};
use viewer::session::Refusal;

const BUDGET: usize = 75;

fn shown(e: EditError) -> String {
    Refusal::Edit(Box::new(e)).to_string()
}

fn name() -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId(3),
        path: Vec::new(),
    }
}

fn param() -> ParamName {
    ParamName::new("width")
}

fn n(id: u64) -> RecipeNodeId {
    RecipeNodeId(id)
}

/// Every `EditError` arm, on a representative payload.
fn edit_refusals() -> Vec<(&'static str, EditError)> {
    use editor_core::edit::CarryForwardDoor;
    use editor_core::{DocParamField, DocParamValue};
    vec![
        ("UnknownNode", EditError::UnknownNode { id: n(9) }),
        (
            "ProfileProgramRefused(Geometry)",
            EditError::ProfileProgramRefused {
                node: n(4),
                refusal: Box::new(ProgramRefusal::Geometry {
                    loop_: 0,
                    step: 2,
                    kind: profile::PathErrorKind::JunctionTangent,
                    rendered: profile::PathError::<f64>::JunctionTangent {
                        margin: 1e-12,
                        arm: 0.5,
                    }
                    .to_string(),
                }),
            },
        ),
        (
            "ProfileProgramRefused(Resolve)",
            EditError::ProfileProgramRefused {
                node: n(4),
                refusal: Box::new(ProgramRefusal::Resolve {
                    slot: SlotId::Distance,
                    source: EvalError::UnknownParam(param()),
                }),
            },
        ),
        (
            "ProfileProgramRefused(Transition)",
            EditError::ProfileProgramRefused {
                node: n(4),
                refusal: Box::new(ProgramRefusal::Transition {
                    loop_: 0,
                    step: 2,
                    state: profile::TipState::Closed,
                    verb: None,
                }),
            },
        ),
        (
            "ProfileProgramRefused(Validate)",
            EditError::ProfileProgramRefused {
                node: n(4),
                refusal: Box::new(ProgramRefusal::Validate(
                    profile::ProfileError::TangencyContradicted {
                        first: profile::SegmentRef {
                            loop_index: 0,
                            segment_index: 1,
                        },
                        second: profile::SegmentRef {
                            loop_index: 0,
                            segment_index: 2,
                        },
                        joint: 2,
                    },
                )),
            },
        ),
        (
            "UnresolvedInput",
            EditError::UnresolvedInput { input: n(9) },
        ),
        ("WouldCycle", EditError::WouldCycle { at: n(4) }),
        (
            "DuplicateInput",
            EditError::DuplicateInput {
                node: n(5),
                input: n(3),
            },
        ),
        (
            "RepeatedDesignation",
            EditError::RepeatedDesignation {
                node: n(5),
                first: 0,
                again: 2,
            },
        ),
        (
            "SelectionNotCanonical",
            EditError::SelectionNotCanonical { node: n(5), at: 1 },
        ),
        (
            "SetMembersOnNonList",
            EditError::SetMembersOnNonList { node: n(5) },
        ),
        (
            "TooFewMembers",
            EditError::TooFewMembers {
                node: n(5),
                found: 1,
            },
        ),
        (
            "DeleteWouldDangle",
            EditError::DeleteWouldDangle {
                id: n(3),
                referenced_by: n(5),
            },
        ),
        (
            "UnknownSlot",
            EditError::UnknownSlot {
                id: n(5),
                slot: SlotId::Radius,
            },
        ),
        (
            "SlotDimensionMismatch",
            EditError::SlotDimensionMismatch {
                slot: SlotId::Distance,
                expected: Dimension::Length,
                found: Dimension::Angle,
            },
        ),
        (
            "MaintenanceRefused",
            EditError::MaintenanceRefused {
                gauge: n(6),
                fault: Some(Box::new(MateFault::ClassNotAdmitted { mate: n(9) })),
            },
        ),
        (
            "MaintenanceUnrecorded",
            EditError::MaintenanceUnrecorded { gauge: n(6) },
        ),
        (
            "StructuralSlotNeedsStructuralEdit",
            EditError::StructuralSlotNeedsStructuralEdit {
                slot: SlotId::Count,
            },
        ),
        (
            "NotStructuralSlot",
            EditError::NotStructuralSlot {
                slot: SlotId::Distance,
            },
        ),
        (
            "SlotUnknownDocParam",
            EditError::SlotUnknownDocParam {
                name: param(),
                node: n(5),
                slot: SlotId::Distance,
            },
        ),
        (
            "SlotDocParamDimension",
            EditError::SlotDocParamDimension {
                name: param(),
                node: n(5),
                slot: SlotId::Distance,
                declared: Dimension::Angle,
                referenced: Dimension::Length,
            },
        ),
        (
            "PayloadUnknownDocParam",
            EditError::PayloadUnknownDocParam {
                name: param(),
                node: n(5),
            },
        ),
        (
            "PayloadDocParamDimension",
            EditError::PayloadDocParamDimension {
                name: param(),
                node: n(5),
                declared: Dimension::Angle,
                referenced: Dimension::Length,
            },
        ),
        (
            "MeasureMalformed",
            EditError::MeasureMalformed {
                node: n(5),
                fault: MeasureNodeFault::RefIndexOutOfRange {
                    verb: "min_clearance",
                    index: 2,
                    refs: 2,
                },
            },
        ),
        (
            "AssertionTarget",
            EditError::AssertionTarget {
                node: n(6),
                measure: n(5),
            },
        ),
        (
            "DeclareInputNotDeclare",
            EditError::DeclareInputNotDeclare {
                node: n(6),
                input: n(5),
            },
        ),
        (
            "AssertionDimension",
            EditError::AssertionDimension {
                node: n(6),
                measure: n(5),
                measured: Dimension::Length,
                bound: Dimension::Angle,
            },
        ),
        (
            "ContinuousParamCannotBeCount",
            EditError::ContinuousParamCannotBeCount { name: param() },
        ),
        (
            "DocParamNotDeclared",
            EditError::DocParamNotDeclared {
                name: param(),
                door: CarryForwardDoor::Notation,
            },
        ),
        (
            "DocParamCountHasNoUnit",
            EditError::DocParamCountHasNoUnit { name: param() },
        ),
        (
            "DocParamCountHasNoDistribution",
            EditError::DocParamCountHasNoDistribution { name: param() },
        ),
        (
            "DocParamUnitMismatch",
            EditError::DocParamUnitMismatch {
                name: param(),
                unit: Dimension::Angle,
                declared: Dimension::Length,
            },
        ),
        (
            "DocParamValueKindMismatch",
            EditError::DocParamValueKindMismatch {
                name: param(),
                declared: Dimension::Count,
                offered: DocParamValue::Continuous(2.5),
            },
        ),
        (
            "PathOffTree",
            EditError::PathOffTree {
                path: ExprPath {
                    node: n(5),
                    slot: SlotId::Distance,
                    path: vec![0, 3],
                },
            },
        ),
        (
            "Dimension",
            EditError::Dimension(DimensionError::Mismatch {
                op: "+",
                left: Dimension::Length,
                right: Dimension::Angle,
            }),
        ),
        (
            "DeclareNamesMissingNode",
            EditError::DeclareNamesMissingNode { name: name() },
        ),
        (
            "ReadSiteMissingNode",
            EditError::ReadSiteMissingNode { at: n(9) },
        ),
        (
            "NonFiniteDocParam",
            EditError::NonFiniteDocParam {
                name: param(),
                field: DocParamField::Offset(DistributionField::Sigma),
            },
        ),
        (
            "InvalidDistribution",
            EditError::InvalidDistribution {
                name: param(),
                fault: DistributionFault::NominalOutsideSupport { lo: 1.0, hi: 0.5 },
            },
        ),
        (
            "RebindTargetMissingNode",
            EditError::RebindTargetMissingNode { name: name() },
        ),
        (
            "RebindUnknownName",
            EditError::RebindUnknownName { name: name() },
        ),
        (
            "RebindKindMismatch",
            EditError::RebindKindMismatch {
                from: EntityKind::Face,
                to: EntityKind::Edge,
            },
        ),
        ("RebindIdentity", EditError::RebindIdentity { name: name() }),
        (
            "RebindNoReferences",
            EditError::RebindNoReferences { name: name() },
        ),
        (
            "WitnessOnNonSketch",
            EditError::WitnessOnNonSketch { node: n(5) },
        ),
        (
            "DuplicateWitnessEntry",
            EditError::DuplicateWitnessEntry { node: n(5) },
        ),
        ("EmptyWitnessBulk", EditError::EmptyWitnessBulk),
        (
            "NameUnresolvedInEvaluation",
            EditError::NameUnresolvedInEvaluation { name: name() },
        ),
        (
            "EvaluationOfAnotherDocument",
            EditError::EvaluationOfAnotherDocument {
                expected: DocumentId::derive("a"),
                found: DocumentId::derive("b"),
            },
        ),
        (
            "RebindAppearanceCollision",
            EditError::RebindAppearanceCollision {
                name: name(),
                kind: AttrKind::Color,
            },
        ),
        (
            "AppearanceWrongKind",
            EditError::AppearanceWrongKind { name: name() },
        ),
        (
            "AppearanceNamesMissingNode",
            EditError::AppearanceNamesMissingNode { name: name() },
        ),
        (
            "AppearanceNotSet",
            EditError::AppearanceNotSet {
                name: name(),
                kind: AttrKind::Label,
            },
        ),
        (
            "InvalidTolerance",
            EditError::InvalidTolerance { value: -1.0e-7 },
        ),
        (
            "MetaUnversioned",
            EditError::MetaUnversioned {
                name: name(),
                key: "supplier".to_owned(),
                error: MetaVersionError::MissingVersion,
            },
        ),
        (
            "MetaNonFinite",
            EditError::MetaNonFinite {
                name: name(),
                key: "supplier".to_owned(),
                path: "price.amount".to_owned(),
            },
        ),
        (
            "MetaNotSet",
            EditError::MetaNotSet {
                name: name(),
                key: "supplier".to_owned(),
            },
        ),
        (
            "RebindMetadataCollision",
            EditError::RebindMetadataCollision {
                name: name(),
                key: "supplier".to_owned(),
            },
        ),
        (
            "Roots",
            EditError::Roots(RootFault::Ancestor {
                ancestor: n(3),
                descendant: n(5),
            }),
        ),
        (
            "PlacementOnNonInstance",
            EditError::PlacementOnNonInstance { node: n(5) },
        ),
        (
            "PlacementRuleMismatch",
            EditError::PlacementRuleMismatch { node: n(5) },
        ),
        (
            "EmptyPlacementList",
            EditError::EmptyPlacementList { node: n(5) },
        ),
        (
            "ImproperPlacement",
            EditError::ImproperPlacement {
                node: n(5),
                determinant: -1.0,
            },
        ),
        (
            "NonFinitePlacement",
            EditError::NonFinitePlacement { node: n(5) },
        ),
        (
            "PlacementAxis",
            EditError::PlacementAxis {
                error: NodeErrorKind::DegenerateDirection {
                    role: "rotation axis",
                }
                .into(),
            },
        ),
        (
            "NonFiniteAlignment",
            EditError::NonFiniteAlignment { node: n(5) },
        ),
        (
            "MateRefused",
            EditError::MateRefused {
                node: n(9),
                fault: Box::new(MateFault::SelfMate {
                    mate: n(9),
                    instance: n(6),
                }),
            },
        ),
        (
            "UpdateOnNonInstance",
            EditError::UpdateOnNonInstance { node: n(5) },
        ),
        (
            "PinUnchanged",
            EditError::PinUnchanged {
                node: n(6),
                pin: ContentPin::of_bytes(b"bracket v3"),
            },
        ),
    ]
}

/// The path refusals a program edit forwards whole, at their longest:
/// each one the feature tree's census measures over 60 words.
fn long_path_refusals() -> Vec<(&'static str, profile::PathError<f64>)> {
    use profile::PathError as P;
    use profile::{CornerReason, CornerRefusal, FilletLeg, FilletLegCarrier, NoCornerReason};
    vec![
        (
            "SeamArrivalOffDirection",
            P::SeamArrivalOffDirection {
                margin: 1e-3,
                arm: 0.5,
            },
        ),
        (
            "JunctionCusp",
            P::JunctionCusp {
                margin: 1e-12,
                arm: 0.5,
            },
        ),
        (
            "FilletCarrierBelowSceneResolution",
            P::FilletCarrierBelowSceneResolution {
                turn: 0.5,
                radius: 1e-9,
                scale: 10.0,
                resolution: 1e-8,
                predicate: "fillet_scene_resolution",
                margin: 1e-12,
            },
        ),
        (
            "NoCornerOfPair(two)",
            P::NoCornerOfPair {
                radius: 0.3,
                corners: vec![
                    CornerRefusal {
                        at: geom_core::Point2::new(0.25, -0.5),
                        reason: CornerReason::AnchorOutsideTrimmedExtent {
                            side: FilletLeg::Outgoing,
                            carrier: FilletLegCarrier::Line,
                            setback: 0.4,
                            available: 0.3,
                        },
                    },
                    CornerRefusal {
                        at: geom_core::Point2::new(1.25, 0.5),
                        reason: CornerReason::NoTangentCircle(
                            NoCornerReason::OffsetCarriersDisjoint,
                        ),
                    },
                ],
            },
        ),
    ]
}

/// A mate fault of every arm, for the two edit arms that forward one.
fn mate_faults() -> Vec<(&'static str, MateFault)> {
    use editor_core::{Clash, LeverRefusal, MateSide, Subgroup};
    use geom_core::{Band, FrameError, FrameInput, Indeterminate, MarginDiag, Tol};
    let band = Band::linear(Tol::witness()).expect("the witness band");
    let doc_ref = editor_core::DocRef {
        id: DocumentId::derive("bracket"),
        pin: ContentPin::of_bytes(b"bracket v3"),
    };
    vec![
        (
            "PosesOfAnotherDocument",
            MateFault::PosesOfAnotherDocument {
                expected: DocumentId::derive("a"),
                found: DocumentId::derive("b"),
            },
        ),
        (
            "Frame",
            MateFault::Frame {
                mate: n(9),
                side: MateSide::A,
                error: FrameError::Degenerate {
                    input: FrameInput::Aim,
                    indeterminate: None,
                },
            },
        ),
        (
            "ClassNotAdmitted",
            MateFault::ClassNotAdmitted { mate: n(9) },
        ),
        (
            "TableLacks",
            MateFault::TableLacks {
                mate: n(9),
                what: "a clocking rider on a planar rest",
            },
        ),
        (
            "Indeterminate",
            MateFault::Indeterminate {
                mate: n(9),
                diag: Box::new(Indeterminate {
                    margin: MarginDiag::Value(3.0e-10),
                    band,
                    predicate: Some("mate_coaxial"),
                }),
            },
        ),
        (
            "Band",
            MateFault::Band {
                error: geom_core::BandError::Empty {
                    zero: 1.0e-6,
                    escalate: 1.0e-9,
                },
            },
        ),
        (
            "Contradictory",
            MateFault::Contradictory {
                held: n(8),
                added: n(9),
                predicate: "mate_coaxial",
                clash: Clash::Length { metres: 0.002 },
            },
        ),
        (
            "Under",
            MateFault::Under {
                mate: n(9),
                parent: n(6),
                child: n(7),
                residual: Subgroup::Se3,
            },
        ),
        (
            "DanglingHead",
            MateFault::DanglingHead {
                mate: n(9),
                side: MateSide::B,
                head: n(4),
            },
        ),
        (
            "PlacerRefused",
            MateFault::PlacerRefused {
                mate: n(9),
                side: MateSide::B,
                placer: n(4),
                error: NodeErrorKind::EmptyOperand { input: n(3) }.into(),
            },
        ),
        (
            "PartSelectsAnotherCopy",
            MateFault::PartSelectsAnotherCopy {
                mate: n(9),
                side: MateSide::A,
                part: n(6),
                named: 2,
                selected: 5,
            },
        ),
        (
            "SelfMate",
            MateFault::SelfMate {
                mate: n(9),
                instance: n(6),
            },
        ),
        (
            "Unleverable",
            MateFault::Unleverable {
                mate: n(9),
                refusal: LeverRefusal::NoExtent {
                    instance: n(6),
                    part: doc_ref,
                },
            },
        ),
    ]
}

/// The forwarding arms, over every payload they can carry that the
/// representative rows above do not reach.
fn forwarded_edit_refusals() -> Vec<(String, EditError)> {
    let mut rows = Vec::new();
    for (arm, p) in long_path_refusals() {
        rows.push((
            format!("ProfileProgramRefused(Geometry/{arm})"),
            EditError::ProfileProgramRefused {
                node: n(4),
                refusal: Box::new(ProgramRefusal::Geometry {
                    loop_: 0,
                    step: 2,
                    kind: p.kind(),
                    rendered: p.to_string(),
                }),
            },
        ));
    }
    for (arm, fault) in mate_faults() {
        rows.push((
            format!("MaintenanceRefused({arm})"),
            EditError::MaintenanceRefused {
                gauge: n(6),
                fault: Some(Box::new(fault.clone())),
            },
        ));
        rows.push((
            format!("MateRefused({arm})"),
            EditError::MateRefused {
                node: n(9),
                fault: Box::new(fault),
            },
        ));
    }
    rows
}

/// **Every edit refusal the status line draws fits the budget.** Each
/// `EditError` arm, and each forwarding arm over what it forwards,
/// rendered through [`Refusal::Edit`] and counted.
#[test]
fn every_edit_refusal_renders_within_the_budget() {
    let mut over = Vec::new();
    let rows = edit_refusals()
        .into_iter()
        .map(|(arm, e)| (arm.to_owned(), e))
        .chain(forwarded_edit_refusals());
    for (arm, e) in rows {
        let text = shown(e);
        let words = text.split_whitespace().count();
        eprintln!("MEASURE {words} Edit/{arm}: {text}");
        if words > BUDGET {
            over.push(arm);
        }
    }
    assert!(
        over.is_empty(),
        "the edit arms over {BUDGET} words: {over:?}"
    );
}
