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
