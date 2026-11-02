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
//! `MateFault` arm inside `MateRefused`, every
//! `StepIdFault` arm an edit door raises inside `StepIdsRefused`, and
//! the longest path refusals inside `ProfileProgramRefused` (the
//! feature tree's rows in `editor-core/tests/refusal_concision_chains.rs`
//! render every `PathError` arm).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::program::ProgramRefusal;
use editor_core::{
    AttrKind, ContentPin, Dimension, DimensionError, DistributionFault, DistributionField,
    DocumentId, EditError, EntityKind, EvalError, Label, MateFault, MetaVersionError,
    NodeErrorKind, RecipeNodeId, SlotId, SpokenName, SpokenNode, StableName, StepIdFault, VarName,
};
use test_utils::refusal::Admission;
use test_utils::refusal::tagged;
use viewer::session::Refusal;

fn shown(e: EditError) -> String {
    Refusal::Edit(Box::new(e)).to_string()
}

fn stable_name() -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: RecipeNodeId::new(0, tagged(3)),
        path: Vec::new(),
    }
}

/// The name as an arm whose minting node is live speaks it.
fn name() -> SpokenName {
    editor_core::test_support::spoken_name(stable_name(), s(3, "Extrude"))
}

/// The name as an arm whose minting node is not live speaks it.
fn missing() -> SpokenName {
    SpokenName::absent(stable_name())
}

fn param() -> VarName {
    VarName::from_static("width")
}

/// `param()` as a refusal speaks it.
fn spoken_var() -> pncad::document::SpokenVar {
    pncad::document::SpokenVar::new(pncad::document::VarId::new(0, tagged(7)), Some(param()))
}

fn n(id: u64) -> RecipeNodeId {
    RecipeNodeId::new(0, tagged(id))
}

/// Constraint `id`'s minted id.
fn c(id: u64) -> pncad::document::ConstraintId {
    pncad::document::ConstraintId(pncad::document::MintId::new(0, tagged(id)))
}

/// Constraint `id` of placement 7, the copy every mate row's bundle
/// places.
fn r(id: u64) -> pncad::document::MateRef {
    pncad::document::MateRef::new(n(7), c(id))
}

/// Node `id` as a refusal raised over a document holding it as a
/// `kind` speaks it: labelled, the longest spelling a sentence names a
/// node by, and by the kind the arm really names, so a template that
/// says the noun its node already says is read here as it ships.
fn s(id: u64, kind: &'static str) -> SpokenNode {
    let label = Label::new("base plate").expect("a valid label");
    editor_core::test_support::spoken_labelled(n(id), kind, label)
}

/// Every `EditError` arm, on a representative payload.
fn edit_refusals() -> Vec<(&'static str, EditError)> {
    use editor_core::edit::CarryForwardDoor;
    use editor_core::{DocParamField, FreeValue};
    vec![
        (
            "UnknownNode",
            EditError::UnknownNode {
                id: SpokenNode::absent(n(9)),
            },
        ),
        (
            "ProfileProgramRefused(Geometry)",
            EditError::ProfileProgramRefused {
                node: s(4, "Profile"),
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
                node: s(4, "Profile"),
                refusal: Box::new(ProgramRefusal::Resolve {
                    slot: SlotId::Distance,
                    source: EvalError::UnresolvedVar {
                        var: pncad::document::VarId::new(0, tagged(7)),
                    },
                }),
            },
        ),
        (
            "ProfileProgramRefused(Transition)",
            EditError::ProfileProgramRefused {
                node: s(4, "Profile"),
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
                node: s(4, "Profile"),
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
            EditError::UnresolvedInput {
                input: SpokenNode::absent(n(9)),
            },
        ),
        (
            "WouldCycle",
            EditError::WouldCycle {
                at: s(4, "Extrude"),
            },
        ),
        (
            "SelectionShape(Repeated)",
            EditError::SelectionShape {
                node: s(5, "Shell"),
                slot: SlotId::Operand(editor_core::OperandSlot::Open),
                fault: editor_core::SelectionFault::Repeated { first: 0, again: 2 },
            },
        ),
        (
            "SelectionShape(NotCanonical)",
            EditError::SelectionShape {
                node: s(5, "Fillet"),
                slot: SlotId::Operand(editor_core::OperandSlot::Selection),
                fault: editor_core::SelectionFault::NotCanonical { at: 1 },
            },
        ),
        (
            "SelectionShape(Kind)",
            EditError::SelectionShape {
                node: s(5, "Fillet"),
                slot: SlotId::Operand(editor_core::OperandSlot::Selection),
                fault: editor_core::SelectionFault::Kind {
                    at: 1,
                    found: editor_core::EntityKind::Face,
                    expected: editor_core::EntityKind::Edge,
                },
            },
        ),
        (
            "SelectionNameKind",
            EditError::SelectionNameKind {
                node: s(5, "Fillet"),
                slot: SlotId::Operand(editor_core::OperandSlot::Selection),
                at: 1,
                name: name(),
                found: editor_core::EntityKind::Face,
                expected: editor_core::EntityKind::Edge,
            },
        ),
        (
            "SetMembersOnNonList",
            EditError::SetMembersOnNonList {
                node: s(5, "Extrude"),
            },
        ),
        (
            "SetProgramOnNonProfile",
            EditError::SetProgramOnNonProfile {
                node: s(5, "Extrude"),
            },
        ),
        (
            "OperandUnresolved",
            EditError::OperandUnresolved {
                node: s(5, "Extrude"),
                slot: SlotId::Operand(pncad::document::OperandSlot::Profile),
                read: Box::new(pncad::document::Operand::Node(s(3, "Profile").id())),
            },
        ),
        (
            "AmbiguousOutput",
            EditError::AmbiguousOutput {
                input: s(3, "Split"),
                slot: SlotId::Operand(pncad::document::OperandSlot::From),
                ports: vec!["above", "below"],
            },
        ),
        (
            "DefinesNothing",
            EditError::DefinesNothing {
                input: s(3, "Assertion"),
                slot: SlotId::Operand(pncad::document::OperandSlot::Target),
            },
        ),
        (
            "UnknownSlot",
            EditError::UnknownSlot {
                id: s(5, "Extrude"),
                slot: SlotId::Radius,
            },
        ),
        (
            "SlotDimensionMismatch",
            EditError::SlotDimensionMismatch {
                slot: SlotId::Distance,
                expected: pncad::document::SlotKind::Is(pncad::document::VarKind::Length),
                found: Dimension::Angle,
            },
        ),
        (
            "StructuralSlotNeedsStructuralEdit",
            EditError::StructuralSlotNeedsStructuralEdit {
                slot: SlotId::Stations,
            },
        ),
        (
            "NotStructuralSlot",
            EditError::NotStructuralSlot {
                slot: SlotId::Distance,
            },
        ),
        (
            "SlotUnknownVarName",
            EditError::SlotUnknownVarName {
                name: param(),
                node: s(5, "Extrude"),
                slot: SlotId::Distance,
            },
        ),
        (
            "SlotVarKind",
            EditError::SlotVarKind {
                var: Box::new(spoken_var()),
                node: s(5, "Extrude"),
                slot: SlotId::Distance,
                found: pncad::document::VarKind::Angle,
                expected: pncad::document::SlotKind::Is(pncad::document::VarKind::Length),
            },
        ),
        (
            "SlotUnresolvedVar",
            EditError::SlotUnresolvedVar {
                var: pncad::document::SpokenVar::new(
                    pncad::document::VarId::new(0, tagged(7)),
                    None,
                ),
                node: s(5, "Extrude"),
                slot: SlotId::Distance,
            },
        ),
        (
            "PayloadUnknownVarName",
            EditError::PayloadUnknownVarName {
                name: param(),
                node: s(5, "Measure"),
            },
        ),
        (
            "PayloadVarKind",
            EditError::PayloadVarKind {
                var: spoken_var(),
                node: s(5, "Measure"),
                declared: pncad::document::VarKind::Angle,
                referenced: Dimension::Length,
            },
        ),
        (
            "PayloadUnresolvedVar",
            EditError::PayloadUnresolvedVar {
                var: pncad::document::SpokenVar::new(
                    pncad::document::VarId::new(0, tagged(7)),
                    None,
                ),
                node: s(5, "Measure"),
            },
        ),
        (
            "AssertionDimension",
            EditError::AssertionDimension {
                node: s(6, "Assertion"),
                measured: Dimension::Length,
                bound: Dimension::Angle,
            },
        ),
        (
            "ConstructionReadsObserved",
            EditError::ConstructionReadsObserved {
                node: s(5, "Extrude"),
                slot: SlotId::Distance,
                var: Box::new(spoken_var()),
            },
        ),
        (
            "ContinuousVarCannotBeCount",
            EditError::ContinuousVarCannotBeCount { var: spoken_var() },
        ),
        (
            "UnknownVar",
            EditError::UnknownVar {
                var: param().into(),
                door: CarryForwardDoor::Notation,
            },
        ),
        (
            "VarNameTaken",
            EditError::VarNameTaken {
                name: param(),
                holder: spoken_var(),
            },
        ),
        (
            "VarNameUnchanged",
            EditError::VarNameUnchanged { var: spoken_var() },
        ),
        (
            "AnonymousVarUnread",
            EditError::AnonymousVarUnread {
                var: pncad::document::SpokenVar::new(
                    pncad::document::VarId::new(0, tagged(7)),
                    None,
                ),
            },
        ),
        (
            "DeleteAnonymousVar",
            EditError::DeleteAnonymousVar {
                var: pncad::document::SpokenVar::new(
                    pncad::document::VarId::new(0, tagged(7)),
                    None,
                ),
            },
        ),
        (
            "VarKindFixed",
            EditError::VarKindFixed {
                var: spoken_var(),
                kind: pncad::document::VarKind::Count,
                offered: pncad::document::VarKind::Length,
            },
        ),
        (
            "NotAFreeVar",
            EditError::NotAFreeVar {
                var: spoken_var(),
                door: editor_core::CarryForwardDoor::Value,
            },
        ),
        (
            "DefinitionCycle",
            EditError::DefinitionCycle {
                var: spoken_var(),
                through: vec![
                    spoken_var(),
                    pncad::document::SpokenVar::new(
                        pncad::document::VarId::new(0, tagged(8)),
                        Some(VarName::from_static("height")),
                    ),
                ],
            },
        ),
        (
            "DefinitionTooLarge",
            EditError::DefinitionTooLarge {
                var: spoken_var(),
                nodes: 4097,
            },
        ),
        (
            "DefinitionUnknownVarName",
            EditError::DefinitionUnknownVarName {
                var: spoken_var(),
                name: VarName::from_static("height"),
            },
        ),
        (
            "DefinitionUnresolvedVar",
            EditError::DefinitionUnresolvedVar {
                var: spoken_var(),
                read: pncad::document::SpokenVar::new(
                    pncad::document::VarId::new(0, tagged(8)),
                    None,
                ),
            },
        ),
        (
            "DefinitionVarKind",
            EditError::DefinitionVarKind {
                var: spoken_var(),
                read: pncad::document::SpokenVar::new(
                    pncad::document::VarId::new(0, tagged(8)),
                    Some(VarName::from_static("height")),
                ),
                declared: pncad::document::VarKind::Angle,
                referenced: Dimension::Length,
            },
        ),
        (
            "VarCountHasNoUnit",
            EditError::VarCountHasNoUnit { var: spoken_var() },
        ),
        (
            "VarCountHasNoDistribution",
            EditError::VarCountHasNoDistribution { var: spoken_var() },
        ),
        (
            "VarUnitMismatch",
            EditError::VarUnitMismatch {
                var: spoken_var(),
                unit: Dimension::Angle,
                declared: Dimension::Length,
            },
        ),
        (
            "VarValueKindMismatch",
            EditError::VarValueKindMismatch {
                var: spoken_var(),
                declared: Dimension::Count,
                offered: FreeValue::Continuous(2.5),
            },
        ),
        (
            "PathOffTree",
            EditError::PathOffTree {
                node: s(5, "Extrude"),
                slot: SlotId::Distance,
                path: vec![0, 3],
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
            EditError::DeclareNamesMissingNode { name: missing() },
        ),
        (
            "NameStepNeverMinted",
            EditError::NameStepNeverMinted {
                name: editor_core::test_support::spoken_name(
                    StableName {
                        kind: EntityKind::Edge,
                        node: n(3),
                        path: vec![editor_core::RoleSeg::RimEdge(
                            editor_core::CapEnd::End,
                            editor_core::ProfileEdgeRef::Piece {
                                step: editor_core::StepId::new(0, tagged(9)),
                                role: editor_core::PieceRole::Leg,
                            }
                            .into(),
                        )],
                    },
                    s(3, "Extrude"),
                ),
                step: editor_core::StepId::new(0, tagged(9)),
            },
        ),
        (
            "ReadSiteMissingNode",
            EditError::ReadSiteMissingNode {
                at: SpokenNode::absent(n(9)),
            },
        ),
        (
            "NonFiniteVar",
            EditError::NonFiniteVar {
                var: spoken_var(),
                field: DocParamField::Offset(DistributionField::Sigma),
            },
        ),
        (
            "InvalidDistribution",
            EditError::InvalidDistribution {
                var: spoken_var(),
                fault: DistributionFault::NominalOutsideSupport { lo: 1.0, hi: 0.5 },
            },
        ),
        (
            "RebindTargetMissingNode",
            EditError::RebindTargetMissingNode { name: missing() },
        ),
        (
            "RebindUnknownName",
            EditError::RebindUnknownName { name: missing() },
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
            EditError::WitnessOnNonSketch {
                node: s(5, "Extrude"),
            },
        ),
        (
            "DuplicateWitnessEntry",
            EditError::DuplicateWitnessEntry {
                node: s(5, "Profile"),
            },
        ),
        (
            "TakeOverRepeatsCopy",
            EditError::TakeOverRepeatsCopy {
                node: s(5, "Place"),
            },
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
            EditError::AppearanceNamesMissingNode { name: missing() },
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
            "WorldIsUndeletable",
            EditError::WorldIsUndeletable {
                node: s(0, "World"),
            },
        ),
        (
            "ConstraintOnNonPlacement",
            EditError::ConstraintOnNonPlacement {
                node: s(5, "Extrude"),
            },
        ),
        (
            "IndicesUnrelated",
            EditError::IndicesUnrelated {
                node: s(5, "Place"),
                a: Box::new(spoken_var()),
                b: Box::new(pncad::document::SpokenVar::new(
                    pncad::document::VarId::new(0, tagged(8)),
                    Some(VarName::new("j").expect("a name")),
                )),
            },
        ),
        (
            "UnknownConstraint",
            EditError::UnknownConstraint {
                node: s(5, "Place"),
                constraint: c(9),
            },
        ),
        (
            "PickPath",
            EditError::PickPath {
                node: s(5, "Union"),
                slot: pncad::document::SlotId::Operand(pncad::document::OperandSlot::Member(0)),
                at: Some(1),
            },
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
            "MateRefused",
            EditError::MateRefused {
                node: s(7, "Place"),
                fault: Box::new(MateFault::NotOnTheCopy { mate: r(9) }),
                held: Default::default(),
            },
        ),
        (
            "UpdateOnNonInstance",
            EditError::UpdateOnNonInstance {
                node: s(5, "Profile"),
            },
        ),
        (
            "PinUnchanged",
            EditError::PinUnchanged {
                node: s(6, "InstantiatePart"),
                pin: ContentPin::of_bytes(b"bracket v3"),
            },
        ),
    ]
}

/// One witness per arm of a forwarded fault, chained from `first`.
/// Each `next` matches every arm with no wildcard, so an arm added to
/// the fault does not compile until it is given a place in the chain —
/// or a stated reason for having none.
fn witnesses<T>(first: T, next: fn(&T) -> Option<T>) -> Vec<T> {
    let mut chain = vec![first];
    while let Some(following) = chain.last().and_then(next) {
        chain.push(following);
    }
    chain
}

/// A witness's arm, by its `Debug` identifier: the row's name.
fn variant(witness: &impl core::fmt::Debug) -> String {
    format!("{witness:?}")
        .split(|c: char| !c.is_alphanumeric())
        .next()
        .unwrap_or_default()
        .to_owned()
}

fn next_distribution_fault(fault: &DistributionFault) -> Option<DistributionFault> {
    match fault {
        DistributionFault::SigmaNotPositive { .. } => {
            Some(DistributionFault::NominalOutsideSupport { lo: 1.0, hi: 0.5 })
        }
        DistributionFault::NominalOutsideSupport { .. } => None,
        // No row: no edit door raises it, because `distribution_fault_error`
        // routes a non-finite offset to `NonFiniteVar`, which has
        // its own.
        DistributionFault::NonFinite { .. } => None,
    }
}

fn next_step_id_fault(fault: &StepIdFault) -> Option<StepIdFault> {
    use editor_core::StepId;
    match fault {
        StepIdFault::Preminted => Some(StepIdFault::LoopCount { loops: 2, given: 1 }),
        StepIdFault::LoopCount { .. } => Some(StepIdFault::Shape {
            loop_: 0,
            authored: 4,
            given: 3,
        }),
        StepIdFault::Shape { .. } => Some(StepIdFault::NotThisProfiles {
            step: StepId::new(0, tagged(7)),
        }),
        StepIdFault::NotThisProfiles { .. } => Some(StepIdFault::Repeated {
            step: StepId::new(0, tagged(7)),
        }),
        StepIdFault::Repeated { .. } => None,
        // No row: no edit door raises it. It is the load door's word,
        // and an edit that writes a name spelling a step the document
        // never minted refuses `NameStepNeverMinted`, which has its own.
        StepIdFault::NotMinted { .. } => None,
    }
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
    use editor_core::{LeverRefusal, MateSide, VarKind};
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
            "Overconstrained",
            MateFault::Overconstrained {
                placement: n(7),
                constraints: vec![c(8), c(9)],
                free: 0,
            },
        ),
        ("NotOnTheCopy", MateFault::NotOnTheCopy { mate: r(9) }),
        (
            "KindsDiffer",
            MateFault::KindsDiffer {
                mate: r(9),
                on: VarKind::Plane,
                to: VarKind::Axis,
            },
        ),
        (
            "NoChart",
            MateFault::NoChart {
                value: r(9),
                why: editor_core::NoChartWhy::SlideTurnsABall,
            },
        ),
        (
            "SideUnevaluated",
            MateFault::SideUnevaluated {
                mate: r(9),
                side: MateSide::To,
                refusal: Box::new(NodeErrorKind::EmptyOperand { input: n(3) }.into()),
            },
        ),
        (
            "Unleverable",
            MateFault::Unleverable {
                mate: r(9),
                refusal: Box::new(LeverRefusal::Reach {
                    instance: n(6),
                    part: doc_ref,
                    refusal: editor_core::ReachRefusal::NoExtent,
                }),
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
                node: s(4, "Profile"),
                refusal: Box::new(ProgramRefusal::Geometry {
                    loop_: 0,
                    step: 2,
                    kind: p.kind(),
                    rendered: p.to_string(),
                }),
            },
        ));
    }
    // Each states its own recourse, so each is rendered, not only the
    // representative row's — every arm, from the witness chains below.
    for fault in witnesses(StepIdFault::Preminted, next_step_id_fault) {
        rows.push((
            format!("StepIdsRefused({})", variant(&fault)),
            EditError::StepIdsRefused {
                node: s(4, "Profile"),
                fault,
            },
        ));
    }
    for fault in witnesses(
        DistributionFault::SigmaNotPositive { sigma: 0.0 },
        next_distribution_fault,
    ) {
        rows.push((
            format!("InvalidDistribution({})", variant(&fault)),
            EditError::InvalidDistribution {
                var: spoken_var(),
                fault,
            },
        ));
    }
    for (arm, fault) in mate_faults() {
        rows.push((
            format!("MateRefused({arm})"),
            EditError::MateRefused {
                node: s(7, "Place"),
                fault: Box::new(fault),
                held: Default::default(),
            },
        ));
    }
    rows
}

/// The clause labels an edit refusal legitimately opens with, each on
/// the row namespace that writes it: the node, measure, sketch step or
/// mate the refusal is about, and a pair's corner list.
const LABELS: &[(&str, &str)] = &[
    ("Edit/ProfileProgramRefused(Geometry", "loop 0 step 2"),
    (
        "Edit/ProfileProgramRefused(Geometry/NoCornerOfPair(",
        "at corner",
    ),
];

/// The rows that state no recourse — no `Recourse:`, no "There is no way
/// through", and none of the shared unlabelled repairs — by exact row
/// id, grouped under the row that files them with their owner.
const FILED_NO_RECOURSE: &[&str] = &[
    // work/profilerest/paths-refusals-short-of-the-shape-guard.md
    "Edit/ProfileProgramRefused(Resolve)",
    "Edit/ProfileProgramRefused(Transition)",
    "Edit/ProfileProgramRefused(Validate)",
];

/// **Every edit refusal the status line draws meets the standard.**
/// Each `EditError` arm, and each forwarding arm over what it forwards,
/// rendered through [`Refusal::Edit`] and held to
/// [`test_utils::refusal::problems`]: the budget, no stage prefix, no
/// `Debug` struct, no arena key, one recourse. Every admission must
/// admit something a row renders, so one its owner's fix made stale goes
/// red.
#[test]
fn every_edit_refusal_renders_within_the_budget() {
    let mut problems = Vec::new();
    let mut used = std::collections::BTreeSet::new();
    let mut names = Vec::new();
    let rows = edit_refusals()
        .into_iter()
        .map(|(arm, e)| (arm.to_owned(), e))
        .chain(forwarded_edit_refusals());
    for (arm, e) in rows {
        let text = shown(e);
        let name = format!("Edit/{arm}");
        eprintln!("MEASURE {} {name}: {text}", text.split_whitespace().count());
        let scoped: Vec<&(&str, &str)> = LABELS
            .iter()
            .filter(|(ns, _)| name.starts_with(ns))
            .collect();
        for prefix in test_utils::refusal::stage_prefixes(&text, &[]) {
            let prefix = prefix.trim_end_matches(':');
            if let Some((ns, l)) = scoped.iter().find(|(_, l)| *l == prefix) {
                used.insert(format!("LABELS {ns} {l}"));
            }
        }
        let allowed: Vec<&str> = scoped.iter().map(|(_, l)| *l).collect();
        let no_recourse = format!("{name} states no recourse");
        for problem in
            test_utils::refusal::problems_admitting(&name, &text, &allowed, false, ADMISSIONS)
        {
            if FILED_NO_RECOURSE.contains(&name.as_str()) && problem.starts_with(&no_recourse) {
                used.insert(format!("FILED_NO_RECOURSE {name}"));
            } else {
                problems.push(problem);
            }
        }
        names.push(name);
    }
    for entry in LABELS
        .iter()
        .map(|(ns, l)| format!("LABELS {ns} {l}"))
        .chain(
            FILED_NO_RECOURSE
                .iter()
                .map(|n| format!("FILED_NO_RECOURSE {n}")),
        )
    {
        if !used.contains(&entry) {
            problems.push(format!(
                "the admission {entry} admits nothing a row renders"
            ));
        }
    }
    problems.extend(test_utils::refusal::unclaimed_admissions(
        ADMISSIONS,
        names.iter().map(String::as_str),
    ));
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// The rows that name a document or a version by its hex id, by exact
/// row id and the exact span, each filed with its owner: `EditError`'s
/// pairing and pin arms.
const ADMISSIONS: &[Admission<'static>] = &[
    Admission {
        row: "Edit/EvaluationOfAnotherDocument",
        span: "3e23e8160039594a33894f6564e1b134",
        filed: "work/doctail/part-refusals-name-documents-by-hex-id.md",
    },
    Admission {
        row: "Edit/EvaluationOfAnotherDocument",
        span: "ca978112ca1bbdcafac231b39a23dc4d",
        filed: "work/doctail/part-refusals-name-documents-by-hex-id.md",
    },
    Admission {
        row: "Edit/PinUnchanged",
        span: "9515831d455a13139e7a712b440337b3447c4b9f3b969d034020eacf0fd8a56d",
        filed: "work/doctail/part-refusals-name-documents-by-hex-id.md",
    },
];
