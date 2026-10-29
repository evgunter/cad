---
id: departure-reads-no-curved-corner
kind: issue
title: The injectivity check reads a curved shared edge only where it is a cell's one boundary edge, so a corner where it meets another edge refuses, and a cylinder-cylinder tee cannot be certified embedded
status: open
opened: 2026-10-29
priority: P1
cost: H
---

## Finding

From CURVEDEP (`departure-presents-no-curved-shared-edge`).

`topo::injective`'s curved presentation (`present_curved`) reads a curved edge only where it is the one boundary edge in reach of a cell.
- The column argument behind it needs the curve to be the only boundary in the rectangle it presents.
- A cell holding a vertex where a curved shared edge meets another edge is therefore read by no presentation, at any depth.
- `beside_curve` refuses it `Structural` (both faces then refuse `Uncertified { Departure }` at once, never at the floor).

The boolean tee `crossing_equal_rods` builds at spins `(0, 0)` (rod `A` mitred by `z = x`, united with rod `B` from `x = 0.2`) refuses so in 36 splits. Its junction is arcs of the bisector ellipse meeting the rods' lines and rims at vertices. The pin is `a_cylinder_cylinder_tee_refuses_typed_at_its_curved_corners` in `crates/sweep/tests/self_intersection_proves_injectivity.rs`.

The iso corners (`convex`, `zero_of`'s `Corner`/`Point`) are the existing reading for two coordinate lines; nothing reads a corner with a curve in it.

## What a fix needs

A presentation of a convex corner whose sides include a curve. One candidate: the region on the face side of the curved edge's closed form continued past the vertex. It holds the face's part of the cell where every other boundary curve in reach lies on its face side, and that must be decided at the vertex itself, where the other edge meets the continuation.

Equal-radius tees also pass through the walls' tangent points when the junction reaches the node plane; there no first-order separator is strict, which is `the-second-order-arm-certifies-a-declared-wedge-0-edge`'s ground or a node arm of its own.
