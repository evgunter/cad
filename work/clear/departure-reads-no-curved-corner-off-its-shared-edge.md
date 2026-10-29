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

`topo::injective`'s corner presentation (`present_corner`, `reading`) reads a corner where a curved edge meets another in two cases only: when the shared edge is a curve `present_curved` reads, the other edge straight in the chart; and when the shared edge is a coordinate line of a cylinder or torus chart, the other a harmonic chart curve. Two kinds of corner fall outside them. `beside_curve` refuses both `Structural`, so a pair whose two faces both meet one refuses at once.

- **A pair touching at the corner's vertex alone.** `corner_of` needs one of the corner's two edges on the shared cell. A face pair that shares only the vertex has neither, so no zero side holds the vertex. The iso corners answer this with `Zero::Point`; a curved corner has no such reading.
- **A plane face whose shared edge is a line meeting an arc.** The line reading (`present_line_corner`) is built on curved charts only. A plane has `present_plane`'s affine line presentation, which needs a span check (`self_intersection_span`) that a line ending at the corner fails.

## Evidence

The boolean tee `crossing_equal_rods` at spins `(0, 0)`, `B` from `x = 0.2`, pinned by `a_cylinder_cylinder_tee_refuses_typed_at_a_vertex_its_junction_shares_alone` in `crates/sweep/tests/self_intersection_proves_injectivity.rs`. It refuses `Uncertified { Departure }`: nine refusals, every one at a root pair (both faces' whole windows), the rest of the body splitting 384 times beside them:
- vertex-only pairs: `B`'s wall against `A`'s mitre and against `A`'s wall, at `(0.2, ±0.98, 0.2)`, where `B`'s cap circle meets the mitre plane; `A`'s wall halves against `B`'s, at `(1, 0, 1)`, where `A`'s seam line meets the junction;
- across a line: `A`'s mitre face (arcs and a line) against `B`'s cap (a line and arcs), along the line where the mitre plane cuts the cap, whose ends are those corners; and `A`'s wall against the mitre.

The junction never reaches the node plane `x = 0` (`x0 > 0` keeps the walls' tangent points out of both solids), so these refusals are not the second-order ground.

## What a fix needs

- For the vertex alone: a point zero for a curved corner. The quadrant reading generalises if the line reading's rectangle is cut at the vertex on the moving axis too, and the curve is decided to leave into that quadrant on both axes; then `Zero::Point`'s two derivatives apply.
- For a plane's line edge: the line reading through `present_plane`'s affine map, with `leaves_line`'s functional the line's face-side normal and the span check dropped at the vertex end.
