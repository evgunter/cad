---
id: a-flush-reading-finds-no-point-on-a-nurbs-member-edge
kind: issue
title: The flush rule's Track reads every point as off a NURBS member edge, because Curve3::param_near has no parameter for a NURBS carrier
status: open
opened: 2026-10-07
priority: P3
cost: M
refs: [a-crossing-of-a-nurbs-edge-ties-for-want-of-its-parameter]
---


## What

`emit_topo::Track::offset` (the flush rule's reading of a curved
member edge, `names/README.md` "Flush edges") recovers a point's place
through `Curve3::param_near` and answers `None`, which means "off the
carrier", when that has no parameter. It has none for a NURBS carrier,
so on a NURBS member edge every point reads as off it. A union edge
running flush along one would then not be cited as a piece of it.

Unreachable today: the boolean refuses an operand with a NURBS edge
(`topo::boolean::reduce::gate_operand_edges`, `CurvedEdgeUnsupported`),
and a union runs its members through the same verb (`eval::wire::wire_union`).

Found by the sweep for `param_near` sites in
`work/emit/a-crossing-of-a-nurbs-edge-ties-for-want-of-its-parameter.md`.
That row's `emit_topo::chord_along` orders two points on one NURBS
piece, but it gives no parameter, and an on-the-edge test needs one, or
a certified distance to the piece.
