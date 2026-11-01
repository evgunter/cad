---
id: the-edge-edge-rule-descends-a-flanks-projection-not-its-bound
kind: issue
title: resolve_edge_edge descends a flank's projection off the common line with its bound's bend, and spline sector faces take no vertex-pair charge
status: open
opened: 2026-11-01
priority: P2
cost: M
parent: the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading
---


Found by the stage-2 lane (`tangrim/sector-passes-descend`), building
`the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading`.

1. **The projection.** `recl::resolve_edge_edge`'s membership
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
2. **A spline sector face takes no vertex-pair charge.** `orbit_corners`
   reads each sector face's lever (`PierceLever::of`) for the
   vertex-pair pass's curvature charge, but a spline face's lever is its
   chart's certified arm over the sectors' reach
   (`AtRestPolicy::spline_chart_arm`, as `vtxfac` reads the pierced
   face's), which `build_sectors` (`T: Decide`) cannot call. It takes
   `NO_CURVATURE`, as every face did before stage 2.
