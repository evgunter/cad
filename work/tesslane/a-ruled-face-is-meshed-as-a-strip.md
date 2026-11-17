---
id: a-ruled-face-is-meshed-as-a-strip
kind: unit
title: A cylinder or cone face bounded by two rows and two rulings is meshed as a strip, written by walking its rows, in the chart lane at δ
status: parked
opened: 2026-11-17
priority: P1
cost: H
blocked_on: [a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it]
---

Unit 4c of the chart lane (`work/tesslane/plan.md`). The final state of the design fork of 2026-11-17 (row 279 of `docs/DESIGN-FORK-LOG.md`), agreed by three designers after three rounds. It builds `crates/mesh/README.md` `[mesh.sizing]`'s two bullets "The triangulation is built to what the theorem assumes" and "A ruled face is meshed as a strip".

## Terms

- **Ruled face**: a face on a cylinder or a cone. Its chart lines `u = const` are straight: *rulings*. On a cone every ruling passes through the apex.
- **Row**: a rim of the face, or the apex counted as a row of one point.
- **Split side**: a straight side of the face carried by two or more edges, so it has a vertex part-way along. Booleans make these.
- **Rectangle lane / chart lane**: `mesh::curved` and `mesh::trimmed`.
- **Headroom**: the chart lane sizes at half of δ where it has no theorem (`UNPROVED_HEADROOM`).

## What it is

A ruled face bounded by two rows and two rulings lays no interior points and is meshed by walking its two rows: sort each row's boundary points by angle about the axis, start at one side with the edge joining the rows' first points, and at each step advance on whichever row's next point comes first in angle, emitting the triangle on the current cross edge and that point. The apex is a row of one point, which gives the fan. A vertex on a straight side divides the one triangle that side is an edge of, fanned from that triangle's third corner. The face sizes at δ with no headroom.

Its home is the chart lane, chosen by the face's domain and a fact on the cylinder's and cone's chart-table rows, readable before the chord pass (the chord pass reads the face's target through its lane).

## Why it is within δ

The front edge `a_i–b_j` keeps `max(a_i, b_j) ≤ min(a_{i+1}, b_{j+1})` (it holds at the start, where the rows' ends coincide, and each advance preserves it). So the triangle `(a_i, a_{i+1}, b_j)` spans at most one chord of row `a` or one of row `b` in angle. A triangle whose corners' angles lie within a range lies in the convex hull of the surface between the two rulings at the ends of that range, and deviates by no more than the sagitta of the larger rim's chord over it: what `sizing::cone_grid_step` and `sizing::sagitta_step` already hold to δ at the face's column step. Triangles made by a side vertex lie in the plane of the triangle they divide.

**Owed as written proof beside the step functions:** the convex-hull step for a cone triangle on three rulings (two designers rate it sure, one likely).

## What it makes true, and retires

- Splitting a straight edge adds one triangle to each face beside it and never makes a face refuse or changes its density otherwise.
- No triangle lies along one ruling, and none spans two columns.
- It closes `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`, removes the interim route's density price on split-side faces, and removes the rows the chart lane lays on every ruled rectangle, which is what unit 6's density measurement waits on.

## Measured by the designers (PR 7043's head, debug, default ε)

Cone-face triangles, unit 45° cone, full turn:

| δ | plain, rectangle lane | plain, chart lane today | split side, chart lane today | split side, strip |
|---|---|---|---|---|
| 0.1 | 8 | 58 | 60 | 10 |
| 0.01 | 24 | 604 | 606 | 26 |
| 0.001 | 72 | 6168 | 6170 | 74 |

The strip column is a prototype that held edges in the rectangle lane, not the walk; it emits the counts the walk would. About 300 cells (split cones, frustums, cylinders, cones of half-angle 5° and 1°; δ 0.1 to 0.001) mesh at the unsplit count plus one per vertex and pass `check_mesh` and the winding check; T1's 18 cells also pass the probes' outward and true-distance checks. A frustum's rims are chorded at different counts from δ 0.03 down (51 against 43 points at δ 0.001 on a half turn, radii 2 and 1), which the walk covers and a rule joining "partners at one angle" does not.

## Not measured

- The walk itself: nobody has built it.
- Whether a strip's corner triangle can make a chord its neighbouring face also makes (the chart lane's ear and shared-side cures exist to stop that; the prototype's suite run showed no edge used four times).
- A one-face full-turn wall (its two sides are one seam edge): the chart lane's search path refuses it `SeamTwice`; the walk could take it.
- A face on both nappes of a cone.

## Fallback the designers stand behind

If the walk is judged too large a step inside the chart lane: no rows, plus one held edge per split side, with a test that fails if any triangle spans more than one chord in angle. Its unsplit half then rests on a proof per chart that Delaunay zips two parallel rows or two concentric arcs locally.
