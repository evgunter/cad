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
//! `MateFault` arm inside `MateRefused`, every `CountMismatch` inside
//! `PlacementRuleMismatch`, every
//! `StepIdFault` arm an edit door raises inside `StepIdsRefused`, and
//! the longest path refusals inside `ProfileProgramRefused` (the
//! feature tree's rows in `editor-core/tests/refusal_concision_chains.rs`
//! render every `PathError` arm).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::program::ProgramRefusal;
use editor_core::{
    AttrKind, ContentPin, CountMismatch, Dimension, DimensionError, DistributionFault,
    DistributionField, DocumentId, EditError, EntityKind, EvalError, FrameSite, Label, MateFault,
    MeasureNodeFault, MetaVersionError, NodeErrorKind, ParamName, RecipeNodeId, RootFault, SlotId,
    SpokenName, SpokenNode, StableName, StepIdFault,
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
        node: RecipeNodeId(tagged(3)),
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

fn param() -> ParamName {
    ParamName::from_static("width")
}

fn n(id: u64) -> RecipeNodeId {
    RecipeNodeId(tagged(id))
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
    use editor_core::{DocParamField, DocParamValue};
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
                    source: EvalError::UnknownParam(param()),
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
            "DuplicateInput",
            EditError::DuplicateInput {
                node: s(5, "Union"),
                input: s(3, "Extrude"),
            },
        ),
        (
            "RepeatedDesignation",
            EditError::RepeatedDesignation {
                node: s(5, "Shell"),
                first: 0,
                again: 2,
            },
        ),
        (
            "SelectionNotCanonical",
            EditError::SelectionNotCanonical {
                node: s(5, "Fillet"),
                at: 1,
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
            "TooFewMembers",
            EditError::TooFewMembers {
                node: s(5, "Union"),
                found: 1,
            },
        ),
        (
            "DeleteWouldDangle",
            EditError::DeleteWouldDangle {
                id: s(3, "Profile"),
                referenced_by: s(5, "Extrude"),
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
                expected: Dimension::Length,
                found: Dimension::Angle,
            },
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
                node: s(5, "Extrude"),
                slot: SlotId::Distance,
            },
        ),
        (
            "SlotDocParamDimension",
            EditError::SlotDocParamDimension {
                name: param(),
                node: s(5, "Extrude"),
                slot: SlotId::Distance,
                declared: Dimension::Angle,
                referenced: Dimension::Length,
            },
        ),
        (
            "PayloadUnknownDocParam",
            EditError::PayloadUnknownDocParam {
                name: param(),
                node: s(5, "Measure"),
            },
        ),
        (
            "PayloadDocParamDimension",
            EditError::PayloadDocParamDimension {
                name: param(),
                node: s(5, "Measure"),
                declared: Dimension::Angle,
                referenced: Dimension::Length,
            },
        ),
        (
            "MeasureMalformed",
            EditError::MeasureMalformed {
                node: s(5, "Measure"),
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
                node: s(6, "Assertion"),
                measure: s(5, "Extrude"),
            },
        ),
        (
            "DeclareInputNotDeclare",
            EditError::DeclareInputNotDeclare {
                node: s(6, "Union"),
                input: s(5, "Extrude"),
            },
        ),
        (
            "AssertionDimension",
            EditError::AssertionDimension {
                node: s(6, "Assertion"),
                measure: s(5, "Measure"),
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
                                step: editor_core::StepId(tagged(9)),
                                role: editor_core::PieceRole::Leg,
                            },
                        )],
                    },
                    s(3, "Extrude"),
                ),
                step: editor_core::StepId(tagged(9)),
            },
        ),
        (
            "ReadSiteMissingNode",
            EditError::ReadSiteMissingNode {
                at: SpokenNode::absent(n(9)),
            },
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
            "Roots",
            EditError::Roots(RootFault::Ancestor {
                ancestor: s(3, "Extrude"),
                descendant: s(5, "Fillet"),
            }),
        ),
        (
            "OffsetOnNonInstance",
            EditError::OffsetOnNonInstance {
                node: s(5, "Extrude"),
            },
        ),
        (
            "GaugeOnNonPlaced",
            EditError::GaugeOnNonPlaced {
                node: s(5, "Extrude"),
            },
        ),
        (
            "GaugeNotLive",
            EditError::GaugeNotLive {
                node: s(5, "InstantiatePart"),
                gauge: SpokenNode::absent(n(3)),
            },
        ),
        (
            "NotAGauge",
            EditError::NotAGauge {
                node: s(5, "InstantiatePart"),
                gauge: s(3, "Extrude"),
            },
        ),
        (
            "GaugeCycle",
            EditError::GaugeCycle {
                node: s(5, "Gauge"),
                gauge: s(3, "Gauge"),
            },
        ),
        (
            "WouldStartPlacing",
            EditError::WouldStartPlacing { mate: s(9, "Mate") },
        ),
        // `PlacementRuleMismatch`: every shape, each spoken with the
        // node kind that raises it, in `forwarded_edit_refusals`.
        (
            "EmptyPlacementList",
            EditError::EmptyPlacementList {
                node: s(5, "PlacedUnion"),
            },
        ),
        (
            "ImproperPlacement",
            EditError::ImproperPlacement {
                node: s(5, "InstantiatePart"),
                at: FrameSite::Step { index: 1 },
                determinant: -1.0,
            },
        ),
        (
            "NonFinitePlacement",
            EditError::NonFinitePlacement {
                node: s(5, "InstantiatePart"),
                at: FrameSite::Step { index: 0 },
            },
        ),
        (
            "NonRigidPlacement",
            EditError::NonRigidPlacement {
                node: s(5, "InstantiatePart"),
                at: FrameSite::Listed { index: 2 },
                check: "transform_rigid_col01_orth",
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
            "NonFiniteAlignment",
            EditError::NonFiniteAlignment { node: s(5, "Mate") },
        ),
        (
            "MateRefused",
            EditError::MateRefused {
                node: s(9, "Mate"),
                fault: Box::new(MateFault::SelfMate {
                    mate: n(9),
                    instance: n(6),
                }),
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

fn next_root_fault(fault: &RootFault) -> Option<RootFault> {
    match fault {
        RootFault::NotLive { .. } => Some(RootFault::Duplicate {
            root: s(3, "Extrude"),
        }),
        RootFault::Duplicate { .. } => Some(RootFault::Ancestor {
            ancestor: s(3, "Extrude"),
            descendant: s(5, "Fillet"),
        }),
        RootFault::Ancestor { .. } => Some(RootFault::Uncovered {
            node: s(4, "Extrude"),
        }),
        RootFault::Uncovered { .. } => None,
    }
}

fn next_distribution_fault(fault: &DistributionFault) -> Option<DistributionFault> {
    match fault {
        DistributionFault::SigmaNotPositive { .. } => {
            Some(DistributionFault::NominalOutsideSupport { lo: 1.0, hi: 0.5 })
        }
        DistributionFault::NominalOutsideSupport { .. } => None,
        // No row: no edit door raises it, because `distribution_fault_error`
        // routes a non-finite offset to `NonFiniteDocParam`, which has
        // its own.
        DistributionFault::NonFinite { .. } => None,
    }
}

/// The count mismatch after `shape`, every arm in turn.
fn next_count_mismatch(shape: &CountMismatch) -> Option<CountMismatch> {
    match shape {
        CountMismatch::ListedOnPattern => Some(CountMismatch::ListedWithCount),
        CountMismatch::ListedWithCount => Some(CountMismatch::SteppedWithoutCount),
        CountMismatch::SteppedWithoutCount => None,
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
            step: StepId(tagged(7)),
        }),
        StepIdFault::NotThisProfiles { .. } => Some(StepIdFault::Repeated {
            step: StepId(tagged(7)),
        }),
        StepIdFault::Repeated { .. } => Some(StepIdFault::Collides {
            step: StepId(tagged(7)),
        }),
        StepIdFault::Collides { .. } => None,
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
                    margin: MarginDiag::value(3.0e-10),
                    band,
                    predicate: Some("mate_coaxial"),
                    terminal_sliver: false,
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
                placer_row: pncad::document::PlacerRow::Silent,
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
                refusal: Box::new(LeverRefusal::Reach {
                    instance: n(6),
                    part: doc_ref,
                    refusal: editor_core::ReachRefusal::NoExtent,
                }),
            },
        ),
        (
            "OffsetDisagrees",
            MateFault::OffsetDisagrees {
                instance: n(7),
                root: n(6),
                predicate: "mate_member_translation_zero",
                clash: Clash::Length { metres: 0.002 },
            },
        ),
        (
            "OffsetUnchecked",
            MateFault::OffsetUnchecked {
                instance: n(7),
                cause: Box::new(editor_core::OffsetCheck::Placement {
                    node: n(3),
                    error: NodeErrorKind::EmptyOperand { input: n(2) }.into(),
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
    for fault in witnesses(
        RootFault::NotLive {
            root: SpokenNode::absent(n(9)),
        },
        next_root_fault,
    ) {
        rows.push((
            format!("Roots({})", variant(&fault)),
            EditError::Roots(fault),
        ));
    }
    for shape in witnesses(CountMismatch::ListedOnPattern, next_count_mismatch) {
        let kind = match shape {
            CountMismatch::ListedOnPattern => "Pattern",
            CountMismatch::ListedWithCount | CountMismatch::SteppedWithoutCount => "PlacedUnion",
        };
        rows.push((
            format!("PlacementRuleMismatch({})", variant(&shape)),
            EditError::PlacementRuleMismatch {
                node: s(5, kind),
                shape,
            },
        ));
    }
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
                name: param(),
                fault,
            },
        ));
    }
    for (arm, fault) in mate_faults() {
        rows.push((
            format!("MateRefused({arm})"),
            EditError::MateRefused {
                node: s(9, "Mate"),
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
    (
        "Edit/PlacementRuleMismatch(ListedOnPattern)",
        "Pattern \"base plate\"",
    ),
    (
        "Edit/PlacementRuleMismatch(ListedWithCount)",
        "PlacedUnion \"base plate\"",
    ),
    (
        "Edit/PlacementRuleMismatch(SteppedWithoutCount)",
        "PlacedUnion \"base plate\"",
    ),
    ("Edit/EmptyPlacementList", "PlacedUnion \"base plate\""),
    ("Edit/MeasureMalformed", "Measure \"base plate\""),
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
    // work/paths/paths-refusals-short-of-the-shape-guard.md
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
