---
id: a-unions-flush-reading-is-straight-only
kind: issue
title: A union counts curved coincident member edges as two lines and names a curved flush stretch for the member the fold kept: Flush reads only straight edges
status: open
opened: 2026-10-13
priority: P3
cost: M
---


## What

Found in review of the vertex-run build (branch `emit/vertex-run`).
The union's flush reading is gated on straight carriers:

- `emit_union::Flush::edge_cover` returns no cover for an edge that is
  not straight (`straight`), so a curved finished edge lying within two
  members' coincident curved edges is not in `Flush::edges`, and its
  piece is named for the member edge its fold name carries
  (`group_member_edges`'s `(Some(..), [k])` arm), which is the member
  the fold kept.
- `Flush::meeting` counts flush lines once through `Flush::same_way`,
  which answers `None` for any curved pair, so two coincident curved
  member edges through a vertex are two lines of its run, where N2
  counts them once as the least.

The names README's flush paragraph states no carrier restriction for
the union's reading. The subtract's reading is along the carrier
(`emit_topo::Track`), and
`a-flush-reading-finds-no-point-on-a-nurbs-member-edge.md` covers only
its NURBS gap, not circles at a union.

No fixture known: two members whose coincident circular edges meet
another member at one point would show it (two cylinders sharing a rim,
crossed by a plate through the rim).

## Next

Read a curved edge's cover along its carrier, as the subtract does,
and count coincident curved lines once by the same reading.
