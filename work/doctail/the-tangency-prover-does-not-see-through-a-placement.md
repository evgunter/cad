---
id: the-tangency-prover-does-not-see-through-a-placement
kind: issue
title: The tangency prover proves a traced boss's heading on a cylinder built turned on a frame, but not on the same cylinder placed turned by values
status: closed
closed: 2026-11-22
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

Two more rows pass now for the wrong reason, and the fix re-examines them. While the replay answers `ReadTheAxis` for every placed cylinder, their `asks_for_the_axis` assert cannot go red for the reason it states:

- `intent_s6_e_traced_on_a_face::a_hole_parallel_to_the_face_by_value_only_asks_for_the_axis`: at `a = 0` the wall's axis leaves the sketch's `z` by `sin a`, which should be what asks for the axis, rather than the placement's map.
- The first half of `intent_s6_e_traced_on_a_face::a_frame_turned_in_radians_asks_for_the_axis_read_off_the_face`: the float `π/2` turn's rounding, rather than the map.

Once the prover sees through a placement, check that each row still asks for the axis, and for its stated reason: the residual names the angle or the rounding, not the `World` map.

## Closed

The replay saw the placement all along: the placed spelling's heading component held all three of the placement's spins as symbols (its `y` and `z` spins are written zeros, each a variable). It was not shown zero because its form outgrew the door's 256 terms; it discharged at 4096. The `Through` spelling was the same, with five symbols (the angle and the box's corners). The bloat was the face chart's spin at `Sym`: `settle` turned the folded map by `v − atan2(…)` of it, and renormalized the least turn's axes. `Sym<T>` now sets a chart's spin from its frames, `rot(a, θ)·S·O`, as the interval lane does (`SolveScalar::spins_by_frames`). The two are one map over the reals, and each spelling proves at 256.

- `a_tilted_cylinders_boss_is_tangent_for_every_tilt` is lifted to `by_its_heading`.
- `a_cylinder_tilted_on_a_frame_read_off_a_corner_is_tangent_for_every_tilt` (new) proves the `Through` spelling.
- `a_boss_on_a_box_mated_flat_onto_the_cap_is_tangent_for_every_tilt` stays pinned red. It needs 512 terms, through two nested charts: `coincide/a-heading-through-a-chart-on-a-mated-copy-outgrows-the-doors-budget`.
- The two `Owed` rows each assert a placed twin that proves, so their ask cannot be the placement's map. The hole row's twin is at `a = 0.2`: at `a = 0` the twin itself asks, `coincide/a-boss-on-a-placed-cylinders-cap-at-zero-tilt-asks-for-the-axis`.
- `coincidence_door::the_door_proves_every_composed_dies_joint` (renamed from `…two_thirds…`): all 42 of the tour die's joints now prove, up from 28. Those are the pips placed through charts turned off the die's axes.
