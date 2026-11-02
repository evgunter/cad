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

## Fixed (2026-11-02, wire-refuse1, branch `wirerefuse/list-positions`)

`grep -rn 'unwrap_or(u32::MAX)' crates/editor-core/src` finds 0 sites (was 23).
Each class got one answer, and none is a clamp.

- **Positions in an authored list addressed by `OperandSlot` (row 0).**
  `OperandSlot::Section`, `Member` and `Traced` hold `usize`. The
  change stays inside the type and its readers: JSON spells the
  number the same, no derived `Hash` feeds a content key, and nothing
  outside editor-core and pncad-py named the `u32`. That removes
  `node.rs` ×3, `operand.rs` ×3, `edit.rs`, and the `eval/wire.rs` loft
  and member seats, plus a spelling the grep could not see: five
  `(0u32..)` / `(1u32..)` zips (`program.rs` `DrawnOn` ×3,
  `refactor.rs`, `eval/wire/trace.rs`). Those wrap in release past
  `u32`. pncad-py's `operand_index` lost its `usize::try_from(..).ok()`.
- **The same, on refusal payloads (row 0).** `TraceStepFault::NoSuchFace { traced }`,
  `NoSuchCrossing { found }` and `EditError::FreshUnread { index }`
  are `usize`. `FreshUnread` was clamped at `u16::MAX`.
- **A member's position among an n-ary operation's inputs** (judged
  pairs, a fold step's member, `UnionFoldStep`, `member_ranks`, an
  assemble's and a distribution's cited inputs). No door bounds a
  spelled list or a family's member count, so these refuse typed
  through the new `names::input_position`
  (`NamingError::Emission`), the same way `names::output_body` refuses
  a body index.
- **A family member's body index** (`wire_combine`'s `whole`,
  `distribute`'s family pieces): `names::output_body`, the bound every
  multi-body value already shares.
- **Kernel output:** `names::to_u32`, which is `unreachable!`.
  `Backing::Decided` rows in the union fold and in `distribute` use
  `ROWS_OF_ONE_BODY`, after topo's `cite_minted` ("a body's rows
  number fewer than u32"). `product.rs` `gather_sources`' input
  position: each grafted source adds at least one solid to the
  product's body arena.

Second pass (`unwrap_or(<int>::MAX)` for every width, `(Nu32..).zip`,
`as u32`): the `u8` index positions, the dead widening clamps and
two kernel-bounded zips are filed as
`an-indexed-reads-index-position-is-clamped-to-u8`. `mate/solve.rs`'s
`u8` count clamps and then takes `.min(6)`, so it agrees with the
unclamped count, and its `(3u32..)` zips are test code.
`role.rs`'s `as u32` compares enum discriminants.
