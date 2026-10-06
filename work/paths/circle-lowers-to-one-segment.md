---
id: circle-lowers-to-one-segment
kind: unit
title: circle and circle_split(n ≥ 1) lower to one full-turn segment per arc; lily migrates; re-baseline
status: parked
opened: 2026-09-25
priority: P1
cost: D
parent: lower-profiles-to-carrier-and-interval-not-vertex-and-bulge
blocked_on: [one-segment-loop-through-builders, a-plane-across-a-one-face-wall-meets-its-wrap-edge-once, one-segment-loop-revolves-and-lofts-to-one-wall]
---


Unit 4 of the #3218 lowering. `circle(c, r)` lowers through `circle_split`'s kernel with n = 1, phase 0, and stays its own verb in the program (Ev, #3218 q3). `CircleSplitCount` refuses n = 0. The anchor's n = 2 paragraph goes. `lily::foot` drops its `circle_split(3)` workaround; bossplate and twopeg keep theirs, which are deliberate. Goldens, censuses, names and Python pins re-baseline. EMIT's `Piece(0)/Piece(1)` circle names become one `Carrier` (settled on #3202: second names break). Loft over a circle waits on TESS's `lofted-circle-sections-are-unmeshable…`.

## From unit 3 (2026-10-06)

- Revolve and loft refuse a one-segment loop (`OneSegmentLoop`) until
  `a-one-segment-loops-revolve-and-loft-wall-wraps-a-period-with-no-seam`
  is settled; this unit makes every revolve or loft of a `circle` take
  that answer.
- An extruded one-segment cylinder cut ACROSS its wall by a plane (a
  slab, a pocket floor) refuses `Join(SingleSiteSectionLoop)`
  (`work/join/closed-in-face-section-loop-has-one-site.md`), where
  today's two-arc cylinder builds. That row is parked on D10.
- `lift::lift_seamed` still refuses fewer than two vertices
  (`LiftRefusal::TooFewVertices`, "a loop needs at least two vertices"):
  a one-segment loop lifts to `circle` once `circle` lowers to one.
