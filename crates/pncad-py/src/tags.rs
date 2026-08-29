//! Stable discriminant tags for the document layer's refusals.
//!
//! Typed exceptions carry the structured error, never strings. The
//! exception's machine payload is a stable **tag** — a discriminant
//! name a caller can branch on, which no `Display` prose gives it
//! because prose is not a stable interface — and its human message is
//! the kernel error's own `Display`, never a `Debug` dump.
//!
//! Neither [`EditError`] nor [`NodeErrorKind`] is re-exported with a
//! field-level accessor set, so a tag plus the rendered message is the
//! whole of what a scaffold can read today.
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
//! Full per-variant field projection (node ids, slots, operand roles)
//! is deferred to the unit that binds the complete surface.

use pncad::document::{
    AssemblyError, CheckEvidence, ChecksError, DimensionError, EditError, InlineError, MateFault,
    NodeErrorKind, PersistError, PlacementRuleFault, RecordedProgramError, RefusedRef, RootFault,
    SplitError, UpdateError,
};
use pncad::geom_core::{FrameError, FrameInput};
use pncad::mesh::TessellateError;
use pncad::prelude::BlendKind;
use pncad::profile::PathError;
use pncad::select::{DanglingRef, InterrogateError, ReadbackError};
use pncad::step_import::StepImportError;
// All three STL refusals are prelude-curated; the module path is the
// spelling this file uses throughout, not a reach past the façade.
use pncad::stl::{BinaryHeaderError, SolidNameError, StlError};
use pncad::workspace::WorkspaceError;

/// The stable tag for a PATHS authoring refusal.
///
/// `PathError` implements `Display`, so the human message is the
/// kernel's own prose and the tag is the branchable discriminant —
/// the [`persist_error_tag`] treatment, not the `Debug`-dump one.
pub fn path_error_tag(err: &PathError<f64>) -> &'static str {
    match err {
        PathError::JunctionTangent { .. } => "junction_tangent",
        PathError::JunctionCusp { .. } => "junction_cusp",
        PathError::TangentLineClose { .. } => "tangent_line_close",
        PathError::SameCarrierJunction { .. } => "same_carrier_junction",
        PathError::NoCornerForFillet { .. } => "no_corner_for_fillet",
        PathError::AnchorOutsideTrimmedExtent { .. } => "anchor_outside_trimmed_extent",
        PathError::FilletOffsetLeverTooShort { .. } => "fillet_offset_lever_too_short",
        PathError::ArcLegOnOpenFillet { .. } => "arc_leg_on_open_fillet",
        PathError::SeamRetrimsArcFirstSide => "seam_retrims_arc_first_side",
        PathError::Structure { .. } => "guided_structure",
        PathError::DegenerateArcSpec { .. } => "degenerate_arc_spec",
        PathError::NonpositiveLeg { .. } => "nonpositive_leg",
        PathError::NonpositiveFilletRadius { .. } => "nonpositive_fillet_radius",
        PathError::NonpositiveCircleRadius { .. } => "nonpositive_circle_radius",
        PathError::CircleSplitCount { .. } => "circle_split_count",
        PathError::ArcContinueNeedsArcCarrier => "arc_continue_needs_arc_carrier",
        PathError::ArcContinueOffCarrier { .. } => "arc_continue_off_carrier",
        PathError::ZeroDirection { .. } => "zero_direction",
        PathError::ArcViaCollinear { .. } => "arc_via_collinear",
        PathError::DegenerateArcChord { .. } => "degenerate_arc_chord",
        PathError::ArcCenterNotEquidistant { .. } => "arc_center_not_equidistant",
        PathError::DegenerateArcCenter { .. } => "degenerate_arc_center",
        PathError::FarEndAnchorWithoutFillet => "far_end_anchor_without_fillet",
        PathError::Escalated { .. } => "escalated",
        PathError::Band(_) => "band",
        PathError::UnderdeterminedLeg { .. } => "underdetermined_leg",
        PathError::OverdeterminedJunction { .. } => "overdetermined_junction",
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

/// The stable tag for an edit refusal.
pub fn edit_error_tag(err: &EditError) -> &'static str {
    match err {
        EditError::UnknownNode { .. } => "unknown_node",
        EditError::ProfileProgramRefused { .. } => "profile_program_refused",
        EditError::UnresolvedInput { .. } => "unresolved_input",
        EditError::WouldCycle { .. } => "would_cycle",
        EditError::DeleteWouldDangle { .. } => "delete_would_dangle",
        EditError::UnknownSlot { .. } => "unknown_slot",
        EditError::SlotDimensionMismatch { .. } => "slot_dimension_mismatch",
        EditError::StructuralSlotNeedsStructuralEdit { .. } => {
            "structural_slot_needs_structural_edit"
        }
        EditError::NotStructuralSlot { .. } => "not_structural_slot",
        EditError::UnknownDocParam { .. } => "unknown_doc_param",
        EditError::DocParamDimensionMismatch { .. } => "doc_param_dimension_mismatch",
        EditError::ContinuousParamCannotBeCount { .. } => "continuous_param_cannot_be_count",
        EditError::DocParamNotDeclared { .. } => "doc_param_not_declared",
        EditError::DocParamValueKindMismatch { .. } => "doc_param_value_kind_mismatch",
        EditError::PathOffTree { .. } => "path_off_tree",
        EditError::Dimension { .. } => "dimension",
        EditError::DeclareNamesMissingNode { .. } => "declare_names_missing_node",
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
        EditError::UpdateOnNonInstance { .. } => "update_on_non_instance",
        EditError::PinUnchanged { .. } => "pin_unchanged",
        // A mate's alignment is authored geometry, so the non-finite
        // refusal is the placement one's sibling and tags beside it.
        EditError::NonFiniteAlignment { .. } => "non_finite_alignment",
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
        NodeErrorKind::ToleranceConflict { .. } => "tolerance_conflict",
        // Its own tag rather than the ε conflict's: both refuse every
        // node for a whole-run reason, but the recourses are different
        // — one is "replay in a process whose ε matches", the other is
        // "ask for the box at a scalar that can carry it".
        NodeErrorKind::ParamBox { .. } => "param_box",
        NodeErrorKind::WrongOperand { .. } => "wrong_operand",
        NodeErrorKind::EmptyOperand { .. } => "empty_operand",
        NodeErrorKind::DegenerateDirection { .. } => "degenerate_direction",
        NodeErrorKind::Band { .. } => "band",
        NodeErrorKind::MissingSlot { .. } => "missing_slot",
        NodeErrorKind::Escalated { .. } => "escalated",
        NodeErrorKind::AxisNotInSketchPlane { .. } => "axis_not_in_sketch_plane",
        NodeErrorKind::NonPositiveCount { .. } => "non_positive_count",
        NodeErrorKind::PlacementsUncertified { .. } => "placements_uncertified",
        NodeErrorKind::PlacementRule(fault) => placement_rule_fault_tag(fault),
        NodeErrorKind::UnschedulableCycle => "unschedulable_cycle",
        NodeErrorKind::Naming { .. } => "naming",
        NodeErrorKind::DeclareResolve { .. } => "declare_resolve",
        NodeErrorKind::DeclareBothOperands { .. } => "declare_both_operands",
        NodeErrorKind::DeclareUnsupportedPair { .. } => "declare_unsupported_pair",
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

/// The stable tag for a mate-solve refusal. Each arm is
/// a different recourse: add the complementary mate, delete one of the
/// clashing pair, rebind the stranded head, author the missing
/// primitive, or move the geometry out of the band.
pub fn mate_fault_tag(fault: &MateFault) -> &'static str {
    match fault {
        MateFault::Frame { .. } => "mate_frame_degenerate",
        MateFault::ClassNotAdmitted { .. } => "mate_class_not_admitted",
        MateFault::TableLacks { .. } => "mate_table_lacks",
        MateFault::Indeterminate { .. } => "mate_indeterminate",
        MateFault::Band { .. } => "mate_band",
        MateFault::Contradictory { .. } => "mate_contradictory",
        MateFault::Under { .. } => "mate_under",
        MateFault::DanglingHead { .. } => "mate_dangling_head",
        MateFault::SelfMate { .. } => "mate_self",
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

/// The stable tag for a persistence refusal (the v4
/// doors). `PersistError` DOES implement `Display`, so unlike the two
/// above the human message is real prose — the tag is still the
/// machine payload a caller branches on.
pub fn persist_error_tag(err: &PersistError) -> &'static str {
    match err {
        PersistError::NonFinite { .. } => "non_finite",
        PersistError::Distribution { .. } => "distribution",
        PersistError::ProfileProgram { .. } => "profile_program",
        PersistError::Serialize { .. } => "serialize",
        PersistError::Header { .. } => "header",
        PersistError::HeaderId { .. } => "header_id",
        PersistError::IdMismatch { .. } => "id_mismatch",
        PersistError::UnknownSchema { .. } => "unknown_schema",
        PersistError::SchemaTooOld { .. } => "schema_too_old",
        PersistError::Migration(_) => "migration",
        PersistError::Parse { .. } => "parse",
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
        WorkspaceError::RandomnessUnavailable { .. } => "randomness_unavailable",
        WorkspaceError::Update { .. } => "update",
    }
}

/// The stable tag for a STEP IMPORT refusal.
///
/// `StepImportError` implements `Display`, so the human message is the
/// importer's own prose naming the entity id and line; this is the
/// branchable discriminant. Twenty-one arms, and unlike
/// [`workspace_error_tag`]'s door **every one of them is reachable**
/// through `import_step` — a caller distinguishing a malformed file
/// from an unsupported entity from a tier refusal has no other way to
/// do it, because the id and line live in prose.
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
        RefusedRef::NodeGone => "ref_node_gone",
        RefusedRef::Vanished => "ref_vanished",
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
        AssemblyError::AtRest { .. } => "at_rest",
        AssemblyError::Uncertified { .. } => "uncertified",
    }
}

/// The stable tag for a split refusal.
pub fn split_error_tag(err: &SplitError) -> &'static str {
    match err {
        SplitError::EmptyCut => "empty_cut",
        SplitError::UnknownCutNode { .. } => "unknown_cut_node",
        SplitError::PartIdCollides { .. } => "part_id_collides",
        SplitError::SeveredEdge { .. } => "severed_edge",
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
