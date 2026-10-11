---
id: a-tilted-plug-in-its-hole-refuses-at-the-ellipse-rims
kind: issue
title: A tilted plug back in its hole refuses CurvedPierceUnsupported at every pose: the one-carrier arm's interior question reads no ellipse on the wall's boundary
status: open
opened: 2026-10-11
priority: P2
cost: M
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
