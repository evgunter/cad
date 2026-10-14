---
id: a-tilted-plug-in-its-hole-refuses-at-the-ellipse-rims
kind: issue
title: A tilted plug back in its hole refuses CurvedPierceUnsupported at every pose: the one-carrier arm's interior question reads no ellipse on the wall's boundary
status: closed
opened: 2026-10-11
priority: P2
cost: M
branch: cleave/one-carrier-reads-ellipses
closed: 2026-10-12
---


Found by `cleave/continuation-box-fallback` (the
`continuation-scan-box-fallback-reads-a-world-axis-run` lane) while
building a continuation that glues across an ellipse.

## What

The fixture is `crates/sweep/tests/continuation_pose.rs`'s
`plug_and_hole` at a tilt `θ > 0`: a radius-0.25 rod along
`(sin θ, 0, cos θ)` through the slab `[−3, 3] × [−1, 1] × [0, 1]`.
The plug is `slab ∩ rod` and the holed slab is `slab ∖ rod`. Putting
the plug back in its hole is valid input with closed forms: the union
is the slab (12), the intersection is empty, and the differences give
back each operand.

Measured on main at `0a700cf7ac`, ε = 1e-9, for θ = 0.3 and π/3, at
the identity and at turns about x, y, z and (1, 2, 3):

- `∪`, `∩` and `plug ∖ hole` refuse
  `CurvedPierceUnsupported { operand: A, face: <hole wall>, edge: <plug seam ruling> }`;
- `hole ∖ plug` refuses the same way on the hole's ellipse rim
  against the plug's wall.

The refusal is the same at every pose. The upright twin (θ = 0, circle
rims) builds every op at its closed form at every pose and every ε
row; that is the plugged pin in `continuation_pose.rs`.

The ladder decides each wall pair `SameOpposite` and each top and
bottom pair `SameOriented`, so the glue door hands the pairs on as
`Rest` and continuation. Declaring the wall pairs `Rest` explicitly
refuses identically.

## Why

A temporary `eprintln!` at each `frontier()` site of
`boolean::reduce::curved_face_arm` showed that all 32 refusals were
raised in `reduce::interior` (`BoundaryCrossing::Unread => Err(frontier())`).
`interior` asks `carrier_cross::boundary_crossing` with
`BoundaryReads::LinesAndCircles`. The hole wall's boundary holds the
two ellipse rims, so the question goes unread. `BoundaryReads` itself
holds the `WithEllipses` reading, which the undeclared lying-on lane
uses.

## Open point

`BoundaryReads::LinesAndCircles` says that the one-carrier arms read
lines and circles only "so the D10 hold leaves their reach where it
was". Since booleans glue on Zero (PR 4496), those arms serve
value-decided pairs, not only declared ones. Whether widening them to
`WithEllipses` is still held is the owner's call to make before the
fix. Neighbours: `tang/an-ellipse-lying-on-a-wall-keeps-the-door`,
where the swept edge is the ellipse; here the ellipse is on the
boundary.

## Built (branch cleave/one-carrier-reads-ellipses)

The open point is settled without Ev. The "so the D10 hold leaves their
reach where it was" sentence was agent-written (PR 4174), not a
ruling. The hold covered units built on declared pairs, which stage 4
F retired, and the one-carrier arms are now D10's own glue-on-Zero
arms, so their reach is ordinary frontier work.

- `carrier_cross::BoundaryReads` is gone. `boundary_crossing` reads
  lines, circles and ellipses in both roles for both callers
  (`reduce::interior` and `reduce::lying_on`). The swept span is
  metred by the carrier's speed at the candidate, read from the
  neutral `crate::carrier_speed`, which `reduce::wall_crossing` and
  `carrier_touch::speed_bound` also read. Each side's reach is its own
  edge's span at the greatest speed.
- `carrier_cross::meetings` reads each carrier pair in one arm, and a
  mirror swaps the roles:
  - line × ellipse meets where the line crosses the ellipse's plane;
  - circle × ellipse meets on the planes' common line;
  - ellipse × ellipse meets on the planes' common line
    (`ellipse_meets_plane`, row `bool_carrier_cross_line_meets_ellipse`,
    levered as the common line's gap from tangency,
    `disc / (2a²b²·√qa)`).

  Two ellipses read coplanar are taken as one curve only where they
  are one within the band (`bool_carrier_cross_ellipses_one_curve`).
  Otherwise they are unread, or escalate in band.

Measured on main at `aa7af40329`: all five ops refuse
`CurvedPierceUnsupported` at every pose, θ = 0.3 and π/3, at ε =
1e-9, 1e-6 and 1e-12. On the branch, at all three ε and every pose:
`plug ∩ hole` and `hole ∩ plug` are empty, `plug ∖ hole` and
`hole ∖ plug` build at their closed forms (tier 3), and the union
refuses `GermEdgeCarrierUnsupported`
(`a-tilted-plug-union-refuses-at-the-germ-segment-and-section-copy-rosters`).
Pinned by `continuation_pose::a_tilted_plug_in_its_hole_reads_alike_at_every_pose`.

The guard that held the old reach,
`a_ruling_lying_on_a_wall::a_continuation_on_a_wall_bounded_by_an_ellipse_keeps_the_door`,
is now `…_builds_every_op`. All six ops build at their closed forms at
all three ε (∪ 2.5π in both orders, t ∖ r 0.5π, r ∖ t π, ∩ π in both
orders), at tiers 3 and 3′, with their census pinned.
