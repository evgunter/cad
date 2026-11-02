---
id: an-indexed-reads-index-position-is-clamped-to-u8
kind: issue
title: editor-core: an indexed read's index position is clamped to u8::MAX, and four widening casts carry a dead clamp
status: open
opened: 2026-11-02
priority: P4
cost: E
---



Found by the second pass of the sweep for
`list-positions-substitute-u32-max-for-an-index-they-could-not-cast`
(branch `wirerefuse/list-positions`). That pass used the patterns
`unwrap_or(<int>::MAX)` and `(Nu32..).zip` across `crates/editor-core/src`.

## Finding

**An index position narrowed to `u8` (authored ground).**
`SlotId::Index { seat, k: u8 }` addresses the `k`-th index of an
indexed read (`xs[i, j]`). Two sites narrow `k` with a clamp:

- `crates/editor-core/src/operand.rs`, `BodyRead::rows` (two spellings:
  shared and exclusive), `u8::try_from(k).unwrap_or(u8::MAX)`;
- `crates/editor-core/src/eval/wire.rs`, the member read that resolves
  a read's indices (`(0..rank).map(|k| SlotId::Index { k: u8::try_from(k).unwrap_or(u8::MAX), .. })`).

A read's index count must equal its family's rank (`InputFault::IndexRank`,
at both doors). No door bounds the rank, so a family keyed by more than
256 indices gives two indices one slot. Row 0 is the first question:
`k: usize` would match what `OperandSlot`'s positions became.

**Dead clamps on conversions that cannot fail (no alias).** Each of these
widens to a type that holds every value on the targets the workspace
builds, so the `unwrap_or` never fires. It still reads as a saturation:

- `usize::try_from(<u32>).unwrap_or(usize::MAX)`:
  `crates/editor-core/src/product.rs` (a split half's `output_body`),
  `crates/editor-core/src/eval/wire.rs` (the same, in the split
  projection), and `crates/editor-core/src/names/emit_topo.rs` (`ix`);
- `i64::try_from(<len>).unwrap_or(i64::MAX)`:
  `crates/editor-core/src/edit.rs` `check_loft_shape` (a `Vec`'s length
  is at most `isize::MAX`), and `eval/wire.rs` `need_count`'s `wide`.
  `wide` is documented as a clamp of a domain end, so it may be right as
  written;
- `u64::try_from(n).unwrap_or(u64::MAX)`: `crates/editor-core/src/eval/counters.rs`.

**Kernel-bounded `zip(0u32..)` (no alias today).**
`crates/editor-core/src/checks/at_rest.rs` `copy_groups` and
`crates/editor-core/src/product.rs` (the censused `decided` groups)
number a product's grafted copies with `zip(0u32..)`. Each copy grafts
at least one solid into the product's body, whose arena numbers its
solids within `u32`, so the range cannot run out. In release, a broken
bound would wrap silently rather than panic.

## What is owed

The `u8` positions take row 0 or a door bound on a family's rank. Each
widening becomes the infallible form or an `unreachable!` that states
why it cannot fail. The two zips state their bound, as
`product.rs` `gather_sources` does for its input position (`names::to_u32`).

## The u8 half fixed (2026-11-02, `wirerefuse/list-positions` review pass)

`SlotId::Index { k }` is `usize` (row 0), so `BodyRead::rows` and the
member read no longer clamp. Its label widens before the `+ 1`
(`operand::ordinal`). What stays open here: the dead widening clamps
and the two kernel-bounded zips.
