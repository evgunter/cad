---
id: offset-derive-meets-an-undefined-normal-at-a-collapsed-side
kind: issue
title: the offset door moves a spline wall along its normal, which a collapsed side has none of
status: open
opened: 2026-11-22
priority: P3
cost: M
refs: [a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint]
---


A lead raised by CHARTTAIL's collapsed-column design fork (design-fork
row 296), filed by the `charttail/collapsed-column` lane and **not
measured**. A loft can now build a NURBS wall with a collapsed boundary
side, where `S_u × S_v` vanishes along the whole side: a loft's pinned
corner (`crates/sweep/tests/pinned_corners_build.rs`). The offset door
moves a spline wall point by point along its own normal.
`crates/topo/src/offset_derive.rs`'s module docs describe the held
boundary rows that way. At the collapsed side that normal is
undefined. What the door does there was not read: it may refuse, take
the limit normal, or produce a wrong point.

## Fix

Shell or offset a wedge built by
`pinned_corners_build::the_two_section_wedge_builds_as_its_prism`. If
the door does not refuse typed or build correctly, read the
collapsed side from `geom_brep::chart_singularity::singular_points`
and refuse it, naming the side, until an offset of a pole is designed.
