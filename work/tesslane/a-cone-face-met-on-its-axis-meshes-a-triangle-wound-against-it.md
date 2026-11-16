---
id: a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it
kind: issue
title: T1's cone joined to a box whose edge crosses its axis meshes a cone triangle that reach_probes' geometry reads as wound against the face
status: open
opened: 2026-11-16
---


## Finding

`reach_probes`' T1 pose (the cone of half-angle `π/6` and radius `1.2`,
turned a quarter about `y`; the box `(0,6)×(0,6)×(−3,3)` turned `−50°` about
`z`) with the box's edge at `(xf·s, height − 1.5·s, 0)`, `xf = 0`: the edge
crosses the cone's axis. For `s = 0.6` and `0.3`, the ops `A ∪ B`, `B ∪ A`
and `A ∖ B` (A the cone): `mesh::tessellate` answers `Ok`, `check_mesh`
passes, and `reach_probes::geometry` refuses the mesh: *a Cone face's
triangle winds against the face*.

| s | op | δ `2e-2` | δ `5e-3` | δ `1e-3` |
|---|---|---|---|---|
| 0.6 | `A ∪ B`, `B ∪ A`, `A ∖ B` | passes | wound against | wound against |
| 0.3 | the same three | passes | passes | wound against |

Each named triangle has a corner at a low shared mesh id (`0`, `9`, `11`): a
vertex of the body, by its id the cone's apex or a corner on the axis. The
other three ops at these poses, and every op at `xf = 0.5, 0.8, −0.5`, pass.
Measured 2026-11-16 on `tesslane/cone-near-apex`, default ε, debug.

## Not this branch's

The same nine cells answer the same way with the cone arm set back to the
chart-box certificate, rows uniform in `v` and the ear rule (main's three,
emulated on that branch; main itself not rebuilt). So neither the cone
certificate, the graded rows nor the shared-side rule makes them.

## Not measured

Which lane emits the triangle; whether it is a fold the lane's winding
census misses (it reads the kernel's normal at the centroid's foot,
`TrimmedFace::outward`) or a reading of the probe's own (`geometry` takes
the surface's closed-form normal at the centroid, and a triangle with a
corner on the axis has its centroid's foot near where that normal turns).
The mesh encloses the body's volume to the chordal bound, by the same
check. No row pins it: `geometry` is a probe-file check.
