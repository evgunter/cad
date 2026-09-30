---
id: carve-refusals-short-of-the-shape-guard
kind: issue
title: carve: refusals the viewer draws that state no recourse, by the shape guard's census
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

These rows, raised through `ExtrudeError`, `RevolveError`, `TubeError`, `SkinError`, `LoftError` and `BlendError` (`crates/sweep/src/`), render with none. Each is admitted
by exact id, under the comment naming this file:

- `crates/editor-core/tests/refusal_concision_chains.rs`, `FILED_NO_RECOURSE`:
  18 feature-tree rows.

Families: `Blend`, `Extrude`, `Loft`, `Revolve`, `Skin`, `Tube`.

## A label the shape guard reads as a stage prefix

`Blend/SurgeryInvariant` renders `— at face FaceKey(…): the blend
surgery contradicted …`, admitted by `FILED`.

## Details the guard now reads

`every_blend_detail_renders_within_the_budget` now reads the four arms
built directly with a `detail:` field as well as through their helpers.
That found four raise sites the helper-only reader never rendered
(`battery.rs`: two `BodyNotIntact`, two `UnsupportedRunOut`); all four
fit. Two of them put developer words on screen: "for the curvature
headroom predicate".

## Repair shape

Rewrite each arm at its source to the standard: add the recourse the
raise site supports, or say "There is no way through" where none
exists (`geom_core::KERNEL_DEFECT_ENDING` and its siblings for a
kernel defect). Read the raise sites first: a recourse is a claim.
Then drop the row's entry. The lists carry a must-fire check
(`every_admission_admits_a_row_it_is_needed_for`, and the same check
in the edit and at-rest suites), so an entry left behind after the fix
goes red.

## A stage for a subject (CHROME fix pass, PR 3457)

The shape guard now reads a clause whose subject is a stage — a gerund
with a wrapper verb, `<doing something> refused:` — as a label. These
are admitted by exact row and label in
`refusal_concision_chains.rs` `FILED`:

- `Revolve/VoidInsertion`: "inserting the cavity of hole loop 1
  refused:" (`sweep/src/revolve/mod.rs`).

## Escalations that offer a declaration the door cannot take (CHROME triage)

(From the triage in `work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`.) These arms render the whole `Indeterminate`, so they end in
`COINCIDENCE_RECOURSE` ("declare the coincidence, …"). None of the
nodes that raise them takes a declaration: `Node::Extrude`,
`Node::Revolve`, `Node::Tube`/`HollowTube` and `Node::Loft`
(`crates/editor-core/src/node.rs` near :1691–:1849) carry no
`declare`. Each already states a subject. What is owed is
`source.payload()` and a routed `Recourse:`.

- `ExtrudeError` (`crates/sweep/src/extrude.rs`):
  `ExtrusionEscalated` (near :199), `CosurfaceEscalated` (:215),
  `SliverJoin` (:227), `SliverRim` (:244).
- `RevolveError` (`crates/sweep/src/revolve/mod.rs`):
  `AxisEscalated` (:374), `AngleEscalated` (:385), `SliverRadius`
  (:404, which also prepends its own "move the profile" lever),
  `SliverAxisClearance` (:424), `CosurfaceEscalated` (:491),
  `SliverJoin` (:502), `SliverRim` (:513).
- `LoftError::StackingEscalated` (`crates/sweep/src/loft.rs` :179).
- `TubeError::Escalated` (`crates/sweep/src/revolve/tube.rs` :149):
  PR 3457 routed its subject by predicate name, but its recourse is
  still the forwarded menu.

Extrude and revolve offer the menu on purpose, on their definite arms
too, and `extrude.rs` `extrusion_pair_carries_the_shared_recourse`
and `revolve/mod.rs` `revolve_pairs_carry_the_shared_recourse` pin
it. Whether the declare lever stays there is this program's call; the
triage only records that neither door has a declaration for it to name.

The same menu also reaches these doors whole through carriers on other
ground. `CapPlane`/`SidePlane` forward `NewellError`, and
`Pcurve` forwards `PcurveMintError`. Both are filed on
`work/issues/unowned-viewer-refusals-short-of-the-shape-guard.md`.
The repair can be made at either end.
