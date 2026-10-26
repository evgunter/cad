---
id: a-meet-row-on-a-vertex-a-boolean-minted-on-an-edge-omits-the-cell-that-minted-it
kind: issue
title: A boolean's Meet row on a vertex it minted on an edge names the edge and not the cell whose crossing minted it, so one point spells two ways across a union's member orders
status: dispatched
opened: 2026-10-19
priority: P1
cost: M
---


Filed by WIRE's `wire-dmlog` lane (branch `wire/union-decision-log`).

## What

`topo/src/boolean/ops.rs` `spell` reads a vertex the reduction minted on
an edge as that edge (`input_end`, through `EdgeSplits::minted_on`), and
where that widening would change what the row says it publishes the
`Relation::Meet` of the two cells read. The point was minted where the
edge crossed some cell of the other operand, and `EdgeSplits` keeps only
the parent edge, so the row names the edge and the cell the point was
decided on, never the cell whose crossing minted it. `Meet` then says
"these carriers share a point", which two collinear legs, or a line and
a plane, do anyway: true, but weaker than the decision.

Measured on `union_pinch_member_order::pinch` (two blocks on a plate,
`union_member_orders_decide_alike::a_vertex_row_is_the_unions_once_in_every_member_order`
and `a_re_cited_record_cites_the_rows_its_steps_decided_in_every_member_order`):
- folded last (`[2, 1, 0]`), the plate's step decides the points its top
  made on the two blocks' legs one, and `topo` spells it
  `Meet {Edge@m1, Edge@m2}`;
- folded first, the second block's step meets the point the first one's
  leg made on the plate, a `Crossing` the fold named, and `wire` spells it
  `Meet {Edge@m1, Edge@m2, Face@m0}`.

One point, two spellings, against DM4's "one fact has one spelling"
(`crates/editor-core/REFERENCES.md`, "Contact is judged pairwise").

## What it needs

The reduction keeping, for each vertex it mints on an edge, the cell of
the other operand whose crossing minted it (where one did), so `spell`
can widen the vertex to both: the edge and that cell.

## The `OnCarrier` arms drop it too (PR 5378's review)

`spell`'s two `OnCarrier` arms lose the minting cell the same way:
- a point of one operand on a vertex minted on the other's edge, the
  second cell read as that edge, keeps `OnCarrier` on the edge;
- two points decided one, the first a vertex minted on an edge, spelled
  the other way round onto that edge.

Each is true: the point lies on the edge. Each is also weaker than the
decision, which put it where the edge crossed the minting cell. The fix
above serves all three arms: with the minting cell kept, the second
cell widens to the edge and that cell, so the row becomes a `Meet` of
the point and both.

## The ring tie's case (PR 5753, 2026-10-26)

`topo::Coincidence::third` (`coincidence.rs`) carries a `Meet`'s third
cell. `boolean/ops.rs` `spell` fills it with the pierced face where a
ring tie's two cells both widen
(`a-fold-steps-ring-tie-row-names-a-pierce-vertex-its-operand-does-not-hold`).
The pinch measured above is that case. Folded last,
`union_pinch_member_order::pinch` now spells
`Meet {Edge@m1, Edge@m2, Face@m0}` too, so the point has one spelling in
every member order
(`union_member_orders_decide_alike::a_vertex_row_is_the_unions_once_in_every_member_order`).

What stays open is the minting cell outside ring ties: the three arms
above, for a vertex minted on an edge that no ring tie decides.

**It needs `Coincidence` to hold a list of cells.** A ring tie whose
pierce was minted on an edge meets in four cells: the edge, its minting
cell, the face holding the strut and the pierced face. Where the pierced
face is the minting cell, those are three. A fold row is already spelled
as "the set of member cells that meet" (`crates/editor-core/REFERENCES.md`,
DM4, "A row names member cells"), and `NamedCoincidence::cells` is a
list. The kernel row's `[RowCell; 2]` plus `third` is the stopgap: it
keeps `Copy` and reaches no four-cell case today.

## Moved (2026-10-26)

Moved from `work/cleave/` to WIRE by `git mv` in PR 5753, banded P1 (a
cell the row omits is a misnamed row), on the CLEAVE orchestrator's
ruling. PR 5765's triage had left it unbanded on CLEAVE while 5753 was
editing it.
