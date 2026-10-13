---
id: crossing-equal-cylinders-take-the-parallel-radical-plane-and-desync
kind: issue
title: A germ between two equal-radius cylinders on crossing axes takes the parallel pair's radical plane and refuses JoinDesync
status: closed
opened: 2026-10-12
priority: P2
cost: M
branch: cleave/crossing-cylinders-join
closed: 2026-10-13
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

## Built (branch cleave/crossing-cylinders-join)

Measured first on `1b2ae6356d`, at ε = 1e-9, 1e-6 and 1e-12, at spins
`(0.4, 0.3)`, `(1.0, −0.7)` and `(0, 0)` and with `B` from `x = 0.5`:
all 24 runs refuse the row's `JoinDesync`. A probe at the `Cylinders`
arm showed the germ is `OnEdge` on the ellipse's solid and `InFace` on
the other's. `germ_section_loops` therefore reads the edge's ellipse
frame (`along_edge_conic`), and the face-pair frame dispatch, which
reads the axes, is never asked of this pair. `kind_arm` then sent the
pair to `Cylinders` on kinds alone.

The fix is in `boolean/join.rs`:

- `segment_arm` reads each segment's kind arm in `bool_connect` before
  any surgery. A cylinder pair keeps `KindArm::Cylinders` only when its
  axes read parallel. Otherwise it falls through to the edge-plane
  lane.
- The axes are read by `cylinder_pair_axes`, the frame dispatch's own
  reading, factored out of it. Both callers read from the same point
  and lever (`frame_lever`).
- In band, the reading escalates `Escalated { Coincidence(Section,
  Moot) }`, the frame's own refusal for that margin. Its lever is filed
  as `cleave/an-in-band-axis-parallelism-offers-a-proximity-lever`.

The scene now reaches the edge-plane lane and refuses at its copy of
the ellipse edge, at every ε and pose above. That copy is filed as
`cleave/the-edge-plane-lane-copies-no-ellipse-edge`, with what lies
behind it.

Pins:

- `crates/sweep/tests/crossing_equal_rods.rs` holds every op to that
  exact ending, under that row's id.
- `parallel_cylinder_join::seams_lying_on_the_walls_read_the_axes_at_the_walls_span`
  reaches the in-band arm through a door. Two parallel rods, each
  rod's seam ruling lying on the other's wall, B tipped so the tip reads
  in band over the walls' span and parallel at the radius. The pin goes
  red when the escalation is read as parallel, and when the join reads
  at the radius.
- Unit row:
  `frame_dispatch_tests::cylinder_pair_axes_read_parallel_apart_and_in_band_by_their_lever`.
