---
id: a-meet-row-on-a-vertex-a-boolean-minted-on-an-edge-omits-the-cell-that-minted-it
kind: issue
title: A boolean's Meet row on a vertex it minted on an edge names the edge and not the cell whose crossing minted it, so one point spells two ways across a union's member orders
status: open
opened: 2026-10-19
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
