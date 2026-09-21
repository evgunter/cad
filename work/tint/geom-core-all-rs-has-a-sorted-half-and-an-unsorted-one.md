---
id: geom-core-all-rs-has-a-sorted-half-and-an-unsorted-one
kind: issue
title: geom-core tests/all.rs accumulates in two halves and its ADDING A SUITE line names neither
status: open
opened: 2026-09-21
priority: P4
cost: E
refs: [ring-1-interval-type-ungated]
---

## What

`crates/geom-core/tests/all.rs` declares its suites in two halves: a
sorted `#[path]` block, then `test_utils::every_suite_file_is_aggregated!()`,
then twenty-plus further entries below the macro, unsorted and
blank-line separated, one per unit that has landed since. Both halves
compile and the guard sees both, so nothing is broken; what is missing
is a stated rule. The header's ADDING A SUITE paragraph says "drop the
file in `tests/` AND add a `#[path]` line below" and names neither
half, so each arriving lane guesses, and the guess that keeps the file
tidy is not the one the bottom of the file demonstrates.

The cheap fix is one sentence in the header saying which half a new
suite goes in (and why the other half exists); the thorough one is to
sort the whole list and let the guard's own failure mode — a suite
file that is not listed — remain the only thing a lane has to think
about. Either is a TINT/TCOST call about test-tree vocabulary, not an
individual unit's.

Sibling crates' `all.rs` files have the same shape and are worth the
same sweep in whichever direction this settles.

## Where

- `crates/geom-core/tests/all.rs` (the header's ADDING A SUITE
  paragraph; the macro call and the entries below it)
- found by RING-1's review (R2 S5), which added its pin to the sorted
  half

