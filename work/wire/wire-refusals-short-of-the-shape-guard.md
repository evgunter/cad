---
id: wire-refusals-short-of-the-shape-guard
kind: issue
title: wire: refusals the viewer draws that state no recourse, by the shape guard's census
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

These rows, raised through `NodeErrorKind`'s own arms (`crates/editor-core/src/eval/mod.rs`) and the payloads they forward (`EvalError`, the resolve, read-back, measure, seed, placement-rule and naming refusals), render with none. Each is admitted
by exact id, under the comment naming this file:

- `crates/editor-core/tests/refusal_concision_chains.rs`, `FILED_NO_RECOURSE`:
  94 feature-tree rows.

Families: `AssertionDimension`, `AxisInDifferentPlane`, `BlendSelectionEmpty`, `BlendSelectionKind`, `BlendSelectionResolve`, `CrossingUnverified`, `CurvedSolidFrontier`, `DeclareResolve`, `DeclareSiteNotAnOperand`, `DeclareUnsupportedPair`, `DegenerateDirection`, `DerivedFrameSection`, `EmptyHalf`, `EmptyOperand`, `Expr`, `FaceFrameKind`, `FaceFrameNotPlanar`, `FaceFrameReadback`, `FaceFrameResolve`, `FrameDirection`, `InstanceOutOfRange`, `MeasureClearanceRefused`, `MeasureMalformed`, `MeasureNonFinite`, `MeasureNotParallel`, `MeasureRefResolve`, `MeasureRefUnreadable`, `MeasureSelectionKind`, `MeasureUnsupported`, `MissingInput`, `MissingSlot`, `Naming`, `NonPositiveCount`, `ParamBox`, `ParamSourceAttach`, `PayloadExpr`, `PlacementRule`, `PlacementsUncertified`, `ProfileAnchor`, `ProfileLaneReplay(Flipped)`, `ProfileLaneReplay(None)`, `ProfilePieces`, `Seed`, `SeedPinnedSection`, `ShellLaneUnsupported`, `ShellOpenKind`, `ShellOpenResolve`, `ToleranceConflict`, `UnschedulableCycle`, `VerbArity`, `WitnessBifurcation`, `WrongOperand`.

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

- `crates/editor-core/src/eval/mod.rs`, the measure's not-parallel arm:
  "… needs them parallel, and {predicate} decided they …".
- `crates/editor-core/src/names/geompred.rs`, the selection refusals:
  "… '{predicate}' left it inside the band …" (two arms).
- `NodeErrorKind::Escalated` now words the four names it is raised with
  (a direction's length, a full revolve turn, two parallel checks) and
  falls back to "a decision this node takes" for any other; a name
  raised there later wants its words added.
