---
id: a-vertex-on-face-record-whose-vertex-the-join-took-reads-its-face-by-key
kind: issue
title: A vertex-on-face record whose vertex the join took reads a divided face by its key, not the piece the point rests on
status: dispatched
opened: 2026-10-11
priority: P2
cost: M
branch: fuse/records-read-the-piece
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
