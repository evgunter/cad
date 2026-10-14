---
id: the-thread-count-digest-cannot-see-dispute-coverage
kind: issue
title: The thread-count digest cannot see dispute coverage: its Sym line prints neither theorems_disputed nor retried
status: closed
opened: 2026-10-12
priority: P3
cost: E
branch: nurbs/digest-dispute-coverage
closed: 2026-10-12
pr: 4887
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

## Closed (2026-10-12)

`counts_line` prints `disp=` (`SymCounts::theorems_disputed`) on every
`Sym` line; all three ε are re-cut and nothing else moved. It reads 0
on all twelve lines. To settle the coverage question, the digest was
also run with the meter's arm chosen on the weights' value channel, as
before PR 4834: `frozen` reads 1062 / 1060 / 1061 (`sym_arc_loft`) and
1178 / 1177 / 1177 (`sym_thin_strip`), and `disp` is still 0 at every
ε. So the rational arm's extra freezes cost no dispute on these lofts.

`retried` is not printed. The session block opens `with_session`,
which installs no retry ladder, so the column would be zero by
construction: an assertion no bug could break.

