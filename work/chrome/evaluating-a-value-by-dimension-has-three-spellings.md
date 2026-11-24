---
id: evaluating-a-value-by-dimension-has-three-spellings
kind: issue
title: Evaluating an expression as a SlotValue by its dimension is spelled three times
status: open
opened: 2026-11-22
priority: P2
cost: E
---


## Finding

Found by FORMWORDS' `formwords/dimension-ladders` sweep for `Dimension`
branches in `crates/viewer/src`. That unit gave the field-writing
ladder (unit options, rendering unit, drag tick, count tick) one home,
`props::DimensionField`; this is the same shape on a different
question — *how a value of dimension D is evaluated* — and was left
alone as not that unit's subject.

Three sites each write `if dimension == Dimension::Count { eval_count(..)
.map(SlotValue::Count) } else { eval(..).map(SlotValue::Continuous) }`:

- `props::reading` (`crates/viewer/src/props.rs`), its `at` closure;
- `props::value_in` (`crates/viewer/src/props.rs`);
- `DocSession`'s constant fold of a typed expression
  (`crates/viewer/src/session.rs`, the `folded` binding beside
  `Refusal::ConstantRefused`).

They agree today. A fourth reader of a `SlotValue` would spell it a
fourth time, and the rule "a `Count` is evaluated exactly, as an
integer" has no door of its own to be changed at. A
`props::evaluated(expr, env) -> Result<SlotValue, _>` read by all
three is the obvious repair.

Not instances: `SlotValue::of` (`props.rs`) is the door from an `f64`
draft to a `SlotValue`, and `session.rs`'s `freed` match mints a
declaration rather than evaluating one.
