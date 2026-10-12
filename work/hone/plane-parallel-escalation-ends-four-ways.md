---
id: plane-parallel-escalation-ends-four-ways
kind: issue
title: the planes-parallel decision's escalation ends four ways across topo's doors
status: open
opened: 2026-10-12
---


(ENCL, from the review of PR 4811.)

## What

`bool_plane_parallel` asks one question, `PlaneRung::Parallel`'s
subject ("whether the two planes are parallel"), and its undecided
refusal ends four different ways depending on the door that renders it.
D4 ¶1 wants one ending per decision:

- `boolean/refusal_routes.rs` `Coincide::ending` (≈L729–731):
  `Coincide::Planes` (the screen door) ends in a frontier
  (`PLANES_FRONTIER`), and `Coincide::OnPlanes` (the on-pair door) ends
  in the lever alone (`coincidence_move_arm!`).
- `boolean/refusal_routes.rs` `BooleanDecision::Neighbours(PlaneRung::Parallel)`
  (≈L1504 in the route ending, ≈L2448 in the subject/ending table) is
  sized (`NEIGHBOUR_ENDING`, `SizedPass::Positive`).
- `merge_faces.rs` `DeclaredPlanes(PlaneRung::Parallel | PlaneRung::Norm)`
  (≈L1047 subject, ≈L1073 ending) ends in `Unsized::Defect`.

The margin is the normals' sine levered over an extent. Whether that
is a size the user chose (sized) or a direction's disagreement (lever
alone) is the open question. The defect arm's comment says the declared
rung bridges in-band parallelism, so only a poisoned norm reaches it. If
so, the arm's subject should be the norm's, not the parallel rung's.

## Repair shape

Pick one kind for the parallel rung's escalation, and end it that way
at every door. A door whose escalation is really a different decision
(the declared rung's poisoned norm) states that decision's subject.
