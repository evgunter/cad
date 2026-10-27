---
id: main-red-tilted-cut-upper-moves-in-the-thread-count-digest-at-default-eps
kind: issue
title: main red: tilted_cut_upper's volume, area and pads move by a few ulps in the thread-count digest at the default eps, since 692f50942c
status: open
opened: 2026-10-27
priority: P2
cost: E
---


## Finding

Found by PCERT E1b's suite round (PR #5797), and red on main itself.
`crates/sweep/tests/mass_props_are_thread_count_invariant.rs`'s
`the_walk_matches_the_serial_golden_at_four_threads` fails at the
default ε on one row only, `tilted_cut_upper`:

| field | got | golden |
|---|---|---|
| v | `3fd921fb54442d1b` | `3fd921fb54442d18` |
| a | `40096d2e7648c6cb` | `40096d2e7648c6c9` |
| vpad | `3e81270c3c2aaaac` | `3e812c48b2000000` |
| apad | `3ea9ba925a200000` | `3ea9c26d0ae00000` |

The verdict hash (`n=10 h=c5311dd0c395a321`) and every other row,
including the sym-session counts (`sym_thin_strip` frozen 168), match
the golden. Volume and area move by 3 and 2 ulps, and the pads shrink
slightly.

Red on main `be2b7a139c` and on `2f9e6f9e39`, with output identical to
the PR's tree. The digest held at all three ε in E1b's earlier round
on a tree carrying main `692f50942c`, so the move landed between
`692f50942c` and `be2b7a139c`; not bisected. The 1e-12 sibling is
`tilted-cut-upper-pads-moved-in-the-thread-count-digest-at-1e-12` (closed).
