---
id: a-chord-beside-a-spline-wall-is-minted-on-a-scaffold
kind: issue
title: An along-edge chord beside a spline wall is still minted on a line or circle scaffold: the mint certifies no pair or chart image its copy could restate there
status: open
opened: 2026-11-23
---


Found by HELDRECORDS' elbow-seams fix pass (2026-11-23), PR 7471
(ruling 3: fix the stand-in at its minter).

## What

`chord_join::along_edge_spec` now copies an edge with its own at-rest
description, of every carrier kind, except beside a spline: where the
chord divides a spline face, or the edge's description names a spline
wall (an `Intersection`/`TangentIntersection` pair, or a `Chart`), a line
or a circle is still minted as `segment_of_line`/`arc_of_circle`'s
scaffold. Measured on `sweep::a_rim_arc_on_a_cylinder_beside_a_spline_wall`:
the copy's own `Chart { cap plane, image, declared }`, re-homed in the
tube's spline wall (`chord_description`), refuses at the mint
`DescriptionUncertified { Unimplemented }`; keeping it in its own chart
with the declaration dropped refused the same way, untraced further.
With the scaffold, that row and `a_sheath_grafted_on_its_blade` read as
on main. The scaffold is described where it rests by the describe pass
only on a recorded merge skip; otherwise tier 3 refuses it
(`ScaffoldAtRest`), so nothing is silent.

## Owed

Which certificate the mint lacks beside a spline (the plane × NURBS
lane's `limbs`, or a chart image derived on a spline face), and the
copy's own description stated there, so the scaffold branch goes.
