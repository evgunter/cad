---
id: a-far-body-the-read-out-refuses-the-picture-draws
kind: issue
title: A body far out builds, its read-out and STEP export refuse to compose it, and the viewer draws it uncertified
status: open
priority: P3
opened: 2026-10-27
---


Found by INTENT stage 3 E's review (R2's far-from-origin note) and
pinned by `crates/pncad/tests/all.rs`
`a_far_turned_copy_builds_and_its_read_out_refuses_to_compose`.

Under E a copy shares its source's body under its frame, so a block's
copy turned and stood near 1e7 builds: nothing in the model maps it.
Two readers then disagree about it:

- The façade's read-out (`pncad::export::body_in_space`,
  `crates/pncad/src/export.rs`) composes the body into the world with
  `topo::transform_rigid`, which re-certifies every edge there and
  refuses at that distance (`Certify { edge, source: AnalyticRung3
  (KernelLimit { operand: Plane, .. }) }`): a Python `Value.body` raises
  `ExportError` `compose_refused`, and `step_for_node` refuses
  `ExportError::Compose`.
- The viewer's picture moves the copy's tessellation on the CPU
  (`crates/viewer/src/pickindex.rs` `drawn`, `crates/viewer/src/scene.rs`
  `ScenePart::placed`) and certifies nothing, so it draws the copy.

R2 reports that before E such blocks did not build: the copy was
rebuilt in the world at `place`, where the same certification ran. The question
is whether the picture should refuse too: draw only what the read-out
would certify, or mark a part drawn without a certificate. It is a
question about the picture's contract, not E's frames.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to VGEOM's charter (viewer geometry and display). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).

## A copy that far out no longer builds (ONEHOME, PR 7088)

Since PR 7088, the door that places a copy asks the session box where it stands (`Framed::asked_where_it_stands`, `[placement.frame]`). A copy past the 1 km box refuses at its placement, so the 1e7 block this row describes no longer builds. Inside the box, composing a copy into the world rounds below ε, so the read-out composes every copy that builds. The pin is re-scoped and renamed `a_far_turned_copy_refuses_at_its_placement_and_exports_nothing` (`crates/pncad/tests/all.rs`): the placement refuses with `OutsideSessionBox` alone, and `step_for_node` refuses on the node's standing (`ExportError::Standing`).

What may remain is a different shape: a body that builds inside the box but whose picture and read-out still disagree, for example a curved face bulging past the box (`work/oneround/the-session-box-reach-misses-a-curved-faces-bulge.md`). Unless one of those is found, the question this row asks (should the picture refuse what the read-out refuses) has no case left, and the row is a candidate to close.
