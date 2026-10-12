---
id: continuation-scan-box-fallback-reads-a-world-axis-run
kind: issue
title: The undeclared-continuation scan's box fallback reads a long world-axis overlap of two ellipse or spline edge boxes as a shared curve, so its refusal depends on the pose
status: closed
opened: 2026-10-06
priority: P1
cost: M
closed: 2026-10-11
pr: 4626
---


Found by the operand-gate pose lane
(`boolean-operand-gate-separates-only-along-world-axes` (REACH, closed by PR 4122))
sweeping `topo/src` for box overlaps read as a verdict. Unmeasured:
read off the code; no fixture has been built for it.

## What

`boolean::reduce::refuse_undeclared_continuations` asks whether two
faces of a same-oriented undeclared carrier pair meet along their
boundary, edge pair by edge pair, behind an edge-box overlap
(`bx.overlaps(&by)`). `edges_share_a_curve` decides it point-on-edge
through `Decide` wherever both carriers have a point parameter. For an
ellipse or a spline edge it falls back to the boxes: they share a
curve when the two padded world boxes overlap on every axis and run
past `POINT_TOUCH_RUN` pads on one. The run along a world axis is a
fact about the pose: two edges that touch at one point and leave it
along a direction off every axis overlap by more than four pads on the
axis they lean along, so the scan refuses an undeclared coincidence
that turning the pair would not.

## What would close it

A pose-free reading of "shares a curve" for the carriers with no point
parameter: the run measured along the edges' own direction (the chord
between an edge's ends), or a point parameter for the ellipse. The
refusal direction is unchanged either way (over-reporting a meeting
refuses).

## Built (branch cleave/continuation-box-fallback)

**This no longer reproduces: the scan is gone.** `refuse_undeclared_continuations`,
`edges_share_a_curve` and `POINT_TOUCH_RUN` were deleted by
`c6e9e3d95b` ("booleans glue on Zero"), which reached main in PR 4496
(merge `80fc1b19ad`, 2026-10-10). An undeclared pair that the ladder
decides is one carrier is now glued by `boolean::glue::decided_declarations`.
A face-pair row is kept only where the reduction placed a cell of one
face's closure on the other's (`glue::touched`, from the reduction's
pending rows rather than boxes). No box run decides anything in its
place.

**The row's premise, read against the deleted code** (`c6e9e3d95b^`):

- an ellipse never reached the fallback. `Curve3::param_near` has
  answered an ellipse since `e808caa449` (2026-08-29). Only a `Nurbs`
  carrier or an uncertified edge curve reached it.
- each of those has a poison edge box (`edge_box_rule` →
  `NoSoundBox`). `f64::min`/`max` drop the NaN, so the run on each
  axis was the other edge's own extent. The fallback therefore read
  "shares a curve" for almost every pair, at every pose. It was an
  over-refusal that did not depend on the pose, not a pose-dependent
  one.

**Pinned:** `crates/sweep/tests/continuation_pose.rs`, a rotation family
of 13 poses (the identity, plus three turns about each of x, y, z and
(1, 2, 3)). Every op, in both orders, must build at its closed form,
with readings equal to the identity's.

- **apart**: a plug cut by a slab from a rod tilted π/3, so its top
  and bottom are bounded by ellipses, beside a brick a 0.1 gap past
  it on the same planes. The world boxes of the ellipse and the brick
  edges overlap at turned poses only. No face-pair row may be
  recorded at any pose.
- **plugged**: the upright plug back in its hole. The union is the
  slab (6 faces, 6 face-pair rows) at every pose.

Both pins pass at ε = 1e-9, 1e-6 and 1e-12. As a mutation check,
replacing `glue::touched` with the unfiltered rows turns the apart pin
red at its first turned pose (`0.3 about z`: 1 face-pair row where the
identity has 0).

Filed: `cleave/a-tilted-plug-in-its-hole-refuses-at-the-ellipse-rims`.
Added evidence to `restread/at-infinity-probe-measures-in-closed-form-only`.

## Closed (PR 4626, 2026-10-11)

The scan no longer exists (glue-on-Zero, PR 4496), so this does not
reproduce. The pose-independence it asked for is pinned by
`crates/sweep/tests/continuation_pose.rs` (above).
