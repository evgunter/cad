---
id: the-sym-frozen-column-moved-with-the-speed-meter-memo
kind: issue
title: PR 4803's speed-meter memo moved the thread-count digest's validate_geometric frozen column on main (1060 → 1065, 1177 → 1181)
status: closed
opened: 2026-10-12
priority: P1
cost: E
refs: [4803, the-arc-loft-thread-count-digest-is-red-on-main]
closed: 2026-10-12
pr: 4814
---


## Measured

Found by FLUX's PR 4746, whose diff seeds the sweep slow set on CI. The test is
`sweep::all mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads`
at ε = 1e-9. It fails on `origin/main` alone at `ea91da9a2a` (a main-only
worktree), with the same diff CI showed.

Only the `# sym-session` `validate_geometric` lines move. Their decision
counts, `sz` and `num` do not:

| line | committed `frozen` | main |
|---|---|---|
| `sym_arc_loft validate_geometric` | 1060 | 1065 |
| `sym_thin_strip validate_geometric` | 1177 | 1181 |

A first-parent bisect over main's merges since PR 4551 (`aa1f124298`, the
last re-cut) shows:
- `98e7807a4f` (PR 4765) passes;
- `880e5d2508` (PR 4803, `nurbs/speed-meter-memo`: memoise the NURBS speed
  meter on the carrier) fails.

`854efb839a` and `6ce0c5b848` also pass. PR 4803's own CI did not run the
sweep slow set, since its diff touches `geom` only.

## The question

The memo changes how many expression nodes the symbolic session freezes
rather than builds. NURBS decides whether the move is right. If it is,
re-cut the digest line on the merge base and say what moved (the test's
message: "not a baseline to preserve"). If it is not, fix the memo.

## Closed

PR 4814 (`5e46b73af5`, merged 2026-10-12 08:45Z) re-cut both `Sym` lines at
all three ε, the same move this row measures: `sym_arc_loft` 1062/1060/1061 →
1067/1065/1066 and `sym_thin_strip` 1178/1177/1177 → 1182/1181/1181. Only
`frozen` moved. The class (a geom meter change merged without the digest that
reads it) is logged on NURBS's log at 07:50Z. The check is now part of every
NURBS meter dispatch.
