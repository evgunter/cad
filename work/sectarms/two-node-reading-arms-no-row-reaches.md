---
id: two-node-reading-arms-no-row-reaches
kind: issue
title: Two node-reading arms no row reaches: a sweep germ at a node, and a site on two arcs off their shared node
status: open
opened: 2026-11-03
priority: P3
cost: E
---


Filed by the cylinder × cylinder stage 3b-ii lane (PR 6466). Its
mutation run found two arms of the C5 site reading that no row kills.

## What

- **`join::germ_branch`, a site at a node** (`SiteReading::AtNode` →
  `NodeUnbuilt { GermOnNode }`). Mutant m10 (the arm answering
  `Branch::Only`) passes every row of `geom-brep` `cylinder_pair_section`,
  `topo` `node::tests` and `wall_ring_rows`, `sweep` `bisector_nodes`,
  `verbs_germarms2::every_pose…` and `crossing_equal_rods`. Every
  node-touching pose refuses earlier:
  - the factored crossing at a rim vertex on a node (`OnBoundary`);
  - the quadric's door at a seam through it;
  - `node::bisector_datum` at a pierce on both planes.

  The retired `SectionLoops::branch_at → None` arm was unreached the same
  way: no sweep or editor row expects `GermOnNode`.
- **`CarriedSection::site`, two arcs that share a node, read off it**
  (escalation on the node distance, a `SizedPass::Zero` gate rejection).
  Mutant m9 (keep the last arc read) passes every row. At the bisector's
  square crossings, a site within band of both arcs reads within band of
  each arc's end, so `section_site_in_span` escalates first; the arm is
  reachable only at a crossing shallower than the band's ratio allows.

## Done when

Each arm is reached by a row, at a node pose that gets past the earlier
doors or at a pair with a shallow node crossing (a spline node, spline
3b). Otherwise each is shown unreachable and folded into the reading
that precedes it.
