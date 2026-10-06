---
id: germ-frame-levers-a-plane-cylinder-tilt-at-the-radius
kind: issue
title: the germ frame levers a plane×cylinder pair's axis tilt at the radius, not at the wall's length
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found by PR 4118's third full review (NOTE-3). Inherited from main.

## What

`boolean::join::pair_section_frame` (`crates/topo/src/boolean/join.rs:1864`)
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

`Reach::lever_from` (`crates/geom-brep/src/extent.rs`) now floors a
measured lever at the pivot's distance from `at`. `frame_reading`
(`crates/topo/src/boolean/join.rs:1477`) reads at the centre of the wall's
boundary vertices, so that floor reaches about the radius, not the
length.

The cylinder pair's lever (`join.rs:1832`, the longer of the larger
radius and the walls' span) does reach the walls' length.

## The shape of a fix

Lever the plane×cylinder pair as the cylinder pair is: at the wall's
span, or better at an exact distance from the pivot to the wall's
consumed points (the boundary edges' per-carrier distances,
`Reach::Span`'s rule). Re-measure the rows in `frame_dispatch_tests` that
pin the plane×cylinder frame. The coin row pins that the lever is not
LONGER than the radius on a short wall, and it must stay green.
