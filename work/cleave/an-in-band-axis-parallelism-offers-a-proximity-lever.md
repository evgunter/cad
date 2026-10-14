---
id: an-in-band-axis-parallelism-offers-a-proximity-lever
kind: issue
title: An in-band reading of a cylinder pair's axis parallelism offers the user a proximity lever
status: open
opened: 2026-10-13
priority: P3
cost: E
refs: [crossing-equal-cylinders-take-the-parallel-radical-plane-and-desync]
---


Found in review of PR 4863 (`cleave/crossing-cylinders-join`).

## What

When `cc_axes_parallel` reads a cylinder pair's axes in the band
(`join::cylinder_pair_axes`, read by the frame dispatch and by
`segment_arm`), the refusal is `frame_refusal`'s
`Escalated { Coincidence(Section, Moot) }`. Its ending
(`refusal_routes`, `Coincide::Section => Ending::Lever(MEET)`) tells the
user to "move the parts so they clearly meet or clearly stand apart
there". That is a translation lever, but the reading is an angle: the
sine between the axes, levered at the walls' span. Walls that plainly
cross along two rulings get the advice to move apart, when the input
to change is the tilt between the axes, which should be made clearly
zero or clearly not.

A public door reaches it:
`parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_at_the_walls_span`.
Every op of the tipped pose ends there. The frame dispatch's own in-band
reading of the same row ends there too
(`frame_dispatch_tests::the_germ_frame_levers_a_cylinder_pair_at_its_walls_span`).

## Direction

Give the parallelism reading a decision of its own, with a lever that
names the axes' angle. One way is a `Coincide` variant, or a
`BooleanDecision` beside `WallRoots(AxisParallel)`, whose ending
already speaks of axes. The section predicates that are distances can
keep `MEET`.
