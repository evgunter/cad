---
id: the-tangency-prover-does-not-see-through-a-placement
kind: issue
title: The tangency prover proves a traced boss's heading on a cylinder built turned on a frame, but not on the same cylinder placed turned by values
status: open
opened: 2026-11-22
priority: P1
cost: M
---

Found by the fix pass of `a-face-charts-turn-is-levered-over-none-of-what-it-carries` (PR 7151). A face's chart now stands only bare in a profile's frame seat (`PoseConstruction::FaceChartUnseated`). So `intent_s6_e_traced_on_a_face::turned_cylinder` could no longer frame its disc as Spin(Slide(the box top's chart)) about `x` by the free angle `a`. That spelling proved its rows exactly.

## What

`[placement.placement]`: "a body is the same to its readers whether it was built in its space, placed there, or placed through other spaces". The coincidence door's replay does not honour this for a traced boss's heading (`coincide::replay::swept_along`).

**Repro** (`intent_s6_e_traced_on_a_face::a_tilted_cylinders_boss_is_tangent_for_every_tilt`):
1. A disc of radius `RADIUS` is drawn free and extruded `HEIGHT`.
2. It is placed into a box's space by values: spin about `x` by the free `a` at `0.2`, every other value zero (`turned_cylinder`).
3. A boss is drawn on the copy's top cap tracing its wall (`draw_on_face`).

`tangency` then answers `Unproven { …, constructions: [… maps: [World(<the placement>)] …], replay: Numeric, … recourse: ReadTheAxis }`. With the disc drawn on Spin(Slide(chart)) by the same `a`, the same boss proved `by_its_heading`.

`a_boss_on_a_box_mated_flat_onto_the_cap_is_tangent_for_every_tilt` is the same, with a box placed on the copy's cap.

**A third spelling** also comes back `Numeric`, with no map in the residual: Spin(Slide(Through(the box's vertical edge, a box vertex))) by `a` (`corner_frame_in`). It reads the box's axes exactly, with no number. So the gap may be wider than the placement's map: the replay may prove the heading only through a face chart's exact-identity least turn. Check that first.

## Owed

The replay sees the wall's axis along the sketch's `z` as a polynomial identity through a placement by values, as it does through a frame. Both rows above are pinned to the current `Unproven` (`test_utils::pinned_red::row`, citing this row). When this lands the pins go red, and the rows return to `by_its_heading`.

