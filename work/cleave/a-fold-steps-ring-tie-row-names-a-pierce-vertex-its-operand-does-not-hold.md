---
id: a-fold-steps-ring-tie-row-names-a-pierce-vertex-its-operand-does-not-hold
kind: issue
title: A union fold step's ring-tie row names a pierce vertex the reduction minted off every edge split, so no operand table names it and the union holds it out of its log
status: open
opened: 2026-10-19
priority: P1
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
