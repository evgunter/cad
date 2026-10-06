---
id: the-rest-lanes-glue-reads-its-correspondence-unfused
kind: issue
title: The REST lane's glue loop reads its vertex correspondence without the earlier glues' fusions
status: parked
opened: 2026-10-05
priority: P3
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
---

## What

`boolean/rest.rs` `try_rest_union` builds `vmap` (A vertex → B vertex in
result keys) once, then glues the patch pairs in BFS order, each through
`glue_pair` → `zip_seam` or `slit_zip`. Each glue fuses vertices
(`ZipReport::vertex_merges`, B's copy dying into A's), and the next glue
reads `vmap` unchanged: a vertex two patch pairs share at a point (not
along a run, which `slit_zip` takes) is named by its dead B key in the
second glue. `zip.rs` `align` then finds no ring half from it and
refuses `SeamOrientation` or `ZipCorrespondence`.

The seamed pipeline re-reads its map after every zip
(`ops.rs` `fused_through(&vertex_map, &rep.vertex_merges)`); this loop
does not.

## Measured

On the pinch-apex witness
(`work/zip/the-rest-lane-zips-no-pinch-apex.md`), pinch first, before
the lane refused a pinch apex at its correspondence: `SeamOrientation`.
With `vmap` re-read through each glue's `vertex_merges`
(`zip::survivor`), the second glue fused the already fused corner
onward and refused `Euler(SelfLoopEdge)`. That witness now refuses
`PinchApex` before any glue, so no row reaches this loop with a vertex
two pairs share; the row below has to be built. So the re-read
alone is not the fix: `zip_seam` has no reading of a seam vertex its
two cycles already share, where `zip_folded` names one
(`RestZipFrontier::FoldVertexFused`).

## Direction

Re-read `vmap` through each glue's fusions, and give `zip_seam` the
shared-vertex case (or a typed frontier for it), with a two-patch row
that shares one vertex between its pairs.

## Parked on the D10 hold (2026-10-06)

This row is on declared-contact ground, so it waits on `d10-one-way-to-say-intent-is-unbuilt` (`work/join/log.md`, the 2026-10-03 hold). D10 stage 4 retires the declared-REST zip: `work/intent/the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms.md`. When the hold lifts, close this row if its code is gone, or move it to the join if its scene still refuses there.
