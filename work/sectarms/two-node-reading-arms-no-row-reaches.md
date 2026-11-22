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

## A spline node (2026-11-13)

With spline × implicit arcs carried to their nodes
(`sectarms/spline-node-carry`), the second arm is still not reached by
a row that a mutant could fail. A site in a node's box and off the node
reads on every arc whose boxes hold it. Those arcs share the node, so
it escalates by this arm. m9 still survives there, because the
escalation that a site in a node's box owes on its own
(`CarriedSection::site`'s box arm, row
`pair_section::wiring::a_site_in_a_nodes_box_on_one_arc_escalates`)
answers it the same way.

Outside the node's box, the carry holds two arcs of one node apart by
their boxes (`spline_implicit::apart`), each widened by the band, and
refuses `PadsMeet` where two of them meet. So a site outside the box
reads on two such arcs only where the carry has already refused them.
A box's pad there is the farthest its window's image stands from the
carrier, which on the cross and the lemniscate is up to 4 and 9 times
the window's distance from the node (both reviewers of PR 6897
measured it); it is not a share of that distance.

The first arm waits on a node boolean that reaches the germ. The saddle
row stops before the join reads any site, at the plane's chord along the
block's side wall
(`a-planes-carried-arc-across-a-flared-spline-wall-misses-its-own-image-at-rest`),
and a node arc's chord does not certify at rest
(`a-node-arcs-chord-misses-its-at-rest-chart-image`).
