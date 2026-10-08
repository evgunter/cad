---
id: a-unions-same-member-declared-pair-records-no-row
kind: issue
title: An n-ary union's declared pair whose two sites are one member is verified at its fold step, and that step's row is dropped
status: open
opened: 2026-10-08
priority: P3
cost: M
---


## What

INTENT stage 4 PR B names a union's coincidence rows from its pairwise
judgement (`crates/editor-core/src/eval/wire.rs`, `wire_union` →
`judge_pairwise_contact`), where each pair runs on the two members'
own bodies, so its cells name by the members' own tables and the rows
are the same in every member order (FORK-S4-4's recommendation (a)).

`judge_pairwise_contact` skips a declared pair whose two sites are one
member (`if i == j { continue; }`): `Node::Union`'s docs route it as
that member's own carried contact, fed at the fold step the member
joins at. The fold step's boolean verifies it at its declaration door
and records a row, but that row's cells are keyed in the step's
operands (the accumulation and the member view), which name by the
fold's internal tables, and `wire_union` reads no fold step's rows.
So the glue is decided from values and recorded nowhere the lint reads.

## What closing it takes

Name the fold step's rows for same-member pairs through the member's
own table (the member's body is the step's operand B, or operand A at
the first step), or judge a same-member pair on the member's body
alone. Which one depends on FORK-S4-4's answer (whether the pairwise
judgement stays the union's census).
