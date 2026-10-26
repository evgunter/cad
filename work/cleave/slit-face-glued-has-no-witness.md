---
id: slit-face-glued-has-no-witness
kind: issue
title: SlitFaceGlued's refusal paths have no witness: the ring lane's pinch-ray refusal, best_arc finding no arc, and the fate cut's unread tip
status: open
opened: 2026-10-21
priority: P3
cost: M
---


Found reviewing PR 5545 (`cleave/fate-cut-ends-at-a-slit-tip`).

## What

`BooleanError::SlitFaceGlued` (`crates/topo/src/boolean/mod.rs`) is
raised on three paths, its `gap` (`SlitFaceGap`) naming which, and no test or probe reaches any of them:

- `RingRun`: `join.rs` `pinch_ray_refusal`, where `choose_roles` refuses a ring
  run that encloses no area through a slit's tip (the calls in the
  section loop, `join.rs` ~1420), on a lane `glued_slit_roles` does not
  take (a curved face's ring lane);
- `ArcsSeparate`: `join.rs` `glued_slit_roles`, where `best_arc` finds both arcs
  separating a pending pair (`Capture::Separates`);
- `TipUnread`: `fate.rs` `tip_wraps`, where a fate cut ends at a corner whose two
  straight bounds leave along one ray and the loop's turns are not read:
  a curved face, a curved edge on the loop, or half-turns both ways.

Since PR 5545 no scene of `a_keel_on_the_floor_subtracts_as_a_slit.rs`,
`a_ring_crossed_along_its_line_places_its_germs_apart.rs` or the PR's
review probes (2043 cells per ε) refuses it.

## Owed

Build a scene for each path and pin it, or show the path unreachable
and remove it. Candidates: a slit or ring in a cylinder's or cone's
face glued to a coaxial face of another solid (the curved ring lane,
and `tip_wraps` on a curved face); a slit in a planar face whose
outline carries an arc, flush with a face that the op drops.
