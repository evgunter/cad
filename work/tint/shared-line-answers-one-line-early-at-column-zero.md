---
id: shared-line-answers-one-line-early-at-column-zero
kind: issue
title: test_utils::source::line answers the previous line for an offset at column 0
status: open
opened: 2026-09-24
priority: P3
cost: D
---

## What

`crates/test-utils/src/source.rs` `line` (~424) documents itself as
"**The 1-based line `at` falls on**" and is
`text[..at].lines().count()`. `str::lines` does not yield the empty
line after a trailing `\n`, so for an `at` at the first byte of a line
the count is the number of lines BEFORE it: offset 0 answers `0`, and
an offset just after the `n`-th newline answers `n` rather than `n+1`.
Any offset past column 0 answers correctly.

Column 0 is exactly where a top-level item keyword sits, so every
census that reports an `impl`, `fn`, `struct` or `mod` it found at the
start of a line cites the line above it. Observed on LANE-4P's fix
pass (PR 3165): a planted `impl<T: Tr<{ 1 }>> Planted<T> {` on line 5
of `crates/topo/src/zz_plant.rs` was reported as
`crates/topo/src/zz_plant.rs:4`, and a test fixture with an `impl` on
line 1 answered `x.rs:0`.

## Proposed

`text[..at].matches('\n').count() + 1` (or `bytecount` of `\n` plus
one), with a row in `source.rs`'s tests at offset 0, at the first byte
after a newline, and mid-line. About 165 files import from
`test_utils::source`; a test that asserts a column-0 line number
through `line` moves by one and is re-baselined, not preserved.

## Found by

LANE-4P's fix pass (PR 3165), writing a row that asserted the line of
an unreadable `impl` head.
