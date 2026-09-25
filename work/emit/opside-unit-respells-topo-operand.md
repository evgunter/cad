---
id: opside-unit-respells-topo-operand
kind: issue
title: emit_topo's OpSide<()> re-spells topo::Operand {A, B}: two dataless operand-side vocabularies
status: closed
opened: 2026-09-23
priority: P4
cost: E
branch: emit/small-rows
closed: 2026-09-24
pr: 3175
---


## What

`crates/editor-core/src/names/emit_topo.rs` uses `OpSide<()>` as a
dataless operand side. Two places do this:
- the merged-chord read-through: `chord_descent`'s `want` and
  `chord_kind`'s `own`;
- `OpSide::other`.

`topo::Operand { A, B }` (`crates/topo/src/boolean/mod.rs`) already is
that vocabulary, and `crates/editor-core/src/eval/wire.rs` speaks it
(`topo::Operand::A`/`B`). So editor-core has two spellings of "which
operand, no key": one where a caller converts through `.with(())`, and
one where the kernel names it.

## Why it matters

Low: nothing is wrong. But a reader meets two answers to one question,
and a helper written for one (`other`, a side-to-`Operand` map) does
not serve the other.

## Found by

PR 3120's delta review.
