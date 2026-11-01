---
id: the-thread-count-digests-sym-frozen-counts-moved-on-main
kind: issue
title: mass_props_are_thread_count_invariant's sym-session frozen counts no longer match the committed digest on main (sym_arc_loft 1060→1065, sym_thin_strip 1177→1181)
status: closed
opened: 2026-10-12
closed: 2026-10-26
---


`sweep::all mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads`
(`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`, its
assertion at :538) is red on a clean checkout of `origin/main` at
`faf9995c`, at the default ε. It is a slow-set row, so main's per-PR
gate runs it only for a diff that touches sweep; PR 4743 (GERM) met it
on its hosted CI (run 38216487012) and reproduced it on clean main.

Every f64 line matches. Only the sym session's `frozen=` counts on
`validate_geometric` differ:

| line | digest (right) | measured (left) |
|---|---|---|
| `sym_arc_loft validate_geometric` | `frozen=1060` | `frozen=1065` |
| `sym_thin_strip validate_geometric` | `frozen=1177` | `frozen=1181` |

Every other field on those lines (`decisions`, `sz`, `num`, `shapes`) is
unchanged, so the walk's verdicts are the same and only the count of
frozen values moved. Something merged since the last re-cut
(`f50ab134`) freezes more values in the sym session's geometric
validation. The row says how to settle it: if the new count is right,
re-cut the table on the merge base and say what moved; if not, find
the change that froze them. Both earlier items on this row
(`work/nurbs/the-arc-loft-thread-count-digest-is-red-on-main.md`,
`work/nurbs/main-slow-set-reds-on-two-rows-since-4553.md`) are closed,
and neither covers this move.

## Closed (2026-10-26, tracker sweep)

**Superseded.** The thread-count digests have been re-cut several times
since this row was filed, for example `8bfc8e2319` (2026-10-21, which
names its `sym_arc_loft validate_geometric` move), `0da1cd59e7`
(2026-10-23) and `1cfffd8748` (2026-10-24). The lines this row compares
no longer exist. `crates/sweep/tests/thread-count-digest/eps-1e-9.txt:11`
and `:13` read `frozen=0` and `frozen=168` where the row had 1060/1065
and 1177/1181. A digest red on main today is a new reading and wants a
row of its own.
