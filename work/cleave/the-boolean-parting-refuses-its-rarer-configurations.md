---
id: the-boolean-parting-refuses-its-rarer-configurations
kind: issue
title: The Boolean's parting refuses typed where an end lies on the face more than once, on one hole, on its boundary mid-edge, or under a record it would move
status: open
opened: 2026-10-19
priority: P2
cost: M
---


Found by the Boolean slit-form lane (`cleave/boolean-builds-the-slit-form`,
the parent row `a-boolean-builds-a-reflex-edge-lying-inside-a-face`).

## What

`topo::boolean::slit` parts an edge lying in a face whose material both
the edge's faces leave the line into, and refuses
`BooleanError::EdgeInFace` with an `EdgeInFaceGap` construction gap,
rather than keep the rider form, where:

- `Shared`: an end lies on the face's loops more than once, or the
  edge's walls go into two faces' material (two coincident faces);
- `Pocket`: both ends lie on one hole of the face, so the parting cuts
  a piece of face off the hole, which needs a containment reading to
  tell the piece from the hole;
- `EndOnBoundary`: an end touches the face's boundary inside an edge of
  it, or at another vertex on its point (a record says so);
- `Recorded`: a contact record names the face the parting divides;
- `Tangent`: a wall tangent to the face along the line lies on a torus
  or spline, whose bend off its tangent plane is not read;
- `NoNormal`: a wall has no normal at the edge's midpoint.

None is reached by the per-PR set of the topo, sweep and editor-core
suites at default, 1e-6 or 1e-12 (the slow set, at default, in part).
Each is a configuration the ratified sentence decides;
each is a construction gap, with no lever.

## Fix

Build each: `Pocket` by nesting the cut-off loop as `nest_slit` nests a
slit (PR 5350); `EndOnBoundary` by dividing the boundary edge at the end
and fusing the two vertices; `Recorded` by placing the record's cell on
the piece holding it; `Tangent` with the torus's bend.
