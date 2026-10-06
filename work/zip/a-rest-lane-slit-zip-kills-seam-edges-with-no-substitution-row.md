---
id: a-rest-lane-slit-zip-kills-seam-edges-with-no-substitution-row
kind: issue
title: The REST lane's slit zip kills R-interior run edges and vertices with no substitution row, and its new edge rows have no test
status: open
opened: 2026-10-03
priority: P3
cost: E
---

## The finding

Filed by FUSE's PR 3955 (step 1 of PR 3881's build: cell-pair contact
records carried by substitution). The PR's own fix pass deleted this
row with no close note, and its dual review's lane 2 re-check (n1, n2)
found that part of the finding still holds. The orchestrator re-files
it here with the corrected premise.

- **Still unrecorded.** In `crates/topo/src/boolean/rest.rs`, around
  lines 1866–1906, the slit zip kills the interior run edges
  (`interior_edges`, by `kef`/`kev`) and their run vertices. These get
  no `Edge → Face` (or `Vertex → Face`) substitution row in
  `Descendants`. A `(vertex, edge)` or edge/edge record naming one of
  them would leave the result. PR 3955 added rows only for the
  coincident b/a copies at each fold and at the final pair.
- **Untested.** Deleting `report.edge_merges.extend(edge_merges)` in
  `glue_pair` leaves the whole `-p topo` suite green, and so does
  deleting the slit-fold rows that feed it. Nothing catches a
  regression there.
- **Earlier premise corrected.** The first filing paired each ring
  edge with `seam_edges[j]`. That was wrong: ring edge j lies on
  `ob[j−1]`'s segment, and PR 3955 fixed `ZipReport::edge_merges` to
  `seam_edges[(j+n−1)%n]`.

## When it becomes reachable

Once FUSE's step 2 runs the join at every output stage, so that `ve`
and `ee` records reach a later op's REST lane. Until then no carried
record can name these cells.

## Owed

- Write the `Edge → Face` and `Vertex → Face` rows for the slit zip's
  killed interior edges and run vertices.
- Add a row that goes red when `glue_pair` drops `edge_merges` or the
  fold rows.

## Left by FUSE step 2 (2026-10-06)

FUSE's step 2 of the PR 3881 build (branch `fuse/join-every-stage`)
carries `ve` and `ee` records into every boolean, the REST lane
included, so the row is reachable. Step 2 did not fix it, and built no
witness for it. Its reading of `slit_zip`, for whoever picks this up:

- Both contact patches glue away, so the R-interior run edges and run
  vertices look like they leave the boundary for material, with no
  face of the result holding their interiors. If so, a record naming
  one is consumed (the door's drop rule) and an `Edge → Face` row
  would be wrong there. A witness should settle it before any row is
  written.
- The second owed item stands: nothing turns red when `glue_pair`
  drops `edge_merges` or the fold rows.
