---
id: the-legal-operand-check-reads-only-that-a-far-union-builds
kind: issue
title: The legal-operand check reads only that a union with a far brick builds, not that it holds the brick and the result
status: open
opened: 2026-10-11
priority: P3
cost: E
---



Found by the review of CLEAVE's PR 4725 (`cleave/missed-ray-side`).

## What

Two helpers ask whether a boolean result is a legal operand, and both
read only that `topo::union(result, brick (50..51)³)` returns `Ok`:

- `crates/sweep/tests/common/differential.rs` `outcome`, the
  `operand=` flag. 27 suites under `crates/sweep/tests/` read
  `outcome`.
- `crates/sweep/src/test_support.rs` `assert_legal_operand`.

A union that builds the wrong body passes both. Two examples: a
result read inside-out so the union answers for its complement, or a
far brick dropped.

PR 4725 re-baselined a row's `operand=false` to `true` on this
reading. Its review found that `true` asserts only that the union
builds, so a wrong union would pass.

## The shape to give

Hold the union to its volume: the result's certified volume plus the
brick's 1, within both pads. Also hold it to its solid count: the
result's solids plus one. PR 4725 does this by hand in
`a_plane_across_a_one_face_wall.rs` `every_op`. Moving it into the two
helpers covers every suite that reads them.
