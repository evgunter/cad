---
id: the-ruled-region-span-holds-two-stage6-scenes-off-the-registry
kind: issue
title: shaft_and_bearing and boss_tangent_by_value join the corpus registry once the ruled-region span certifies at Interval
status: open
opened: 2026-10-29
priority: P2
cost: E
refs: [6023, main-red-the-meeting-axes-pose-refuses-on-a-ruled-region-span-since-5664]
---

Two of INTENT stage 6's corpus scenes stand beside the registry
(`corpus::stage6_beside_the_registry`), which requires every lane green,
because their unions are green at `f64` and escalate at `Interval` on
`bool_ruled_region_span`, with an enclosure about a whole turn wide
(`[−0.785…, 2.356…]`):

- `crates/editor-core/tests/corpus/shaft_and_bearing.rs`: a shaft pressed
  through a revolved ring's bore and a revolved collar, the shaft's wall
  one cylinder with both bores.
- `crates/editor-core/tests/corpus/boss_tangent_by_value.rs`: a box whose
  wall is tangent to an extruded cylinder.

`coincidence_door::escalates_at_interval` pins the predicate, from
`two_copies_on_one_axis_glue_their_walls_unproven` and
`a_boss_typed_tangent_to_a_cylinder_glues_one_unproven_tangency`. The
likely shared cause is the open CHARTTAIL row
`main-red-the-meeting-axes-pose-refuses-on-a-ruled-region-span-since-5664`
(the same predicate, a whole-turn enclosure at `Interval`, since PR 5664
added `boolean/ruled_region.rs`); not measured here.

**Done when** both unions evaluate at `Interval`, the two pins flip, and
the two scenes move into `corpus::documents()`.

