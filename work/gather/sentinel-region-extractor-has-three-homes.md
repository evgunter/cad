---
id: sentinel-region-extractor-has-three-homes
kind: issue
title: Three hand-rolled sentinel-region extractors; test_utils::source::sentinel_region is now the home and topo's copy has not moved to it
status: open
opened: 2026-09-12
priority: P1
cost: E
---



## Finding

Raised by the delta review of PR 2480, as a class with a sweep
obligation. Confidence `sure`, the reviewer's.

A guard whose subject is one REGION of a file brackets it with two
sentinel comments and reads between them. Three sites had written that
walk themselves:

- `crates/editor-core/src/eval/mod.rs`'s node-kind vocabulary census —
  a `split_once` pair.
- PR 2480's operand-door census — a private `fn region`.
- `crates/topo/tests/shell_tolerance_chain.rs`'s `fn region(&self)`,
  which is nearly line-for-line the second.

`test_utils::source` is the declared home for exactly this class, and
none of the three was in it.

## What PR 2480 did

Hoisted it: `test_utils::source::sentinel_region(text, what, begin,
end) -> Range<usize>` is the home now, and **two of the three callers
moved onto it** — the operand-door census (which is the new one) and
`eval/mod.rs`'s node-kind census, both WIRE's ground.

## What is left

`crates/topo/tests/shell_tolerance_chain.rs`'s `fn region(&self)` has
not moved. It is TOPO's, TCOST's and TINT's ground by
`scripts/work.py territory`, so PR 2480 announced it rather than
editing it. Its shape differs in one way worth carrying across rather
than dropping: it also answers the FILE LINE the region starts at, so a
hit is reported where a reader can open it. If that is wanted generally,
it belongs on the shared helper rather than in the one caller.

**The sweep this row does not close**: the reviewer would look at every
file in the tree carrying an `X BEGIN` / `X END` comment pair, not only
at the three sites named above. Nobody has taken that grep.

## A second instance of the same shape, in the same file

The floor-message idiom — *"the census found only N — the sentinels or
the scan have drifted from the thing they read"* — is written three
times in one file. Two of PR 2480's three census rows replaced their
count floors with set equalities at the delta review's direction, so
the idiom has fewer instances than it did; whether the survivors want a
helper is the same question one level down.
