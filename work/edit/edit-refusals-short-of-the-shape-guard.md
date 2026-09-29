---
id: edit-refusals-short-of-the-shape-guard
kind: issue
title: edit: refusals the viewer draws that state no recourse, by the shape guard's census
status: open
opened: 2026-09-29
priority: P2
cost: M
---

(CHROME `refusal-residue`, from the shape guard's zero-recourse check.)

## What

`test_utils::refusal::problems` now flags a refusal that states no
recourse: no `Recourse:`, no "There is no way through" in either case,
and none of the shared unlabelled repairs (`BARE_RECOURSES`). The
standard (`work/chrome/error-and-check-text-overflows-its-region.md`,
"The standard a refusal is rewritten to") says the recourse is the
part never to drop, and where there is no way through the sentence
says so.

These rows, raised through `EditError` (`crates/editor-core/src/edit.rs`) on the status line, and a part's failure in the feature tree (`crates/editor-core/src/eval/parts.rs`), render with none. Each is admitted
by exact id, under the comment naming this file:

- `crates/editor-core/tests/refusal_concision_chains.rs`, `FILED_NO_RECOURSE`:
  8 feature-tree rows.
- `crates/viewer/tests/refusal_concision_edits.rs`, `FILED_NO_RECOURSE`:
  65 status-line rows.

Families: `Edit/AppearanceNamesMissingNode`, `Edit/AppearanceNotSet`, `Edit/AppearanceWrongKind`, `Edit/AssertionDimension`, `Edit/AssertionTarget`, `Edit/ContinuousParamCannotBeCount`, `Edit/DeclareInputNotDeclare`, `Edit/DeclareNamesMissingNode`, `Edit/DeleteWouldDangle`, `Edit/Dimension`, `Edit/DocParamCountHasNoDistribution`, `Edit/DocParamCountHasNoUnit`, `Edit/DocParamNotDeclared`, `Edit/DocParamUnitMismatch`, `Edit/DocParamValueKindMismatch`, `Edit/DuplicateInput`, `Edit/DuplicateWitnessEntry`, `Edit/EmptyPlacementList`, `Edit/EmptyWitnessBulk`, `Edit/EvaluationOfAnotherDocument`, `Edit/ImproperPlacement`, `Edit/InvalidDistribution`, `Edit/InvalidTolerance`, `Edit/MaintenanceRefused`, `Edit/MaintenanceUnrecorded`, `Edit/MateRefused`, `Edit/MeasureMalformed`, `Edit/MetaNonFinite`, `Edit/MetaNotSet`, `Edit/MetaUnversioned`, `Edit/NameUnresolvedInEvaluation`, `Edit/NonFiniteAlignment`, `Edit/NonFiniteDocParam`, `Edit/NonFinitePlacement`, `Edit/NotStructuralSlot`, `Edit/PathOffTree`, `Edit/PayloadDocParamDimension`, `Edit/PayloadUnknownDocParam`, `Edit/PinUnchanged`, `Edit/PlacementAxis`, `Edit/PlacementOnNonInstance`, `Edit/PlacementRuleMismatch`, `Edit/ReadSiteMissingNode`, `Edit/RebindAppearanceCollision`, `Edit/RebindIdentity`, `Edit/RebindKindMismatch`, `Edit/RebindMetadataCollision`, `Edit/RebindNoReferences`, `Edit/RebindTargetMissingNode`, `Edit/RebindUnknownName`, `Edit/RepeatedDesignation`, `Edit/Roots`, `Edit/SelectionNotCanonical`, `Edit/SetMembersOnNonList`, `Edit/SlotDimensionMismatch`, `Edit/SlotDocParamDimension`, `Edit/SlotUnknownDocParam`, `Edit/StructuralSlotNeedsStructuralEdit`, `Edit/TooFewMembers`, `Edit/UnknownNode`, `Edit/UnknownSlot`, `Edit/UnresolvedInput`, `Edit/UpdateOnNonInstance`, `Edit/WitnessOnNonSketch`, `Edit/WouldCycle`, `Part`.

## A part's wrapper

Every `Part/*` row opens with `instantiating <document id>@<version>:`,
which the guard reads as a label; `FILED_NAMESPACES` admits it on the
`Part/` namespace. How the tree draws a part's failure is open with Ev
on #3444, so this row does not propose wording; it records that the
document id is on screen.

## Repair shape

Rewrite each arm at its source to the standard: add the recourse the
raise site supports, or say "There is no way through" where none
exists (`geom_core::KERNEL_DEFECT_ENDING` and its siblings for a
kernel defect). Read the raise sites first: a recourse is a claim.
Then drop the row's entry. The lists carry a must-fire check
(`every_admission_admits_a_row_it_is_needed_for`, and the same check
in the edit and at-rest suites), so an entry left behind after the fix
goes red.
