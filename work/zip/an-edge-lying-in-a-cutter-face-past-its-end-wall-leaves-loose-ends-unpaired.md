---
id: an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired
kind: issue
title: A subtract whose cutter face holds an operand edge and ends inside the operand refuses Join(UnpairedLooseEnds), the kernel-bug refusal, on planar and curved operands alike
status: open
opened: 2026-09-25
priority: P0
cost: H
---


Found by CONTACT-2 once the plane×plane join lane stopped refusing a
conic between edge (`chord_join.rs` `between_edge_in_plane`): the
axis-plane lap it was dispatched for got one step further and refused
here instead.

## Measured

Operand A: an extrusion along `z` over `[0, 4]` whose profile has
vertices at `(±0.5, 0)`, so A carries two side edges `x = ±0.5,
y = 0`. Cutter B: `brick((-1, 1), (0, 1), (3, 4.5))`, whose face
`y = 0` holds both of A's side edges over `z ∈ [3, 4]` and whose end
wall `z = 3` sits inside A. `topo::subtract(A, B, Tol::witness())`:

| A | result |
|---|---|
| circle `r = 0.5` (two semicircles) | `Join(UnpairedLooseEnds { count: 4 })` |
| the square with corners `(±0.5, 0)`, `(0, ±0.5)` (all planar) | `Join(UnpairedLooseEnds { count: 4 })` |
| either, cutter `y ∈ [-1, 0]` | the same |
| either, cutter over `z ∈ [-1, 5]` (no end wall inside A) | builds, certifies, exact volume |
| the square, cutter `y ∈ [0.2, 1]` (no edge in the face) | builds, certifies, exact volume |

So the class is an operand edge LYING IN the partner's face, ending where
the partner's edge `y = 0, z = 3` crosses it, and it is not a curved
one: the diamond refuses identically. The refusal is raised at the end
of `boolean/join.rs` `connect` (`leftovers != 0`), and
`UnpairedLooseEnds`'s display calls itself a kernel bug. The
`boolean/ops.rs` module docs list "boundary-on-boundary configurations
that are not pure REST contacts" as surfacing it verbatim, which is a
disclosure but not a frontier a consumer can read: the pose is the
ordinary half-lap joint.

Pinned by `crates/sweep/tests/axis_lap.rs`
`axis_lap_refuses_where_its_planar_twin_does`.

## What the taker owes

Either the join learns to reuse the in-face edge as its section segment
(the REST zip's structural reuse, outside a declared union), or the
configuration refuses at a typed door that names it before the loose
ends are counted.
