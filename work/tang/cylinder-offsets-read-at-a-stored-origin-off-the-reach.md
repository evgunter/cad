---
id: cylinder-offsets-read-at-a-stored-origin-off-the-reach
kind: issue
title: three cylinder-offset rows read a cylinder's stored origin against another carrier's axis, off the reach
status: closed
opened: 2026-10-06
priority: P2
cost: M
closed: 2026-10-07
branch: tang/cylinder-offsets-at-the-reach
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
  is named first. `Reach::lever_between` is the order-free lever.
- `route_pose`'s scalar extent (`crates/geom-brep/src/intersect.rs`,
  the edge's `Reach::Span` levered from the pair's anchors) takes a
  cylinder's stored ORIGIN as its anchor. The origin is any point of
  the axis, so the extent can overstate the reach without bound, and
  the cone×cylinder arm it feeds decides `coc_axes_parallel` on it.
  Over-stating is not safe on a two-sided row whose definite side is
  served (PR 4118's review, MAJOR-1's class). The cylinder's anchor is
  the foot of the edge's reading point on its axis.

## The fix's shape

Read each offset where the consumed extent is, as the two classifiers
now do: for two cylinders, the feet of a `Reach`'s point on the axes
(`Reach::foot_on`), the tilt levered from there (`Reach::lever_between`);
for the cone arm, at the APEX, against the cylinder's own axis, the tilt
levered from the apex by the consumed region's distance from it. Never
a ball around the consumed points as a lever. The chart-region and join sites need a reach
from their callers; the cone arm's callers (`route_pose`'s edge span,
chord_join's cone lane) already hold one.

Found by the class sweep of the `tang/classifiers-read-at-the-reach`
lane (pattern: `x − a·(x·a)` with `x` an origin difference; the hit
list is in that PR's body).

## Closed

By `tang/cylinder-offsets-at-the-reach`:

- `cone_cylinder_section`'s `coc_coaxial` reads the apex's distance from
  the CYLINDER's axis, and the arm's `extent` is the consumed region's
  distance from the apex, every row's pivot.
- `route_pose` takes no anchor from a cylinder: the cone×cylinder arm,
  the only scalar arm a cylinder reaches, is levered from the apex.
- `parallel_radical_plane` reads the axis offset between the axes' feet
  at the germ sites the join connects (`geom_brep::parallel_axes_at`).
- Held by the D10 hold, filed as
  `declared-cylinder-pair-offsets-read-off-the-reach` (parked):
  `chart_region_cyl_offset`. The transfer parameter `c` beside it is
  not a defect (that item says why).
- `carrier_cyl_reach`'s operand-ordered pivot runs on undeclared pairs
  too; it decides the sum and is sound, filed open as
  `carrier-cyl-reach-pivots-on-operand-twos-foot` (P3). `cylinder_data`'s
  offset datum is not a defect: both callers hand it operand 2's foot,
  so it is already read at the reach.
- The cone×cylinder rows still decide the tilt and the offset one at a
  time; `cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time` now
  covers that arm too, at P2.
- The sweep's new hits are filed on their owners' slates:
  `offset-axial-classify-reads-a-stored-origin-against-the-body-axis`
  (OFFSET), `sheet-clip-admits-a-cylinder-at-its-stored-origin-by-an-unlevered-tilt`
  (BAND), `step-adopt-reads-a-plane-origin-against-another-planes-normal`
  (EXCH).
