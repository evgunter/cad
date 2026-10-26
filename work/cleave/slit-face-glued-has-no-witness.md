---
id: slit-face-glued-has-no-witness
kind: issue
title: SlitFaceGlued's RingRun and ArcsSeparate paths have no witness; its TipUnread witness refuses where the other order of ∩ builds
status: open
opened: 2026-10-21
priority: P3
cost: M
---


Found reviewing PR 5545 (`cleave/fate-cut-ends-at-a-slit-tip`).

## What

`BooleanError::SlitFaceGlued` (`crates/topo/src/boolean/mod.rs`) is
raised on three paths, its `gap` (`SlitFaceGap`) naming which:

- `RingRun`: `join.rs` `pinch_ray_refusal`, where `choose_roles` refuses a ring
  run that encloses no area through a slit's tip (the calls in the
  section loop, `join.rs` ~1420), on a lane `glued_slit_roles` does not
  take (a curved face's ring lane);
- `ArcsSeparate`: `join.rs` `glued_slit_roles`, where `best_arc` finds both arcs
  separating a pending pair (`Capture::Separates`);
- `TipUnread`: `fate.rs` `tip_wraps`, where a fate cut ends at a corner whose two
  straight bounds leave along one ray and the loop's turns are not read:
  a curved face, a curved edge on the loop, or half-turns both ways.

No test or probe reaches `RingRun` or `ArcsSeparate`: no scene of
`a_keel_on_the_floor_subtracts_as_a_slit.rs`,
`a_ring_crossed_along_its_line_places_its_germs_apart.rs` or PR 5545's
review probes (2043 cells per ε) refuses either.

## `TipUnread`'s witness (PR 5545's second review)

The floor is `[0,4]²` with its `y = 0` side an arc (bulge 0.2),
extruded over `z ∈ [0, 1]`, less the keel trench of
`a_keel_on_the_floor_subtracts_as_a_slit.rs` over `x ∈ [1.5, 4.5]`
(open at one end). The partner is the same arc outline extruded over
`z ∈ [0, 0.5]`, flush with the floor. `x ∩ y` (partner first) refuses
`SlitFaceGlued { gap: TipUnread }`, at rest and turned, at ε default,
1e-6 and 1e-12. On main `658cad53dd` it refused
`ClassificationInvariant`. `y ∩ x` builds, so **the two orders of ∩
disagree**: the slit's tip sits in a face whose outline carries the
arc, where `tip_wraps` does not read the loop's turns.

## Owed

Read the turns across a curved edge (an arc's turn along it), so the
`TipUnread` witness builds in both orders as `y ∩ x` does; then pin
it. For `RingRun` and `ArcsSeparate`, build a scene and pin it, or show
the path unreachable and remove it. Candidates: a slit or ring in a cylinder's or cone's
face glued to a coaxial face of another solid (the curved ring lane,
and `tip_wraps` on a curved face); a slit in a planar face whose
outline carries an arc, flush with a face that the op drops.
