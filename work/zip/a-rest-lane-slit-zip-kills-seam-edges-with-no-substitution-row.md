---
id: a-rest-lane-slit-zip-kills-seam-edges-with-no-substitution-row
kind: issue
title: The REST lane's slit zip kills R-interior seam edges with no substitution row, so a (vertex, edge) record naming one leaves the result
status: open
opened: 2026-10-03
priority: P3
cost: E
---


## The finding

Found by FUSE's sweep of carry paths onto the substitution door
(`carry` in `crates/topo/src/boolean/ops.rs`, branch
`fuse/cell-pair-contacts`).

`zip_seam` (`crates/topo/src/boolean/zip.rs`) now reports every ring
edge it kills beside the seam edge it lay on (`ZipReport::edge_merges`),
and `Descendants::absorb_zip` writes those as substitution rows, so a
`(vertex, edge)` record naming a zipped edge names the seam edge after.
The REST lane's `glue_pair` (`crates/topo/src/boolean/rest.rs`) builds
its own `ZipReport` and kills the already-fused runs a slit zip consumes
(`interior_edges`) without a row: their interiors become the inside of
the glued contact region, so the substitution is the face they now lie
in, which the lane knows and does not write.

No record can name such an edge today: `(vertex, edge)` records are
minted only by the join, which no boolean output stage runs yet. It
becomes reachable when the join runs at every output stage (FUSE's
build step 2 of PR 3881's ruling).

## Owed

Have `glue_pair` report each killed interior edge with the face that
holds it, and `absorb_zip` write `Edge → Face` rows for them, as the
merge's killed edges already are (`Descendants::absorb_merge`).
