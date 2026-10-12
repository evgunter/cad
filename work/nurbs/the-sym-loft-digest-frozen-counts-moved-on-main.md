---
id: the-sym-loft-digest-frozen-counts-moved-on-main
kind: issue
title: mass_props_are_thread_count_invariant's sym_arc_loft and sym_thin_strip frozen counts moved on main at eps 1e-9
status: open
opened: 2026-10-12
priority: P1
cost: E
---


(ENCL, met on PR 4766's CI, which seeds the sweep slow set. Not fixed
there.)

## Measured

`sweep::mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads`
fails on `origin/main` alone at `faf9995c39` (a main-only worktree,
default eps 1e-9):

```text
cargo nextest run -p sweep --run-ignored all -E 'test(the_walk_matches_the_serial_golden_at_four_threads)'
```

Only the two sym-session rows' `frozen` counts move; decisions, `sz`,
`num`, shapes and verdicts are unchanged:

| row | committed `frozen` | main |
|---|---|---|
| `sym_arc_loft` | 1060 | 1065 |
| `sym_thin_strip` | 1177 | 1181 |

The digest is `crates/sweep/tests/thread-count-digest/eps-1e-9.txt`;
the other two eps rows were not run. Not bisected. The likeliest mover
since the last re-cut (`f50ab134e8`) is `f1d56fba5c` ("geom: memoise
the NURBS speed meter on the carrier", PR 4803), which changes how
often the walls' meter freezes a symbol; confirm before re-cutting.

## Repair shape

Confirm the mover, decide that the new counts are right, and re-cut the
digest at every eps row on main with the reason in the test's history
note (`mass_props_are_thread_count_invariant.rs`, above `expected`).
