---
id: the-edge-edge-rule-descends-a-flanks-projection-not-its-bound
kind: issue
title: resolve_edge_edge descends a flank's projection off the common line with its bound's bend
status: open
opened: 2026-11-01
priority: P2
cost: M
parent: the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading
---


Found by the stage-2 lane (`tangrim/sector-passes-descend`), building
`the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading`.

**The projection.** `recl::resolve_edge_edge`'s membership
(`in_wedge_of`) reads a flank's representative, the bound projected
off the common line (`flank_rep`), with the bound's own `Reach`.
First order is unchanged by that (the flanking plane holds the line,
so the projection's side is the bound's). Since stage 2 a tied
representative descends, and the descent reads the bound's bend (a
conic's curvature) against the face's normal section along the
PROJECTION. On a plane flank that is exact (a plane's section is
nothing); on a curved flank, a bound leaving the line obliquely is
read against the section along the wrong direction. Owed: carry the
bound's own direction in `Rep` and descend it.
