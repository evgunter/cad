---
id: plane-torus-section-gaps-decide-far-coordinates-uncharged
kind: issue
title: The plane×torus section decides its axis-plane and cap gaps on far coordinates' rounding
status: open
opened: 2026-10-14
priority: P2
cost: M
design: true
refs: [torus-circle-arc-classifies-far-frame-carriers-uncharged]
---


## What

`geom_brep::plane_torus_section` (`crates/geom-brep/src/intersect.rs`,
~:2807 and ~:2886) decides two gaps that are differences of far
coordinates, with no charge for their rounding:

- **`pt_axis_plane_gap`**: `d = n·q − n·c`, the plane's stand-off from
  the torus's centre;
- **`pt_cap_gap`**: `r − |h|`, with `h = (n·q − n·c)/(a·n)` the depth
  of the plane's crossing of the axis.

At coordinates `1e5`–`1e6` each of `n·q` and `n·c` is rounded to about
`1e-11`–`1e-10`, past the zero band at ε `1e-12`. So a plane holding
the torus's axis, or touching it along its top or bottom circle, reads
as another section.

## Measured

`ring_path::far_frame_rows::a_torus_circle_the_section_lane_mints_far_reads_its_kind_at_the_origin`
prints the count. It cuts the meridians, parallels and crests of a grid
on the torus `R = 2`, `r = 0.8` by their own planes. Each plane is stored
at the circle's centre or 3.7 units along the plane from it. The frames'
axes are moved to twelve origins `1e5` to `1e6` off the origin, at scale
1 and `1e3`. At ε `1e-12`:

| at the origin | far | count |
|---|---|---|
| meridian circles (3456) | spiric ovals | 984 |
| meridian circles | refused (escalated) | 312 |
| tangent circle (576) | concentric circles `R ± w`, `w` about `1e-5` relative | 102 |
| tangent circle | empty | 72 |
| tangent circle | refused (escalated) | 94 |

None differ at ε unset or `1e-6`. The ovals and the concentric pair are
faithful to the stored data: the plane really does stand off the axis
or cross into the tube by the rounding. `Empty` drops a section the
plane touches along. What a boolean does with each has not been traced
past the section.

## The question

This is the same class as the ring-path far-frame fix
(`ring-path-readings-decide-past-their-resolution-at-far-frames`, PR
5012), but here a Zero verdict *accepts* a kind (the meridian pair, the
tangent circle). Charging the gaps with their resolution widens what is
accepted: a plane within the coordinates' resolution of the axis would
be read as holding it. That needs its own argument, as it does in
`torus-circle-arc-classifies-far-frame-carriers-uncharged`. The
alternative is to stand down (escalate) inside the resolution. Hence
`design: true`.
