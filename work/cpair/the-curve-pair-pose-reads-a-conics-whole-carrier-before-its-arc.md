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

## Evidence: the coplanar touch no longer escalates off the arc (2026-11-01, `tangrim/coplanar-conic-touch`)

The 2026-11-01 census instance above no longer escalates. The coplanar subdivision now places the carriers' touch as a candidate (`two-coplanar-ellipses-have-no-meeting-method`), and the candidate is read on both spans, so a touch off the arc reads no meeting. `sheets_part_on_slanted_rims::rims_crossing_on_tangent_walls_build_every_op` passes tier 3′ on every op at three ε. The row's own two poses (two arcs on near-one circles, and a line beside a circle) take other arms and are unchanged.

## Evidence: a loft door's clearance reaches it on embedded bodies (2026-11-02, PR 6141)

CARVE's loft stacking door (`crates/sweep/src/loft/stacking.rs`,
`arc_pair_meets`) reads each pair of segments of a slab's two
sections through `meet`, on their placed arc spans. Two arcs whose
carriers come near each other off both spans refused there. The
witnesses are from that PR's second review
(`zz_pin_review2_probe.rs`'s `probe_j` and `probe_i`):

- **The D whose arc's circle is tangent to the axis on the half no section holds.**
  Section 0 is the D on the line x = 0 from (0, 1) to (0, −1), with
  its semicircle through (1, 0) in z = 0. Section 1 is the same D
  turned −0.1, −0.4 or −1.2 rad about the axis through (−1 − d, 0, 0)
  along (0, cos t, sin t), for t ∈ {0, 0.3, 0.6, 1.0}. The arc's carrier
  circle touches that axis at (−1, 0, 0) when d = 0, on the half the
  arc does not hold. The sections are apart by about the turn: 0.05 to
  1.13. `meet` on the two arcs refuses `MeetFault::Unsettled`
  (the meter) at d = 0, and `MeetFault::Escalated`
  (`curve_pair_pose_extreme`) at d = 1e-8 to 1e-7.
- **The same pose near a real tangency is not monotone.** The unit
  circle elbow turned about the axis through (1 + d, 0, 0) along
  (0, cos 1, sin 1), by 0.1 rad: its rims are 1.6e-8, 5.4e-8 and 1.6e-7
  apart at d = 3e-7, 1e-6 and 3e-6. `meet` read apart, then refused
  without a verdict, then apart again.

The loft door now decides nothing on an arc pair `meet` cannot settle,
so these bodies build. Its refusals near the band therefore wait on
this row. The question for CPAIR: does span-restricted posing (this
row's "shape to give") belong in the reader, so a caller handing
`meet` two spans gets a verdict about the spans and not the carriers?
