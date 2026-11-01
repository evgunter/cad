---
id: a-unions-flush-reading-cannot-follow-a-nurbs-carrier
kind: issue
title: A union's flush reading cannot follow a NURBS carrier: such an edge is a line of its own and covers nothing, whatever lies flush with it
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [a-flush-reading-finds-no-point-on-a-nurbs-member-edge]
---


## What

The union's flush reading reads a curved edge along its certified
carrier (`emit_topo::EdgeCarrier`, `Track`), and `Track` places a point
through `geom::Curve3::param_near`, which recovers no parameter on a
NURBS carrier. So `EdgeCarrier::of_edge` reads such an edge as
`EdgeCarrier::Unread`:

- in the line grouping (`emit_topo::sets_per_line`, reached from the
  pair's `flush_lines` and the union's `Flush::meeting`) and in
  `emit_union::Flush::same_way`, it is a line of its own, so two NURBS
  member edges running flush through one vertex are two lines of its
  run, where N2 counts them once;
- in `emit_union::Flush::edge_cover` it covers nothing, so a finished
  NURBS edge keeps the name the fold gave it (the member the fold kept)
  rather than the least member edge it lies within.

Unreachable today: the boolean refuses an operand with a NURBS or
spiric edge (`topo::boolean::reduce::gate_operand_edges`,
`CurvedEdgeUnsupported`), and the union folds its members through it.
The arm exists so that the reading refuses nothing a union names once
that gate opens.

## Next

When the gate admits a NURBS edge, place a point on it by a certified
projection (or a certified distance to the piece), as
`a-flush-reading-finds-no-point-on-a-nurbs-member-edge` needs for the
subtract, and read it as `EdgeCarrier::Curved`. A fixture: two lofts
sharing a wavy edge, crossed by a plate through it, in every member
order.

## Moved to SPLICE (2026-10-25)

From SEAMRANK. Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. Re-banded P3 → P1: a union's flush and rest readings on a lofted operand (lily wall 8's sheath on its blade) read these NURBS edges.

Moved from SPLICE to SPLICECARRIER on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

## Re-banded P1 → P3 (SPLICE, 2026-10-29)

The consumer that put this row at P1 is gone. Lily wall 8 is no longer
the sheath grafted on its blade (an exact planar mate between two
lofts): the scene builds the leaf as one loft, and wall 8 is now the
leaf grown out of the stem, a transverse crossing with nothing flush
(`[ev]` PR 5990, PR 6042). The sweep row `a_sheath_grafted_on_its_blade`
still builds the graft, but it stops at its rim's tangency, before any
flush reading. No other live demo or row reaches this reading.
