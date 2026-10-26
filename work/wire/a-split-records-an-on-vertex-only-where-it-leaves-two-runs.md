---
id: a-split-records-an-on-vertex-only-where-it-leaves-two-runs
kind: issue
title: A split records a margin-decided ON vertex only where it leaves two runs on a side, so an ON vertex that only places the cut is no row
status: open
opened: 2026-10-19
priority: P3
cost: E
design: true
---


Filed by WIRE's `wire-citerows` lane (branch
`wire/boolean-rows-every-decision`), from the sweep for the shape of
`a-boolean-records-a-zero-decision-only-where-a-record-cites-it`.

## What

`topo` `splitting/mod.rs` (the split reduction's `on_vertices` loop)
emits a `SplitOn` row for an operand vertex decided ON the plane by a
margin only where its neighbourhood leaves two or more runs on a side
(pieces touch there). An ON vertex with one run is decided by the same
margin and records nothing. The boolean now records every margined
decision (`boolean/ops.rs` `cite_rows`); the split keeps a presence
filter of the shape that unit removed.

## The question

Whether D10 `[coincidence.door]` ("every coincidence the kernel infers
from values is recorded") reaches a split's ON verdict that only places
the cut (D1 calls an ON verdict that only places topology no
coincidence), or only one where pieces touch. Not the boolean's, so not
that unit's.

Signed (WIRE implementer, wire-citerows).

## Slate triage (2026-10-26, CLEAVE)

- Moved to WIRE (`work/wire/`) by `git mv`, id and body unchanged: whose lane filed it: the rows a node publishes and the lineage they are carried by. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
