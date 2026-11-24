---
id: the-drive-leaf-frontier-spells-the-clearance-walks-admission-arithmetic
kind: issue
title: The driver's leaf frontier (drive.rs) spells the same level-synchronous frontier, whole-level budget check and admission arithmetic as topo::clearance's shared walk
status: open
opened: 2026-10-29
priority: P3
cost: M
---

## Finding

CLEAR's LOOPS unit folded `Sweep::run` and `min_separation` into one
frontier walk (`topo::clearance::walk`, with `Walk::split` holding the
depth floor, the admission check and the `f64`-grid refusal). Its
sweep for the same shape found a third copy across the crate line:
`editor-core/src/drive.rs`'s leaf loop in `drive` (near the
`frontier.drain(..)` at line 1395) checks the whole level against
`max_leaves` before it starts, then refuses a split at admission with
`certified + refused + pending + next.len() + 2 > max_leaves` — the
clearance walk's arithmetic over parameter boxes instead of cell pairs.
Both module docs call the shape "the driver's own", so the duplication
is acknowledged, not accidental.

It was not folded in LOOPS: the driver replays a whole level's boxes as
a batch (`frontier.drain(..).zip(verdicts)`) before it classifies any,
and its leaves are records, not receipt counts, so a shared frontier
would be a generic one in a crate both sit on. Whether that type is
worth having is this row's question.
