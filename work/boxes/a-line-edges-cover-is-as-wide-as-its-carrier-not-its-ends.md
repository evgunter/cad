---
id: a-line-edges-cover-is-as-wide-as-its-carrier-not-its-ends
kind: issue
title: A line edge's cover is as wide as its carrier read over its span, not as tight as the chord of its ends, so interval floors read the carrier's width
status: open
opened: 2026-11-09
priority: P2
---


`topo::cover::edge_cover_over` hulls the carrier's cover over the span (`geom::Curve3::cover_over`) with the edge's two ends. On a line the true edge is the segment between its ends, to the certification residual. At the `Interval` lane, though, the carrier's points `o + d·t` are as wide as `d`'s width times `|t|`, so the cover is wider than the chord of the ends that the boxes read before PR 6399. A consumer that reads the cover's centre, such as the pair extent `bool_plane_offset` reads its offsets at, inherits that width. `docm2_part_interval::a7_the_width_ladder_…`'s floor moved one rung (ε/32 → ε/64) because of it (closed `work/green/main-red-docm2-a7-ladder-escalates-at-eps-over-32.md`).

The design reads an edge through its cover, with no kind match in topo (`[one-way.places]`). A tighter line reading therefore belongs in the cover. One shape: intersect the carrier's span cover with the ends' hull widened by the edge's certification residual. Another: centre a ball on a thin point and carry the width in its radius. Either way, measure what it buys on the docm2 ladder first.

(BOXES nightly-reds triage, 2026-11-09)
