---
id: an-engraved-annular-sector-refuses-seam-orientation
kind: issue
title: A blind pocket shaped as an annular sector refuses SeamOrientation once its arcs sweep 135 degrees or more
status: open
opened: 2026-10-02
refs: [a-round-tube-standing-on-a-plate-refuses-seam-orientation]
---


## What

Engraving a C into a cylinder's cap refuses in the kernel. The
cylinder is the `tiltedcut` scene's (`demos/tour/src/curvedcut.rs`:
r = 1 about the z axis, z ∈ [0, 2.5]). The tool is an annular sector
about (−0.5, 0), radii 0.25 and 0.15, drawn on the xy plane at
z = 2.45 as outer arc (`Center`, Ccw), line, inner arc (`Center`, Cw),
line, and extruded 0.1, so it straddles the top cap and sinks 0.05.
`topo::subtract(cylinder, tool)` returns

`SeamOrientation { a_face: FaceKey(5v3), b_face: FaceKey(11v1) }`

which `crates/topo/src/boolean/zip.rs` raises when a matched seam
cycle's ring half-edges do not run antiparallel to the outer cycle,
and which the error calls a kernel bug. The same tool sunk into the
bottom cap refuses the same way.

## Measured

The sweep of both arcs decides it (same radii, same centre, top cap):

| arcs sweep | span (radians about the centre) | result |
|---|---|---|
| 90° | (π/4, 3π/4), (π/2, π), (3π/4, 5π/4) | builds |
| 120° | (π/3, π), (2π/3, 4π/3) | builds |
| 135° | (π/4, π) | SeamOrientation |
| 150° | (π/6, π), (7π/12, 17π/12) | SeamOrientation |
| 160°, 180°, 200° | centred on π | SeamOrientation |
| 270° | opening +x, opening −x, rotated 0.3 | SeamOrientation |
| 270°, each side as two 135° arcs meeting tangent | opening +x | builds, at the closed-form volume |

So the threshold lies between 120° and 135°, and where the span sits
does not matter. A U whose 180° bowl arcs meet their stems tangent
builds at its closed-form volume, so the sweep alone is not the whole
condition: in every refusing row the arc meets two lines at sharp
corners.

## Where it shows

`tiltedcut` draws its C with each side split at the leftmost point
(`glyph_c`), and pins the one-arc C as wall 4 of `curvedcut::walls`,
which panics when the subtraction builds.
