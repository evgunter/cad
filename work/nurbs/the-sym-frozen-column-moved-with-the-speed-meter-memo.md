---
id: the-sym-frozen-column-moved-with-the-speed-meter-memo
kind: issue
title: PR 4803's speed-meter memo moved the thread-count digest's validate_geometric frozen column on main (1060 → 1065, 1177 → 1181)
status: open
opened: 2026-10-12
priority: P1
cost: E
refs: [4803, the-arc-loft-thread-count-digest-is-red-on-main]
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
