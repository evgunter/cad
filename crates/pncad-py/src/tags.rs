//! Stable discriminant tags for the document layer's refusals.
//!
//! §L4 requires typed exceptions carrying the structured error, never
//! strings. Neither [`EditError`] nor [`NodeErrorKind`] implements
//! `Display`, and neither is re-exported with a field-level accessor
//! set, so the SMALLEST faithful reading available to a scaffold is:
//! the exception carries a stable **tag** — a discriminant name, which
//! is structured data a caller can branch on — while the `Debug`
//! rendering is relegated to the human-facing message.
//!
//! The matches below are EXHAUSTIVE on purpose. A new kernel variant
//! breaks this build rather than silently arriving in Python as an
//! untagged refusal; that is the drift alarm, and it fires in hosted
//! CI because this module compiles without Python.
//!
//! Full per-variant field projection (node ids, slots, operand roles)
//! is deferred to the unit that binds the complete surface.

use pncad::document::{
    DimensionError, EditError, NodeErrorKind, PersistError, RecordedProgramError, RootFault,
};
use pncad::profile::PathError;

/// The stable tag for a PATHS authoring refusal (LIB-PYG1).
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
        PathError::ArcCarrierSpelling { .. } => "arc_carrier_spelling",
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
/// (`LoopProgram::from_recorded`, LIB-PYG1). The literal arm carries
/// the expression layer's own tag through rather than flattening it.
pub fn recorded_program_error_tag(err: &RecordedProgramError) -> &'static str {
    match err {
        RecordedProgramError::Literal(inner) => expr_dimension_error_tag(inner),
        RecordedProgramError::SubdivisionCount(_) => "subdivision_count",
        RecordedProgramError::CarrierInChain => "carrier_in_chain",
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
        EditError::PathOffTree { .. } => "path_off_tree",
        EditError::Dimension { .. } => "dimension",
        EditError::DeclareNamesMissingNode { .. } => "declare_names_missing_node",
        EditError::NonFiniteDocParam { .. } => "non_finite_doc_param",
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
        // The product-root invariants (ASM-ROOTS D-2) tag per FAULT,
        // not per wrapper: which invariant broke is what a caller
        // branches on.
        EditError::Roots(fault) => root_fault_tag(fault),
        EditError::PlacementOnNonInstance { .. } => "placement_on_non_instance",
        EditError::ImproperPlacement { .. } => "improper_placement",
        EditError::NonFinitePlacement { .. } => "non_finite_placement",
    }
}

/// The stable tag for a product-root invariant refusal (ASM-ROOTS
/// D-2) — shared by every door that carries a `RootFault`.
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
        NodeErrorKind::ProfileAnchor { .. } => "profile_anchor",
        NodeErrorKind::Extrude { .. } => "extrude",
        NodeErrorKind::Revolve { .. } => "revolve",
        NodeErrorKind::Split { .. } => "split",
        NodeErrorKind::Fillet { .. } => "fillet",
        NodeErrorKind::Boolean { .. } => "boolean",
        NodeErrorKind::Transform { .. } => "transform",
        NodeErrorKind::Skin { .. } => "skin",
        NodeErrorKind::Loft { .. } => "loft",
        NodeErrorKind::CurvedSolidFrontier { .. } => "curved_solid_frontier",
        NodeErrorKind::MissingInput { .. } => "missing_input",
        NodeErrorKind::ToleranceConflict { .. } => "tolerance_conflict",
        NodeErrorKind::WrongOperand { .. } => "wrong_operand",
        NodeErrorKind::EmptyOperand { .. } => "empty_operand",
        NodeErrorKind::DegenerateDirection { .. } => "degenerate_direction",
        NodeErrorKind::Band { .. } => "band",
        NodeErrorKind::MissingSlot { .. } => "missing_slot",
        NodeErrorKind::Escalated { .. } => "escalated",
        NodeErrorKind::AxisNotInSketchPlane { .. } => "axis_not_in_sketch_plane",
        NodeErrorKind::NonPositiveCount { .. } => "non_positive_count",
        NodeErrorKind::UnschedulableCycle => "unschedulable_cycle",
        NodeErrorKind::Naming { .. } => "naming",
        NodeErrorKind::DeclareResolve { .. } => "declare_resolve",
        NodeErrorKind::DeclareBothOperands { .. } => "declare_both_operands",
        NodeErrorKind::DeclareUnsupportedPair { .. } => "declare_unsupported_pair",
        NodeErrorKind::FilletSelectionResolve { .. } => "fillet_selection_resolve",
        NodeErrorKind::FilletSelectionKind { .. } => "fillet_selection_kind",
        NodeErrorKind::FilletSelectionEmpty => "fillet_selection_empty",
        NodeErrorKind::WitnessBifurcation { .. } => "witness_bifurcation",
        // ASM-2A: the seam faults stay separable at the tag level —
        // "the pin does not hold" and "the tolerances disagree" are
        // different recourses, so they are different tags.
        NodeErrorKind::Part { fault, .. } => part_fault_tag(fault),
    }
}

/// The stable tag for an instantiation refusal (ASM-2A D-3).
pub fn part_fault_tag(fault: &pncad::document::PartFault) -> &'static str {
    use pncad::document::PartFault as F;
    use pncad::document::ResolveFault as R;
    match fault {
        F::NoResolver => "part_no_resolver",
        F::Unresolved {
            fault: R::PinMismatch,
            ..
        } => "part_pin_mismatch",
        F::Unresolved {
            fault: R::EpsilonSeam,
            ..
        } => "part_epsilon_seam",
        F::Unresolved {
            fault: R::Unresolved,
            ..
        } => "part_unresolved",
        F::PartProduct { .. } => "part_product",
        F::MultiSolid { .. } => "part_multi_solid",
        F::DepthExceeded => "part_depth_exceeded",
    }
}

/// The stable tag for a persistence refusal (LIB-DOORS F1; the v4
/// doors). `PersistError` DOES implement `Display`, so unlike the two
/// above the human message is real prose — the tag is still the
/// machine payload a caller branches on.
pub fn persist_error_tag(err: &PersistError) -> &'static str {
    match err {
        PersistError::NonFinite { .. } => "non_finite",
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

/// The stable tag for a document-layer export refusal (LIB-DOORS F2:
/// `pncad::export::step_for_node`'s error).
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

/// The stable tag for a whole-document product refusal (ASM-ROOTS
/// D-4: `editor_core::product`'s error).
pub fn product_error_tag(err: &pncad::document::ProductError) -> &'static str {
    use pncad::document::ProductError as E;
    match err {
        E::UnknownNode { .. } => "unknown_node",
        E::RootFailed { .. } => "root_failed",
        E::RootPoisoned { .. } => "root_poisoned",
        E::NoBodyRoots => "no_body_roots",
        E::MultiSolidRoot { .. } => "multi_solid_root",
        E::Graft { .. } => "graft_refused",
        E::SolidInvalid { .. } => "solid_invalid",
        E::ProductInvalid { .. } => "product_invalid",
        E::Naming { .. } => "product_naming",
    }
}

/// The stable tag for an expression-constructor refusal (LIB-DOORS
/// F5: `Expr::literal`'s own error type, no longer pre-checked).
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
