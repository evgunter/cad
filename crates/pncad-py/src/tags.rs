//! Stable discriminant tags for the document layer's refusals — and
//! for the two answers that are not refusals at all.
//!
//! [`resolution_status_tag`] is one: a resolution is a TOTAL answer,
//! one of three states for every name asked, and it crosses as a value
//! rather than a raise. [`distribution_kind_tag`] is the other, and it
//! is a value discriminant one rung further from a refusal — which of
//! E2's four forms a parameter's annotation is. What all three kinds
//! share is the reason a tag exists at all — a caller branches on the
//! discriminant, prose is not a stable interface — and the reason this
//! file is their home: the exhaustive match is a drift alarm that
//! fires in hosted CI, because this module compiles without Python and
//! the `#[pyclass]` it feeds does not. (A value discriminant may live
//! outside this file where its match needs a type this one does not
//! import — `crate::node_kind` is the standing example — and then its
//! roster is pinned in `src/tests.rs` directly instead.)
//!
//! Typed exceptions carry the structured error, never strings. The
//! exception's machine payload is a stable **tag** — a discriminant
//! name a caller can branch on, which no `Display` prose gives it
//! because prose is not a stable interface — and its human message is
//! the kernel error's own `Display`, never a `Debug` dump.
//!
//! A tag is not the whole payload, and the doors are split on that.
//! Most project every arm's fields as attributes beside the tag —
//! `py/readback.rs` and `py/refactor.rs` are the worked examples: one
//! exhaustive `match`, no wildcard arm, every attribute present on
//! every arm and `None` where the arm does not carry it, so `getattr`
//! never raises and a caller never has to branch on `variant` first.
//! The rest cross as tag plus message alone. **#1479 owns that split**
//! — which doors are on which side, and what each unprojected one
//! withholds; this header does not keep a second copy of that census.
//!
//! The matches below are EXHAUSTIVE on purpose. A new kernel variant
//! breaks this build rather than silently arriving in Python as an
//! untagged refusal; that is the drift alarm, and it fires in hosted
//! CI because this module compiles without Python. **One map is the
//! exception**: [`select_refusal_tag`]'s enum is `#[non_exhaustive]`,
//! which forces a wildcard arm and takes the compile-time alarm away
//! with nothing that fires in its place — see that function for what
//! the crossing does instead.
//!
//! **This file is READ as data.** The exhaustive matches guard the
//! existence of a tag; nothing in the compiler guards its VALUE, and
//! the values are the Python-facing contract. So
//! `src/tests.rs`'s `the_whole_tag_table_matches_its_committed_inventory`
//! parses this source at test time and compares every function's
//! literals against an inventory committed there — a rename, an
//! addition, a deletion or a new tag function reds on the default
//! no-interpreter row. Its reader ENUMERATES rather than approximates:
//! every top-level line here must be a comment, a `use` item, a
//! `pub fn NAME(..) -> &'static str {` closed by a `}` in column 0, or
//! a `pub const NAME: &str = "..";`, and every match arm's body must
//! be a literal, a nested `match`, a block around one of those, or a
//! call to another tag function. Anything else fails that test with
//! *I do not understand this* rather than being skipped — so an
//! attribute, a helper, or a cleverer arm added here is a deliberate
//! diff that teaches the reader too, never a silent hole.

use pncad::analysis::{AnalysisPolicyError, MeasureUnavailable, ParamBoxError, SeedError};
use pncad::document::{
    AssemblyError, AttrKind, Attribution, Axis3, CheckEvidence, ChecksError, DimensionError,
    Distribution, DistributionFault, DistributionField, EditError, EvalError, InlineError,
    MateFault, MeasureNodeFault, MeasureUnavailableAt, MetaVersionError, NodeErrorKind, ParseError,
    PersistError, PlacementRuleFault, ProgramFault, ProgramRefusal, RecordedProgramError,
    RefusedRef, Relation, RootFault, SlotId, SnapshotError, SplitError, UpdateError,
};
use pncad::geom_core::{BandError, BandField, FrameError, FrameInput};
use pncad::mesh::TessellateError;
use pncad::prelude::BlendKind;
use pncad::profile::{
    CornerReason, CornerWindow, NoCornerReason, PathError, PathErrorKind, ProfileError,
    ReplayErrorKind, StructureRefusalKind,
};
use pncad::quantity::FmtQuantityError;
use pncad::select::{
    DanglingRef, HitTestError, InterrogateError, MeshPickError, NamingError, NodePickError,
    ReadbackError, Resolution, ResolveError, ResolveIndeterminate,
};
use pncad::step_import::{PromotedKind, StepImportError};
use pncad::sweep::blend::BlendError;
use pncad::sweep::{ExtrudeError, LoftError, RevolveError, SkinError, TubeError};
use pncad::topo::param_source::ParamAttachError;
use pncad::topo::splitting::SplitError as SplitOpError;
use pncad::topo::{
    BooleanError, CensusContact, CensusSubject, EntityId, ShellError, TransformError,
    ValidationError,
};
// All three STL refusals are prelude-curated; the module path is the
// spelling this file uses throughout, not a reach past the façade.
use pncad::stl::{BinaryHeaderError, SolidNameError, StlError};
use pncad::workspace::WorkspaceError;

/// The stable tag for a PATHS authoring refusal.
///
/// `PathError` implements `Display`, so the human message is the
/// kernel's own prose and the tag is the branchable discriminant —
/// the [`persist_error_tag`] treatment, not the `Debug`-dump one.
///
/// The FFI spelling is this crate's to own; the DISCRIMINANT is not.
/// It is `PathError::kind`, and this map keys off it — one stable
/// string per kind, over `PathErrorKind`'s arms rather than `..`, so a
/// new kernel refusal stops this build instead of acquiring a silent
/// tag. A kind with no arm behind it is a phantom, and the fix is to
/// delete it kernel-side; minting a tag for one would publish an FFI
/// name no refusal can ever carry.
pub fn path_error_tag(err: &PathError<f64>) -> &'static str {
    match err.kind() {
        PathErrorKind::JunctionTangent => "junction_tangent",
        PathErrorKind::JunctionCusp => "junction_cusp",
        PathErrorKind::SeamTangent => "seam_tangent",
        PathErrorKind::SeamArrivalOffDirection => "seam_arrival_off_direction",
        PathErrorKind::SeamArrivalLeverTooShort => "seam_arrival_lever_too_short",
        PathErrorKind::ContinuationTargetOffRay => "continuation_target_off_ray",
        PathErrorKind::NoCornerForFillet => "no_corner_for_fillet",
        PathErrorKind::NoCornerOfPair => "no_corner_of_pair",
        PathErrorKind::FilletOffsetLeverTooShort => "fillet_offset_lever_too_short",
        PathErrorKind::ArcLegOnOpenFillet => "arc_leg_on_open_fillet",
        PathErrorKind::SeamRetrimsArcFirstSide => "seam_retrims_arc_first_side",
        PathErrorKind::Structure => "guided_structure",
        PathErrorKind::DegenerateArcSpec => "degenerate_arc_spec",
        PathErrorKind::NonpositiveLeg => "nonpositive_leg",
        PathErrorKind::NonpositiveFilletRadius => "nonpositive_fillet_radius",
        PathErrorKind::NonpositiveCircleRadius => "nonpositive_circle_radius",
        PathErrorKind::CircleSplitCount => "circle_split_count",
        PathErrorKind::PolygonTooFewVertices => "polygon_too_few_vertices",
        PathErrorKind::ArcContinueNeedsArcCarrier => "arc_continue_needs_arc_carrier",
        PathErrorKind::ArcContinueOffCarrier => "arc_continue_off_carrier",
        PathErrorKind::ZeroDirection => "zero_direction",
        PathErrorKind::ArcViaCollinear => "arc_via_collinear",
        PathErrorKind::DegenerateArcChord => "degenerate_arc_chord",
        PathErrorKind::ArcCenterNotEquidistant => "arc_center_not_equidistant",
        PathErrorKind::DegenerateArcCenter => "degenerate_arc_center",
        PathErrorKind::FarEndAnchorWithoutFillet => "far_end_anchor_without_fillet",
        PathErrorKind::Escalated => "escalated",
        PathErrorKind::Band => "band",
        PathErrorKind::UnderdeterminedLeg => "underdetermined_leg",
        PathErrorKind::OverdeterminedJunction => "overdetermined_junction",
    }
}

/// The stable tag for ONE entry of a `no_corner_of_pair` envelope —
/// why that derived corner refused.
///
/// The nested-payload treatment, as `recorded_program_error_tag`'s
/// literal arm gives it: the envelope's own tag says which refusal
/// arrived, and the entry's tag says what the corner's story is, so a
/// caller branches on the reason without parsing the sentence. Over
/// `CornerReason`'s arms rather than `..`, so a new one stops this
/// build instead of acquiring a silent tag.
pub fn corner_reason_tag(reason: &CornerReason<f64>) -> &'static str {
    match reason {
        CornerReason::OutsideAnchors(window) => match window {
            CornerWindow::BehindIncomingRay => "behind_incoming_ray",
            CornerWindow::BehindArrivalAnchor => "behind_arrival_anchor",
        },
        // The constructor door's own vocabulary rides through rather
        // than being flattened: "the radius is too large for this
        // corner" and "every tangent circle touches a leg past this
        // corner" are different situations with different recourses,
        // and the entry carries its own kind.
        CornerReason::NoTangentCircle(reason) => match reason {
            NoCornerReason::OffsetCarriersDisjoint => "offset_carriers_disjoint",
            NoCornerReason::NoCornerSideCandidate => "no_corner_side_candidate",
        },
        CornerReason::AnchorOutsideTrimmedExtent { .. } => "anchor_outside_trimmed_extent",
        CornerReason::EnclosesLegCarrier { .. } => "encloses_leg_carrier",
    }
}

/// The stable tag for a recorded-program lift refusal
/// (`LoopProgram::from_recorded`). The literal arm carries
/// the expression layer's own tag through rather than flattening it.
pub fn recorded_program_error_tag(err: &RecordedProgramError) -> &'static str {
    match err {
        RecordedProgramError::Literal(inner) => expr_dimension_error_tag(inner),
        RecordedProgramError::SubdivisionCount(_) => "subdivision_count",
        RecordedProgramError::CarrierInChain => "carrier_in_chain",
    }
}

/// The stable tag for a selection refusal (the
/// `Evaluation.select_where` door).
///
/// `SelectRefusal` is `#[non_exhaustive]`, so unlike this module's
/// other matches the wildcard arm is FORCED on this crate and the
/// compile-time drift alarm is unavailable. **Nothing replaces it.**
/// `src/tests.rs`'s `select_refusal_tags_are_stable` constructs arms
/// by name and asserts their tags; it cannot construct — and so
/// cannot fail on — an arm the kernel has not shipped yet. What the
/// pin gives is the enumeration the wildcard hides — every arm whose
/// payload it can construct, one assertion each — so an arm added to
/// the kernel is an absence in a list rather than invisible behind
/// the wildcard. The safety property is the crossing itself: an
/// unknown arm refuses typed as `unclassified` (`py/select.rs`),
/// never dropped.
pub fn select_refusal_tag(err: &pncad::select::SelectRefusal) -> &'static str {
    use pncad::select::SelectRefusal as R;
    match err {
        R::InBand { .. } => "in_band",
        R::TiedDisagrees { .. } => "tied_disagrees",
        R::Unreadable { .. } => "unreadable",
        R::NotADatum { .. } => "not_a_datum",
        R::NotALength { .. } => "not_a_length",
        R::PairInBand { .. } => "pair_in_band",
        R::BadValue(_) => "bad_value",
        R::Band => "band",
        _ => "unclassified",
    }
}

/// The stable tag for a NAMED expression slot — `EditError.slot`, the
/// address a refusal is about.
///
/// A slot is a per-node-type NAME, never an index, so the word is the
/// slot's own identity and not a position: `distance`, `count`,
/// `origin_x`. The seven vector families spell their component into
/// the word rather than beside it, because `origin` alone names three
/// slots and a caller branching on it could not tell which expression
/// refused.
///
/// `profile` is the one arm that stops one level, and it stops for the
/// reason [`profile_error_tag`]'s family does: what is left below it —
/// the loop index, the step index and which of the step's arguments —
/// is two integers and a third enum, and no `&'static str` carries an
/// integer. The address is in the refusal's prose; the word says the
/// slot is a profile program's.
pub fn slot_id_tag(slot: &SlotId) -> &'static str {
    match slot {
        SlotId::Origin(axis) => match axis {
            Axis3::X => "origin_x",
            Axis3::Y => "origin_y",
            Axis3::Z => "origin_z",
        },
        SlotId::Normal(axis) => match axis {
            Axis3::X => "normal_x",
            Axis3::Y => "normal_y",
            Axis3::Z => "normal_z",
        },
        SlotId::Direction(axis) => match axis {
            Axis3::X => "direction_x",
            Axis3::Y => "direction_y",
            Axis3::Z => "direction_z",
        },
        SlotId::U(axis) => match axis {
            Axis3::X => "u_x",
            Axis3::Y => "u_y",
            Axis3::Z => "u_z",
        },
        SlotId::V(axis) => match axis {
            Axis3::X => "v_x",
            Axis3::Y => "v_y",
            Axis3::Z => "v_z",
        },
        SlotId::Translation(axis) => match axis {
            Axis3::X => "translation_x",
            Axis3::Y => "translation_y",
            Axis3::Z => "translation_z",
        },
        SlotId::RotationAxis(axis) => match axis {
            Axis3::X => "rotation_axis_x",
            Axis3::Y => "rotation_axis_y",
            Axis3::Z => "rotation_axis_z",
        },
        SlotId::Distance => "distance",
        SlotId::Radius => "radius",
        SlotId::ChamferDistance => "chamfer_distance",
        SlotId::ShellThickness => "shell_thickness",
        SlotId::RevolveAngle => "revolve_angle",
        SlotId::Spin => "spin",
        SlotId::TubeMajorRadius => "tube_major_radius",
        SlotId::TubeMinorRadius => "tube_minor_radius",
        SlotId::TubeWindowStart => "tube_window_start",
        SlotId::TubeWindowEnd => "tube_window_end",
        SlotId::TubeWall => "tube_wall",
        SlotId::RotationAngle => "rotation_angle",
        SlotId::Spacing => "spacing",
        SlotId::Step => "step",
        SlotId::Count => "count",
        SlotId::Instance => "instance",
        SlotId::VDegree => "v_degree",
        SlotId::Stations => "stations",
        SlotId::Profile { .. } => "profile",
    }
}

/// The stable tag for an APPEARANCE attribute's kind — the `kind` an
/// appearance refusal names.
///
/// The attribute's own vocabulary, not the entity's: `EntityKind` says
/// what a name denotes and this says which of the three display
/// attributes a rebind collided on or a clear did not find.
pub fn attr_kind_tag(kind: &AttrKind) -> &'static str {
    match kind {
        AttrKind::Color => "color",
        AttrKind::Label => "label",
        AttrKind::Visibility => "visibility",
    }
}

/// The stable tag for an edit refusal.
pub fn edit_error_tag(err: &EditError) -> &'static str {
    match err {
        EditError::UnknownNode { .. } => "unknown_node",
        EditError::ProfileProgramRefused { .. } => "profile_program_refused",
        EditError::UnresolvedInput { .. } => "unresolved_input",
        EditError::WouldCycle { .. } => "would_cycle",
        // The list-input door's three (DM4/DM5). Tags only: the Python
        // SURFACE for `Node.union` and `SetMembers` is LIB's build,
        // and this match is exhaustive, so the crate's compile is what
        // requires these rows and nothing else here changes.
        EditError::DuplicateInput { .. } => "duplicate_input",
        EditError::RepeatedDesignation { .. } => "repeated_designation",
        EditError::SetMembersOnNonList { .. } => "set_members_on_non_list",
        EditError::TooFewMembers { .. } => "too_few_members",
        EditError::DeleteWouldDangle { .. } => "delete_would_dangle",
        EditError::UnknownSlot { .. } => "unknown_slot",
        EditError::SlotDimensionMismatch { .. } => "slot_dimension_mismatch",
        EditError::StructuralSlotNeedsStructuralEdit { .. } => {
            "structural_slot_needs_structural_edit"
        }
        EditError::NotStructuralSlot { .. } => "not_structural_slot",
        EditError::UnknownDocParam { .. } => "unknown_doc_param",
        EditError::UnknownPayloadParam { .. } => "unknown_payload_param",
        EditError::PayloadParamDimensionMismatch { .. } => "payload_param_dimension_mismatch",
        EditError::MeasureMalformed { .. } => "measure_malformed",
        EditError::AssertionTarget { .. } => "assertion_target",
        EditError::DeclareInputNotDeclare { .. } => "declare_input_not_declare",
        EditError::AssertionDimension { .. } => "assertion_dimension",
        EditError::DocParamDimensionMismatch { .. } => "doc_param_dimension_mismatch",
        EditError::ContinuousParamCannotBeCount { .. } => "continuous_param_cannot_be_count",
        EditError::DocParamNotDeclared { .. } => "doc_param_not_declared",
        EditError::DocParamValueKindMismatch { .. } => "doc_param_value_kind_mismatch",
        EditError::PathOffTree { .. } => "path_off_tree",
        EditError::Dimension { .. } => "dimension",
        EditError::DeclareNamesMissingNode { .. } => "declare_names_missing_node",
        EditError::ReadSiteMissingNode { .. } => "read_site_missing_node",
        EditError::NonFiniteDocParam { .. } => "non_finite_doc_param",
        EditError::InvalidDistribution { .. } => "invalid_distribution",
        EditError::RebindTargetMissingNode { .. } => "rebind_target_missing_node",
        EditError::RebindUnknownName { .. } => "rebind_unknown_name",
        EditError::RebindKindMismatch { .. } => "rebind_kind_mismatch",
        EditError::RebindIdentity { .. } => "rebind_identity",
        EditError::RebindNoReferences { .. } => "rebind_no_references",
        EditError::WitnessOnNonSketch { .. } => "witness_on_non_sketch",
        EditError::DuplicateWitnessEntry { .. } => "duplicate_witness_entry",
        EditError::EmptyWitnessBulk => "empty_witness_bulk",
        EditError::NameUnresolvedInEvaluation { .. } => "name_unresolved_in_evaluation",
        EditError::RebindAppearanceCollision { .. } => "rebind_appearance_collision",
        EditError::AppearanceWrongKind { .. } => "appearance_wrong_kind",
        EditError::AppearanceNamesMissingNode { .. } => "appearance_names_missing_node",
        EditError::AppearanceNotSet { .. } => "appearance_not_set",
        EditError::InvalidTolerance { .. } => "invalid_tolerance",
        EditError::MetaUnversioned { .. } => "meta_unversioned",
        EditError::MetaNonFinite { .. } => "meta_non_finite",
        EditError::MetaNotSet { .. } => "meta_not_set",
        EditError::RebindMetadataCollision { .. } => "rebind_metadata_collision",
        // The product-root invariants tag per FAULT,
        // not per wrapper: which invariant broke is what a caller
        // branches on.
        EditError::Roots(fault) => root_fault_tag(fault),
        EditError::PlacementOnNonInstance { .. } => "placement_on_non_instance",
        EditError::PlacementRuleMismatch { .. } => "placement_rule_mismatch",
        EditError::EmptyPlacementList { .. } => "empty_placement_list",
        EditError::ImproperPlacement { .. } => "improper_placement",
        EditError::NonFinitePlacement { .. } => "non_finite_placement",
        EditError::PlacementAxis { .. } => "placement_axis",
        EditError::UpdateOnNonInstance { .. } => "update_on_non_instance",
        EditError::PinUnchanged { .. } => "pin_unchanged",
        // A mate's alignment is authored geometry, so the non-finite
        // refusal is the placement one's sibling and tags beside it.
        EditError::NonFiniteAlignment { .. } => "non_finite_alignment",
    }
}

/// The stable Python word for which of E2's four forms a
/// [`Distribution`] is.
///
/// Not a refusal: it is the discriminant of a value a caller HOLDS,
/// and the answer to "what did this parameter declare". snake_case
/// like every other stable word this crate publishes, and
/// deliberately not serde's spelling — the saved text writes the Rust
/// variant identifiers and that belongs to the persistence format's
/// compatibility contract, which may not move a Python word and may
/// not be moved by one.
pub fn distribution_kind_tag(dist: &Distribution) -> &'static str {
    match dist {
        Distribution::Band { .. } => "band",
        Distribution::Uniform { .. } => "uniform",
        Distribution::Normal { .. } => "normal",
        Distribution::TruncatedNormal { .. } => "truncated_normal",
    }
}

/// The stable tag for a broken distribution invariant (ERROR-DESIGN
/// E2) — the fault `Distribution::check` answers with.
///
/// ONE tag per fault, shared by both doors that can carry one: the
/// Python `Distribution` constructor raises it directly, and the edit
/// door's `invalid_distribution` refusal carries the same fault, so a
/// caller reads the same word whichever door refused. `non_finite` is
/// the arm the edit door re-routes to `non_finite_doc_param` — the
/// document layer folds a non-finite offset into the document-wide
/// non-finite class — so the two doors agree on the fault and differ
/// on where the document puts it, which is the kernel's own split and
/// not this file's.
pub fn distribution_fault_tag(fault: &DistributionFault) -> &'static str {
    match fault {
        DistributionFault::NonFinite { .. } => "non_finite",
        DistributionFault::SigmaNotPositive { .. } => "sigma_not_positive",
        DistributionFault::NominalOutsideSupport { .. } => "nominal_outside_support",
    }
}

/// Which FIELD of a distribution a fault is about.
///
/// The Python spelling of `DistributionField`, which crosses as this
/// text on the fault's `field` attribute rather than as a class: a
/// three-word closed set naming struct fields is what a caller
/// compares against, and a class would add a name to import for no
/// question it answers. Word for word the kernel's own `Display`.
pub fn distribution_field_tag(field: &DistributionField) -> &'static str {
    match field {
        DistributionField::Sigma => "sigma",
        DistributionField::Lo => "lo",
        DistributionField::Hi => "hi",
    }
}

/// The stable tag for a mass the analysis lane could not price.
///
/// One arm today, and the tag exists anyway for the reason every tag
/// here does: `band_has_no_measure` is what a caller branches on, and
/// a second arm added kernel-side breaks this match rather than
/// arriving in Python untagged.
pub fn measure_unavailable_tag(err: &MeasureUnavailable) -> &'static str {
    match err {
        MeasureUnavailable::BandHasNoMeasure { .. } => "band_has_no_measure",
    }
}

/// The stable tag for a measured expression the construction door
/// refuses.
///
/// One arm today, and the tag exists anyway for the reason every tag
/// here does: `ref_index_out_of_range` is what a caller branches on,
/// and a second arm added kernel-side breaks this match rather than
/// arriving in Python untagged.
///
/// The SAME fault reaches the edit door as
/// `EditError::MeasureMalformed`, which carries its own tag
/// (`measure_malformed`) because what refused there is the EDIT and
/// the fault is its payload. Two tags for one fault, and they answer
/// different questions: which door said no, and what was wrong.
pub fn measure_node_fault_tag(fault: &MeasureNodeFault) -> &'static str {
    match fault {
        MeasureNodeFault::RefIndexOutOfRange { .. } => "ref_index_out_of_range",
    }
}

/// The stable tag for a measure with no value at this build's scalar.
///
/// One arm today. Deliberately NOT sharing a function with
/// [`measure_unavailable_tag`] one screen up: that one is the
/// ANALYSIS lane's band refusal and this one the MEASUREMENT lane's
/// missing enclosure, two kernel types whose names differ by one word
/// and whose questions do not overlap at all.
pub fn measure_unavailable_at_tag(reason: &MeasureUnavailableAt) -> &'static str {
    match reason {
        MeasureUnavailableAt::NeedsEnclosure { .. } => "needs_enclosure",
    }
}

/// The stable tag for a policy the analysis lane cannot honour.
pub fn analysis_policy_error_tag(err: &AnalysisPolicyError) -> &'static str {
    match err {
        AnalysisPolicyError::QuantileMassOutOfRange { .. } => "quantile_mass_out_of_range",
    }
}

/// The stable tag for a placement-rule fault (GROUP-BOOLEAN-DESIGN) —
/// ONE tag per fault, shared by every door that carries one.
///
/// The tags are the EDIT door's own (`placement_rule_mismatch`,
/// `empty_placement_list`, `non_finite_placement`,
/// `improper_placement`), so the same broken rule reads the same
/// whether it is refused at the node constructor, at the edit gate, or
/// at the evaluation backstop — one fault, one spelling, three doors.
pub fn placement_rule_fault_tag(fault: &PlacementRuleFault) -> &'static str {
    match fault {
        PlacementRuleFault::CountSpelling => "placement_rule_mismatch",
        PlacementRuleFault::NoPlacements => "empty_placement_list",
        PlacementRuleFault::NonFiniteFrame { .. } => "non_finite_placement",
        PlacementRuleFault::ImproperFrame { .. } => "improper_placement",
    }
}

/// The stable tag for a frame-construction refusal
/// (`geom_core::linalg::frame`'s constructors).
///
/// `FrameError` implements `Display`, so the human message is the
/// kernel's own prose and the tag is the branchable discriminant. The
/// degenerate arm tags per INPUT: which direction was unusable is what
/// a caller branches on, and the wrapper arm alone would collapse four
/// distinct refusals into one.
pub fn frame_error_tag(err: &FrameError) -> &'static str {
    match err {
        FrameError::Degenerate { input, .. } => match input {
            FrameInput::Aim => "degenerate_aim",
            FrameInput::Tangent => "degenerate_tangent",
            FrameInput::RollReference => "degenerate_roll_reference",
            FrameInput::ReferenceLadder => "degenerate_reference_ladder",
            FrameInput::MirrorNormal => "degenerate_mirror_normal",
        },
        FrameError::Band(_) => "band",
    }
}

/// The stable tag for WHICH band threshold a
/// `BandError::InvalidValue` is about.
///
/// The kernel's `BandField` has a `name()` of its own for messages;
/// this is the FFI spelling, which is this crate's to own, and it is
/// word for word that one.
pub fn band_field_tag(field: &BandField) -> &'static str {
    match field {
        BandField::Zero => "zero",
        BandField::Escalate => "escalate",
    }
}

/// The stable tag for a product-root invariant refusal — shared by
/// every door that carries a `RootFault`.
pub fn root_fault_tag(fault: &RootFault) -> &'static str {
    match fault {
        RootFault::NotLive { .. } => "root_not_live",
        RootFault::Duplicate { .. } => "root_duplicate",
        RootFault::Ancestor { .. } => "root_ancestor",
        RootFault::Uncovered { .. } => "root_uncovered",
    }
}

/// The stable tag for a node's evaluation refusal.
pub fn node_error_tag(kind: &NodeErrorKind) -> &'static str {
    match kind {
        NodeErrorKind::Expr { .. } => "expr",
        NodeErrorKind::Profile { .. } => "profile",
        NodeErrorKind::ProfileReplay { .. } => "profile_replay",
        NodeErrorKind::ProfileLaneReplay { .. } => "profile_lane_replay",
        NodeErrorKind::ProfileAnchor { .. } => "profile_anchor",
        NodeErrorKind::Extrude { .. } => "extrude",
        NodeErrorKind::Revolve { .. } => "revolve",
        // ONE tag for both tube kinds, matching every other op on
        // this map (`revolve` covers ten `RevolveError` arms the
        // same way): the tag names the OP that refused, and the
        // refusal is already attributed to a node whose kind says
        // solid or hollow. Per-arm tags would be worth having, but
        // for every op at once — not for the one op whose unit
        // happened to be written last.
        NodeErrorKind::Tube { .. } => "tube",
        NodeErrorKind::Split { .. } => "split",
        // The two blends share one kernel error type, so the tag is
        // read off the VERB the node is: a chamfer's refusal must not
        // reach Python calling itself a fillet's.
        NodeErrorKind::Blend { verb, .. } => match verb {
            BlendKind::Fillet => "fillet",
            BlendKind::Chamfer => "chamfer",
        },
        NodeErrorKind::Boolean { .. } => "boolean",
        NodeErrorKind::Transform { .. } => "transform",
        NodeErrorKind::Skin { .. } => "skin",
        NodeErrorKind::Loft { .. } => "loft",
        NodeErrorKind::CurvedSolidFrontier { .. } => "curved_solid_frontier",
        NodeErrorKind::MissingInput { .. } => "missing_input",
        NodeErrorKind::MeasureRefResolve { .. } => "measure_ref_resolve",
        NodeErrorKind::MeasureRefUnreadable { .. } => "measure_ref_unreadable",
        NodeErrorKind::MeasureUnsupported(_) => "measure_unsupported",
        NodeErrorKind::MeasureNotParallel { .. } => "measure_not_parallel",
        NodeErrorKind::MeasureNonFinite { .. } => "measure_non_finite",
        NodeErrorKind::MeasureMalformed(_) => "measure_malformed",
        // Its own tag rather than `measure_unsupported`'s: the
        // recourse is "select a body or a face", not "this carrier
        // pair has no closed form".
        NodeErrorKind::MeasureSelectionKind { .. } => "measure_selection_kind",
        // And its own again: the clearance engine refused, so the
        // recourse is the engine's — a wider budget, an admitted
        // carrier — and not the measurement vocabulary's.
        NodeErrorKind::MeasureClearanceRefused(_) => "measure_clearance_refused",
        NodeErrorKind::PayloadExpr { .. } => "payload_expr",
        NodeErrorKind::AssertionDimension { .. } => "assertion_dimension",
        NodeErrorKind::ToleranceConflict { .. } => "tolerance_conflict",
        // Its own tag rather than the ε conflict's: both refuse every
        // node for a whole-run reason, but the recourses are different
        // — one is "replay in a process whose ε matches", the other is
        // "ask for the box at a scalar that can carry it".
        NodeErrorKind::ParamBox { .. } => "param_box",
        // The box arm's twin, and its own tag for the same reason: the
        // recourse is "seed at a scalar with a tangent channel" (or
        // name a continuous parameter), not the box's.
        NodeErrorKind::Seed { .. } => "seed",
        // The seed stopped at a C6/D9-pinned section: its own tag,
        // because the recourse ("this parameter cannot be seeded
        // through a loft or sweep section") is neither the box's nor
        // the scalar's.
        NodeErrorKind::SeedPinnedSection { .. } => "seed_pinned_section",
        NodeErrorKind::WrongOperand { .. } => "wrong_operand",
        NodeErrorKind::EmptyOperand { .. } => "empty_operand",
        NodeErrorKind::DegenerateDirection { .. } => "degenerate_direction",
        NodeErrorKind::NonFiniteDirection { .. } => "non_finite_direction",
        NodeErrorKind::Band { .. } => "band",
        NodeErrorKind::MissingSlot { .. } => "missing_slot",
        NodeErrorKind::VerbArity { .. } => "verb_arity",
        NodeErrorKind::Escalated { .. } => "escalated",
        NodeErrorKind::AxisInDifferentPlane { .. } => "axis_in_different_plane",
        NodeErrorKind::NonPositiveCount { .. } => "non_positive_count",
        NodeErrorKind::PlacementsUncertified { .. } => "placements_uncertified",
        NodeErrorKind::PlacementRule(fault) => placement_rule_fault_tag(fault),
        NodeErrorKind::UnschedulableCycle => "unschedulable_cycle",
        NodeErrorKind::Naming { .. } => "naming",
        NodeErrorKind::ParamSourceAttach(_) => "param_source_attach",
        NodeErrorKind::DeclareResolve { .. } => "declare_resolve",
        NodeErrorKind::DeclareBothOperands { .. } => "declare_both_operands",
        NodeErrorKind::DeclareUnsupportedPair { .. } => "declare_unsupported_pair",
        NodeErrorKind::UnionDeclareStep { .. } => "union_declare_step",
        // The refusal MENU: the boolean's
        // undeclared-contact refusal carrying the candidate
        // declaration; the `finding` payload crosses as a typed
        // attribute beside this tag.
        NodeErrorKind::UndeclaredContact { .. } => "undeclared_contact",
        NodeErrorKind::BlendSelectionResolve { verb, .. } => match verb {
            BlendKind::Fillet => "fillet_selection_resolve",
            BlendKind::Chamfer => "chamfer_selection_resolve",
        },
        NodeErrorKind::BlendSelectionKind { verb, .. } => match verb {
            BlendKind::Fillet => "fillet_selection_kind",
            BlendKind::Chamfer => "chamfer_selection_kind",
        },
        NodeErrorKind::BlendSelectionEmpty { verb } => match verb {
            BlendKind::Fillet => "fillet_selection_empty",
            BlendKind::Chamfer => "chamfer_selection_empty",
        },
        // The shell: ONE tag for the op's refusal family (the
        // `revolve`/`tube` treatment — the kernel's `ShellError` arms
        // are prose in the message), the two open-list refusals in the
        // `chamfer_selection_*` spelling, and the lane refusal.
        NodeErrorKind::Shell(_) => "shell",
        NodeErrorKind::ShellOpenResolve { .. } => "shell_open_resolve",
        NodeErrorKind::ShellOpenKind { .. } => "shell_open_kind",
        NodeErrorKind::ShellLaneUnsupported { .. } => "shell_lane_unsupported",
        // The derived sketch frame's refusals (DOCM-1): the fillet's
        // ladder and kind refusals, one carrier-kind refusal, one
        // read-back refusal, and the section refusal DM1c adds.
        NodeErrorKind::FaceFrameResolve { .. } => "face_frame_resolve",
        NodeErrorKind::FaceFrameKind { .. } => "face_frame_kind",
        NodeErrorKind::FaceFrameNotPlanar { .. } => "face_frame_not_planar",
        NodeErrorKind::FaceFrameReadback { .. } => "face_frame_readback",
        NodeErrorKind::DerivedFrameSection { .. } => "derived_frame_section",
        // The projection node's two refusals (DOCM-2): a half with no
        // material, and an instance index outside the pattern's count.
        // Tags only — the Python surface for `Node.part` is LIB's
        // build, and this match is exhaustive, so the crate's compile
        // is what requires these rows.
        NodeErrorKind::EmptyHalf { .. } => "empty_half",
        NodeErrorKind::InstanceOutOfRange { .. } => "instance_out_of_range",
        NodeErrorKind::WitnessBifurcation { .. } => "witness_bifurcation",
        // The seam faults stay separable at the tag level:
        // "the pin does not hold" and "the tolerances disagree" are
        // different recourses, so they are different tags.
        NodeErrorKind::Part { fault, .. } => part_fault_tag(fault),
        // The mate solve's refusals tag per FAULT, the way
        // the root invariants do — UNDER, CONTRADICTORY and a
        // dangling head carry different recourses, so a caller
        // branches on which one fired, not on "a mate failed".
        NodeErrorKind::Mate(fault) => mate_fault_tag(fault),
        NodeErrorKind::CrossingUnverified { .. } => "crossing_unverified",
    }
}

/// The stable tag for the ARM of the kernel refusal a node error
/// carries — `EvaluationError.inner_kind`, beside
/// [`node_error_tag`]'s `kind`.
///
/// **Two words because there are two enums.** `kind` is the CARRIER's
/// discriminant: which door refused, fixed by the node's kind before
/// any payload is read. This one is the payload's own, and it exists
/// only once the carrier has said which enum it holds. Projecting each
/// where it lives is why the second word is a second ATTRIBUTE rather
/// than a finer spelling of the first: folding the two into one
/// vocabulary would move every shipped `kind` value and leave a
/// caller splitting words by prefix to get back the question it
/// started with.
///
/// `None` on an arm whose refusal has no arms of its own — a payload
/// of numbers, ids, roles or nothing at all. Three further shapes read
/// as `None` and each is a decision, not an omission:
///
/// * an arm whose payload enum is a VALUE and not a refusal (a split
///   half, an entity kind, a carrier kind, a verb): the second word
///   answers "which fault", and what a value-valued field answers is
///   the payload question, whose home is an attribute of its own;
/// * an arm whose own tag is ALREADY the payload's — `Mate`, `Part`
///   and `PlacementRule` read their word off the fault through
///   [`mate_fault_tag`], [`part_fault_tag`] and
///   [`placement_rule_fault_tag`], so the fine word is on the wire
///   under the carrier's name and moving it here would move a shipped
///   `kind` value;
/// * `WitnessBifurcation`, whose payload is the branch solver's
///   telemetry record: the arm is not constructed before the M6
///   solver, and the façade curates the record's discriminant
///   interior, so there is nothing to name and nothing that could
///   carry it.
///
/// Exhaustive like every map here, and the delegations are one level:
/// an inner enum whose own arm carries a third discriminant gives that
/// arm's word, never the third's.
pub fn node_inner_kind_tag(kind: &NodeErrorKind) -> Option<&'static str> {
    match kind {
        NodeErrorKind::Expr { source, .. } => Some(eval_error_tag(source)),
        NodeErrorKind::Profile(inner) => Some(profile_error_tag(inner)),
        NodeErrorKind::ProfileReplay { error, .. } => Some(replay_error_tag(&error.kind)),
        // The lane's own geometry refusing carries no record, and
        // `None` says so rather than naming a decision nothing
        // consumed.
        NodeErrorKind::ProfileLaneReplay { structure, .. } => match structure {
            Some(refusal) => Some(structure_refusal_tag(&refusal.kind)),
            None => None,
        },
        NodeErrorKind::ProfileAnchor { .. } => None,
        NodeErrorKind::Extrude(inner) => Some(extrude_error_tag(inner)),
        NodeErrorKind::Revolve(inner) => Some(revolve_error_tag(inner)),
        NodeErrorKind::Tube(inner) => Some(tube_error_tag(inner)),
        NodeErrorKind::Split(inner) => Some(split_op_error_tag(inner)),
        NodeErrorKind::Blend { error, .. } => Some(blend_error_tag(error)),
        NodeErrorKind::Boolean(inner) => Some(boolean_error_tag(inner)),
        NodeErrorKind::Transform(inner) => Some(transform_error_tag(inner)),
        NodeErrorKind::Skin(inner) => Some(skin_error_tag(inner)),
        NodeErrorKind::Loft(inner) => Some(loft_error_tag(inner)),
        NodeErrorKind::CurvedSolidFrontier { .. } => None,
        NodeErrorKind::MissingInput { .. } => None,
        NodeErrorKind::ToleranceConflict { .. } => None,
        NodeErrorKind::ParamBox { source } => Some(param_box_error_tag(source)),
        NodeErrorKind::Seed { source } => Some(seed_error_tag(source)),
        NodeErrorKind::SeedPinnedSection { .. } => None,
        NodeErrorKind::WrongOperand { .. } => None,
        NodeErrorKind::EmptyOperand { .. } => None,
        // `half` is WHICH side was empty, a value the caller asked
        // for — the payload question, not the fault one.
        NodeErrorKind::EmptyHalf { .. } => None,
        NodeErrorKind::InstanceOutOfRange { .. } => None,
        NodeErrorKind::DegenerateDirection { .. } => None,
        NodeErrorKind::NonFiniteDirection { .. } => None,
        NodeErrorKind::Band(inner) => Some(band_error_tag(inner)),
        NodeErrorKind::MissingSlot { .. } => None,
        NodeErrorKind::VerbArity { .. } => None,
        // The escalation's payload is a MARGIN and a band, not an arm;
        // the predicate that escalated is a name the kernel mints and
        // the message carries.
        NodeErrorKind::Escalated { .. } => None,
        NodeErrorKind::AxisInDifferentPlane { .. } => None,
        NodeErrorKind::NonPositiveCount { .. } => None,
        NodeErrorKind::PlacementsUncertified { .. } => None,
        NodeErrorKind::PlacementRule(_) => None,
        NodeErrorKind::UnschedulableCycle => None,
        NodeErrorKind::Naming(inner) => Some(naming_error_tag(inner)),
        NodeErrorKind::ParamSourceAttach(inner) => Some(param_attach_error_tag(inner)),
        NodeErrorKind::DeclareResolve { error } => Some(resolve_error_tag(error)),
        NodeErrorKind::DeclareBothOperands { .. } => None,
        NodeErrorKind::UnionDeclareStep { .. } => None,
        NodeErrorKind::DeclareUnsupportedPair { .. } => None,
        // The candidate declaration crosses whole, as the `finding`
        // attribute; the refusing predicate's diagnostic is a margin.
        NodeErrorKind::UndeclaredContact { .. } => None,
        NodeErrorKind::BlendSelectionResolve { error, .. } => Some(resolve_error_tag(error)),
        NodeErrorKind::BlendSelectionKind { .. } => None,
        NodeErrorKind::BlendSelectionEmpty { .. } => None,
        NodeErrorKind::Shell(inner) => Some(shell_error_tag(inner)),
        NodeErrorKind::ShellOpenResolve { error } => Some(resolve_error_tag(error)),
        NodeErrorKind::ShellOpenKind { .. } => None,
        NodeErrorKind::ShellLaneUnsupported { .. } => None,
        NodeErrorKind::FaceFrameResolve { error } => Some(resolve_error_tag(error)),
        NodeErrorKind::FaceFrameKind { .. } => None,
        NodeErrorKind::FaceFrameNotPlanar { .. } => None,
        NodeErrorKind::FaceFrameReadback { error } => Some(readback_error_tag(error)),
        NodeErrorKind::DerivedFrameSection { .. } => None,
        NodeErrorKind::WitnessBifurcation(_) => None,
        NodeErrorKind::Part { .. } => None,
        NodeErrorKind::Mate(_) => None,
        NodeErrorKind::CrossingUnverified { .. } => None,
        NodeErrorKind::MeasureRefResolve { error } => Some(resolve_error_tag(error)),
        NodeErrorKind::MeasureRefUnreadable { error, .. } => Some(interrogate_error_tag(error)),
        NodeErrorKind::MeasureNonFinite { source } => Some(eval_error_tag(source)),
        NodeErrorKind::MeasureNotParallel { .. } => None,
        // The unsupported pair names two carrier CLASSES as text the
        // kernel mints; neither is an arm of an enum this file can
        // match, so the pair stays in the prose it is already in.
        NodeErrorKind::MeasureUnsupported(_) => None,
        NodeErrorKind::MeasureMalformed(inner) => Some(measure_node_fault_tag(inner)),
        NodeErrorKind::PayloadExpr { source, .. } => Some(eval_error_tag(source)),
        NodeErrorKind::MeasureSelectionKind { .. } => None,
        // The clearance engine's class name is a `&str` the engine
        // mints behind a feature boundary, not a discriminant this
        // crate can match; it is already the whole of the message.
        NodeErrorKind::MeasureClearanceRefused(_) => None,
        NodeErrorKind::AssertionDimension { .. } => None,
    }
}

/// The stable tag for the ARM of the refusal an edit error carries —
/// `EditError.inner_variant`, beside [`edit_error_tag`]'s `variant`.
///
/// [`node_inner_kind_tag`]'s rule at the other carrier, and the same
/// `None`s: an arm with no inner refusal, and `Roots`, whose word is
/// already the fault's through [`root_fault_tag`].
pub fn edit_inner_variant_tag(err: &EditError) -> Option<&'static str> {
    match err {
        EditError::ProfileProgramRefused { refusal, .. } => Some(program_refusal_tag(refusal)),
        EditError::MeasureMalformed { fault, .. } => Some(measure_node_fault_tag(fault)),
        EditError::Dimension(inner) => Some(expr_dimension_error_tag(inner)),
        EditError::InvalidDistribution { fault, .. } => Some(distribution_fault_tag(fault)),
        // The direction door's refusal is a whole `NodeErrorKind`, so
        // its arm is the same vocabulary `EvaluationError.kind` speaks.
        EditError::PlacementAxis { error } => Some(node_error_tag(error.kind())),
        // The metadata arm's refusal is a SHAPE refusal, so its word
        // says which of the three ways the D7 producer convention was
        // broken rather than which door broke it.
        EditError::MetaUnversioned { error, .. } => Some(meta_version_error_tag(error)),
        EditError::Roots(_) => None,
        EditError::UnknownNode { .. } => None,
        EditError::UnresolvedInput { .. } => None,
        EditError::WouldCycle { .. } => None,
        EditError::DuplicateInput { .. } => None,
        EditError::RepeatedDesignation { .. } => None,
        EditError::SetMembersOnNonList { .. } => None,
        EditError::TooFewMembers { .. } => None,
        EditError::DeleteWouldDangle { .. } => None,
        EditError::UnknownSlot { .. } => None,
        EditError::SlotDimensionMismatch { .. } => None,
        EditError::StructuralSlotNeedsStructuralEdit { .. } => None,
        EditError::NotStructuralSlot { .. } => None,
        EditError::UnknownDocParam { .. } => None,
        EditError::UnknownPayloadParam { .. } => None,
        EditError::PayloadParamDimensionMismatch { .. } => None,
        EditError::AssertionTarget { .. } => None,
        EditError::DeclareInputNotDeclare { .. } => None,
        EditError::AssertionDimension { .. } => None,
        EditError::DocParamDimensionMismatch { .. } => None,
        EditError::ContinuousParamCannotBeCount { .. } => None,
        EditError::DocParamNotDeclared { .. } => None,
        EditError::DocParamValueKindMismatch { .. } => None,
        EditError::PathOffTree { .. } => None,
        EditError::DeclareNamesMissingNode { .. } => None,
        EditError::ReadSiteMissingNode { .. } => None,
        EditError::NonFiniteDocParam { .. } => None,
        EditError::RebindTargetMissingNode { .. } => None,
        EditError::RebindUnknownName { .. } => None,
        EditError::RebindKindMismatch { .. } => None,
        EditError::RebindIdentity { .. } => None,
        EditError::RebindNoReferences { .. } => None,
        EditError::WitnessOnNonSketch { .. } => None,
        EditError::DuplicateWitnessEntry { .. } => None,
        EditError::EmptyWitnessBulk => None,
        EditError::NameUnresolvedInEvaluation { .. } => None,
        EditError::RebindAppearanceCollision { .. } => None,
        EditError::AppearanceWrongKind { .. } => None,
        EditError::AppearanceNamesMissingNode { .. } => None,
        EditError::AppearanceNotSet { .. } => None,
        EditError::InvalidTolerance { .. } => None,
        EditError::MetaNonFinite { .. } => None,
        EditError::MetaNotSet { .. } => None,
        EditError::RebindMetadataCollision { .. } => None,
        EditError::PlacementOnNonInstance { .. } => None,
        EditError::PlacementRuleMismatch { .. } => None,
        EditError::EmptyPlacementList { .. } => None,
        EditError::ImproperPlacement { .. } => None,
        EditError::NonFinitePlacement { .. } => None,
        EditError::UpdateOnNonInstance { .. } => None,
        EditError::PinUnchanged { .. } => None,
        EditError::NonFiniteAlignment { .. } => None,
    }
}

/// The stable tag for a stored metadata value that breaks the D7
/// producer convention — the inner arm of
/// [`EditError::MetaUnversioned`].
///
/// The convention is structural: a map carrying an integer `"v"`
/// field. Its three refusals are three different repairs — wrap the
/// value in a map, add the version, or make the version an integer —
/// and the carrier's own word says only that the convention was
/// broken.
pub fn meta_version_error_tag(err: &MetaVersionError) -> &'static str {
    match err {
        MetaVersionError::NotAMap => "not_a_map",
        MetaVersionError::MissingVersion => "missing_version",
        MetaVersionError::VersionNotInt => "version_not_int",
    }
}

/// The stable tag for a PROFILE validation refusal — the inner arm of
/// [`NodeErrorKind::Profile`], and of a loft's section validation.
///
/// One level, like every map in this group: `band` and `structure`
/// name the arm that refused, and the band's field or the decision it
/// could not honour is a rung further down that this word does not
/// reach for.
pub fn profile_error_tag(err: &ProfileError) -> &'static str {
    match err {
        ProfileError::Band(_) => "band",
        ProfileError::EmptyProfile => "empty_profile",
        ProfileError::TooFewVertices { .. } => "too_few_vertices",
        ProfileError::DegenerateSegment(_) => "degenerate_segment",
        ProfileError::NearFullArc(_) => "near_full_arc",
        ProfileError::NonSimple { .. } => "non_simple",
        ProfileError::TangentialContact { .. } => "tangential_contact",
        ProfileError::TangentJointOutOfRange { .. } => "tangent_joint_out_of_range",
        ProfileError::UndeclaredTangency { .. } => "undeclared_tangency",
        ProfileError::TangencyContradicted { .. } => "tangency_contradicted",
        ProfileError::SliverLoop { .. } => "sliver_loop",
        ProfileError::MultipleOuterLoops { .. } => "multiple_outer_loops",
        ProfileError::NestingTooDeep { .. } => "nesting_too_deep",
        ProfileError::RayCastingExhausted { .. } => "ray_casting_exhausted",
        ProfileError::Escalated { .. } => "escalated",
        ProfileError::Structure(_) => "structure",
    }
}

/// The stable tag for a profile program's REPLAY refusal — the two
/// classes PROFILES-V2 §V1 deliberately keeps apart.
///
/// Keyed off `ReplayError::kind`, the [`path_error_tag`] treatment:
/// the step index is the payload's and rides in the prose, the class
/// is the discriminant. `path` says the chain is well-typed and the
/// geometry refused; the geometry's own word is
/// [`path_error_tag`]'s, one level down.
pub fn replay_error_tag(kind: &ReplayErrorKind<f64>) -> &'static str {
    match kind {
        ReplayErrorKind::Transition { .. } => "transition",
        ReplayErrorKind::Path(_) => "path",
    }
}

/// The stable tag for a guided pass's refusal to honour a consumed
/// decision — `StructureRefusal::kind`.
///
/// Two arms and they are two different situations: the decision could
/// not be DECIDED at this binding (narrow the parameter box), or it
/// was decided the other way (this binding leaves the elaborated
/// structure).
pub fn structure_refusal_tag(kind: &StructureRefusalKind) -> &'static str {
    match kind {
        StructureRefusalKind::Indeterminate(_) => "indeterminate",
        StructureRefusalKind::Flipped { .. } => "flipped",
    }
}

/// The stable tag for the EXTRUDE op's refusal — the inner arm of
/// [`NodeErrorKind::Extrude`].
pub fn extrude_error_tag(err: &ExtrudeError) -> &'static str {
    match err {
        ExtrudeError::Band(_) => "band",
        ExtrudeError::DegenerateExtrusion => "degenerate_extrusion",
        ExtrudeError::ObliqueExtrusion => "oblique_extrusion",
        ExtrudeError::ExtrusionEscalated { .. } => "extrusion_escalated",
        ExtrudeError::CosurfaceEscalated { .. } => "cosurface_escalated",
        ExtrudeError::SliverJoin { .. } => "sliver_join",
        ExtrudeError::SliverRim { .. } => "sliver_rim",
        ExtrudeError::CapPlane { .. } => "cap_plane",
        ExtrudeError::SidePlane { .. } => "side_plane",
        ExtrudeError::Op { .. } => "op",
    }
}

/// The stable tag for the REVOLVE op's refusal — the inner arm of
/// [`NodeErrorKind::Revolve`], and the one the tube's own `revolve`
/// arm names one level down.
pub fn revolve_error_tag(err: &RevolveError) -> &'static str {
    match err {
        RevolveError::Band(_) => "band",
        RevolveError::DegenerateAxis => "degenerate_axis",
        RevolveError::AxisEscalated { .. } => "axis_escalated",
        RevolveError::DegenerateAngle => "degenerate_angle",
        RevolveError::FullRangeAngle => "full_range_angle",
        RevolveError::AngleEscalated { .. } => "angle_escalated",
        RevolveError::VertexCrossesAxis { .. } => "vertex_crosses_axis",
        RevolveError::SliverRadius { .. } => "sliver_radius",
        RevolveError::ArcCrossesAxis { .. } => "arc_crosses_axis",
        RevolveError::SliverAxisClearance { .. } => "sliver_axis_clearance",
        RevolveError::UnsupportedToroid { .. } => "unsupported_toroid",
        RevolveError::NonManifoldAxisContact { .. } => "non_manifold_axis_contact",
        RevolveError::MultipleAxisRuns { .. } => "multiple_axis_runs",
        RevolveError::HoleTouchesAxis { .. } => "hole_touches_axis",
        RevolveError::VoidInsertion { .. } => "void_insertion",
        RevolveError::CosurfaceEscalated { .. } => "cosurface_escalated",
        RevolveError::SliverJoin { .. } => "sliver_join",
        RevolveError::SliverRim { .. } => "sliver_rim",
        RevolveError::CapPlane { .. } => "cap_plane",
        RevolveError::Op { .. } => "op",
        RevolveError::Pcurve(_) => "pcurve",
    }
}

/// The stable tag for a TUBE door's refusal — the inner arm of
/// [`NodeErrorKind::Tube`].
///
/// The three wall arms are reachable only through the hollow door, so
/// this word is also which door refused, without the node carrying a
/// second discriminant for it.
pub fn tube_error_tag(err: &TubeError) -> &'static str {
    match err {
        TubeError::Band(_) => "band",
        TubeError::NonUnitAxis => "non_unit_axis",
        TubeError::NonUnitURef => "non_unit_u_ref",
        TubeError::FrameNotOrthogonal => "frame_not_orthogonal",
        TubeError::DegenerateWindow => "degenerate_window",
        TubeError::FullRangeWindow => "full_range_window",
        TubeError::NonpositiveWall { .. } => "nonpositive_wall",
        TubeError::WallExceedsRadius { .. } => "wall_exceeds_radius",
        TubeError::WallGapCollapsed { .. } => "wall_gap_collapsed",
        TubeError::Escalated { .. } => "escalated",
        TubeError::Revolve(_) => "revolve",
    }
}

/// The stable tag for the SPLIT op's refusal — the inner arm of
/// [`NodeErrorKind::Split`], which carries the kernel's
/// `topo::splitting::SplitError`.
///
/// Not [`split_error_tag`], which is the document layer's split
/// REFACTORING (`Doc.split`): two different types, two different
/// doors, and this one is the plane-through-a-body operation.
pub fn split_op_error_tag(err: &SplitOpError) -> &'static str {
    match err {
        SplitOpError::Reduce(_) => "reduce",
        SplitOpError::Join(_) => "join",
        SplitOpError::Finish(_) => "finish",
        SplitOpError::Pcurves(_) => "pcurves",
    }
}

/// The stable tag for a BLEND op's refusal — the inner arm of
/// [`NodeErrorKind::Blend`], whose own word is already the verb
/// (`fillet` or `chamfer`), so this one is what the verb refused
/// about.
pub fn blend_error_tag(err: &BlendError) -> &'static str {
    match err {
        BlendError::Band(_) => "band",
        BlendError::ChainNotConnected { .. } => "chain_not_connected",
        BlendError::RadiusHeadroom { .. } => "radius_headroom",
        BlendError::FaceClearanceUncertified { .. } => "face_clearance_uncertified",
        BlendError::TangentialEdge { .. } => "tangential_edge",
        BlendError::SpineIrregular { .. } => "spine_irregular",
        BlendError::ChainNotG1 { .. } => "chain_not_g1",
        BlendError::ConvexitySignFlip { .. } => "convexity_sign_flip",
        BlendError::UnsupportedCorner { .. } => "unsupported_corner",
        BlendError::SpineUnsupported { .. } => "spine_unsupported",
        BlendError::ChamferArmUnsupported { .. } => "chamfer_arm_unsupported",
        BlendError::Escalated { .. } => "escalated",
        BlendError::RepeatedEdge { .. } => "repeated_edge",
        BlendError::NonpositiveSize { .. } => "nonpositive_size",
        BlendError::UnsupportedBody { .. } => "unsupported_body",
        BlendError::UnsupportedChain { .. } => "unsupported_chain",
        BlendError::UnsupportedRunOut { .. } => "unsupported_run_out",
        BlendError::UnsupportedGeometry { .. } => "unsupported_geometry",
        BlendError::BodyNotIntact { .. } => "body_not_intact",
        BlendError::SurgeryInvariant { .. } => "surgery_invariant",
        BlendError::RingClearance { .. } => "ring_clearance",
        BlendError::Certify { .. } => "certify",
        BlendError::Op { .. } => "op",
    }
}

/// The stable tag for the BOOLEAN op's refusal — the inner arm of
/// [`NodeErrorKind::Boolean`].
///
/// `undeclared_coincidence` is here and is NOT the refusal the
/// detect/declare protocol raises: the document layer lifts that one
/// to its own `undeclared_contact` carrier word with the candidate
/// declaration attached, and this arm is what survives when a key
/// fails to resolve to a name.
pub fn boolean_error_tag(err: &BooleanError) -> &'static str {
    match err {
        BooleanError::Band(_) => "band",
        BooleanError::CurvedBooleanUnsupported { .. } => "curved_boolean_unsupported",
        BooleanError::CurvedSectorSideUnsupported { .. } => "curved_sector_side_unsupported",
        BooleanError::CurvedPierceUnsupported { .. } => "curved_pierce_unsupported",
        BooleanError::CurvedEdgeUnsupported { .. } => "curved_edge_unsupported",
        BooleanError::PointSplitCarrierUnsupported { .. } => "point_split_carrier_unsupported",
        BooleanError::ArcLoopContainmentUnsupported { .. } => "arc_loop_containment_unsupported",
        BooleanError::ScaffoldingOperand { .. } => "scaffolding_operand",
        BooleanError::NonMaximalFaces { .. } => "non_maximal_faces",
        BooleanError::Escalated { .. } => "escalated",
        BooleanError::UndeclaredCoincidence { .. } => "undeclared_coincidence",
        BooleanError::DeclarationContradicted { .. } => "declaration_contradicted",
        BooleanError::ContactContradicted { .. } => "contact_contradicted",
        BooleanError::UnsupportedDeclarationClass { .. } => "unsupported_declaration_class",
        BooleanError::RimSeamNotDeclarable { .. } => "rim_seam_not_declarable",
        BooleanError::RimCuspArmUnbuilt { .. } => "rim_cusp_arm_unbuilt",
        BooleanError::InvalidDeclaration { .. } => "invalid_declaration",
        BooleanError::PairingMismatch { .. } => "pairing_mismatch",
        BooleanError::ClassificationInvariant { .. } => "classification_invariant",
        BooleanError::CorruptOperand { .. } => "corrupt_operand",
        BooleanError::CrossingInsertion { .. } => "crossing_insertion",
        BooleanError::CurvedPairUnsupported { .. } => "curved_pair_unsupported",
        BooleanError::NurbsExtentUnsupported { .. } => "nurbs_extent_unsupported",
        BooleanError::FallbackExtentUnsupported { .. } => "fallback_extent_unsupported",
        BooleanError::GermFrameUnsupported { .. } => "germ_frame_unsupported",
        BooleanError::GermFrameCylinderPinch { .. } => "germ_frame_cylinder_pinch",
        BooleanError::Euler(_) => "euler",
        BooleanError::Pcurves { .. } => "pcurves",
        BooleanError::Join(_) => "join",
        BooleanError::RestZipUnsupported { .. } => "rest_zip_unsupported",
        BooleanError::JoinDesync { .. } => "join_desync",
        BooleanError::TornComponent { .. } => "torn_component",
        BooleanError::Containment(_) => "containment",
        BooleanError::Revert(_) => "revert",
        BooleanError::SeamOrientation { .. } => "seam_orientation",
        BooleanError::ZipCorrespondence { .. } => "zip_correspondence",
        BooleanError::Merge(_) => "merge",
        BooleanError::ResultInvalid { .. } => "result_invalid",
        BooleanError::ResultVolumeImplausible { .. } => "result_volume_implausible",
        BooleanError::UnrepresentableResult => "unrepresentable_result",
        BooleanError::GraftRecertify(_) => "graft_recertify",
    }
}

/// The stable tag for the rigid-TRANSFORM op's refusal — the inner
/// arm of [`NodeErrorKind::Transform`].
pub fn transform_error_tag(err: &TransformError) -> &'static str {
    match err {
        TransformError::Pcurve { .. } => "pcurve",
        TransformError::Band(_) => "band",
        TransformError::Certify { .. } => "certify",
        TransformError::NotRigid { .. } => "not_rigid",
        TransformError::NonFiniteMap { .. } => "non_finite_map",
        TransformError::NullScaffold { .. } => "null_scaffold",
        TransformError::NurbsPlaceholder => "nurbs_placeholder",
        TransformError::ApproxLaneUnsupported { .. } => "approx_lane_unsupported",
        TransformError::ApproxRecertify { .. } => "approx_recertify",
        TransformError::Corrupt { .. } => "corrupt",
    }
}

/// The stable tag for the SKIN construction's refusal — the inner arm
/// of [`NodeErrorKind::Skin`], and the one a loft's `skin` arm names
/// one level down.
pub fn skin_error_tag(err: &SkinError) -> &'static str {
    match err {
        SkinError::TooFewSections { .. } => "too_few_sections",
        SkinError::SectionShapeMismatch { .. } => "section_shape_mismatch",
        SkinError::SectionProfile { .. } => "section_profile",
        SkinError::DomainNotUnit { .. } => "domain_not_unit",
        SkinError::DegenerateSection { .. } => "degenerate_section",
        SkinError::BadDegree { .. } => "bad_degree",
        SkinError::PathTangentReversal { .. } => "path_tangent_reversal",
        SkinError::Fit(_) => "fit",
        SkinError::KnotAlgebra(_) => "knot_algebra",
        SkinError::Structure(_) => "structure",
    }
}

/// The stable tag for the LOFT body assembly's refusal — the inner
/// arm of [`NodeErrorKind::Loft`].
pub fn loft_error_tag(err: &LoftError) -> &'static str {
    match err {
        LoftError::Band(_) => "band",
        LoftError::Skin(_) => "skin",
        LoftError::Profile(_) => "profile",
        LoftError::Euler(_) => "euler",
        LoftError::CapPlane(_) => "cap_plane",
        LoftError::Pcurve(_) => "pcurve",
        LoftError::SeamStructure { .. } => "seam_structure",
        LoftError::SectionStructure => "section_structure",
        LoftError::ReversedStacking => "reversed_stacking",
        LoftError::DegenerateStacking => "degenerate_stacking",
        LoftError::StackingEscalated { .. } => "stacking_escalated",
    }
}

/// The stable tag for a classification band that could not be formed
/// — the inner arm of [`NodeErrorKind::Band`], and the arm a dozen
/// op refusals carry under their own `band` word one level down.
pub fn band_error_tag(err: &BandError) -> &'static str {
    match err {
        BandError::InvalidValue { .. } => "invalid_value",
        BandError::InvalidLeverArm { .. } => "invalid_lever_arm",
        BandError::Empty { .. } => "empty",
    }
}

/// The stable tag for a name-EMISSION refusal — the inner arm of
/// [`NodeErrorKind::Naming`].
pub fn naming_error_tag(err: &NamingError) -> &'static str {
    match err {
        NamingError::Duplicate { .. } => "duplicate",
        NamingError::Unnamed { .. } => "unnamed",
        NamingError::MissingUpstream { .. } => "missing_upstream",
        NamingError::Emission { .. } => "emission",
        NamingError::Escalated { .. } => "escalated",
    }
}

/// The stable tag for a lowered parameter-identity attach refusal —
/// the inner arm of [`NodeErrorKind::ParamSourceAttach`].
pub fn param_attach_error_tag(err: &ParamAttachError) -> &'static str {
    match err {
        ParamAttachError::StaleKey => "stale_key",
        ParamAttachError::FieldNotOnKind { .. } => "field_not_on_kind",
    }
}

/// The stable tag for the SHELL op's refusal — the inner arm of
/// [`NodeErrorKind::Shell`], which the node carries at its `f64`
/// witness.
pub fn shell_error_tag(err: &ShellError<f64>) -> &'static str {
    match err {
        ShellError::Band { .. } => "band",
        ShellError::Thickness { .. } => "thickness",
        ShellError::NoSolid => "no_solid",
        ShellError::Roles { .. } => "roles",
        ShellError::OperandOuterShells { .. } => "operand_outer_shells",
        ShellError::Partition { .. } => "partition",
        ShellError::WallClearance { .. } => "wall_clearance",
        ShellError::ChartSpansSolids { .. } => "chart_spans_solids",
        ShellError::ChartSenseMixed { .. } => "chart_sense_mixed",
        ShellError::Face { .. } => "face",
        ShellError::OpenFaceStale { .. } => "open_face_stale",
        ShellError::OpenFaceRepeated { .. } => "open_face_repeated",
        ShellError::OpenFacesExhaustShell { .. } => "open_faces_exhaust_shell",
        ShellError::OpenFacesDisconnect { .. } => "open_faces_disconnect",
        ShellError::OpenFaceRingUnsupported { .. } => "open_face_ring_unsupported",
        ShellError::OpenFaceChartPartial { .. } => "open_face_chart_partial",
        ShellError::Lift { .. } => "lift",
        ShellError::Insert { .. } => "insert",
        ShellError::OpenFaceRimNotExpressible { .. } => "open_face_rim_not_expressible",
        ShellError::Rim { .. } => "rim",
        ShellError::Escalated { .. } => "escalated",
        ShellError::Corrupt { .. } => "corrupt",
        ShellError::Pcurve { .. } => "pcurve",
        ShellError::NotValid { .. } => "not_valid",
    }
}

/// The stable tag for a profile PROGRAM's refusal at an edit —  the
/// inner arm of `EditError::ProfileProgramRefused`.
pub fn program_refusal_tag(err: &ProgramRefusal) -> &'static str {
    match err {
        ProgramRefusal::Resolve { .. } => "resolve",
        ProgramRefusal::Transition { .. } => "transition",
        ProgramRefusal::Geometry { .. } => "geometry",
        ProgramRefusal::Validate(_) => "validate",
    }
}

/// The stable tag for a parameter box that could not bind an
/// environment — the inner arm of [`NodeErrorKind::ParamBox`].
pub fn param_box_error_tag(err: &ParamBoxError) -> &'static str {
    match err {
        ParamBoxError::UnknownParam { .. } => "unknown_param",
        ParamBoxError::AxisUnrepresentable { .. } => "axis_unrepresentable",
    }
}

/// The stable tag for an E4 seed that could not bind — the inner arm
/// of [`NodeErrorKind::Seed`].
pub fn seed_error_tag(err: &SeedError) -> &'static str {
    match err {
        SeedError::UnknownParam { .. } => "unknown_param",
        SeedError::CountParam { .. } => "count_param",
        SeedError::TangentUnrepresentable { .. } => "tangent_unrepresentable",
    }
}
/// The stable tag for a mate-solve refusal. Each arm is
/// a different recourse: add the complementary mate, delete one of the
/// clashing pair, rebind the stranded head, author the missing
/// primitive, or move the geometry out of the band.
pub fn mate_fault_tag(fault: &MateFault) -> &'static str {
    match fault {
        MateFault::PosesOfAnotherDocument { .. } => "mate_poses_of_another_document",
        MateFault::Frame { .. } => "mate_frame_degenerate",
        MateFault::ClassNotAdmitted { .. } => "mate_class_not_admitted",
        MateFault::TableLacks { .. } => "mate_table_lacks",
        MateFault::Indeterminate { .. } => "mate_indeterminate",
        MateFault::Band { .. } => "mate_band",
        MateFault::Contradictory { .. } => "mate_contradictory",
        MateFault::Under { .. } => "mate_under",
        MateFault::DanglingHead { .. } => "mate_dangling_head",
        MateFault::PlacerRefused { .. } => "mate_placer_refused",
        MateFault::PartSelectsAnotherCopy { .. } => "mate_part_selects_another_copy",
        MateFault::SelfMate { .. } => "mate_self",
        MateFault::Unleverable { .. } => "mate_datum_too_small_to_lever",
    }
}

/// The stable tag for a declare-sugar refusal (the
/// `Doc.declare`/`Doc.declare_all` doors over
/// `editor_core::declare_all`). The `Edit` arm carries the document
/// layer's own tag through rather than flattening it.
pub fn declare_error_tag(err: &pncad::select::DeclareError) -> &'static str {
    use pncad::select::DeclareError as E;
    match err {
        E::NoFindings => "no_findings",
        E::Edit(inner) => edit_error_tag(inner),
        E::NoMintedId => "no_minted_id",
    }
}

/// The stable tag for a document-seam resolution failure — the
/// vocabulary EVERY door that crosses the seam speaks.
///
/// Two doors cross it: evaluation, through [`part_fault_tag`]'s
/// `Unresolved` arm, and `inline`, which resolves the referenced
/// document in order to splice it. A stale pin is the same fact at
/// both, so it carries the same tag at both; the `part_` prefix names
/// the SEAM, not evaluation.
pub fn resolve_fault_tag(fault: &pncad::document::ResolveFault) -> &'static str {
    use pncad::document::ResolveFault as R;
    match fault {
        R::PinMismatch => "part_pin_mismatch",
        R::EpsilonSeam => "part_epsilon_seam",
        R::Unresolved => "part_unresolved",
    }
}

/// The stable tag for an instantiation refusal.
pub fn part_fault_tag(fault: &pncad::document::PartFault) -> &'static str {
    use pncad::document::PartFault as F;
    match fault {
        F::NoResolver => "part_no_resolver",
        F::Unresolved { fault, .. } => resolve_fault_tag(fault),
        F::PartRootFailed { .. } => "part_root_failed",
        F::PartProduct { .. } => "part_product",
        F::ReferenceCycle { .. } => "part_reference_cycle",
        F::DepthExceeded => "part_depth_exceeded",
    }
}

/// The stable tag for a PROFILE-PROGRAM structure fault — the inner
/// arm of [`PersistError::ProfileProgram`].
///
/// The fault's own payload (the slot, the two dimensions, the loop and
/// step counters) is the profile layer's surface and stays in the
/// message; what crosses here is the word a caller branches on.
pub fn program_fault_tag(fault: &ProgramFault) -> &'static str {
    match fault {
        ProgramFault::SlotDimension { .. } => "slot_dimension",
        ProgramFault::Lattice { .. } => "lattice",
    }
}

/// The stable tag for a document-snapshot invariant refusal — the
/// inner arm of [`PersistError::Snapshot`].
///
/// Nineteen arms, each naming a different invariant the parsed (or
/// in-memory) snapshot broke. The arm's own payload is node ids,
/// names and counts the snapshot door owns; the word is what the
/// persistence door carries out.
pub fn snapshot_error_tag(err: &SnapshotError) -> &'static str {
    match err {
        SnapshotError::OrderMismatch => "order_mismatch",
        SnapshotError::BlendSelectionNotCanonical { .. } => "blend_selection_not_canonical",
        SnapshotError::IdBeyondCounter { .. } => "id_beyond_counter",
        SnapshotError::DanglingInput { .. } => "dangling_input",
        SnapshotError::ForwardInput { .. } => "forward_input",
        SnapshotError::DeclareInput { .. } => "declare_input",
        SnapshotError::WitnessSite { .. } => "witness_site",
        SnapshotError::CountContinuous { .. } => "count_continuous",
        SnapshotError::EpsilonInvalid { .. } => "epsilon_invalid",
        // The product-root list's own invariant vocabulary, carried
        // through: a root fault is the same fact here as at the edit
        // door, so it keeps the tag it has there.
        SnapshotError::Roots(fault) => root_fault_tag(fault),
        SnapshotError::PlacementSite { .. } => "placement_site",
        SnapshotError::PlacementFrame { .. } => "placement_frame",
        SnapshotError::PlacementNotGauge { .. } => "placement_not_gauge",
        SnapshotError::MateAlignment { .. } => "mate_alignment",
        SnapshotError::PlacementRule { .. } => "placement_rule",
        SnapshotError::MeasureRefs { .. } => "measure_refs",
        SnapshotError::InputList { .. } => "input_list",
        SnapshotError::AssertionBound { .. } => "assertion_bound",
        SnapshotError::MetadataUnversioned { .. } => "metadata_unversioned",
    }
}

/// The stable tag for a persistence refusal (the v4
/// doors). `PersistError` DOES implement `Display`, so unlike the two
/// above the human message is real prose — the tag is still the
/// machine payload a caller branches on.
pub fn persist_error_tag(err: &PersistError) -> &'static str {
    match err {
        PersistError::NonFinite { .. } => "non_finite",
        PersistError::Distribution { .. } => "distribution",
        PersistError::DisplayUnit { .. } => "display_unit",
        PersistError::ProfileProgram { .. } => "profile_program",
        PersistError::Serialize { .. } => "serialize",
        PersistError::HeaderId { .. } => "header_id",
        PersistError::IdMismatch { .. } => "id_mismatch",
        PersistError::Parse { .. } => "parse",
        PersistError::Unreadable { .. } => "unreadable",
        PersistError::Snapshot(_) => "snapshot",
        PersistError::EditReplay { .. } => "edit_replay",
        PersistError::ToleranceConflict { .. } => "tolerance_conflict",
        PersistError::ToleranceInvalid { .. } => "tolerance_invalid",
    }
}

/// The stable tag for a WORKSPACE refusal.
///
/// `WorkspaceError` implements `Display`, so the human message is the
/// store's own prose and this is the branchable discriminant — the
/// [`persist_error_tag`] treatment.
///
/// The four wrapping arms keep their own tag rather than carrying the
/// inner [`PersistError`]'s through: the STAGE is the discriminant a
/// caller branches on (a file whose header refused is a different
/// situation from one whose body did), and it is what would be lost
/// by flattening. A caller wanting the inner refusal reads it from
/// the message, exactly as before.
///
/// Exhaustive, per this module's rule, and here that rule is doing
/// real work: only one door raises a `WorkspaceError` into Python
/// today, and its message would be perfectly true under any label —
/// so a mislabelled variant is invisible from Python and invisible in
/// CI. The map is what makes the label a fact about the value instead
/// of a fact about which door happened to raise it.
pub fn workspace_error_tag(err: &WorkspaceError) -> &'static str {
    match err {
        WorkspaceError::Io { .. } => "io",
        WorkspaceError::DuplicateId { .. } => "duplicate_id",
        WorkspaceError::Header { .. } => "header",
        WorkspaceError::UnknownId { .. } => "unknown_id",
        WorkspaceError::Load { .. } => "load",
        WorkspaceError::Pin { .. } => "pin",
        WorkspaceError::PinMismatch { .. } => "pin_mismatch",
        WorkspaceError::Save { .. } => "save",
        WorkspaceError::SaveWouldDuplicateId { .. } => "save_would_duplicate_id",
        WorkspaceError::SaveTargetNotInStore { .. } => "save_target_not_in_store",
        WorkspaceError::RandomnessUnavailable { .. } => "randomness_unavailable",
        WorkspaceError::Update { .. } => "update",
    }
}

/// The stable tag for a STEP IMPORT refusal.
///
/// `StepImportError` implements `Display`, so the human message is the
/// importer's own prose naming the entity id and line; this is the
/// branchable discriminant. Twenty-two arms. Twenty-one are reachable
/// through `import_step` on some input, unlike
/// [`workspace_error_tag`]'s door — a caller distinguishing a
/// malformed file from an unsupported entity from a tier refusal has
/// no other way to do it, because the id and line live in prose. The
/// twenty-second, `vertex_without_point`, announces a corrupt-body
/// state whose reachability the declaration resolver cannot prove
/// either way; it exists so that resolver refuses rather than
/// miscounts.
///
/// The nested arms keep their own tag rather than carrying the inner
/// refusal's through: what the caller branches on is which STAGE of
/// the import refused, and the inner error is in the message.
pub fn step_import_error_tag(err: &StepImportError) -> &'static str {
    match err {
        StepImportError::Syntax { .. } => "syntax",
        StepImportError::DanglingReference { .. } => "dangling_reference",
        StepImportError::WrongEntityType { .. } => "wrong_entity_type",
        StepImportError::MalformedRecord { .. } => "malformed_record",
        StepImportError::UnsupportedEntity { .. } => "unsupported_entity",
        StepImportError::UnsupportedUnit { .. } => "unsupported_unit",
        StepImportError::NothingToImport => "nothing_to_import",
        StepImportError::Structure { .. } => "structure",
        StepImportError::MissingUncertainty => "missing_uncertainty",
        StepImportError::InvalidEpsOverride { .. } => "invalid_eps_override",
        StepImportError::DeclarationUnresolved { .. } => "declaration_unresolved",
        StepImportError::VertexWithoutPoint { .. } => "vertex_without_point",
        StepImportError::MalformedReal { .. } => "malformed_real",
        StepImportError::Topology { .. } => "topology",
        StepImportError::Assembly { .. } => "assembly",
        StepImportError::Adoption { .. } => "adoption",
        StepImportError::RimOffWallBoundary { .. } => "rim_off_wall_boundary",
        StepImportError::RecognitionAmbiguous { .. } => "recognition_ambiguous",
        StepImportError::Pcurves { .. } => "pcurves",
        StepImportError::Placement { .. } => "placement",
        StepImportError::Instance { .. } => "instance",
        StepImportError::TierInvalid { .. } => "tier_invalid",
    }
}

/// The stable tag for the analytic kind a stage-1 recognition
/// estimator DECLINED on — what `StepImportError::RecognitionAmbiguous`
/// carries.
///
/// The carrier's arm does not forward to this one and that is the
/// decision, the `mesh_index` shape: `recognition_ambiguous` names
/// the CONDITION — no answer exists at the interpretation budget —
/// and a caller branching on the import's refusal ladder needs that
/// word to stay put. Which kind's estimator declined is a second
/// question, answered beside the tag rather than in place of it,
/// because the two lead different places: a plane that will not
/// certify is a flatness question at ε_in, a cylinder that will not is
/// an ill-conditioned axis and wants more of the patch.
///
/// The match is exhaustive, so a third promotable kind recognised
/// kernel-side stops this crate compiling instead of arriving under
/// one of these two words. The face and surface entity ids and the
/// conditioning margin stay in the kernel's own `Display`, which is
/// where they already were.
pub fn promoted_kind_tag(kind: &PromotedKind) -> &'static str {
    match kind {
        PromotedKind::Plane => "plane",
        PromotedKind::Cylinder => "cylinder",
    }
}

/// The stable tag for a document-layer export refusal
/// (`pncad::export::step_for_node`'s error).
pub fn export_error_tag(err: &pncad::export::ExportError) -> &'static str {
    use pncad::export::ExportError as E;
    match err {
        E::UnknownNode { .. } => "unknown_node",
        E::NodeFailed { .. } => "node_failed",
        E::Poisoned { .. } => "poisoned",
        E::NotABody { .. } => "not_a_body",
        E::EmptyBoolean { .. } => "empty_boolean",
        E::Step(_) => "step_refused",
        E::Product(inner) => product_error_tag(inner),
    }
}

/// The stable tag for a whole-document product refusal
/// (`editor_core::product`'s error).
pub fn product_error_tag(err: &pncad::document::ProductError) -> &'static str {
    use pncad::document::ProductError as E;
    match err {
        E::EvaluationOfAnotherDocument { .. } => "evaluation_of_another_document",
        E::UnknownNode { .. } => "unknown_node",
        E::RootFailed { .. } => "root_failed",
        E::RootPoisoned { .. } => "root_poisoned",
        E::NoBodyRoots => "no_body_roots",
        E::Graft { .. } => "graft_refused",
        E::SolidInvalid { .. } => "solid_invalid",
        E::ProductInvalid { .. } => "product_invalid",
        E::Naming { .. } => "product_naming",
        E::ContactLineage { .. } => "contact_lineage",
    }
}

/// The stable tag for an expression-constructor refusal
/// (`Expr::literal`'s own error type, matched rather than
/// pre-checked).
pub fn expr_dimension_error_tag(err: &DimensionError) -> &'static str {
    match err {
        DimensionError::Mismatch { .. } => "mismatch",
        DimensionError::MulNeedsScalar { .. } => "mul_needs_scalar",
        DimensionError::DivNeedsScalarDivisor { .. } => "div_needs_scalar_divisor",
        DimensionError::TrigNeedsAngle { .. } => "trig_needs_angle",
        DimensionError::CountNeedsExplicitPromotion { .. } => "count_needs_explicit_promotion",
        DimensionError::NotCount { .. } => "not_count",
        DimensionError::LiteralCountIsInteger => "count_is_integer",
        DimensionError::NonFiniteLiteral => "non_finite",
        DimensionError::DisplayUnitMismatch { .. } => "display_unit_mismatch",
        DimensionError::UnknownDisplayUnit { .. } => "unknown_display_unit",
    }
}

/// The stable tag for a display-formatter refusal (`Length.format` /
/// `Angle.format`).
///
/// One arm today, and the map exists anyway for the reason every
/// other one does: the tag is the branchable interface, and a SECOND
/// arm arriving must break this build rather than reach a caller
/// untagged. A one-arm `match` with no wildcard IS that alarm, at the
/// size this refusal happens to be.
///
/// The tag is `non_finite`, deliberately the same string
/// [`expr_dimension_error_tag`] uses for `NonFiniteLiteral`. Two
/// layers refuse the same fact — a float that is NaN or ±∞ — on
/// opposite doors (one on the way INTO a recipe, one on the way OUT
/// to a human), and a caller branching on the tag is asking what went
/// wrong, not which layer said so; the exception CLASS already
/// answers the second question.
pub fn fmt_quantity_error_tag(err: &FmtQuantityError) -> &'static str {
    match err {
        FmtQuantityError::NonFinite { .. } => "non_finite",
    }
}

/// The stable tag for an expression TEXT-door refusal (`parse_expr`).
///
/// The `Dimension` arm keeps a tag of its OWN rather than borrowing
/// the inner refusal's: what refused is the parse, at a byte offset
/// the inner `DimensionError` does not carry. The inner tag crosses
/// beside it as the exception's `kind`
/// ([`expr_dimension_error_tag`]), so a caller who wants to branch on
/// WHICH reduction was refused still can.
pub fn parse_error_tag(err: &ParseError) -> &'static str {
    match err {
        ParseError::UnexpectedChar { .. } => "unexpected_char",
        ParseError::UnexpectedEnd { .. } => "unexpected_end",
        ParseError::UnexpectedToken { .. } => "unexpected_token",
        ParseError::TrailingInput { .. } => "trailing_input",
        ParseError::MalformedNumber { .. } => "malformed_number",
        ParseError::IntegerOverflow { .. } => "integer_overflow",
        ParseError::UnknownUnit { .. } => "unknown_unit",
        ParseError::UnknownFunction { .. } => "unknown_function",
        ParseError::WrongArity { .. } => "wrong_arity",
        ParseError::UnknownParam { .. } => "unknown_param",
        ParseError::Dimension { .. } => "dimension",
    }
}

/// The stable tag for an evaluation refusal (`eval` / `eval_count`).
///
/// Note what is NOT here: division by zero and out-of-domain trig.
/// The evaluator has no branches to hide a numeric domain behind, so
/// those follow the kernel's poison-value policy through the scalar
/// and reach a caller as `non_finite_result` — on the finished VALUE,
/// at the end of the evaluation, rather than at the operation.
pub fn eval_error_tag(err: &EvalError) -> &'static str {
    match err {
        EvalError::UnknownParam(_) => "unknown_param",
        EvalError::ParamDimensionMismatch { .. } => "param_dimension_mismatch",
        EvalError::CountExprInContinuousEval => "count_expr_in_continuous_eval",
        EvalError::ContinuousExprInCountEval { .. } => "continuous_expr_in_count_eval",
        EvalError::CountOverflow => "count_overflow",
        EvalError::CountToScalarOutOfRange(_) => "count_to_scalar_out_of_range",
        EvalError::NonFiniteResult => "non_finite_result",
    }
}

/// The stable tag for a tessellation refusal.
///
/// Like every other map in this module the tag carries the branchable
/// discriminant beside the kernel's own `Display` prose, which is the
/// human message — the split [`persist_error_tag`] documents.
///
/// The arena keys the arms carry (`FaceKey`, `EdgeKey`) do NOT cross:
/// the whole curation exists to keep them unnameable, so the payload a
/// caller reads is the arms' NUMBERS and prose notes.
pub fn tessellate_error_tag(err: &TessellateError) -> &'static str {
    match err {
        TessellateError::InvalidChordalTolerance { .. } => "invalid_chordal_tolerance",
        TessellateError::UnsupportedSurface { .. } => "unsupported_surface",
        TessellateError::UnsupportedNurbsFace { .. } => "unsupported_nurbs_face",
        TessellateError::UnsupportedCurve { .. } => "unsupported_curve",
        TessellateError::NullScaffoldEdge { .. } => "null_scaffold_edge",
        TessellateError::RingOnCurvedFace { .. } => "ring_on_curved_face",
        TessellateError::EmptyLoop { .. } => "empty_loop",
        TessellateError::MissingEntity { .. } => "missing_entity",
        TessellateError::ResolutionOverflow { .. } => "resolution_overflow",
        TessellateError::CertificateExceeded { .. } => "certificate_exceeded",
        TessellateError::Triangulation { .. } => "triangulation",
        TessellateError::SelfTouchingTrimLoop { .. } => "self_touching_trim_loop",
        TessellateError::UnsupportedCurvedDomain { .. } => "unsupported_curved_domain",
        TessellateError::UnsupportedCurvedShape { .. } => "unsupported_curved_shape",
        TessellateError::Band { .. } => "tolerance_band_unformable",
    }
}

/// The stable tag for an STL writer refusal.
pub fn stl_error_tag(err: &StlError) -> &'static str {
    match err {
        StlError::DegenerateTriangle { .. } => "degenerate_triangle",
        StlError::IndexOutOfRange { .. } => "index_out_of_range",
        StlError::TooManyTriangles { .. } => "too_many_triangles",
        StlError::Io(_) => "io",
    }
}

/// The stable tag for an ASCII solid-name refusal.
///
/// One namespace with [`stl_error_tag`]'s: a Python caller passes the
/// name as a `str` keyword argument, so the newtype's refusal and the
/// writer's arrive on the same exception class and must stay
/// distinguishable. The `solid_name_` prefix is what keeps them so.
pub fn solid_name_error_tag(err: &SolidNameError) -> &'static str {
    match err {
        SolidNameError::Unrepresentable { .. } => "solid_name_unrepresentable",
    }
}

/// The stable tag for a binary-header refusal, in
/// [`stl_error_tag`]'s namespace for the same reason.
pub fn binary_header_error_tag(err: &BinaryHeaderError) -> &'static str {
    match err {
        BinaryHeaderError::TooLong { .. } => "binary_header_too_long",
        BinaryHeaderError::SniffsAscii => "binary_header_sniffs_ascii",
    }
}

/// The stable tag for a mate reference that named no product face
/// (the assembly gate's `Reference` arm rides one).
pub fn refused_ref_tag(why: &RefusedRef) -> &'static str {
    match why {
        RefusedRef::Vanished => "ref_vanished",
        RefusedRef::ReadBelowARoot { .. } => "ref_read_below_a_root",
        RefusedRef::Ambiguous { .. } => "ref_ambiguous",
        RefusedRef::NotAFace { .. } => "ref_not_a_face",
    }
}

/// The stable tag for an at-rest gate refusal.
///
/// The `Product` arm delegates to [`product_error_tag`] rather than
/// collapsing every gather refusal to one tag: a caller branching on
/// "why did my assembly not gather" wants the gather's own answer,
/// and the wrapper adds nothing they can act on. The two namespaces
/// do not collide — the gather's tags are bare (`no_body_roots`), the
/// gate's carry their own words.
pub fn assembly_error_tag(err: &AssemblyError) -> &'static str {
    match err {
        AssemblyError::Product(inner) => product_error_tag(inner),
        AssemblyError::Reference { .. } => "mate_reference_refused",
        AssemblyError::NoAtRestRecord { .. } => "no_at_rest_record",
        AssemblyError::CarriedMintRefusal { .. } => "carried_mint_refusal",
        AssemblyError::AtRest { .. } => "at_rest",
        AssemblyError::Uncertified { .. } => "uncertified",
    }
}

/// **The stable tag for one at-rest attribution** — what a finding
/// says about a declaration it names, and whose declaration it is.
///
/// Four words rather than two, because a caller must be able to tell a
/// refutation from a decline (relabelling the first as the second
/// promotes a verdict against the document into the unrefuted
/// frontier) AND a declaration this document authored from one a part
/// did (`declaration.mate` is then a node of another document, and
/// `of`/`via` are what make that id usable). It lives here, with the
/// refusal tags, so this crate's `TAG_INVENTORY` guards it — that
/// roster reads THIS file — because these words are as much a public
/// Python contract as any tag below.
///
/// Exhaustive over both enums, so a relation or an attribution arm
/// added in the kernel stops this build.
pub fn attribution_tag(attribution: &Attribution) -> &'static str {
    match attribution {
        Attribution::Refuted(_) => "refuted",
        Attribution::Declined(_) => "declined",
        Attribution::Carried {
            relation: Relation::Refuted,
            ..
        } => "carried_refuted",
        Attribution::Carried {
            relation: Relation::Declined,
            ..
        } => "carried_declined",
        Attribution::Unattributed => "unattributed",
    }
}

/// The stable tag for a split refusal.
pub fn split_error_tag(err: &SplitError) -> &'static str {
    match err {
        SplitError::EmptyCut => "empty_cut",
        SplitError::UnknownCutNode { .. } => "unknown_cut_node",
        SplitError::PartIdCollides { .. } => "part_id_collides",
        SplitError::SeveredEdge { .. } => "severed_edge",
        SplitError::OperandSeveredFromMate { .. } => "operand_severed_from_mate",
        SplitError::TornCluster { .. } => "torn_cluster",
        SplitError::UncutParamReference { .. } => "uncut_param_reference",
        SplitError::PartNameReachesRemainder { .. } => "part_name_reaches_remainder",
        SplitError::NameStraddlesCut { .. } => "name_straddles_cut",
        SplitError::BodyNameCrossesCut { .. } => "body_name_crosses_cut",
        SplitError::Pin { .. } => "split_pin",
        SplitError::PartEdit { .. } => "part_edit",
        SplitError::RemainderEdit { .. } => "remainder_edit",
    }
}

/// The stable tag for an inline refusal.
///
/// `Unresolved` delegates to [`part_fault_tag`]'s sibling vocabulary
/// through [`resolve_fault_tag`]: inline crosses the SAME document
/// seam evaluation does, and a stale pin refused here is the stale
/// pin refused there. One vocabulary, so a caller who learned to read
/// `part_pin_mismatch` off an evaluation reads it here too.
pub fn inline_error_tag(err: &InlineError) -> &'static str {
    match err {
        InlineError::UnknownNode { .. } => "unknown_node",
        InlineError::NotAnInstance { .. } => "not_an_instance",
        InlineError::InstanceConsumed { .. } => "instance_consumed",
        InlineError::Unresolved { failure } => resolve_fault_tag(&failure.fault),
        InlineError::EpsilonSeam { .. } => "epsilon_seam",
        InlineError::PartCarriesMetadata { .. } => "part_carries_metadata",
        InlineError::ParamConflict { .. } => "param_conflict",
        InlineError::UnplaceableFrame { .. } => "unplaceable_frame",
        InlineError::InstanceBodyNameReferenced { .. } => "instance_body_name_referenced",
        InlineError::ForeignInstanceName { .. } => "foreign_instance_name",
        InlineError::StrandedPartName { .. } => "stranded_part_name",
        InlineError::Edit { .. } => "inline_edit",
    }
}

/// The stable tag for a whole-document pin update's refusal.
pub fn update_error_tag(err: &UpdateError) -> &'static str {
    match err {
        UpdateError::NoSuchReference { .. } => "no_such_reference",
        UpdateError::AlreadyPinned { .. } => "already_pinned",
    }
}

/// The stable tag for the KERNEL half of a read-back refusal — the
/// carrier read itself, once a name has resolved.
///
/// `Dangling` has two lanes kernel-side and gets one tag per lane,
/// because they are different facts about the model: a topological
/// key that does not resolve is a stale or foreign handle
/// (`dangling_entity`), while a geometry key reached FROM a live
/// entity that does not resolve is a dangling reference inside the
/// body (`dangling_geometry`). Which invariant broke is what a caller
/// branches on, so it belongs in the tag rather than only in the
/// prose. The kernel's own `Display` still states which lookup came
/// back empty, and that prose is the exception's message.
///
/// The match is over `DanglingRef`'s arms, not `..`, so a third lane
/// added kernel-side stops this crate compiling — the same
/// drift alarm the outer arms get from `ReadbackError` not being
/// `#[non_exhaustive]`.
pub fn readback_error_tag(err: &ReadbackError) -> &'static str {
    match err {
        ReadbackError::Dangling {
            what: DanglingRef::Entity(_),
        } => "dangling_entity",
        ReadbackError::Dangling {
            what: DanglingRef::Geometry(_),
        } => "dangling_geometry",
        ReadbackError::NoCanonicalFrame { .. } => "no_canonical_frame",
        ReadbackError::NoCarrier => "no_carrier",
    }
}

/// The stable tag for a read-back door's refusal.
///
/// The `Readback` arm forwards [`readback_error_tag`] rather than
/// wrapping it: a caller branches on which invariant broke, and
/// "the carrier stores no canonical frame" is that fact whether it
/// is reached through a name or through a key.
pub fn interrogate_error_tag(err: &InterrogateError) -> &'static str {
    match err {
        InterrogateError::NodeNotEvaluated { .. } => "node_not_evaluated",
        InterrogateError::NodeFailed { .. } => "node_failed",
        InterrogateError::NodePoisoned { .. } => "node_poisoned",
        InterrogateError::NoSuchName => "no_such_name",
        InterrogateError::Ambiguous { .. } => "ambiguous",
        InterrogateError::WrongKind { .. } => "wrong_kind",
        InterrogateError::WholeBody => "whole_body",
        InterrogateError::NoBodies { .. } => "no_bodies",
        InterrogateError::NoSuchBody { .. } => "no_such_body",
        InterrogateError::Readback(err) => readback_error_tag(err),
    }
}

/// **The standing ladder's "this run has no result for that node"
/// word, as a NAME rather than a repeated literal.**
///
/// The read-back and picking doors reach this spelling through a
/// `match` on a kernel arm ([`interrogate_error_tag`],
/// [`hit_test_error_tag`]), so a rename there is loud. The EVALUATION
/// door has no kernel arm to match on — `Evaluation::result` answers
/// `None` for a node a canceled run never reached, and the reason tag
/// beside it is this crate's own decision — so it spelled the word by
/// hand, and a hand-spelled copy of a shared vocabulary is exactly
/// the divergence `tests::dimension_tags_match_the_kernel_prose`
/// exists for one file over.
///
/// Naming it here makes the copy a reference, and
/// `tests::the_evaluation_door_speaks_the_standing_ladder` pins it
/// against the two doors that DO match, in both directions. (Both
/// named as text rather than as intra-doc links: `tests` is
/// `#[cfg(test)]`, so a link to either would not render.)
pub const NODE_NOT_EVALUATED: &str = "node_not_evaluated";

/// The stable tag for a hit-test refusal — the ray door's own.
///
/// The three standing arms are the SAME vocabulary
/// [`interrogate_error_tag`] speaks for the read-back doors, spelled
/// identically on purpose: "node 7 has no result in this evaluation"
/// is one fact about the run, and a caller that already branches on
/// `node_not_evaluated` from a frame read should not have to learn a
/// second word for it at the pick.
///
/// `unnamed` is the BUG arm (spec D4): the node evaluated and the
/// entity has no name in its table. Its payload is an `EntityRef`,
/// which is an arena key plus a body index — the key does not cross
/// (G1), so what the Python side projects beside this tag is the
/// entity's KIND and its body index, which is the whole of the
/// diagnostic a bug report can act on.
pub fn hit_test_error_tag(err: &HitTestError) -> &'static str {
    match err {
        HitTestError::NodeNotEvaluated { .. } => "node_not_evaluated",
        HitTestError::NodeFailed { .. } => "node_failed",
        HitTestError::NodePoisoned { .. } => "node_poisoned",
        HitTestError::Unnamed { .. } => "unnamed",
    }
}

/// The stable tag for a pick-index build refusal.
///
/// Three arms FORWARD rather than wrap, the
/// [`interrogate_error_tag`] / [`assembly_error_tag`] convention: the
/// standing ladder arrives under [`hit_test_error_tag`]'s words (it is
/// literally that type), and a tessellation refusal arrives under the
/// tessellator's own tag, because "the chordal budget was not finite"
/// is that fact whether it is reached through `Body.tessellate` or
/// through a pick index.
///
/// The `Index` arm does NOT forward, and that is a decision rather
/// than the absence one. `mesh_index` names which door's invariant
/// broke — the pick INDEX's, not the tessellator's and not the
/// evaluation's — and a caller branching on the standing ladder needs
/// that word to stay put. What the payload says underneath it is a
/// second question, answered beside the tag by
/// [`mesh_pick_error_tag`] rather than in place of it.
pub fn node_pick_error_tag(err: &NodePickError) -> &'static str {
    match err {
        NodePickError::Standing(err) => hit_test_error_tag(err),
        NodePickError::NotABody { .. } => "not_a_body",
        NodePickError::NoSuchBody { .. } => "no_such_body",
        NodePickError::Tessellate(err) => tessellate_error_tag(err),
        NodePickError::Index(_) => "mesh_index",
    }
}

/// The stable tag for the pick INDEX's own refusal — what
/// `NodePickError::Index` carries.
///
/// One arm today, and the map exists for the reason the header states
/// rather than for the branch it currently offers: the match is
/// exhaustive, so a second indexing invariant added kernel-side stops
/// this crate compiling instead of silently joining the first under
/// `mesh_index`. The carrier's word says WHICH door refused; this one
/// says which of that door's invariants broke.
///
/// The numbers the arm carries — patch, triangle and the out-of-range
/// position index — stay in the kernel's own `Display`, which is
/// where they already were.
pub fn mesh_pick_error_tag(err: &MeshPickError) -> &'static str {
    match err {
        MeshPickError::PositionOutOfRange { .. } => "position_out_of_range",
    }
}

/// The stable status tag of a resolution VERDICT — a stored name's
/// standing at the next evaluation.
///
/// Not a refusal, and the only entry in this file that is not: every
/// name asked gets exactly one of these three, never a raise. The tag
/// is here anyway for the reason the header states — it is the
/// discriminant a caller branches on, and this file compiles in
/// hosted CI where `py/resolve.rs` does not, so the exhaustive match
/// is where the drift alarm can actually fire.
///
/// **The three words carry the RECOURSE, which is the whole point of
/// keeping them three.** `resolved` needs none. `failed` means the
/// name does not denote in this evaluation and will not come back on
/// its own — the repair is an explicit rebind. `indeterminate` means
/// the NAME is fine and the RUN is not: the minting node failed, was
/// poisoned, or never evaluated, so the reference is unanswerable
/// right now and resolves again when that node does. Collapsing the
/// last two would tell a user to rebind a name that never broke.
///
/// Lower-case snake, the spelling every other tag in this file uses.
/// `Verdict.status` — the other value-carried discriminant in these
/// bindings — capitalizes instead; that divergence predates this and
/// is not repaired here, because a shipped tag value is an interface.
///
/// **What this tag does not reach is the failure's own arm**, and
/// that is a split rather than a gap: [`resolve_error_tag`] and
/// [`resolve_indeterminate_tag`] answer it, and the Python side
/// carries both words — the state on `status`, the arm on `variant`.
/// Keeping them apart is what keeps `status` a three-word vocabulary
/// a caller can exhaust.
pub fn resolution_status_tag(verdict: &Resolution) -> &'static str {
    match verdict {
        Resolution::Resolved(_) => "resolved",
        Resolution::Failed(_) => "failed",
        Resolution::Indeterminate(_) => "indeterminate",
    }
}

/// The stable tag for WHICH failure a stored name met — the arm
/// underneath a `failed` verdict.
///
/// The three words are three REPAIRS, which is the reason the kernel
/// keeps the arms three and the reason they cross. `vanished`: the
/// minting node still evaluates and no table derives the name any
/// more, so the repair is a rebind onto whatever replaced it.
/// `ambiguous`: the name is tie-marked and the kernel will not pick
/// among equally-admissible candidates, so the repair is a refinement
/// — and it is the one arm where a caller has something to CHOOSE.
/// `node_gone`: the minting node left the document, so there is
/// nothing to refine and the rebind is onto a different feature.
///
/// The arms' own names, snake-cased, because the kernel's vocabulary
/// is the one a bug report and a UI should share.
///
/// What does NOT cross beside these is the diagnosis, the tombstone
/// and the tie witness: they are the editor's re-evaluation
/// telemetry, they are not carried through the façade, and the
/// candidate NAMES a caller would refine among already cross as
/// `offers`.
pub fn resolve_error_tag(err: &ResolveError) -> &'static str {
    match err {
        ResolveError::Vanished { .. } => "vanished",
        ResolveError::Ambiguous { .. } => "ambiguous",
        ResolveError::NodeGone { .. } => "node_gone",
    }
}

/// The stable tag for WHY a stored name is unanswerable this run —
/// the arm underneath an `indeterminate` verdict.
///
/// The name is fine in all three and the RUN is not, so no repair
/// here is a rebind; what the three words say is which node to look
/// at. `target_failed`: the minting node failed on its own account.
/// `target_poisoned`: it was poisoned by an upstream failure, so the
/// repair is further up than the node that mints the name.
/// `target_not_evaluated`: a canceled run never reached it, and
/// re-evaluating is the whole of the recourse.
pub fn resolve_indeterminate_tag(cause: &ResolveIndeterminate) -> &'static str {
    match cause {
        ResolveIndeterminate::TargetFailed { .. } => "target_failed",
        ResolveIndeterminate::TargetPoisoned { .. } => "target_poisoned",
        ResolveIndeterminate::TargetNotEvaluated { .. } => "target_not_evaluated",
    }
}

/// The stable tag for a refusal of the advisory-check registry ITSELF
/// (DISCIPLINES-DESIGN DS6) — the checks could not be run.
///
/// A check that ran and disagreed is a FINDING and never reaches this
/// map; [`check_evidence_tag`] is that vocabulary. Keeping the two
/// namespaces apart is the report/gate posture at the tag level:
/// "not checked" and "checked and wrong" are different answers.
///
/// The `Product` arm cannot delegate the way [`assembly_error_tag`]
/// does — the kernel carries the gather's refusal as its RENDERED
/// message, not as a `ProductError` value (a report is `Clone` and
/// `PartialEq` and that type is neither) — so the tag names the stage
/// that refused and the message carries the gather's own prose.
pub fn checks_error_tag(err: &ChecksError) -> &'static str {
    match err {
        ChecksError::Root { .. } => "root_without_value",
        ChecksError::Band { .. } => "band",
        // Named for the FACT, as `product_unavailable` is: the pair is
        // not a pair. It mirrors `ProductError`'s own arm, and the two
        // tags stay distinct because the doors are — one is the
        // gather's refusal of a foreign evaluation, this is the
        // registry's refusal of a foreign evaluation or subject.
        ChecksError::EvaluationOfAnotherDocument { .. } => "evaluation_of_another_document",
        ChecksError::Product { .. } => "product_unavailable",
    }
}

/// The stable tag for one finding's evidence.
///
/// A VALUE's discriminant rather than a refusal's, on the
/// [`refused_ref_tag`] / [`mate_fault_tag`] precedent: what a caller
/// branches on is which fact was found, and `Display` prose is not a
/// stable interface for that. It lives here, beside the refusal maps
/// and not in the PyO3 layer, so the exhaustive match compiles — and
/// the drift alarm fires — on the default no-Python build.
pub fn check_evidence_tag(evidence: &CheckEvidence) -> &'static str {
    match evidence {
        CheckEvidence::Connectedness { .. } => "connectedness",
        CheckEvidence::Escalated { .. } => "escalated",
        CheckEvidence::Unsupported { .. } => "unsupported",
        CheckEvidence::StaleExpectation { .. } => "stale_expectation",
        CheckEvidence::NotSeparated { .. } => "not_separated",
        CheckEvidence::SeparationUnavailable { .. } => "separation_unavailable",
    }
}

/// The stable tag for ONE validator finding — which
/// `ValidationError` arm the body failed on.
///
/// **The one map on this list whose word crosses in a SEQUENCE.** Every
/// other tag here answers a refusal that reports one fault, so its word
/// is a scalar attribute. A validator reports every fault it found in
/// one raise, so this word rides `ValidationFinding.variant`, one entry
/// per finding, and `ValidationError.failure_count` is that list's
/// length. The exception is argued by the door's shape and nowhere
/// else: it is the one door that reports MANY refusals at once.
///
/// Exhaustive over the kernel enum with no wildcard, like every map
/// here. `ValidationError` is closed and its own docs put the
/// obligation on the sites that CLASSIFY: a match mapping it onto a
/// smaller vocabulary must say what it does with each new failure
/// kind, because a wildcard answers for the new kind silently. This is
/// such a site, and the tag it mints is that answer.
pub fn validation_error_tag(err: &ValidationError) -> &'static str {
    match err {
        ValidationError::Band { .. } => "band",
        ValidationError::DanglingDescription { .. } => "dangling_description",
        ValidationError::UncertifiableSurface { .. } => "uncertifiable_surface",
        ValidationError::PoisonedSurfaceDescription { .. } => "poisoned_surface_description",
        ValidationError::ApproxCertification { .. } => "approx_certification",
        ValidationError::ApproxLaneUnsupported { .. } => "approx_lane_unsupported",
        ValidationError::DegenerateTorus { .. } => "degenerate_torus",
        ValidationError::DegenerateTorusEscalated { .. } => "degenerate_torus_escalated",
        ValidationError::NonpositiveTorusTube { .. } => "nonpositive_torus_tube",
        ValidationError::EdgeCertification { .. } => "edge_certification",
        ValidationError::DescriptionNotAdjacent { .. } => "description_not_adjacent",
        ValidationError::PlanarFaceResidual { .. } => "planar_face_residual",
        ValidationError::PlanarFaceEscalated { .. } => "planar_face_escalated",
        ValidationError::PlanarBoundaryResidual { .. } => "planar_boundary_residual",
        ValidationError::PlanarBoundaryEscalated { .. } => "planar_boundary_escalated",
        ValidationError::SliverDihedral { .. } => "sliver_dihedral",
        ValidationError::TransverseNotIntrinsic { .. } => "transverse_not_intrinsic",
        ValidationError::ScaffoldAtRest { .. } => "scaffold_at_rest",
        ValidationError::TangentNotIntrinsic { .. } => "tangent_not_intrinsic",
        ValidationError::UndeclaredCusp { .. } => "undeclared_cusp",
        ValidationError::LaminaWedge { .. } => "lamina_wedge",
        ValidationError::LoopRoleInverted { .. } => "loop_role_inverted",
        ValidationError::CurvedSenseInverted { .. } => "curved_sense_inverted",
        ValidationError::NegativeVolume => "negative_volume",
        ValidationError::VolumeUncomputable { .. } => "volume_uncomputable",
        ValidationError::Pcurve { .. } => "pcurve",
        ValidationError::RingMeetsOuter { .. } => "ring_meets_outer",
        ValidationError::RingContactEscalated { .. } => "ring_contact_escalated",
        ValidationError::UndeclaredContact { .. } => "undeclared_contact",
        ValidationError::StaleContactDeclaration { .. } => "stale_contact_declaration",
        ValidationError::ContactContradicted { .. } => "contact_contradicted",
        ValidationError::CensusEscalated { .. } => "census_escalated",
        ValidationError::CensusUnsupported { .. } => "census_unsupported",
        ValidationError::CensusLaneUnsupported { .. } => "census_lane_unsupported",
        ValidationError::CensusUndecidable { .. } => "census_undecidable",
        ValidationError::DanglingTopology { .. } => "dangling_topology",
        ValidationError::DanglingGeometry { .. } => "dangling_geometry",
        ValidationError::NextPrevMismatch { .. } => "next_prev_mismatch",
        ValidationError::LoopCycleOverrun { .. } => "loop_cycle_overrun",
        ValidationError::ParentLoopMismatch { .. } => "parent_loop_mismatch",
        ValidationError::UnreachableHalfEdge { .. } => "unreachable_half_edge",
        ValidationError::EdgeHalvesIdentical { .. } => "edge_halves_identical",
        ValidationError::EdgeSlotBackpointerMismatch { .. } => "edge_slot_backpointer_mismatch",
        ValidationError::HalfEdgeUnclaimed { .. } => "half_edge_unclaimed",
        ValidationError::HalfEdgeMultiplyClaimed { .. } => "half_edge_multiply_claimed",
        ValidationError::EdgeNotAntiparallel { .. } => "edge_not_antiparallel",
        ValidationError::EmanatingStartMismatch { .. } => "emanating_start_mismatch",
        ValidationError::EmptyLoopVertexWithEmanating { .. } => "empty_loop_vertex_with_emanating",
        ValidationError::LoneVertexWithIncidence { .. } => "lone_vertex_with_incidence",
        ValidationError::VertexOrbitOverrun { .. } => "vertex_orbit_overrun",
        ValidationError::OrbitForeignMember { .. } => "orbit_foreign_member",
        ValidationError::SplitVertexOrbit { .. } => "split_vertex_orbit",
        ValidationError::OuterListedAsRing { .. } => "outer_listed_as_ring",
        ValidationError::BackPointerMismatch { .. } => "back_pointer_mismatch",
        ValidationError::OrphanEntity { .. } => "orphan_entity",
        ValidationError::MultiplyOwned { .. } => "multiply_owned",
        ValidationError::OrphanGeometry { .. } => "orphan_geometry",
        ValidationError::SolidWithoutShells { .. } => "solid_without_shells",
        ValidationError::ShellWithoutFaces { .. } => "shell_without_faces",
        ValidationError::EdgeAcrossShells { .. } => "edge_across_shells",
        ValidationError::ComponentEulerViolation { .. } => "component_euler_violation",
        ValidationError::MissingProvenance { .. } => "missing_provenance",
        ValidationError::LeakedProvenance { .. } => "leaked_provenance",
        ValidationError::ScaffoldingEmptyLoop { .. } => "scaffolding_empty_loop",
        ValidationError::ScaffoldingStrutVertex { .. } => "scaffolding_strut_vertex",
        ValidationError::ShellDisconnected { .. } => "shell_disconnected",
        ValidationError::NullScaffoldShared { .. } => "null_scaffold_shared",
        ValidationError::LeakedNullFaceRecord { .. } => "leaked_null_face_record",
        ValidationError::StaleNullFaceLoop { .. } => "stale_null_face_loop",
        ValidationError::NullEdgeAtRest { .. } => "null_edge_at_rest",
        ValidationError::NullFaceAtRest { .. } => "null_face_at_rest",
    }
}

/// The stable tag for WHAT a census refusal is about — one entity, or
/// the candidate face pair.
///
/// Two arms and two different recourses, which is why the word is
/// worth a caller's branch: an `entity` subject is one carrier outside
/// the certifiable inventory, and the repair is that carrier's —
/// simplify it, or certify it through a supported lane. A `face_pair`
/// is a candidate CONTACT, and the repair is the declaration protocol
/// — declare the coincidence, or separate the two faces. The pair is
/// unordered as a subject, so a caller resolving a refusal against its
/// own records matches the pair either way round.
pub fn census_subject_tag(subject: &CensusSubject) -> &'static str {
    match subject {
        CensusSubject::Entity(_) => "entity",
        CensusSubject::FacePair(_, _) => "face_pair",
    }
}

/// The stable tag for an entity's KIND — the discriminant of the
/// arena reference a census subject names.
///
/// The KEY does not cross and this word is what stands in its place:
/// Python holds an opaque `Body` handle and no arena key, so the kind
/// is the whole of what a caller can read about the site. The key
/// itself is in the kernel's own `Display` prose on the joined
/// message, which is where it already was.
pub fn entity_id_tag(entity: &EntityId) -> &'static str {
    match entity {
        EntityId::Solid(_) => "solid",
        EntityId::Shell(_) => "shell",
        EntityId::Face(_) => "face",
        EntityId::Loop(_) => "loop",
        EntityId::HalfEdge(_) => "half_edge",
        EntityId::Edge(_) => "edge",
        EntityId::Vertex(_) => "vertex",
    }
}

/// The stable tag for WHICH coincidence the tier-3′ census found.
///
/// The arms are not one fact and the word is the difference between
/// two opposite recourses: an `edge_face_pierce` is interpenetration
/// and categorically undeclarable, while an `edge_edge_overlap` is
/// certifiable through the bounding-record reconstruction today. A
/// caller that cannot tell them apart cannot tell "declare this" from
/// "you cannot declare this".
pub fn census_contact_tag(contact: &CensusContact) -> &'static str {
    match contact {
        CensusContact::VertexVertex { .. } => "vertex_vertex",
        CensusContact::VertexOnFace { .. } => "vertex_on_face",
        CensusContact::VertexOnEdge { .. } => "vertex_on_edge",
        CensusContact::EdgeFacePierce { .. } => "edge_face_pierce",
        CensusContact::EdgeEdgeCross { .. } => "edge_edge_cross",
        CensusContact::EdgeEdgeOverlap { .. } => "edge_edge_overlap",
        CensusContact::EdgeFaceOverlap { .. } => "edge_face_overlap",
        CensusContact::ConformalPatch { .. } => "conformal_patch",
    }
}
