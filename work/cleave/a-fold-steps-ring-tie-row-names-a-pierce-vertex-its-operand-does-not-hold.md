---
id: a-fold-steps-ring-tie-row-names-a-pierce-vertex-its-operand-does-not-hold
kind: issue
title: A union fold step's ring-tie row names a pierce vertex the reduction minted off every edge split, so no operand table names it and the union holds it out of its log
status: closed
opened: 2026-10-19
priority: P1
pr: 5753
branch: cleave/ring-tie-spells-its-pierce-vertex
closed: 2026-10-26
---


Filed by WIRE's `wire-dmlog` lane (PR 5378's fix pass).

## What

`emit_union_borders::a_boss_whose_corner_touches_the_divider_is_part_of_it`
(`corner_boss`, order `[1, 2, 0]`) has a fold step whose `topo` rows
include `OnCarrier [A Vertex(33v1), A Edge(26v1)]`, `VertexFusion`,
margin `Value(0.0)`. No record cites it. The accumulation holds 16
vertices and its table does not name vertex 33.

By the reading of `boolean/ops.rs` `ledger`, `decide_one` and
`spell`: this is a ring tie, two pierces of one face served by one ring
(`vtxfac::ring_groups`). One pierce vertex was minted on an edge and
reads back to it (`input_end`). The other was minted without an
`EdgeSplits` row, which looks like a held pierce's strut
(`vtxfac::faces_holding_pierce`). It stays a clone key, so `spell`'s
swap arm publishes a cell that is not of the op's input.

A pair boolean would hit the same thing: `coincide::name_rows`
refuses a cell no input names. `eval/wire.rs` `wire_combine` holds an
uncited fold row like this out of the union's log (`FoldRow::Unnamed`).
A cited one still refuses.

## What it needs

`spell` reading such a vertex to the input cell it was minted in (the
piercing operand's face that holds the strut), and the ring tie spelled
as the `Meet` of the two pierces' cells and the pierced face
(`PierceRingRecord::face`). Then the union's hold-out goes.

## Raised to P1 (WIRE orchestrator, 2026-10-20)

D9 as ratified on PR 5411 says no input should be able to lead to a
known panic. This is a known-reached kernel defect: the corner-boss
union reaches `ROW_CELL_NOT_IN_TABLE`. editor-core's `held_out` currently
absorbs it silently ([loud-failure]). WIRE's conversion deletes
`held_out` and makes the site a panic, which is right only once this is
fixed. WIRE holds that step until this row closes.

## Measured, and spelled (2026-10-26)

The reading holds. Instrumented on `corner_boss` in order `[1, 2, 0]`,
the step's ring tie is `RingTie { a: 22v1, b: 33v1 }`. Vertex 22 is
minted on edge 26, and the edge-split record reads it back to that edge. Vertex 33 has
no split row. It is the vertex `vtxfac::mint_held` mints inside a face
of the piercing operand (the slab's `x = 1.4` wall, which holds the
boss's corner edge). One ring on the plate's top serves both pierces:
`PierceRingRecord { face: 1v1, pierces: [22v1, 33v1] }`.

A topo row now holds that decision in input cells:
- `boolean/mod.rs` `Mints` records where the reduction minted each
  vertex: on an edge it split, or inside a face where a held pierce
  stands (`Mints::minted_in`).
- `boolean/ops.rs` `input_end` reads a held vertex as its face.
- `ledger` keeps each tie's decision with the face its ring serves
  (`PierceRingRecord::face`, through `ring_of`), in `Ledger::pierced`.
- `spell` spells a point whose two cells both widen as the `Meet` of
  the two cells read and the pierced face. The decision was taken
  against that face. Here the two cells alone would not fix the point,
  because the boss's edge lies in the slab's wall.
- The sweep's check, that every input cell a row names (third
  included) is held by its input, is parked on branch
  `cleave/rows-name-input-cells-guard`. It waits for WIRE's re-chart
  row, whose cells still fail it on main.
- The third cell rides `topo::Coincidence::third`. editor-core names
  it (`coincide::name_rows`), and a fold step spells it in member space
  (`eval/wire.rs` `in_member_space`).

The corner boss's union logs the row in every member order, spelled
alike as the meeting of the boss's edge at (1.4, 1.0), the slab's
`x = 1.4` wall and the plate's top. Every pair boolean of the
slab-and-boss union with the plate logs it too: union and intersect in
both orders, and the subtract of the plate. On main each of those
refuses `ROW_CELL_NOT_IN_TABLE` in `coincide::name_rows`. Pinned by
`emit_union_borders::the_corner_boss_logs_its_corner_alike_in_every_order_and_every_pair_boolean`,
which fails on main at `[1, 2, 0]`.

The same rule spells the two blocks pinching on a plate
(`union_pinch_member_order::pinch`, folded last) as
`Meet {Edge@m1, Edge@m2, Face@m0}`, as its other orders already did.
That is the measured case of
`a-meet-row-on-a-vertex-a-boolean-minted-on-an-edge-omits-the-cell-that-minted-it`.

### The sweep

The sweep checks every row a boolean returns through `topo::boolean_op_with`
against its operands: a row cell the operand body does not hold
(`records::holds`). It also counts each `ROW_CELL_NOT_IN_TABLE` raised in
`coincide::name_rows` and each `held_out` firing in `wire_combine`.
Default ε, over editor-core's `all` tests matching
`emit_|union|subtract|intersect|boolean|coincid|name_invariance|pinch|wire_legal`
(343 tests) and topo's whole `all` suite (1136):

| | main | branch |
|---|---|---|
| editor-core rows naming an unheld cell | 12, in 3 tests | 0 |
| editor-core `ROW_CELL_NOT_IN_TABLE` / `held_out` | 12 / 12 | 0 / 0 |
| topo rows naming an unheld cell | 77, in 5 tests | 0 |
| sweep `all` (3156 tests) cells of this class | 160, in 4 `a_pierce_held_in_a_curved_face` tests | 0 |

Every hit on main has this row's shape: a held pierce's vertex
`OnCarrier` an edge. The editor-core hits are in
`emit_union_borders::a_boss_whose_corner_touches_the_divider_is_part_of_it`,
`emit_union_borders::a_corner_touching_the_near_half_of_a_joined_seam_is_part_of_the_divider`,
and the corner boss in `every_fold_row_names_every_member_cell_its_cells_are_made_of`.
The topo hits are `a_vertex_read_again_classes_every_edge::every_edge_a_vertex_read_again_reads_is_classed_against_the_germ`
(60) and four `three_solids_on_one_line` tests (17). topo names no
rows, so those pass on main.

What the sweep cannot see:
- a minted key that collides with a live key of the input;
- rows from a split, or from any door but `boolean_op_with`;
- tests outside those sets;
- ε rows other than the default.

The sweep-crate counts hold at ε 1e-6 and 1e-12 as well. `sweep` also
holds 72 re-chart cells, the same on main and on the branch. Those are
a different class, on WIRE's
`a-re-charted-operands-rows-may-name-cells-the-re-chart-minted`.

