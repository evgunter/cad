---
id: pick-index-triangle-table-rebuilt-every-build
kind: issue
title: the pick index copies every triangle and recomputes every box on every build, memo hit or miss
status: open
opened: 2026-09-12
parent: PERF-10
---



## The residue

`MeshPick::assemble` (`crates/editor-core/src/resolve/pick.rs`) walks
every triangle of every patch on every build: it copies the three
corners into the pick table (72 bytes a triangle) and recomputes each
triangle's box, whether the patch's tree is then served from
`PickMemo`'s tree level or built. The memo therefore saves
`Bvh::build` only. Measured (PERF-9, release, tour die's one-pip
edit, 85 of 89 patches hit): `MeshPick::build_with` 36 ms against
`MeshPick::build` 62 ms — the 36 ms residual is the table copy, the
box recompute and the four missed trees, and it is now the largest
term of the memo-primed index (34 ms total; tessellation 15).

## What a fix is

Memoise the patch's pick table and its boxes WITH the tree, under the
same key: a `PickPatch` served whole (tree, corners, boxes, face) makes
a hit O(1) per patch and leaves the top-level tree over `#faces` boxes
as the only per-build work. The hit test then compares nothing per
triangle — the digest plus `Bvh::is_over` on the stored boxes still
prove the entry, but the boxes come from the memo, so the proof costs
the comparison and not the recompute; or the entry is trusted under
the digest alone once the patch memo's own byte-compared hit is
threaded through to the index seam. Either shape keeps `mesh` free of
`bvh`. Memory: the stored corners are the mesh's positions again
(three points a triangle); measure against the patch memo's bytes.
