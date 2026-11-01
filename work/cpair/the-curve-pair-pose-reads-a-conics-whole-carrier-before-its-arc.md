---
id: the-curve-pair-pose-reads-a-conics-whole-carrier-before-its-arc
kind: issue
title: The curve-pair pose reads a conic's whole carrier before its arc: two arcs 2 m apart on near-one circle, and a line beside a circle whose near touch is off the arc, escalate
status: open
opened: 2026-10-18
priority: P3
cost: M
---

## What

Found by the class sweep of RESTREAD's
`census-meet-reads-two-segments-on-one-line-two-metres-apart-as-coincident-lines`
(branch `restread/meet-apart-on-one-line`). That unit made the router
(`geom_brep::curve_pair::pose`, `route`) read the gap between the two
spans' projections on the line before the tilt and the offset, so two
segments on one line far apart, or a line far along from a conic's
extent, read `Pose::Apart` instead of escalating on a carrier
reading. The same shape remains wherever the pose reads a conic as its
whole carrier, not its arc:

- **Two arcs of near-one circle, far apart along it.** The unit
  circle over `(0, ½)` and a circle `3ε` larger on one centre over
  `(π, π + ½)`, at the `meet_rows` band (`ε = 10⁻⁹`, `K = 10`):
  `conic_conic` asks `one_curve` first, over the whole carriers, and
  escalates `curve_pair_pose_one_curve` at `3.0·10⁻⁹`, though the arcs
  are about two metres apart.
- **A line in a circle's plane beside it, its near touch off the arc.**
  The line `y = 1 + 3ε` over `(−½, ½)` against the unit circle's arc
  `(π, π + ½)`: the projections overlap, so the gap does not decide,
  and `line_conic`'s in-plane first harmonic is read over the whole
  circle, escalating `curve_pair_pose_extreme` at `−3.0·10⁻⁹` at the
  circle's top, about 1.4 to 1.7 m from the arc.

Both measured with a scratch row in `meet_rows` on that branch; its
review re-measured them with this row, the fix's red-on-main witness
(both `meet` calls refuse `MeetFault::Escalated` on that branch):

```rust
let unit = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0);
let big = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0 + 3.0 * ZERO);
let arcs = meets((&unit, (0.0, 0.5)), (&big, (PI, PI + 0.5)));
let l = line([0.0, 1.0 + 3.0 * ZERO, 0.0], [1.0, 0.0, 0.0]);
let beside = meets((&l, (-0.5, 0.5)), (&unit, (PI, PI + 0.5)));
```
Neither is seen on the near-tangent probe's grid, so the weight is the
shape, not a count. Each refuses typed; none misanswers.

## The shape to give

Read whether the near locus the carrier reading is about lies on both
spans before escalating it: an escalated extreme or one-curve margin
whose locus (the extreme's angle, or the arcs' own extents) is
definitely off a span says nothing about the spans. The two arcs'
chordal hulls apart past the band, or the extreme's angle definitely
outside the arc together with both arc ends definitely clear, are the
candidate certificates.

## Evidence: a census row reaches it (2026-11-01, `splitdoor/rims-seam-orientation`)

In the same bodies as `two-coplanar-ellipses-have-no-meeting-method`'s 2026-11-01 evidence (`sheets_part_on_slanted_rims::rims_crossing_on_tangent_walls_build_every_op`), the section arc piece from `(1, ±1, z)` to `(0, 0, 0)` lies about a metre from its carriers' touch at `(2, 0, 0)`. Read against each piece of B's rim, it still escalates `curve_pair_coplanar_sub_side` (`MeetFault::Touch`). The touch is read on the whole carriers, off that arc's span.
