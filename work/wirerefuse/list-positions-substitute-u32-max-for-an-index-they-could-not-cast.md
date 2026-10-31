---
id: list-positions-substitute-u32-max-for-an-index-they-could-not-cast
kind: issue
title: editor-core: a list position past u32 is clamped to u32::MAX, so two positions alias one slot
status: open
opened: 2026-10-19
priority: P3
cost: M
---


Found by the sweep of
`eval-handles-internal-breaks-three-ways-and-run-op-carries-node-facts-as-loose-options`
(silent defaults in `eval/`). Filed, not fixed there: it is not an
internal break (the evaluator disagreeing with itself) but a position
in an authored list, and the shape is crate-wide.

## Finding

`u32::try_from(i).unwrap_or(u32::MAX)` turns an index that does not
fit a `u32` into `u32::MAX`, so every index past the bound names the
same slot, member or row. There are 24 sites in `crates/editor-core/src`
(`grep -rn 'unwrap_or(u32::MAX)'`), and they fall into two classes.

**Positions in an authored list** (input ground):

- `Node::operand_rows`' `listed` and its siblings in `node.rs`, which
  number a loft's sections, a union's members and a placement's
  shapes;
- in `eval/wire.rs`: `run_op`'s `all` closure and its `Node::Place`
  arm, `wire_combine`'s member slots (`O::Member`, the member
  `index`), the fold step's `member`, and the judged rows' `place`
  pair;
- `edit.rs`, `operand.rs`, `product.rs` and `assembly.rs`. These have
  not been classified site by site.

**Counts of kernel output** (internal-break ground: the evaluator's own
records disagreeing with a `u32` index space they were minted into):

- in `eval/wire.rs`: the union fold's `Backing::Decided(base + k)`
  recitation, the published rows' `Backing::Decided(i)` recitation,
  `copy_of`'s copy `position`, and `wire_place`'s
  `u32::try_from(flat.len())` span bounds (the last two are D10
  placement ground).

**Three spellings today.** Beside the clamp, `names::to_u32`
(`names/emit.rs`) refuses the same overflow typed, as
`NamingError::Emission { what }`. Its callers are `emit_shell`,
`discriminate`, `emit_topo`, `eval/anchor.rs` and `eval/wire.rs`'s
`loop_coordinates`, which refuses one loop past the bound
(`addresses_every_loop_and_refuses_one_past_the_bound`). The third
spelling is the `unreachable!` that D9's row 4 would give.

## Shape to give

Each class gets one answer, and neither answer is a clamp.

- An authored list: a document door that refuses a list longer than
  `u32::MAX` positions, which makes every cast below it `unreachable!`
  (D9's row 4); or a typed refusal at each cast.
- Kernel output: `unreachable!` or `names::to_u32`'s typed refusal,
  whichever the internal-break rule settles on
  (`eval-handles-internal-breaks-three-ways-and-run-op-carries-node-facts-as-loose-options`,
  part 1).

## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).

## Re-counted (2026-10-26, tracker sweep)

`grep -rn 'unwrap_or(u32::MAX)' crates/editor-core/src` finds 23 sites:
`eval/wire.rs` 12, `operand.rs` 5, `node.rs` 4, `edit.rs` 1,
`product.rs` 1. `assembly.rs` no longer holds one.
