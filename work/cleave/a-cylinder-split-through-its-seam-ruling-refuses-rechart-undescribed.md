---
id: a-cylinder-split-through-its-seam-ruling-refuses-rechart-undescribed
kind: issue
title: A solid cylinder split by a plane through its seam ruling (0.4 rad off tangency) refuses Finish(Euler(RechartUndescribed{seam})) with both normals at every eps; off the seam the same pose builds
status: dispatched
opened: 2026-10-06
priority: P0
cost: M
branch: cleave/seam-ruling-split
---


## What

Found by the delta review of PR 4098 (DR-4098b, a pre-existing NOTE). A solid cylinder split by a
plane that contains its seam ruling, tilted 0.4 rad off tangency to the wall, refuses
`Finish(Euler(RechartUndescribed{seam}))`. This happens:
- with both normals;
- at ε 1e-6, 1e-9 and 1e-12;
- on main (`cadf2ed188`), on PR 4098's head, and merged with PR 4120.

The same pose through any ruling that is not the seam builds. A user does not know where a seam
is, so this is an ordinary split refusing on ordinary geometry: P0.

The refusing door is `splitting/finish.rs`'s re-chart of the faces a null pair divides (the S7
site of PR 4098's review). The reviewer's probes are in
`~/.local/share/cad-work/cleave-review-4098b/scratch/rvb_probes.rs` (session-local; rebuild the
pose from this paragraph if they are gone).

## Owed

- Measure first: which edge is the `seam` the re-chart cannot describe, and why the seam ruling is
  special.
- Then fix it where it starts, so the pose builds both ways at the closed-form volumes, with valid
  halves. Do not special-case the seam.
- Sweep cones, and full-revolve walls, for the same plane through a seam.
