---
id: cylinder-offsets-read-at-a-stored-origin-off-the-reach
kind: issue
title: three cylinder-offset rows read a cylinder's stored origin against another carrier's axis, off the reach
status: open
opened: 2026-10-06
priority: P2
cost: M
---


## What

The defect `plane-cylinder-section-reads-its-gap-at-the-stored-origin`
closed in the plane×cylinder and equal-cylinder classifiers has three
siblings: each projects one cylinder's STORED origin perpendicular to
ANOTHER carrier's axis, after a row that admits a relative tilt up to
the zero band over some lever. A point-to-line distance against the
cylinder's OWN axis does not move with where the origin is stored; one
against another axis moves by the tilt times how far the stored origin
stands along the axis from where the verdict is consumed, and nothing
bounds that distance.

- `cone_cylinder_section` (`crates/geom-brep/src/intersect.rs`),
  `coc_coaxial`: `q = o − apex`, `d = |q − a(q·a)|`, `a` the CONE's
  axis, after `coc_axes_parallel` admits `‖a×b‖·extent` in the zero
  band. A cylinder stored 1000 m along its axis from the cone's apex
  carries `1000·θ` into the coaxiality margin. The scalar `extent`
  has no pivot either; the apex is the natural one.
- The cylinder pair's radical-plane normal in the boolean join
  (`parallel_radical_plane`, `crates/topo/src/boolean/join.rs`, the
  `BOOL_JOIN_CC_AXIS_OFFSET` row): `w = (o2 − o1) − a1((o2 − o1)·a1)` at the
  stored origins, decided through `UnitVec3::new`.
- `chart_region_cyl_offset` (`crates/topo/src/chart_region.rs`): the
  perpendicular offsets `perp_a`, `perp_b` of each stored origin from
  the other's axis, after `chart_region_cyl_tilt` admits a tilt levered
  at `hyp`; the transfer parameter `c = (b.origin − a.origin)·a.axis`
  beside it reads the stored origins too.

Two more sites read a cylinder's position off the reach, found by PR
4118's full review:

- `carrier_cyl_reach` (`crates/topo/src/boolean/carrier_eq.rs`, the
  cylinder pair) pivots on operand 2's foot (`reach.foot_on(o2, a2)`)
  and measures `apart` from operand 1's stored origin against `a1`. It
  decides the sum, as it should, but its pivot depends on which operand
  is named first. `ExtentBall::lever_between` is the order-free lever.
- `route_pose`'s scalar extent (`replace_face::pose_reach`,
  `crates/topo/src/replace_face.rs`) takes a cylinder's stored ORIGIN
  as its anchor. The origin is any point of the axis, so the extent can
  overstate the reach without bound, and the cone×cylinder arm it feeds
  decides `coc_axes_parallel` on it. Over-stating is not safe on a
  two-sided row whose definite side is served (the review's MAJOR-1
  class). The cylinder's anchor is the foot of the edge's ball on its
  axis.

## The fix's shape

Read each offset where the consumed extent is, as the two classifiers
now do: the feet of an `ExtentBall`'s centre on the axes
(`ExtentBall::foot_on`), the tilt levered from there
(`ExtentBall::lever_between` for two axes, the apex for a cone). The
chart-region and join sites need a ball from their callers; the cone
arm's callers (`route_pose`'s edge ball, chord_join's cone lane) already hold one.

Found by the class sweep of the `tang/classifiers-read-at-the-reach`
lane (pattern: `x − a·(x·a)` with `x` an origin difference; the hit
list is in that PR's body).
