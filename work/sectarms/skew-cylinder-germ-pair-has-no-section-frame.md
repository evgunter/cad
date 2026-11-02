---
id: skew-cylinder-germ-pair-has-no-section-frame
kind: issue
title: Two cylinder walls with skew axes, or coplanar axes and unequal radii, refuse GermFrameUnsupported: cylinder x cylinder is not on the section value
status: dispatched
opened: 2026-10-04
priority: P1
cost: H
refs: [4025, cylinder-sphere-germ-pair-has-no-join-lane]
pr: 6139
branch: sectarms/cylinder-pair-on-the-value
---

Found by the cylinder × sphere frame lane's sweep of the germ pairs that
reach `FrameError::NoArm` (branch `join/cylinder-sphere-frame`, 2026-10-04).

## Measured

Poses (`kind_pair_sweep_probe`, run on this branch and not committed:
shapes at the origin, B moved by a rotation then a translation, ∪ in both
member orders): a drum `cyl(0.5, 1.0)` (radius 0.5 about `z`,
`z ∈ [−1, 1]`, `common::germ_pair`), a ball of radius 0.5 revolved about
`y`, a donut revolved about `y` (major 0.6, minor 0.2), a frustum, and a
brick `[−0.4, 0.4]³`; the six placements are rotation 0 then
`t = (0.45, 0.1, 0.05)`; 1.5707963 rad about `x` (a hair short of a right angle), `(0.3, 0.2, 0.1)`; 0.7 rad about
`x`, `(0.1, 0.35, 0.2)`; 1.1 rad about `(0.3, 1, 0.2)`,
`(0.42, −0.15, 0.3)`; 0.4 rad about `y`, `(0.6, 0, 0)`; 0.9 rad about
`(1, 1, 0)`, `(0.05, 0.1, 0.02)`.

Drum against drum, placements 2, 3 and 5 (skew axes), refuse
`GermFrameUnsupported { a_kind: Cylinder, b_kind: Cylinder }` in both
orders (6 lines). So does placement 1 turned by exactly `π/2` (the PR
4025 review's `skew_placement_one`: perpendicular axes 0.3 apart), in
both orders, for 8 lines over four skew placements; at the probe's
1.5707963 rad that placement stops earlier, at
`CurvedSectorSideUnsupported` (a sector side decided Zero at a margin
of 7.5e-17). All are raised by `boolean::join::pair_section_frame`'s coplanarity
split (the section table's `cc_axes_coplanar`, which the frame asks;
`bool_germ_frame_axes_coplanar` when this was filed. Definite: skew keeps
`NoArm`).
Placement 0 (parallel axes) passes the frame and stops at the lane
(`parallel-cylinder-germ-pair-has-no-join-arm`, JOIN, closed by PR 4031); placement 4
meets the crossing pair's nodes (built since TANG U1; `NodeUnbuilt` where a node is not).

## What a fix has to supply

A frame the skew pair's quartic turns about once per loop. With radii
`r₁ ≤ r₂` and axis gap `e`, a section with `e + r₁ < r₂` is two loops,
each a graph over the thinner cylinder's whole circle, so it turns about
the thinner axis — the argument `cs_transverse_frame` makes for the
two-loop cylinder × sphere section, and the same frame shape. The
one-loop case (`|r₂ − r₁| < e < r₁ + r₂`) needs its own proof that some
axis (the common perpendicular is the candidate) sees the loop turn
monotonically. Past the frame, the pair meets the same lane door as the
cylinder × sphere pair (`cylinder-sphere-germ-pair-has-no-join-lane`):
no chord lane for a non-planar section.

## Survey (2026-10-31, SECTARMS orchestrator)

Re-measured on main at b357165a (a survey probe, uncommitted):

- p0 (parallel) and p4 (the crossing pair's nodes) build.
- p2, p3 and p5, p1 at exactly π/2, and one more skew pose refuse
  `GermFrameUnsupported { Cylinder, Cylinder }` in both orders.
- p1 at 1.5707963 rad stops earlier, at a first-order sector side
  (filed: `apex/a-near-right-angle-cylinder-pair-stops-at-a-first-order-sector-side`).

**Coplanar axes with unequal radii are a second unserved family.**
`intersecting_cylinder_axes` maps the closed-form door's refusal of that
pose (`SectionError::UnequalRadii` at the survey; `RoutesToGeneralRung`
since PR 6139) to `NoArm`.

**The design retires the frame this row asked for.** The fix is to put
cylinder × cylinder on `geom_brep::pair_section`. The tangency margins are
closed form: `e − (r₁+r₂)` and `e − |r₁−r₂|`, with `e` the gap along
`a₁×a₂`. The components are counted by those margins, and the march comes
from the existing implicit arms. That move also retires two second
readings of the pair's coincidence and tangency: `glue.rs`
`crossing_cylinder_margins` and `section_cert::cylinder_cylinder`.

**A design question sits under it, so `design: true` and re-costed H.**
Every pair on the value today has a bounded side (a sphere, or a torus's
`R + r` ball), and C5 says no face's extent enters a value keyed by
surface pair. Two cylinders have no bounded side:

- parallelism can only be decided against a length;
- a nearly parallel crossing pose has a section ball of about `r/θ`;
- the parallel pose's components are open lines, which `carry` refuses.

A second question goes with it: whether the equal-radius bisector pose,
whose ellipses cross at two nodes, lives on the value with nodes. Moving
it there as the value stands would regress p4. The cone-frames row
(`cone-germ-pairs-against-a-curved-face-have-no-section-frame`) has the
same unbounded-pair question.

## Decided (2026-10-31, Ev on `[ev]` PR 6127; fork row 224, revised by 226)

Cylinder × cylinder goes on the section value in every pose, decided once
on closed-form margins:
- parallelism as `sin θ · L`, at the radius `L` of the operation's region;
- crossing axes: the gap `e` against `r₁ + r₂` and against `|r₁ − r₂|`;
- parallel axes: the axis distance against the same two levels.

A pair traced in ℝ³ is read over its own ball cut by one region per
operation: the ball about the operands' overlap, padded. No face's extent
enters it.

Components are closed, end on the domain's boundary, or end at a node.
Every node is a recorded Zero tangency placed in closed form. The bisector
ellipses carry, so p4 keeps building. The pinch refuses typed. No march
that starts at a node is built.

The build retires five readings: the frame's cylinder arm,
`cylinder_pair_axes`, `crossing_cylinder_margins`,
`section_cert::cylinder_cylinder` and `section_cert::Reach`. It shares the
component-with-ends type with the NURBS arm (PR 6121), so that type lands
once.

Build-time items:
- `of_pair_section` must read a node's connected set as one component;
- reading a pinch as the guard's `Touch` needs its own argument;
- the march door seeds on the thinner wall's chart cut by the region;
- seeding uses the pair's scale, not the region's.

## 2026-11-02: what stage 3 retires (stage 2 review, PR 6151)

Stage 2 left two staging seams for stage 3 to retire:
- **The join's gate.** `SectionTable::join_reads` (through
  `held_at_the_join` and `carried_at_the_join`) keeps two cylinders off
  the join's readings of the table, so the join still reads them through
  its frame and bisector lanes.
- **The frame lane's cylinder reading at `frame_lever`.** That lane
  (`cylinder_pair_axes`, `cylinder_pair_frame`, `crossing_cylinder_reading`
  for `node.rs`) reads at the faces' reach, while the table's records
  come from the pose read across the operation's region. In band the two
  can disagree until stage 3 makes the join read the pose.

Stage 3 also owns the per-arc `winds` of the bisector arcs, which store a
fact about their node-joined set.

## 2026-11-02: the ruling sweep (branch `sectarms/cylinder-ruling-sweep`)

Every crossing pose now counts. The thinner wall's ruling sweep
(`pair_section::ruling`) lists one loop or two, cut by the region:
- arcs between certified cut ends;
- loops the region holds whole, each witnessed.

The march seeds from the witnesses and ends at the region's ball. The
carry matches the branches one to one, which closes exhaustiveness. The
provisional `Counted`/`Marched` split by the pair's ball is gone.
Stage 3 (the join) is next. Its seams are unchanged: `join_reads`, and
the frame lane's reading at `frame_lever`.

