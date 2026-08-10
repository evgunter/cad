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

use pncad::document::{EditError, NodeErrorKind};

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
    }
}
