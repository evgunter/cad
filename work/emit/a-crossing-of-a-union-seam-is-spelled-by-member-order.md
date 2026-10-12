---
id: a-crossing-of-a-union-seam-is-spelled-by-member-order
kind: issue
title: A union vertex where a member's face crosses a seam of two others, and the seam pieces it ends, are spelled by member order
status: open
opened: 2026-10-12
priority: P2
cost: M
---



## What

Found while building the touch ruling (branch `emit/touch-crossing`),
which took every edge-crossing absence out of
`crates/editor-core/tests/emit_union_rim_piece_ranks.rs`'s
`KNOWN_ABSENT`. What is left there is one class.

**The vertex.** A member's face crosses a seam line of two other
members, and the union names the vertex `Crossing { edge: Seam { .. },
face, sense }`. The same is true of the seam-edge pieces that end at
it, and in `cross` of the `Merged` faces those pieces bound. The
spelling differs by member order, so a name one fused order publishes
is absent from another.

**Measured.**
- `r1three` (`a` and slabs `S1`, `S2`, `S3`; `S3` crosses the seams
  `a`'s top cap makes with `S1`'s and `S2`'s walls): 480 absences. Each
  of the 24 orders lacks 20 of 24 distinct crossing names.
- `cross`: 140 absences over 14 orders and 16 distinct names. These
  are 56 crossings, 56 seam pieces and 28 merged faces.

No name rebinds: `a_name_two_member_orders_both_publish_denotes_the_same_geometry`
holds every name two orders both publish to one geometry.

## Next

Find which field moves between orders: the sense, the rank, or the
side order of the cited seam. Then name the vertex from the finished
body, as `emit_union::Flush::crossing` does for a member edge.
