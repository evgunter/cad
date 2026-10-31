---
id: departure-reads-no-curved-corner-off-its-shared-edge
kind: issue
title: The injectivity check reads a curved corner only through a shared edge of a curved chart or a plane's arc, so a pair touching at the corner's vertex alone, or across a plane's line edge that meets an arc, refuses; the tee refuses so
status: open
opened: 2026-10-29
priority: P2
cost: M
---

## Finding

From CORNER (`departure-reads-no-curved-corner`).

`topo::injective`'s corner presentation (`present_corner`, `reading`) reads a corner where a curved edge meets another in two cases only: when the shared edge is a curve `present_curved` reads, the other edge straight in the chart; and when the shared edge is a coordinate line of a cylinder's chart, the other a harmonic chart curve. Three kinds of corner fall outside them. `beside_curve` refuses each `Structural`, so a pair whose two faces both meet one refuses at once.

- **A pair touching at the corner's vertex alone.** `corner_of` needs one of the corner's two edges on the shared cell. A face pair that shares only the vertex has neither, so no zero side holds the vertex. The iso corners answer this with `Zero::Point`; a curved corner has no such reading.
- **A corner of two curved edges.** The curve reading needs the other edge straight in the chart (`straight`). The line reading needs the read edge a coordinate line.
- **A plane face whose shared edge is a line meeting an arc.** The line reading (`present_line_corner`) is built on curved charts only. A plane has `present_plane`'s affine line presentation, which needs a span check (`self_intersection_span`) that a line ending at the corner fails.

## Evidence

The boolean tee `crossing_equal_rods` at spins `(0, 0)`, `B` from `x = 0.2`, pinned by `a_cylinder_cylinder_tee_refuses_typed_at_a_vertex_its_junction_shares_alone` in `crates/sweep/tests/self_intersection_proves_injectivity.rs` (which pins `refused == 9`). It refuses `Uncertified { Departure }` with nine refusals. The CORNER review instrumented them: all at depth 0, both faces' whole windows.

- **Six vertex-only pairs.** These are the walls against the mitre and against each other:
  - at `(0.2, ±0.98, 0.2)`, where `B`'s cap circle meets the mitre plane;
  - at `(1, 0, 1)`, where `A`'s seam line meets the junction.
- **One plane–plane pair across a `Line`.** `A`'s mitre against `B`'s cap, along the line where the mitre plane cuts the cap (in the review's run: mitre `5v9`, cap `6v5`, edge `10v3`). Its ends are corners where the line meets each face's arcs.
- **Two cylinder–mitre pairs across `Ellipse` edges.** These are each of `A`'s wall halves against the mitre, across its arc of the mitre's ellipse (`4v1`–`5v9` on `3v5`, `7v1`–`5v9` on `14v1`). The likely cause is a corner of two curved edges, which no reading reads:
  - the mitre's two ellipse arcs meet at `(−1, 0, −1)`, on `A`'s seam;
  - on the wall, the mitre's arc gives way to the junction's arc of the same ellipse at `(0.2, ±0.98, 0.2)`.

  At the root the cell also holds three or more walls in reach, which no corner reading takes either.

The junction never reaches the node plane `x = 0` (`x0 > 0` keeps the walls' tangent points out of both solids), so none of these refusals is the second-order ground.

## What a fix needs

- For the vertex alone: a point zero for a curved corner. The quadrant reading generalises if the line reading's rectangle is cut at the vertex on the moving axis too, and the curve is decided to leave into that quadrant on both axes; then `Zero::Point`'s two derivatives apply.
- For two curved edges: the other edge's height over the continued curve, decided along it, in place of `leaves_graph`'s straight-line rate. Where the two are arcs of one closed form (the ellipse split at a vertex), `continued` reads them as one curve.
- For a plane's line edge: the line reading through `present_plane`'s affine map, with `leaves_line`'s functional the line's face-side normal and the span check dropped at the vertex end.
