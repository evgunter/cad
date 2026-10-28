---
id: a-trimmed-sphere-face-holding-a-pole-inside-it-refuses
kind: issue
title: a trimmed sphere face that holds a chart pole inside it, or visits a vertex at two chart locations off a pole, refuses at the sphere arm
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Filed by TESS's sphere-arm unit
(`sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane`), as
the residue of its pole decision.

**The decision.** The trimmed lane's sphere arm meshes a face that
meets a chart pole only at a loop vertex: there the polygon holds the
pole at the azimuth of each meridian that meets it, so the pole is a
side `v = ±π/2` of the trim polygon, and the CDT's triangles on that
side collapse to a fan (`mesh::trimmed::sphere_polygon_shape`,
`pole_side_columns`). Witnesses: every half-band remnant of the
`y`-poled balls in `crates/sweep/tests/tilted_sphere_pair.rs` and
`snowman.rs`, and the oblique fillet corners, whose chart poles on one
foot (`crates/sweep/tests/m5_pr12_fix_pass.rs`,
`the_oblique_corners_mesh`).

**What refuses, typed** (`UnsupportedCurve`, naming the face's first
tilted circle):

- a face holding a pole INSIDE it: its loop winds about the pole, so
  its image does not close in the chart (a polygon side turning more
  than 3π/2 in azimuth);
- a face whose loop visits one vertex at two chart locations off a
  pole (a seam traversed twice);
- an image running past a pole mid-edge (`|v| > π/2`).

None of the unit's witnesses reaches these (the kernel's balls are two
half-bands whose seam meridians run pole to pole, so their faces meet
a pole at a vertex), and no in-tree row pins one: they are backstops,
not measured frontiers, until a body reaches one.

**What a fix owes.** A trim polygon for a face around a pole: a
face-local sphere chart with its poles off the face (the chord points'
3-D positions read through that chart's closed-form inverse), or the
curved lane's pole-cap handling (`curved::pole_columns`) carried into
the trimmed lane's polygon.
