---
id: a-unions-flush-reading-is-straight-only
kind: issue
title: A union counts curved coincident member edges as two lines and names a curved flush stretch for the member the fold kept: Flush reads only straight edges
status: closed
opened: 2026-10-13
priority: P3
cost: M
closed: 2026-10-14
branch: emit/union-curves
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

## Closed (branch `emit/union-curves`, 2026-10-14)

- `emit_union::Flush::edge_cover` reads every finished edge: the
  straight-only gate is gone, and `joined_candidates` takes the
  finished edge's own kind, so a curved edge is read along its
  certified carrier through `joined_along` (`Track::cover`), as the
  pair boolean reads it.
- `Flush::same_way` and `Flush::meeting`'s collapse of flush lines read
  a curved member edge on its carrier: `emit_topo::EdgeCarrier` (a
  `Segment` or a `Track`) and `Track::runs_with`. `least_per_line`,
  which the pair boolean's `flush_lines` shares, takes it, so two arcs
  of one circle at a vertex are one line, the least, in both emitters;
  `straight_segment` is gone.
- `Track::offset` reads a periodic carrier's parameter anchored half a
  period from the start and decides a point at the start to be offset
  zero first. The old reading, anchored at the middle and reduced into
  one period, spanned the whole period at Interval for a point at a
  closed edge's own vertex: the union's first curved reading,
  `m4_pr8_corpus_interval`, `m10_p_fence`, `m10_di_dual_corpus` and
  `review_m5_pr9_doc_probe`'s boss union, escalated
  `name_frag_on_member_edge` with a margin enclosure of [−4.03, 4.76]
  until it did.

Rows: `crates/editor-core/tests/emit_union_curved_flush.rs`. `h` is the
half disc over the arc 0°–180° and `q` over 90°–270°, both extruded 1,
and `w` is a wall through the top rim at about 64° and 116°.
- `a_rim_joined_along_two_flush_arcs_is_named_for_both_in_either_order`:
  each rim of `h ∪ q` is one arc 0°–270°, `Merged` of the two rim arcs,
  one table in both orders.
- `a_wall_across_two_flush_arcs_publishes_one_table_in_every_order`:
  all six orders of `h, q, w` publish one table; the top rim's pieces
  are `h`'s arc (0°–64°) and `q`'s (116°–270°), and the vertex at 116°
  is `Crossing { q's arc, w's wall }`. On origin/main the order
  `h, q, w` refuses `a union vertex lies on a finished edge that is
  neither a member edge's piece nor a seam`.
- `emit_topo::rim_holding_rows::arcs_of_one_circle_are_one_line_either_way_along_it`:
  `EdgeCarrier::runs_with` and `least_per_line` over arcs of one circle
  either way, a wider circle and a straight chord.

Moved: `emit_ball_cut_by_brick.rs` now pins `one member edge meets no
face at a union vertex no seam reaches` for all four intersect and union
cases (was `UNCLASSIFIED_SIDE`): the ball's two meridians lie on one
great circle and are one line at the turned pole now. Evidence is in
`a-whole-ball-cut-by-a-tilted-brick-refuses-an-unclassified-member-edge-side`.

Not reached: two finished edges at one vertex lying within two
different coincident curved member edges, which is where the collapse
changes a union name rather than a refusal. `h ∪ q`'s pieces each lie
within one arc, so each vertex cites one, and an identical-arc pair
(two half discs over one arc) refuses `FallbackExtentUnsupported` in
the boolean in two of three orders. The unit row covers the reading.

Sweep: `is_straight(`, `Segment::of_edge(`, `.on_line(`, `runs_with(`
and `edge_carrier_kind` in `names/`. Left: `chord_on_rim`, behind
`chord_within`'s kind match; and `rank_along_seam`, which ranks only a
straight seam as N2 states
(`crossings-of-a-union-seam-of-several-curves-tie`). Blind spot: a
lies-on test over raw point distances; `holds_whole` and
`least_vertex` compare points to points, which reads alike on any
carrier.

