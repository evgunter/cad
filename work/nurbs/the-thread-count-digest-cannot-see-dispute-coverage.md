---
id: the-thread-count-digest-cannot-see-dispute-coverage
kind: issue
title: The thread-count digest cannot see dispute coverage: its Sym line prints neither theorems_disputed nor retried
status: open
opened: 2026-10-12
priority: P3
cost: E
---


Found by the review of PR 4869 (NOTE-4).

The `sweep` thread-count digest's `Sym` line comes from `counts_line` in
`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`. It prints
`decisions`, `sz`, `sg`, `reg`, `rref`, `rcon`, `num`, `frozen` and
`shapes`. It does not print `SymCounts::theorems_disputed` or
`SymCounts::retried`.

That matters since PR 4869. With the meter computed inside the session,
PR 4834's rational-arm choice freezes the meter's forms on the two `Sym`
lofts' `validate_geometric`. Measured at ε 1e-9 on `sym_arc_loft`, the
freeze sites in the contradiction checks on signs the numeric channel
decided go from 54 to 1848; the review puts it at about 1.8k sites per
validation. A check over a frozen form can only dispute less than the
same check over the expanded form. Whether that lost any dispute
cannot be read off the digest: `frozen` moved, while the column that
would show lost coverage is not printed.

Proposal: add `theorems_disputed` (and `retried`, which costs nothing
more) to `counts_line`. Every `Sym` line of all three `eps-*.txt`
files moves with it, so it lands as its own re-cut, with the PR
4834 / 4869 baseline it reads stated beside it.
