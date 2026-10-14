---
id: a-vertex-on-face-record-whose-vertex-the-join-took-reads-its-face-by-key
kind: issue
title: A vertex-on-face record whose vertex the join took reads a divided face by its key, not the piece the point rests on
status: closed
opened: 2026-10-11
priority: P2
cost: M
closed: 2026-10-13
pr: 4820
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

## 2026-10-13, fixed by PR 4820 (FUSE, PR 4893)

CONTACTHOLD's one carry (PR 4820) retired `Descendants::holder`, and
with it the by-key read this row names.

- A row the reduction found, on a face the join divided, goes through
  `boolean::ops::place_found`. The face becomes its live pieces and the
  chords between them (`face_pieces`), and the row lands on every pair
  of images the census's record confirm reads as meeting
  (`records::landings`).
- An operand's carried rows go through `boolean::ops::carried`
  (`operand_fate`, then the same `landings`).

So a row whose vertex the join or the merge took reads its other cell,
the joined edge or the kept face, against each piece by that cell's
geometry. Neither the key nor the vertex's point is read.

**Pinned.** `boolean::ops::found_on_a_divided_face::a_taken_vertexs_row_lands_on_the_piece_its_edge_lies_in`
(PR 4893) kills a record's vertex and substitutes it with an edge
lying in the walled-off piece: the bottom edge of a small prism
standing on that piece, grafted in as a second shell. The record
lands as `EdgeFace(edge, walled-off piece)`. With `place_found`'s
divided arm skipped, so the face is read by key, it lands on the key
piece and the row goes red.

PR 4893 first built a point-based door here: a ledger of each row's
operand point, `holder(face, rest)`, and a check that a kept vertex
sits at that point. It dropped that door on merging 4820, because a
second door beside the one carry is the defect this row names. Two
results from it stand:

- **Why nothing reaches the taken arm from a boolean today.** The join
  takes only a valence-2 vertex between two edges on one carrier, and
  the merge kills only seam and strut vertices. In a boolean every
  such vertex is a section or seam vertex, which a zip fuses and
  `carry_rows` skips (`desc.fused`). Instrumentation on ea1088c5
  (reverted) counted 0 rows whose vertex was taken and whose face the
  section divided. It covered the 2,721-body lattice, the two-stage
  carried corpus (4,536 runs), the topo/sweep/editor-core suites
  (8,342 tests) and 2,940 random star-prism scenes under four ops.
- **If a point-based check ever returns, `zip::share_points` must be
  in it.** `share_points` rebinds every live vertex of a point class
  onto the class's smallest key, and those vertices are not in
  `desc.fused`. A class is the union-find closure of seam pairs
  (`zip::point_classes`), each of them one point within the zero band
  (`reduce`'s `one_vertex_at`). So two keys of a class of k ≥ 3 can
  sit up to (k−1)·`band.zero()` apart, past the band, and a check of a
  moved vertex against its operand point would escalate a rebind that
  is the op's own. Measured with a temporary probe over
  topo/sweep/editor-core ci at ε = 1e-9, 1e-6 and 1e-12:
  - 149 moved vertices at each ε, all within the zero band;
  - classes of 2, 3 and 4 keys: 113, 30 and 6 per ε;
  - so 36 classes per ε have the shape that can cross the band, and
    none did.
