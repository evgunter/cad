---
id: list-positions-substitute-u32-max-for-an-index-they-could-not-cast
kind: issue
title: editor-core: a list position past u32 is clamped to u32::MAX, so two positions alias one slot
status: open
opened: 2026-10-19
---


Found by the sweep of
`eval-handles-internal-breaks-three-ways-and-run-op-carries-node-facts-as-loose-options`
(silent defaults in `eval/`). Filed, not fixed there: it is not an
internal break (the evaluator disagreeing with itself) but a position
in an authored list, and the shape is crate-wide.

## Finding

`u32::try_from(i).unwrap_or(u32::MAX)` turns a list position that does
not fit a `u32` into `u32::MAX`, so every position past the bound
names the same slot or member. 24 sites in `crates/editor-core/src`
(`grep -rn 'unwrap_or(u32::MAX)'`), among them:

- `Node::operand_rows`' `listed` and its siblings in `node.rs`, which
  number a loft's sections, a union's members and a placement's shapes;
- `eval/wire.rs`: `run_op`'s `all` closure and its `Node::Place` arm,
  `wire_combine`'s member slots and fold-step members, the union
  fold's `Backing::Decided` recitation, `wire_place`'s spans;
- `edit.rs`, `operand.rs`, `product.rs`, `assembly.rs`.

`eval/wire.rs`'s `loop_coordinates` refuses the same overflow for a
profile's loops (`addresses_every_loop_and_refuses_one_past_the_bound`),
so the class is handled two ways.

## Shape to give

One answer for the class: a document door that refuses a list longer
than `u32::MAX` positions (then every cast below it is `unreachable!`,
D9's row 4), or a typed refusal at each cast. Not a clamp.
