---
id: a-needle-funnel-cut-at-its-apex-refuses-folded
kind: issue
title: A needle funnel cut 1e-5 above its apex refuses FoldedTriangle or BoundaryEar at every δ tried: its upper half's cone face
status: open
opened: 2026-11-16
---


## Finding

The needle funnel `revolved[(0,0), (0.02,1), (0,1)]` (half-angle about
`0.02`) split by the plane through `(0, h, 0)` with normal
`(sin 0.3, cos 0.3, 0)`. The half above the cut has a cone face between the
section ellipse, `h` from the apex, and the rim. Measured on
`tesslane/cone-near-apex` (default ε, debug), that half:

| h | δ `1e-3` | δ `2e-4` | δ `3e-5` |
|---|---|---|---|
| `1e-1`, `1e-2`, `1e-3` | meshes | meshes | meshes |
| `1e-5` | `FoldedTriangle` | `BoundaryEar` | `FoldedTriangle` |

The half below the cut meshes at every cell, and funnels of rim radius `0.1`
mesh both halves at every cell. The row
`reach_probes::a_needle_funnel_cut_at_its_apex_refuses_typed`
(`crates/mesh/tests/reach_probes.rs`) pins the `h = 1e-5` row.

Each refusal is typed and loud; no mesh is returned.

## What it is not

Not the certificate and not the lattice's rows: with the cone arm reading
the ear alone (PR 6965's first review head, and its merge base) `h = 1e-5`
refuses `FoldedTriangle` at all three δ, and `h = 1e-3` at δ `1e-3` and
`2e-4` as well. `h = 1e-3` meshes since the arm's shared-side rule
(`mesh::trimmed`, `attempt`) stopped splitting ears at a vertex between two
different neighbours.

## Cause

Not measured. The winding census (`TrimmedFace::attempt`) drops the lattice
corners of a triangle wound against the surface and rebuilds; here it ends
at a fold with no lattice corner to drop, or at the round budget. The
section ellipse is `1e-5` from the apex of a cone whose development spans
`0.06` rad for the half turn, so in the development the whole small end is
a few `1e-5` across and a few `1e-7` wide: a likely place for a triangle
between boundary points to turn over the chord beside it, and for the one
neighbouring plane cap to hold both ends of most sides there (the
`BoundaryEar` cell). Related to
`a-needle-cone-face-meshes-thin-triangles-beside-its-rulings.md` (the same
geometry, accepted meshes) and not the same row: these refuse.
