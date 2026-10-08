---
id: a-unions-same-member-declared-pair-records-no-row
kind: issue
title: An n-ary union's declared pair whose two sites are one member is verified at its fold step, and that step's row is dropped
status: parked
opened: 2026-10-08
priority: P1
cost: M
blocked_on: [coincidences-are-recorded-at-one-door]
---


## What

A glue decided from values and recorded nowhere the lint reads: a D10
hole (D10, Coincidence: every value decision is recorded at the door).

INTENT stage 4 PR B names a union's coincidence rows from its pairwise
judgement (`crates/editor-core/src/eval/wire.rs`, `wire_union` →
`judge_pairwise_contact`). Under #4323 (FORK-S4-4, FORK-S4U, fork log
row 94) that pass stays as the union's coincidence door, with rows in
member space; the list is the author's stated fold order, and the fold
reads one verdict per carrier pair through member lineage and re-decides
none.

`judge_pairwise_contact` skips a declared pair whose two sites are one
member (`if i == j { continue; }`): `Node::Union`'s docs route it as
that member's own carried contact, fed at the fold step the member
joins at. The fold step's boolean verifies it at its declaration door
and records a row, but that row's cells are keyed in the step's
operands (the accumulation and the member view), which name by the
fold's internal tables, and `wire_union` reads no fold step's rows.
So the glue is decided from values twice over nothing the lint reads:
the pass never judges it, and the fold's row is dropped.

## What closing it takes

Under #4323 a union's rows are in member space and the fold re-decides
nothing the pass did not. So the same-member pair's row is spelled in
the member's own table (the member's body is the step's operand B, or
operand A at the first step, so the step's row maps to member names
through that operand's view), it is the one row for that decision, and
it comes out in the author's list order with the rest. Whether the pass
decides the pair itself or the fold step's verdict is read through
lineage is the implementer's, provided the decision is made once.

The row that pins it: a three-member union declaring a pair within one
member records that pair's row, its cells named by that member, and
`run_checks` reports it.
