---
id: plane-torus-section-gaps-decide-far-coordinates-uncharged
kind: issue
title: The plane×torus section decides its axis-plane and cap gaps on far coordinates' rounding
status: closed
opened: 2026-10-14
priority: P2
cost: M
refs: [torus-circle-arc-classifies-far-frame-carriers-uncharged, no-door-rejects-geometry-past-eps-f64-resolution]
closed: 2026-10-14
branch: germ/pt-section-gaps
pr: 5125
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

## Ruled (2026-10-14): no charge at the section; the far reading is the box's

Weighed by two independent designers and reconciled, then decided by
the orchestrator under Ev's standing discretion (PR 4895). It is not a
design fork at the section, and no ratified text moves.

- **No resolution charge at the section.** A Zero at
  `pt_axis_plane_gap` or `pt_cap_gap` accepts a coincidence and mints
  geometry from it (the meridian pair, the tangent circle). Charging
  the gaps with the coordinates' resolution would widen that
  acceptance with distance from the origin: a tolerance that grows
  with position, which D4 ¶1 forbids.
- **The far readings are not the section's to fix.** Every misread
  above sits at `1e5`–`1e6` m, 100–1000× outside D4 ¶4's 1 km session
  box, which nothing enforces yet. That is
  `work/scalar/no-door-rejects-geometry-past-eps-f64-resolution.md`,
  which now carries a seam note pointing here.
- **Inside the box the section reads as at the origin.** Both
  designers measured turned and axis-aligned frames out to `1e3`–`5e3`
  m at ε `1e-12` and at `EPS_MIN`: every class builds or refuses
  exactly as at the origin. From about `7e3`–`1e4` m `transform_rigid`
  already refuses a turned torus or bar.
- **The frame rule (D10) still applied.** The gaps now read the
  difference of stored points, `d = n·(q − c)` and
  `h = n·(q − c)/(a·n)`, as `plane_sphere_section` and the cone and
  cylinder arms already do. A designer measured, at the box edge at
  `EPS_MIN`, the escalations falling from 4 to 0, and at `3e3` m the
  wrong kinds from 150 to 0.

## Closed (`germ/pt-section-gaps`)

- `geom_brep::plane_torus_section` reads `d = n.dot(q - c)` and
  `h = n.dot(q - c) / a.dot(n)`.
- The section's far-frame row is now
  `ring_path::far_frame_rows::a_torus_circle_the_section_lane_mints_in_the_box_reads_its_kind_at_the_origin`:
  frames out to the box's edge at scales 1 and `1e2`, read at the run's
  ε and at `EPS_MIN`, asserting no other section and no other kind.
  Against the old gaps it finds 6 meridian sections refused at the box
  edge (scale 1, `EPS_MIN`); with the change, none.
- `sweep` `spiric_rim::the_minted_rim_survives_a_rigid_re_pose` holds
  its fields to four ulps of the carrier's reach rather than one ulp
  of each field: the offset moved by `2.8e-17` at `1/128`.
