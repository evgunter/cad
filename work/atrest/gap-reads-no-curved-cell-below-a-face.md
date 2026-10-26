---
id: gap-reads-no-curved-cell-below-a-face
kind: issue
title: Gap has no arm for a curved edge, a vertex or an edge beside or against a curved face, a saddle vertex, or a near-flat edge
status: open
priority: P1
cost: M
opened: 2026-10-26
refs: [gap-reads-vertex-and-edge-cells]
---

`gap-reads-vertex-and-edge-cells` gave `Gap` its vertex and edge arms (`crates/editor-core/src/eval/measure.rs`, `gap` and its module table). A cell's material is read as the half-spaces of the plane faces around it (`Wedge`, `Corner`, `Fold`), so these refuse `MeasureUnsupported`:

- **a circular or curved edge** in any pair (`carrier_of` reads it as `Carrier::Other("a circular edge")`), so a rim resting on a plate cannot be asserted;
- **a line edge beside a curved face** as the reading side: an edge against a vertex or against another edge, where the edge's `wedge` is `None` ("a line edge beside a curved face"), e.g. a cylinder's seam ruling;
- **a vertex on a curved face** as the outer of two vertices ("a vertex on a curved face");
- **a saddle vertex** as the outer of two vertices ("a saddle vertex"): its incident edges are some convex and some concave, so its material is neither the intersection nor the union of its faces' half-spaces;
- **a line edge against a sphere face**, and a line edge against a cylinder face whose axis does not run along it (`NotParallel`);
- **a vertex or a line edge against a cone, torus or NURBS face**, in either seat (`Carrier::Other("a cone face")` and the like);
- **a vertex against an edge or a vertex** where the edge's wedge or the vertex's corner does not read for any other reason: an unreadable face or an empty orbit (`NO_WEDGE`, `NO_CORNER`, which name the curved-face case);
- **a line edge whose ends do not read** ("a line edge whose ends do not read"), a body the evaluation built wrong;
- **a near-flat edge**: `gap_edge_convexity` escalates where an edge's dihedral, levered by its faces' reach, lies in the band's sliver, and every edge–vertex, edge–edge or vertex–vertex `Gap` reading that wedge refuses `Escalated`. The crossing decision of an edge–edge `Gap` (`gap_wedge_crossing`) never refuses.

Each such contact stays loud under stage 5 C's quieting rule, since no `Gap` over its cells evaluates. The fix is an arm per case. A curved face's tangent plane at the cell stands in for a plane face's, and a saddle is read sector by sector. Each arm keeps the sign convention the module states: zero at rest, positive apart, negative inside.
