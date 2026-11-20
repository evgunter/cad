---
id: cross-owes-census-window-and-conic-touch-arc-rows
kind: issue
title: curve_face::cross owes the census's every-built-pair row, a window placement fuzz, and touch-arc rows on the conic arms
status: open
opened: 2026-11-20
---


Found by U3a (PR 7226), whose rows (`curve_face::pose_rows`, the forced
ladder's `conic_quadric::ladder_lane_fuzz`) do not cover three of
`docs/GERM-CROSSING-U3-SPEC.md` §a.6's owed rows:

- **The census serves every built pair** of `route` through
  `census::point::edge_face` (`crates/topo/src/census/point.rs`): a
  touch read `Met`, a crossing or `Apart` read `Stale`, an unbuilt pair
  refused typed; and the mutant restoring `edge_face`'s kind match
  (circle × plane and line × sphere only) must red it. No census-level
  row reads a circle × sphere or a line × wall touch today.
- **Windows placed, meetings outside them still reported**
  (`window_fuzz`'s shape) on `cross`: a root in band of an end, and two
  roots in band of each other, each a window (`cross.rs`, `place`), the
  meetings outside it exact.
- **Touch arcs on the conic arms**: `pose_rows::a_touchs_arc_holds_the_in_band_arc`
  reads a circle on a plane and a line on a sphere and on a wall. The
  first-harmonic lane's graze (a circle on a wall or a sphere) and the
  cone's line graze have no oracle row of their arc.
