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

## Escalations that offer a declaration the door cannot take (CHROME triage)

(From the triage in `work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`.) These arms render the whole `Indeterminate`, so they end in
`COINCIDENCE_RECOURSE` ("declare the coincidence, …"), at doors that
take no declaration:

- `NodeErrorKind::Escalated` (`crates/editor-core/src/eval/mod.rs`
  near :1420, `Display` near :2227). PR 3457 gave it a subject by
  predicate. Its raisers are the eval direction norm and
  `DATUM_UNIT_NORM` (`eval::wire::refusal` near :868), the
  revolve's `revolve_full_vs_partial` (`wire.rs` near :1704), and the
  measure's parallelism predicates (`wire.rs` near :2438). A datum, a
  revolve and a measure take no declaration (`node.rs`: `Datum`,
  `Revolve` near :1698, `Measure` near :2496). The `match` that
  names the subject can name the recourse too.
- `NamingError::Escalated` (`crates/editor-core/src/names/emit.rs`
  near :360, `Display` near :540): "no name can be decided because …
  is too close to call: {source}". Naming follows the op, and no
  declaration names a naming discriminator. EMIT claims this file too.
- `SelectRefusal::InBand` (`crates/editor-core/src/names/geompred.rs`
  near :213, `Display` near :319). The viewer does not reach it: the
  selection queries reach users through `pncad::select` and Python.
  A query takes no declaration.

  `geompred.rs` :307–312 argues on purpose that the three-lever
  sentence is right here, because "a selection margin IS a decidability
  question". Every escalation is a decidability question, though, and
  that is not what makes "declare the coincidence" advice. It is advice
  only where some declaration can name the decision. `select`,
  `select_where` and the `GeomPred` comparisons take none, and no
  `Declare` node feeds a selector. The levers this door does have are
  the geometry, the tolerance, and the selector's own comparand (its
  `Cmp` bound or datum distance). Moving the comparand off the cliff is
  the one lever the shared sentence leaves out. As with extrude and
  revolve on CARVE, whether to keep the menu is WIRE's call; this row
  records that the door has no declaration for it to name.

`SelectRefusal::PairInBand` (near :277) is not listed. It is the flush
detector's in-band pair, and declaring that pair on the Boolean's
`declare` input is a real way through.

## One entry retired (PROPS, props/recourse-grammar, 2026-10-01)

`Skin/KnotAlgebra` is out of `FILED_NO_RECOURSE`:
`geom_core::spline::algebra::KnotAlgebraError` now names a repair on
every arm, enforced by `every_knot_algebra_error_arm_names_a_recourse`
beside it, so the row the admission was holding open renders a recourse
and the admission went red for being unused — which is
`every_admission_admits_a_row_it_is_needed_for` working.
