---
id: wire-refusals-short-of-the-shape-guard
kind: issue
title: wire: refusals the viewer draws that state no recourse, by the shape guard's census
status: closed
opened: 2026-09-29
priority: P2
cost: M
closed: 2026-10-18
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
  31 feature-tree rows (2026-10-18; the families list below is the one the row opened with).

Families: `AssertionDimension`, `AxisInDifferentPlane`, `BlendSelectionEmpty`, `CurvedSolidFrontier`, `DeclareResolve`, `DeclareSiteNotAnOperand`, `DeclareUnsupportedPair`, `DegenerateDirection`, `EmptyHalf`, `EmptyOperand`, `Expr`, `FaceFrameNotPlanar`, `FaceFrameReadback`, `FrameDirection`, `InstanceOutOfRange`, `MeasureClearanceRefused`, `MeasureMalformed`, `MeasureNonFinite`, `MeasureNotParallel`, `MeasureRefUnreadable`, `MeasureSelectionKind`, `MeasureUnsupported`, `MissingInput`, `MissingSlot`, `Naming`, `NonPositiveCount`, `ParamBox`, `ParamSourceAttach`, `PayloadExpr`, `PlacementRule`, `PlacementsUncertified`, `ProfileAnchor`, `ProfileLaneReplay(Flipped)`, `ProfileLaneReplay(None)`, `ProfilePieces`, `Seed`, `SelectResolve`, `ShellLaneUnsupported`, `ToleranceConflict`, `UnschedulableCycle`, `VerbArity`, `WitnessBifurcation`, `WrongOperand`.

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

## Triage (2026-10-17, WIRE orchestrator)

WIRE's block of `FILED_NO_RECOURSE` is down to 79 entries; the `Declare*`, `AxisInDifferentPlane` and `ParamSourceAttach` families are gone. The `PlacementRule/*`, `PlacementsUncertified`, `InstanceOutOfRange`, `FaceFrame*` and `FrameDirection` entries stand on stage 3 ground and are NOT this row's to write now; the rest (`Select*`, `Expr/*`, `MeasureRefUnreadable/*`, `Naming/*`, `Seed/*`, `Profile*`) can start. The escalation-menu section predates the declare door's deletion: re-read `COINCIDENCE_RECOURSE` before acting on it.

## The unheld families written (WIRE, PR 5298, 2026-10-18)

The `Select*`, `Expr/*`, `MeasureRefUnreadable/*`, `Naming/*`, `Seed/*` and `ProfileLaneReplay` entries now state a recourse or end in the kernel-defect or not-yet ending, so their entries are gone. `WitnessBifurcation`, `MeasureNonFinite` and `PayloadExpr` went with them, because their payloads now carry one. Three entries stay, each under its comment:

- `MeasureRefUnreadable/Readback`: its sentence is `topo::ReadbackError`'s, which the held `FaceFrameReadback/*` rows render too;
- `MeasureRefUnreadable/NodeFailed` and `/NodePoisoned`: `NodeStanding`'s repair is false for a node a mate cluster refused (`work/wirehold/kernel-standing-names-a-cluster-refused-node-as-its-own-failure.md`).

**The escalation-menu section is answered.** `geom_core::COINCIDENCE_RECOURSE` is `coincidence_move_arm!()`, "move the geometry", and offers no declaration, so none of the three doors it names (`NodeErrorKind::Escalated`, `NamingError::Escalated`, `SelectRefusal::InBand`) offers one any more. The section's question went away when the declare door was deleted.

## Resolution (2026-10-18)

Closed by PR 5298: every refusal in the unheld families ends in one
labelled `Recourse:`, or in the kernel-defect or not-yet ending. The
31 entries left on `FILED_NO_RECOURSE` are held ground (D10), parked on
WIREHOLD. That includes `MeasureRefUnreadable/NodeFailed` and
`/NodePoisoned`, which wait on
`wirehold/kernel-standing-names-a-cluster-refused-node-as-its-own-failure`,
and `MeasureRefUnreadable/Readback`, whose sentence is
`topo::ReadbackError`'s, shared with held `FaceFrameReadback/*`.
