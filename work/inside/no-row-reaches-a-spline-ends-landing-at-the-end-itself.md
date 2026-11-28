---
id: no-row-reaches-a-spline-ends-landing-at-the-end-itself
kind: issue
title: No row reaches spline_landings' landing at a spline's own end (EndReading::Within at the end): dropping it survives every region row
status: open
opened: 2026-11-25
priority: P2
cost: E
---


Filed by EMBEDWALK's PR 7772 fix pass (`embedwalk/share-nothing-region`),
after merging 7755's end-reading fix (`5e012f13f3`).

## What

`region::spline_landings` (`crates/topo/src/region.rs`) turns a near end
into a landing only where the ring composite read it
`EndReading::Within { until }` with `until` the end itself (`at_end`); any
other stretch grazes. The mutant that drops those two end landings
(`met.extend(at_end(edge.t0, ..)?.map(..))` and the `t1` twin, keeping the
`?` so a graze still grazes) survives every row in `topo --lib`'s
`sphere_region`, `torus_region`, `region::` and `solid_contain` tests
(245 + 22 run), including `sphere_region_spline_rows` and EMBEDWALK's
`torus_region::tests::a_leg_through_a_splines_joint_reads_no_wrong_side`.

That joint row's legs pass exactly through the joint of a closed spline
(both ends on the leg's plane, transversal), and the composite reads both
ends `Within` with `until` past the end, so the path grazes and the
landing arm is never reached.

## Owed

A fixture where the composite decides the crossing at the end itself (a
spline end exactly on a leg's plane, the spline leaving it at a slope the
composite resolves at that end), with a query whose closest crossing is
that end, so that dropping the landing reads the next crossing's side. Or,
if that arm cannot be reached by any spline the boolean mints, say so
and drop it.
