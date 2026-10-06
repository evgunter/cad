---
id: an-engraved-annular-sector-refuses-seam-orientation
kind: issue
title: A blind pocket shaped as an annular sector refuses SeamOrientation once its arcs sweep 135 degrees or more
status: closed
opened: 2026-10-02
priority: P0
cost: H
refs: [a-round-tube-standing-on-a-plate-refuses-seam-orientation]
parent: JOIN-3
closed: 2026-10-02
---


## What

A normal verb fails on normal geometry. Engraving a C into a
cylinder's cap refuses in the kernel. The cylinder is the `tiltedcut`
scene's (`demos/tour/src/curvedcut.rs`: r = 1 about the z axis,
z ∈ [0, 2.5]). The tool is an annular sector about (−0.5, 0), radii
0.25 and 0.15, drawn on the xy plane at z = 2.45 as outer arc
(`Center`, Ccw), line, inner arc (`Center`, Cw), line, and extruded
0.1, so it straddles the top cap and sinks 0.05.
`topo::subtract(cylinder, tool)` returns

`SeamOrientation { a_face: FaceKey(5v3), b_face: FaceKey(11v1) }`

raised in `crates/topo/src/boolean/zip.rs` when a matched seam
cycle's ring half-edges do not run antiparallel to the outer cycle.
The variant's doc and its `Display` in `crates/topo/src/boolean/mod.rs`
call that a kernel bug.

## Measured

The same one-arc C refuses `SeamOrientation` in every role tried
(SHOW lane and its review, 2026-10-02):

| pose | result |
|---|---|
| pocket in the cylinder's top cap | SeamOrientation |
| pocket in the cylinder's bottom cap | SeamOrientation |
| pocket in a planar box's top face | SeamOrientation |
| pocket in a planar box's bottom face | SeamOrientation |
| through-cut of the cylinder | SeamOrientation |
| union, as a boss on the cylinder's cap | SeamOrientation |

The arcs' sweep decides it, at a threshold that moves with the radii
(pocket in the cylinder's top cap; spans placed anywhere about the
centre):

| radii | builds | refuses |
|---|---|---|
| 0.25 / 0.15 | 90°, 120°, 125° | 130°, 135°, 150°, 160°, 180°, 200°, 270° |
| 0.4 / 0.1 | 90° to 150° | 180°, 270° |

Each side drawn as two arcs meeting tangent at the apex builds at the
closed-form volume at every sweep and both radii pairs. A U whose 180°
bowl arcs meet their stems tangent also builds, so in every refusing
row the arc meets two lines at sharp corners.

## Where it shows

`tiltedcut` draws its C with each side split at the apex
(`c_outline(Sides::SplitAtApex, ..)`), and pins the one-arc C as
wall 4 of `curvedcut::walls`, which panics when the subtraction builds.

## Closed (JOIN-3, PR 3895)

Claimed from ZIP by JOIN-3, whose fix it is. The C's match on the cap's
ring closed its run with the straight chord, and past a sweep near
130° that chord winds the run the other way round from the arc the
join mints, so the ring lane picked the wrong role order and the zip
found the seams parallel. With the run closed by the segment's own
curve (`chord_join::SegmentCurve`, `loop_winding::RunClosing`) every
pose in the table builds at its closed form, through tiers 2 and 3′
and the at-rest certificate: the top cap, the bottom cap, a box's top
face, a through cut and a boss, at sweeps 135°, 180° and 270°, radii
0.25 / 0.15 and 0.4 / 0.1
(`sweep/tests/axis_lap.rs`, `an_engraved_one_arc_c_builds_at_every_sweep`).
Closing the run with the straight chord again turns that row red with
`SeamOrientation` at the first pose. `tiltedcut` draws its C with one
arc per side, and its wall 4 is retired.
