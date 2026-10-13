---
id: a-unions-member-lookups-skip-a-non-unique-row-silently
kind: issue
title: The union's Flush readings skip a member row that is tied or not an edge/face with no signal
status: open
opened: 2026-10-13
priority: P3
cost: E
---


## What

Found in review of the vertex-run build (branch `emit/vertex-run`).
`names/emit_union.rs` reads a member's own table for a member entity
the union already holds by name, and where the lookup is not one
entity of the expected kind it `continue`s, dropping that entity from
the reading with no signal. A member's tied row, or a name that is no
longer that kind's, then reads as "not there":

- `Flush::lines_at` (~682): a set's member edge whose row is not
  `Entry::Unique`, or not an edge, is not on the vertex's lines.
- `Flush::sides` (~916): the same, so the edge's faces are not among
  the sides, and a seam its faces bound can be cited as a line.
- `Flush::edge_cover` (~638): a face pair `face_key` does not resolve
  contributes no candidate rims.
- `Flush::least_vertex` (~1200): a face `face_key` does not resolve
  contributes no member vertex, so a corner can be missed.

Sweep: `else { continue }` and `let … else` beside `table.lookup`,
`face_key` and `unique_name` in `emit_union.rs`. `sense_against`'s row
gathering (~971) is the same shape, but a missing row there ends in
the unclassified-side refusal, so it is loud.

## Next

Decide per site whether a tied member row is a reachable case (the
member's own tie) or a bug. Refuse the bug-class ones, as
`member_edge` does, and name the tie where it is a case.
