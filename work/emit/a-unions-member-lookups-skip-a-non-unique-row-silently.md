---
id: a-unions-member-lookups-skip-a-non-unique-row-silently
kind: issue
title: The union's Flush readings skip a member row that is tied or not an edge/face with no signal
status: closed
opened: 2026-10-13
closed: 2026-10-14
priority: P3
cost: E
branch: emit/union-readings
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

## Closed (branch `emit/union-readings`, 2026-10-14)

Per site, in `names/emit_union.rs`:

- **A member FACE is a member's own tie, a case.** N2 settles it: a
  parent is a set of member-face entities, and a tied row's candidates
  are separate parents spelled alike. `Flush` now carries each finished
  face's member faces by entity (`face_ents`, off `Parent::entities`),
  so `Flush::edge_cover` and `Flush::least_vertex` read a face by its
  key and never look its name up. `face_key` is gone.
- **A member VERTEX is a case.** `Flush::least_vertex` cites a tied
  member vertex by its tied name and says it is tied;
  `cite_member_edges` puts that row through the tie lane (`TieRows`),
  so two finished vertices at two candidates are one `Tied` row and one
  alone narrows to `Unique` (N4, ties propagate). An unnamed member
  vertex is `Emission`.
- **A member EDGE is a case the union has no rule for.** A tie stands
  where one edge is needed, which is `MemberEdgeTied`, as
  `member_edge` already refused. `Flush::edge_cover` lists a tied rim
  under its tied name (an unnamed one is `Emission`), and
  `Flush::lines_at` and `Flush::sides` read every edge through
  `member_edge`, so a tied one refuses typed rather than dropping out.
  `unique_name` is gone.
- **Rows on a line** (`sense_against`'s gathering and the crossing
  ranking's `on_line` rows): a tied row is read by each candidate edge,
  so the piece a point lies on is found whichever candidate it is. A
  row naming no edge is `Emission`.

Rows (`emit_union` unit tests, over a unit cube read as its own
member): `a_member_face_its_member_ties_is_read_by_its_key`,
`a_member_vertex_its_member_ties_is_cited_with_its_tie`,
`a_member_edge_its_member_ties_refuses_where_its_faces_are_read`. Each
fails on the old reading (an empty cover, `None`, `Ok` with no faces).

Sweep: `table.lookup`, `name_ref_of`, `is_tied`, `on_line` and
`Entry::Unique` in `emit_union.rs`, and every `else { continue }` and
`=> continue` there. Left: `member_edge`'s by-line reading (the least
untied row on a line, documented), and `Links::judged`'s skips, which
read the kernel's records, not a member table. Blind spot: a lookup
whose miss is turned into a default by `unwrap_or`/`map_or` rather than
a `continue`; a grep for those in the file finds none over a member
table (they read `Incidence`, `Flush`'s own maps, a merge record and a
set's constituents).

No golden moved; the union and emission suites pass unchanged.
