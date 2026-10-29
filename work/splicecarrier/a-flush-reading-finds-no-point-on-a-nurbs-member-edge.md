---
id: a-flush-reading-finds-no-point-on-a-nurbs-member-edge
kind: issue
title: The flush rule's Track reads every point as off a NURBS member edge, because Curve3::param_near has no parameter for a NURBS carrier
status: open
opened: 2026-10-07
priority: P3
cost: M
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

## Moved to SPLICE (2026-10-25)

From SEAMRANK. Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. Re-banded P3 → P1: a union's flush and rest readings on a lofted operand (lily wall 8's sheath on its blade) read these NURBS edges.

## Re-pointed (WIRE orchestrator, 2026-10-26)

Closed by PR 5758 (2026-10-26, "nothing is ranked", Ev's ruling on PR 5705). `chord_along` is deleted. The finding stands on `Track::offset`.

Moved from SPLICE to SPLICECARRIER on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

## Re-banded P1 → P3 (SPLICE, 2026-10-29)

The consumer that put this row at P1 is gone. Lily wall 8 is no longer
the sheath grafted on its blade (an exact planar mate between two
lofts): the scene builds the leaf as one loft, and wall 8 is now the
leaf grown out of the stem, a transverse crossing with nothing flush
(`[ev]` PR 5990, PR 6042). The sweep row `a_sheath_grafted_on_its_blade`
still builds the graft, but it stops at its rim's tangency, before any
flush reading. No other live demo or row reaches this reading.
