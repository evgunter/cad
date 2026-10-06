---
id: corner-valence-reads-a-self-closed-edge-once
kind: issue
title: corner_at, cap_incidence and the corner admission count a self-closed edge once at its vertex
status: open
opened: 2026-10-06
---


## Finding (the class sweep of `self-closed-link-sharing-its-vertex-records-two-junctions`)

`walk_chains` (`crates/sweep/src/blend/battery.rs`) counts a
self-closed link at its one vertex twice: it arrives there and leaves.
So a self-closed requested link beside one other requested link at its
vertex is a corner, and the other link's chain is OPEN, ending there.
That end then reaches predicate 6. Every valence read on that path
counts EDGES through `Body::edges_of_vertex` /
`Body::faces_of_vertex`, which list a self-closed edge (or a face
reached twice) ONCE:

- `battery::corner_at` — `valence = edges.len()` and
  `named = edges.filter(requested).count()`. A vertex carrying a
  self-closed rim, one seam and one other edge has degree 4 and reads
  `valence == 3`, so it takes the trihedral arm.
- `battery::cap_incidence` — `let [_, _, _] = incident[..]`, the same
  distinct-edge count.
- `admit::Corner::admit` — `let [f0, f1, f2] = faces[..]` over the
  deduped face orbit.

The walk now counts link ends and these count distinct edges; at a
self-closed edge's vertex the two disagree. Whether the trihedral arm
then refuses or classifies wrongly is not traced. It is not reachable
today: no body the tree builds has a non-seam edge ending at a
self-closed rim's vertex (the seams refuse `TangentialEdge` before the
walk), which is the same gate the walk's row sat behind.

## Fix shape

Read the degree, not the distinct-edge count: a self-closed edge in
the fan counts twice in `corner_at`'s valence and in
`cap_incidence`'s arity; or refuse an open chain end at a vertex whose
fan holds a self-closed edge as a corner configuration with its own
tag. Row it on a body once one door builds the shape.
