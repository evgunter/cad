---
id: shell-moves-every-chart-of-a-solid-through-one-simultaneous-door
kind: issue
title: shell moves a solid that is neither planar nor axial chart by chart through replace_faces_offset, so each intermediate section lands on a moved fit's window edge; one general simultaneous door owes every edge as its moved surfaces' section and every corner as their root
status: open
priority: P2
cost: H
refs: [a-wall-seam-between-two-fits-has-no-section, a-saddle-walls-offset-fit-stalls-short-of-the-default-eps]
opened: 2026-10-10
---


Filed from the wall-seam fork (PR 4515). The designed final state is in `a-wall-seam-between-two-fits-has-no-section`'s `## Designed`; this item is its door half.

## The gap

`shell`'s door ladder (`crates/topo/src/shell.rs`, `enum OffsetDoor` and `offset_door`, around :3476-3512) has three doors:
- `PlanesTogether`, when every face is a plane;
- `ChartsTogether`, for an axial body;
- `PerChart`, for anything else: `replace_faces_offset_staged` one face group at a time (around :1440).

Under `PerChart`, each face moves against held neighbours. So each edge is first derived as the section of one moved surface with one HELD surface, and re-derived when the neighbour moves.

On a lofted wall that intermediate section lies `d·cot φ` (φ the interior dihedral) from the moved fit's row. That changes sign at 90°, so on the twisted loft it falls outside the moved fit's window along part of a seam. This is the measured top-rim `NoBranch` at `d = 5e-10` in `a-wall-seam-between-two-fits-has-no-section`'s `## Measured`. Whether a body shells then depends on the order faces are taken and on intermediate bodies nobody asked for.

## Owed

One general simultaneous door for every solid the ladder now sends to `PerChart`:
- It takes `ChartMove`s for all of the solid's charts and mints every new surface.
- Each edge is derived as the section of its two MOVED surfaces through C5. The old carrier seeds the branch and its sense, as `derive_edge` seeds today. An edge is transported where the pair's relative motion carries it rigidly: two planes moving in parallel, or a G1 pair.
- Each corner is the root of the ≥ 3 moved surfaces that meet it, under `solve_corners`' pairwise-agreement rule with every surface moved. This adds fit × spline-carrier roots to the section lane; today it roots only a plane.
- `offset_planes_together` (Cramer) and `offset_charts_together` (the meridian half-plane) are its closed-form arms.
- `replace_faces_offset` stays the single-face verb, no longer `shell`'s driver.
- O4's "anything else chart by chart" sentence (`crates/geom-brep/README.md`; agent text) is reworded with the code.

The door can land before the NURBS × NURBS arm. Against bodies whose moved pairs route today (planes and fits), the loft's rims and corners derive simultaneously, and its wall–wall seams refuse `NeighborPairUnroutable(Nurbs, Nurbs)` until the arm lands. Re-baseline `encl_curved_loft_shell` and `offd_r1_probes`' seam rows to that.

Every planar and axial body must keep its outcome and its stored bits. That needs a merge-base differential.
