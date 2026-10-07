---
id: germ-frame-levers-a-plane-cylinder-tilt-at-the-radius
kind: issue
title: the germ frame levers a plane×cylinder pair's axis tilt at the radius, not at the wall's length
status: closed
opened: 2026-10-06
priority: P3
cost: M
closed: 2026-10-07
---


Found by PR 4118's third full review (NOTE-3). Inherited from main.

## What

`boolean::join::pair_section_frame` (`crates/topo/src/boolean/join.rs:1888`)
hands `plane_cylinder_section` a `Reach::Measured` whose length is the
cylinder's RADIUS. `pc_axis_plane_parallel` meters the axis's tilt off
the plane at that lever. On main the same radius went in as the scalar
`extent` (`git show <main>:crates/topo/src/boolean/join.rs`, the
plane×cylinder arm of `pair_section_frame`).

A tilt `θ` moves the section along the wall by `θ` times the distance
from the pivot. That distance is the wall's own extent along its axis,
not its radius. On a wall much longer than its radius, the radius
understates the lever. A tilt the wall's length reads as definite is
then read as Zero, and the frame names the straight chord's frame
(`TangentLine` / `ParallelLines`) for a section that leaves the plane
across the wall. An under-stated lever is the wrong-answer direction.

The lever is the bare radius: `Reach::lever_from`
(`crates/geom-brep/src/extent.rs`) returns a measured length as given,
and the floor at the pivot's distance from `at` lives only in
`Reach::lever_between`, the cylinder pair's reading. `frame_reading`
(`crates/topo/src/boolean/join.rs:1501`) reads at the centre of the
wall's boundary vertices.

The cylinder pair's lever (`join.rs:1856`, the longer of the larger
radius and the walls' span) does reach the walls' length.

## The shape of a fix

Lever the plane×cylinder pair as the cylinder pair is: at the wall's
span, or better at an exact distance from the pivot to the wall's
consumed points (the boundary edges' per-carrier distances,
`Reach::Span`'s rule). Re-measure the rows in `frame_dispatch_tests` that
pin the plane×cylinder frame. The coin row pins that the lever is not
LONGER than the radius on a short wall, and it must stay green.

## Outcome (2026-10-07)

`germ_section_frame` levers the plane×cylinder pair at its wall face's
axial extent from the reading point (`boolean::join::frame_extent`,
`splitting::rules::face_axial_extent`, a conic arc read over the span
it holds), with twice the face's farthest distance from that point as
its reach across the wall (`geom_brep::Reach::Face`). A pair handed to
`pair_section_frame` without its faces still levers at the radius; the
coin row pins that path and stays green. Rows (`frame_dispatch_tests`):
`a_long_walls_tilt_is_levered_at_its_length_not_its_radius`,
`a_short_walls_tilt_is_levered_at_its_axial_extent_not_its_radius`,
`a_rims_bulge_levers_the_germ_frames_tilt`,
`a_partial_rim_levers_the_frame_at_its_arcs_reach_not_the_whole_turn`.
The cylinder pair's span has the same vertex-only blind spot, filed as
`germ-cylinder-pair-span-misses-a-curved-edges-bulge`.
