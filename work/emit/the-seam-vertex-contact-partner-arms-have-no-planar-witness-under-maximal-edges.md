---
id: the-seam-vertex-contact-partner-arms-have-no-planar-witness-under-maximal-edges
kind: issue
title: The pair emitter's contact-partner seam-vertex arms have no planar witness once maximal edges join a touch vertex away
status: open
opened: 2026-10-06
priority: P2
cost: E
---



## What

`name_boolean_vertices` (`crates/editor-core/src/names/emit_topo.rs`)
names a seam vertex with one operand edge and nothing else from the
other operand by its contact-record partner: the arms
`([ae], [], _, Some(pb))` and `([], [be], Some(pa), _)`. Their subject
is a vertex the reduction minted on one operand's edge where the other
operand's vertex touches it, with nothing zipped there. Such a vertex
has valence 2, both its edges pieces of one edge between the same two
faces.

Under maximal edges (PR 4140, `docs/DESIGN.md`'s merge-stage clause)
the output stage joins that vertex away whenever the edge is straight,
and the touch becomes the record (vertex, edge). Found by FUSE step 4
(`fuse/set-names`): the three `emit_boolean_vertex_keys` rows that
pinned the partner arms by name (the ell and tip in both orders, the
assembly touch, the bar and tip) now build no vertex at the touch, and
were rewritten to pin the touched edge whole on the right side.

## What is left

The arms are reachable only where the touched edge is curved, which the
join does not take yet
(`work/fuse/curved-joinable-vertices-are-left-unjoined.md`). No row
witnesses that today.
- If curved joins land, the arms' subject is gone everywhere and they
  should go with it.
- Until then, a curved witness (a tip whose apex touches a cylinder's
  rim circle away from its canonical cut) would keep them honest.
