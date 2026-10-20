---
id: a-ring-path-along-a-ruling-or-crest-arcs-carrier-reads-nothing
kind: issue
title: A ring path along a cone ruling arc's line, or the torus's crest cylinder, reads nothing even off the arc
status: open
opened: 2026-10-12
priority: P4
cost: M
---

## What

Found by the sweep of `germ/one-face-torus-pierce-ring`, which fixed
the conic case: `ring_path::path_parity` gave up on any path with a
piece running along a loop arc's plane, and now reads such a piece
apart from the arc where it is decided so (`misses_in_plane`, and
`split_ring_path_segment_apart` for a segment with an end on the
plane). Two arms keep the old shape, giving up even where the piece
is decided off the arc:

- **A cone's segment along a ruling arc's own line**
  (`ring_path.rs`, `path_parity`'s `(Piece::Segment, LoopArc::Line)`
  arm, `split_ring_path_rulings_apart`): a path from a point of a
  ruling arc's line, off the arc's span, runs its segment along that
  line, and the rulings read Zero apart, so the path says nothing even
  where the segment and the arc are disjoint on the line.
- **A torus path along the crest cylinder `ρ = R`**
  (`path_parity`'s `(Piece::Arc, LoopArc::Crest)` arm,
  `split_ring_path_crest_graze`; and the `(Piece::Segment,
  LoopArc::Crest)` arm, `split_ring_path_crest_side`): a path from a
  point on the tube's top or bottom circle, off the crest arc's span,
  runs its parallel along that circle. The row
  `ring_path::tests::a_path_from_the_tubes_top_or_bottom_reads_the_oracle_or_nothing`
  pins "the oracle or nothing" for it.

Neither is measured to be reached through the public ops. A ring
vertex on the crest circle needs a solid touching the torus's top or
bottom, which the touching poses refuse first (F0, held by D10); a
ring vertex on a cone ruling arc's line was not looked for.

## Done when

Each arm reads a piece decided disjoint from the arc on the arc's own
carrier as missing it (the disjoint-spans reading `misses_in_plane`
makes for a circle), with an oracle row from points on those carriers;
or a public pose shows neither is reachable and the arms say why.
