---
id: a-vertex-on-face-record-whose-vertex-the-join-took-reads-its-face-by-key
kind: issue
title: A vertex-on-face record whose vertex the join took reads a divided face by its key, not the piece the point rests on
status: closed
opened: 2026-10-11
priority: P2
cost: M
closed: 2026-10-12
pr: 4893
---


Unmeasured, read. Found by FLUSH's sweep for
`a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime`.

## What

`boolean::ops::carry_rows` reads a vertex-on-face record's face through
`Descendants::holder`, which picks the live piece of a divided face
whose interior holds the record's vertex. That needs the vertex's
point in the result, so it runs only where the vertex is still a live
vertex. Where the join took the vertex (its record reads the joined
edge, `Descendants::cells`), the face is still chased by key alone, the
defect `holder` fixed: a key piece that does not hold the point keeps
an edge-face pair the census would not confirm, or a discarded key
piece drops it.

## Owed

The point of the joined vertex, kept with its substitution row (or
read from the operand clone before the join), so `holder` can answer
for it; a pose where a joined vertex rested on a divided face.

## A vertex the merge killed (2026-10-11, PR 4650's review r1)

The same holds where the record's vertex is no longer a vertex because
`merge_coplanar_faces` killed it: `Descendants::absorb_merge` substitutes
the killed vertex with the group's kept face, so the vertex end of the
row `live()`s to a `Cell::Face`, and `carry_rows`' vertex-on-face loop
reads the record's own face by key (the non-vertex arm beside
`Descendants::holder`). A divided face then reads its key piece whatever
piece the killed vertex's point lay on. Owed alongside the joined vertex:
the killed vertex's point kept with its substitution row.

## 2026-10-12, fixed (FUSE, `fuse/records-read-the-piece`)

`Descendants::holder(face, rest)` is the one read of a vertex-on-face
record's face. `Rest::Live(vertex)` covers a vertex still live: the
piece it bounds, else the piece whose interior holds its point in the
result. `Rest::Dead(point)` covers a vertex the join or the merge took:
the piece whose interior holds the point the vertex had in its operand
clone. The ledger logs that point for every vertex-on-face row
(`boolean::ops::rests`), and a missing point refuses `JoinDesync`. The
vertex-only spelling is gone.

**Premise.** No stage moves a kernel vertex. The graft, the zips and
the output stage copy each point bit for bit. A zip's fusion is the one
stage that leaves a vertex at another's point, within ε, and
`carry_rows` skips the rows of a fused vertex before it reads the face.
So the operand point of a dead vertex is the point it had in the
result. `piece_holds` places it with the census's own band: a margin
within the band of a piece's carrier or boundary escalates as
`SelfCheck(CarriedLineage)` rather than pick a side.

**Witness.** No suite reaches the arm. On ea1088c5, instrumentation
(reverted) counted a vertex-on-face row whose vertex was dead and whose
face the section divided. Across the 2,721-body lattice, the two-stage
carried corpus (4,536 runs), the full topo/sweep/editor-core suites
(8,342 tests) and 2,940 random star-prism scenes under four ops, it
counted 0. The door lands with unit rows on a hand-divided face
(`boolean::ops::holder_rows`, `a_dead_vertexs_*`): a vertex the join
took lands on the piece holding its rest, a rest on the chord drops,
and a missing rest refuses. Reading the dead vertex's face by key turns
the first red.
