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

## 2026-10-12, fixed (FUSE, PR 4893)

`Descendants::holder(face, rest)` is the one read of a vertex-on-face
record's face. The rest is the point the record's vertex has in its
operand clone, which the ledger logs for every vertex-on-face row
(`boolean::ops::rests`). A row with no logged point refuses
`JoinDesync`. The vertex-only spelling is gone.

- `Rest::Kept { vertex, at }` covers a vertex still in the result: the
  piece it bounds, else the piece whose interior holds `at`.
- `Rest::Taken(at)` covers a vertex the join or the merge took: the
  piece whose interior holds `at`.

This rules out the owed line above ("the killed vertex's point kept
with its substitution row"): no geometry is added to lineage rows, and
the point is read from the operand instead.

**Premise, guarded.** No stage moves a kernel vertex. A zip's fusion and
`zip::share_points` are the two stages that leave a vertex at another's
point, within ε, and `carry_rows` skips the rows of both kinds before it
reads the face. So
the operand point is the point the vertex has in the result.
`carry_rows` checks that on every kept vertex, bit for bit, and refuses
`JoinDesync` where they differ. `piece_holds` places the point with the
census's own band: a margin within the band of a piece's carrier or
boundary escalates as `SelfCheck(CarriedLineage)`, kept or taken alike.

**Why nothing reaches the taken arm today.**

- The join takes only a valence-2 vertex between two edges on one
  carrier.
- The merge kills only seam and strut vertices.
- In a boolean, every such vertex is a section or seam vertex. A zip
  fuses each of those, or `zip::share_points` moves it onto its class's
  key, and `carry_rows` skips the rows of both kinds before any face
  read.

So `carry` cannot reach the taken arm today, and lane B's three tried
poses agree. The door and its rows stay anyway: they cost little, and
an operand carrying a valence-2 collinear vertex with its own
vertex-on-face row would reach the arm.

**Witness.** No suite reaches the taken arm. On ea1088c5,
instrumentation (reverted) counted a vertex-on-face row whose vertex
was dead and whose face the section divided. Across the 2,721-body
lattice, the two-stage carried corpus (4,536 runs), the full
topo/sweep/editor-core suites (8,342 tests) and 2,940 random
star-prism scenes under four ops, it counted 0. The door lands with
unit rows on a hand-divided face (`boolean::ops::holder_rows`,
`a_dead_vertexs_*`): a taken vertex lands on the piece holding its
rest, a rest on the chord drops, an in-band rest escalates, and a
missing rest refuses. Reading a taken vertex's face by key turns the
first two red.
