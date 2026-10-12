---
id: crossing-equal-cylinders-take-the-parallel-radical-plane-and-desync
kind: issue
title: A germ between two equal-radius cylinders on crossing axes takes the parallel pair's radical plane and refuses JoinDesync
status: review
opened: 2026-10-12
priority: P2
cost: M
branch: cleave/crossing-cylinders-join
---



Found by `cleave/one-carrier-reads-ellipses` while looking for a
fixture for `tang/an-ellipse-lying-on-a-wall-keeps-the-door`.

## What

Two rods of radius 1 on crossing axes, the shape that row names:

- `A`: a z-axis rod from `z = −3` to `3` (turned `0.4` rad about `z`)
  less the half-space under the mitre plane `z = x`, so `A`'s bottom
  rim is the ellipse `z = x` on its wall;
- `B`: an x-axis rod from `x = 0.2` to `3` (turned `0.3` rad about
  `x`). Its wall holds `A`'s ellipse, since `z = x` is a bisector plane
  of the two walls; `B`'s end cap at `x = 0.2` keeps the second ellipse
  `z = −x` and the walls' two tangent points `(0, ±1, 0)` out of both
  operands.

Valid input with closed forms: `vol A = 3π`, `vol B = 2.8π`, and
`A ∩ B = [x·acos x − √(1−x²) + (1−x²)^{3/2}/3]` from `0.2` to `1`
(about `0.39237`).

Measured at ε = 1e-9 on `cleave/one-carrier-reads-ellipses`: all six
ops (`∪`, `∩` and `∖`, both orders) refuse
`JoinDesync { what: "a parallel cylinder pair's radical plane cut a
wall in a conic" }`. The same refusal at spins `(1.0, −0.7)` and
`(0, 0)` and with `B` from `x = 0.5`. On main at `aa7af40329` the
same scene refuses earlier, `CurvedPierceUnsupported` on `A`'s ellipse
against `B`'s wall: the lying-on lane did not read an ellipse. That
branch admits it, and the reduction now runs to the join.

## Why

Read from the code, not instrumented: `join::kind_arm` sends every
`(Cylinder, Cylinder)` germ pair to `KindArm::Cylinders`, whose doc
says "parallel axes only", and a pair with a kind arm keeps it (the
edge-plane lane refuses a pair that has one). `parallel_radical_plane`
reads the axes "as unit and parallel, as the frame dispatch read them
before admitting the pair (`pair_section_frame`)", takes the axis
offset from `geom_brep::parallel_axes_at`, and so names a plane these
walls do not meet in; each wall's section table cuts that plane in a
conic, and `rulings_are_straight` refuses `JoinDesync`. Its doc says
"No pose reaches either refusal through a public door", which this
scene falsifies. Whether `pair_section_frame` was asked of this pair,
and answered, before the join is not measured: its own row says
intersecting axes take the pinch door
(`the_non_parallel_cylinder_pair_splits_on_coplanarity_alone`), so the
first thing to measure is which door admitted the pair.

## Direction

The germ here runs along `A`'s ellipse edge inside `B`'s wall, so it
is the edge-plane lane's shape (`KindArm::None`'s
`GermLane::EdgePlane`: "the germ runs along a conic edge of one solid
in a curved face of the other"). The fix is either a kind arm that
reads the axes before taking `Cylinders`, so a coplanar crossing pair
goes to a lane that holds its plane sections, or a typed frontier
refusal where no lane does. A `JoinDesync` is a kernel invariant, and
valid input must not reach one.
