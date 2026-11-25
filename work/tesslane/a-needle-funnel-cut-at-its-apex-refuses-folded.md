---
id: a-needle-funnel-cut-at-its-apex-refuses-folded
kind: issue
title: A needle funnel cut 1e-5 above its apex refuses FoldedTriangle or UnclearedSharedSide (then BoundaryEar) at every δ tried: its upper half's cone face
status: open
opened: 2026-11-16
priority: P3
cost: M
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

(TESSLIE, PR 7198, 2026-11-21) `TessellateError::BoundaryEar` is renamed `UnclearedSharedSide` (Python tag `uncleared_shared_side`), because the trimmed lane's rule is now about a shared side, not an ear. The cells above that say `BoundaryEar` were measured under the old name and rule; they are not re-measured here.

## Measured cause, one band (LATHE, 2026-11-23)

With a full revolve's wall one face (`lathe/full-revolve-one-band`), the half
above the cut is one cone face a whole turn round, and the row re-measures
(default ε, debug): `h = 1e-3` refuses `FoldedTriangle` at δ `1e-3` and meshes
at `2e-4` and `3e-5`; `h = 1e-5` refuses `UnclearedSharedSide` at δ `1e-3` and
`FoldedTriangle` at `2e-4` and `3e-5`.

At `h = 1e-3`, δ `1e-3` every folded triangle joins two consecutive points of
the section ellipse (slant `1e-3`) to a lattice node of the first row (slant
`0.0125`) or a later rebuild's node, more than a quarter turn round the axis
from them: `Δu` of 2.3 to 2.9 rad, the far corner near the seam (`u ≈ 0.42` or
`5.86`) and the pair at `u ≈ 2.1 – 4.2`. No row stands between the two:
`AnalyticRow::lattice_rows` lays none within the radius every chord certifies
at, so the CDT in the development, a wedge `2π·sin α ≈ 0.126` rad wide, zips
the small end to the first row. A chord triangle whose near pair sits at a
small radius takes its plane's side from that pair, and its centroid's foot
lies toward the far corner, so past a quarter turn its normal points into the
surface. The census's cure, dropping the triangle's grid corners, re-joins the
pair to the next far node, round after round, to the budget. On a half-turn
face the same zipper reaches half as far round, which is why `h = 1e-3` meshed.

Fixed on `lathe/mesh-budget-one-band` (LATHE's mesh-budget PR): the cone
arm's rows now grade every gap to its lower radius
(`crates/mesh/src/chart_table.rs`, `reach_rows`, read by
`AnalyticRow::lattice_rows`): no gap's slant is longer than the developed arc
an eighth of a turn spans at its lower slant, from the small end outward, so a
triangle across it no longer reaches far round. Both `h = 1e-3` and `h = 1e-5`
then mesh at every δ of the row, both halves, each mesh the body's, the
halves' volumes summing to the cone's within the chord loss
(`reach_probes::a_needle_funnel_cut_at_its_apex_meshes_at_every_delta`).
Splitting each far-reaching fold at its developed centroid instead was tried
and not kept: it clears in 7 rounds against the budget of 6 and then trips the
identified-vertex fan census (#678).
