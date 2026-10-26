---
id: a-boolean-zero-decided-and-consumed-in-place-reaches-no-row
kind: issue
title: A Zero a boolean decides and consumes in place, never reaching its pending decisions, is no row and no sweep sees it
status: open
opened: 2026-10-19
priority: P3
cost: M
---


Filed by WIRE's `wire-citerows` lane (PR 5352), the blind spot of its
sweep.

## What

PR 5352 makes every decision a boolean records in its pending list
(`boolean/reduce.rs` `ContactAcc::decide`, `pend`, `ops.rs`
`decide_one`) with a margin a row, or holds it unspelled
(`BooleanNaming::unspelled`). Its sweep read only what reaches that
list or a `Coincidence` constructor. A margin a boolean decides Zero
and acts on without pushing a pending row (a sign read that picks a
branch, merges two keys, or skips a pair) is invisible to it: 41 files
under `crates/topo/src/boolean/` mention `Zero`, and none of their
in-place Zero arms was audited.

## What it needs

An audit of every `Sign::Zero` / Zero-classified arm under
`crates/topo/src/boolean/`: each either reaches `pend` (or the glue
door's recorded rows), refuses, or is shown to decide no coincidence
(D1: an ON verdict that only places topology). The hit list and each
disposition go in the row. D10 `[coincidence.door]`: every coincidence
the kernel infers from values is recorded.

Signed (WIRE implementer, wire-citerows).

## Slate triage (2026-10-26, CLEAVE)

- Re-banded P2 → P3: an audit of Zero arms no sweep has read: tooling that keeps a recorded decision from going silently missing (D10 `[coincidence.door]`), P3 by `work/README.md` Priority.
- Moved to WIRE (`work/wire/`) by `git mv`, id and body unchanged: whose lane filed it: the rows a node publishes and the lineage they are carried by. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
