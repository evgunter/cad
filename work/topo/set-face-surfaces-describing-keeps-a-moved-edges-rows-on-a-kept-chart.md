---
id: set-face-surfaces-describing-keeps-a-moved-edges-rows-on-a-kept-chart
kind: issue
title: set_face_surfaces_describing keeps a listed edge's rows on a face that keeps its chart, across a spec that moves the carrier or interval
status: open
opened: 2026-10-06
priority: P3
refs: [set-edge-curve-keeps-a-certified-edges-rows-across-a-reparameterization]
---


Found by the lane that closed
`set-edge-curve-keeps-a-certified-edges-rows-across-a-reparameterization`,
sweeping for doors that keep a certified edge's rows across a write that
moves its carrier or interval. Found by reading; not reproduced.

`Body::set_face_surfaces_describing` (`crates/topo/src/attach.rs`)
re-describes its listed certified edges through `replace_edge_curve`, with
no site mint. A face it re-charts drops its rows (`set_face_surface`'s
rule). A face on the other side of a listed edge that keeps its chart keeps
the edge's row, and a listed spec may move the carrier or the interval:
the offset doors list the re-derived boundary curves of the moved faces
(`replace_face::move_points_then_rechart`, from `replace_faces_offset`,
`offset_together` and `offset_axial`). Those doors close with
`mint_pcurves`/`mint_pcurves_of`, so their results are re-minted. A
consumer calling the public door directly gets the stale row at rest.
Tier 3 reads it (`RowInterval`/`Certify`), but a later site mint on that
face keeps the image, and `pcurves::site_rows`' kept-image `debug_assert!`
fires.

`Body::set_edge_curve` now re-mints in this case: where
`Body::description_moves` reads a move, it plans through
`Body::description_rows` under `Remints::Every`. This door cannot reuse
that plan as is. Its faces move charts in the same write, so a kept
face's plan would have to read the far side's chart as the door leaves
it. The closing options:

- plan the kept-chart faces of moved listed edges through
  `description_rows` (the moved faces' rows are dropped either way);
- or drop the moved listed edges' rows on kept-chart faces, as
  `set_face_surface` drops a moved face's (C4).

The second is what `replace_faces_offset` now does mid-op for every edge
that ends at a vertex it moved.
