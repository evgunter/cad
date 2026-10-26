---
id: line-wedges-lever-their-signs-by-the-edges-length
kind: issue
title: The census's line wedges lever every sign by the edge's length, so a long edge on narrow faces decides an in-band overlap as interference
status: open
opened: 2026-10-26
priority: P2
cost: M
refs: [material-and-convexity-are-read-three-ways, gap-reads-vertex-and-edge-cells]
---


Found by the post-merge review of PR 5779 (minor 1; probe `review_probe_thin_bars_overlap_within_the_band_at_their_reach` on branch `review/atrest-5779-post` @ 8acd9163b).

`crates/topo/src/census.rs` `line_wedges` (the coincident-line wedge reader that #5529 added) levers every sign at the edge's length (`arm = e.len`), not at how far the faces reach from the line. A long edge on narrow faces therefore decides an angle whose material is in band.

**Example:** two bars on the z-axis, A = [0,w]² and B its quadrant-2 neighbour turned −θ, at w = 1e-3 and θ = 1e-7. The material overlap is at most wθ = 1e-10 < ε. Yet the reader decides `CoincidentWedgesOverlap`. Since #5779, `census_interference_solids` reads that as the two solids' interference, which contradicts #5745's `an_overlap_within_the_band_is_no_interference`. It still refuses, so nothing goes silent. The wrong part is the "interference" label: localization runs, and the arrivals' records are blocked.

This is the class `gap-reads-vertex-and-edge-cells`'s review caught (M1: a dihedral levered by the edge's length), against `geom_brep::dihedral`'s rule that a wedge is levered by its faces' reach (`topo::wedge_reach`). Lever `line_wedges` the same way. If `material-and-convexity-are-read-three-ways`'s `topo::link` can read a coincident line's wedges, route it there instead, and note that it is a fourth material reader.
