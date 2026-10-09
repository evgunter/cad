---
id: circle-lowers-to-one-segment
kind: unit
title: circle and circle_split(n ≥ 1) lower to one full-turn segment per arc; lily migrates; re-baseline
status: parked
opened: 2026-09-25
priority: P1
cost: D
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
blocked_on: [a-plane-across-a-one-face-wall-meets-its-wrap-edge-once]
---


Unit 4 of the #3218 lowering. `circle(c, r)` lowers through `circle_split`'s kernel with n = 1, phase 0, and stays its own verb in the program (Ev, #3218 q3). `CircleSplitCount` refuses n = 0. The anchor's n = 2 paragraph goes. `lily::foot` drops its `circle_split(3)` workaround; bossplate and twopeg keep theirs, which are deliberate. Goldens, censuses, names and Python pins re-baseline. EMIT's `Piece(0)/Piece(1)` circle names become one `Carrier` (settled on #3202: second names break). Loft over a circle waits on TESS's `lofted-circle-sections-are-unmeshable…`.

## From unit 3 (2026-10-06)

- Revolve and loft refuse a one-segment loop (`OneSegmentLoop`) until
  `one-segment-loop-revolves-and-lofts-to-one-wall` builds the wrap
  edge (#4175); this unit makes every revolve or loft of a `circle`
  take it.
- Validate's `ByConstruction` full-turn arm (`seg::build_loop_seg`)
  decides only the turn's reach and sense: no door constructs a full
  turn yet, so the arm trusts the construction to write its start on
  its carrier and |Δθ| = 2π. This unit's `circle_split` kernel is that
  construction, and owns showing it.
- An extruded one-segment cylinder cut ACROSS its wall by a plane (a
  slab, a pocket floor) refuses `Join(SingleSiteSectionLoop)`
  (`work/join/closed-in-face-section-loop-has-one-site.md`), where
  today's two-arc cylinder builds. That row is parked on D10.
- `lift::lift_seamed` still refuses fewer than two vertices
  (`LiftRefusal::TooFewVertices`, "the chain vocabulary spells a loop
  of at least two vertices"):
  a one-segment loop lifts to `circle` once `circle` lowers to one.

## Evidence from JOIN (PR 4345, branch `join/wrap-edge-section-loop`)

- The 2026-10-06 note above says the cylinder cut across its wall by a
  slab or pocket floor refuses `Join(SingleSiteSectionLoop)`. With
  JOIN's wrap-edge arm (`a-plane-across-a-one-face-wall-meets-its-wrap-edge-once`)
  that cut builds. In every op and both orders it reaches its closed
  form, tiers 2 and 3′ and the certificate:
  - `crates/sweep/tests/a_plane_across_a_one_face_wall.rs`;
  - `one_segment_loop.rs`
    `a_boolean_on_an_extruded_seam_wall_builds_along_and_across_it`.
- A washer, an annular one-segment tube through a plate, still refuses
  when its two wrap edges sit at different azimuths. That is filed as
  `work/join/annular-one-segment-tube-through-a-plate-refuses-section-loop-undecided.md`.
