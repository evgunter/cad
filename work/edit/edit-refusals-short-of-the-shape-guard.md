---
id: edit-refusals-short-of-the-shape-guard
kind: issue
title: edit: refusals the viewer draws that state no recourse, by the shape guard's census
status: spec
branch: edit/edit-refusal-recourse
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

The `instantiating <document id>:` label this row first recorded is
gone: EDIT's carried-refusal unit reworded the part wrapper, and the
hex ids it still prints are admitted span by span (`ADMISSIONS`).

## Repair shape

Rewrite each arm at its source to the standard: add the recourse the
raise site supports, or say "There is no way through" where none
exists (`geom_core::KERNEL_DEFECT_ENDING` and its siblings for a
kernel defect). Read the raise sites first: a recourse is a claim.
Then drop the row's entry. The lists carry a must-fire check
(`every_admission_admits_a_row_it_is_needed_for`, and the same check
in the edit and at-rest suites), so an entry left behind after the fix
goes red.

## A predicate's name on screen (CHROME fix pass, PR 3457)

A predicate's static name is routing: `geom_core::IndeterminatePayload`
no longer renders it, and every escalation now says in words what was
being decided (`test_utils::refusal::subjectless_escalations`). These
sites still put a name in the sentence; each wants the decision in
words, with the name left to `Debug`:

- `crates/editor-core/src/resolve/mod.rs`, the three flip reports:
  "predicate {predicate} flipped from {from} to {to} …".

## Ruled and spec'd (2026-09-29, EDIT orchestrator) — the status-line half, single review

**Tier:** single review (one Opus FULL). A recourse is a claim about what
the user can do, so every added one is checked against its raise site.

**Scope: the `EditError` status-line rows only.** That is
`crates/viewer/tests/refusal_concision_edits.rs`'s `FILED_NO_RECOURSE`,
raised through `crates/editor-core/src/edit.rs`. Two groups are out of
scope:
- **The eight feature-tree rows** (`refusal_concision_chains.rs`, the
  `Part` family). They wait for PR #3482 (part/product refusals), which
  is live on those rows.
- **Arms the placement unit reshapes or deletes.** The placement design
  on `[ev]` #3441 removes maintenance and moves placement onto gauges.
  These arms are skipped, not rewritten:
  - `Edit/MaintenanceRefused`, `Edit/MaintenanceUnrecorded`;
  - `Edit/ImproperPlacement`, `Edit/NonFinitePlacement`,
    `Edit/PlacementOnNonInstance`, `Edit/PlacementAxis`,
    `Edit/PlacementRuleMismatch`, `Edit/EmptyPlacementList`;
  - `Edit/MateRefused`, whose carried text MSOLVE owns.

  Leave their entries with a comment naming the placement row.

**The rule, per the row's repair shape:**
1. Read each arm's raise site or sites. Then either add the recourse
   the site supports, or end with "There is no way through" where none
   exists, using the shared endings. Never invent a recourse the edit
   door cannot honour.
2. Drop the arm's `FILED_NO_RECOURSE` entry. The must-fire check reds a
   stale entry.
3. Stay within the 75-word budget, and pass `problems` in full.
4. Keep every Python tag word frozen, because tags do not change. Any
   Display text a Python or viewer test pins is re-baselined and named
   in the PR body.
5. The predicate-name sites in `resolve/mod.rs` (the three flip reports)
   are in scope: say the decision in words, and leave the name to
   `Debug`.

**Rows:** the edits roster, green with the list shortened. Add one
planted mutant per family class (a recourse removed) that reds, and
record it.

**When done:** the feature-tree half and the placement arms remain on
this row, so it stays open, and the unit sets the `## Built` section.
