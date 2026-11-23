---
id: saddle-wall-certifies-at-1e-12-where-its-row-pins-the-cap
kind: issue
title: sweep saddle_wall_offset_fit: at CAD_TOLERANCE_EPS=1e-12 the saddle wall now certifies (3.85e-10), red against the row that pins it stopping at the cap
status: open
opened: 2026-10-13
priority: P2
cost: E
---


## Finding

`saddle_wall_offset_fit::a_saddle_wall_certifies_on_a_grid_longer_in_one_direction_than_a_square_cap`
(`crates/sweep/tests/saddle_wall_offset_fit.rs`, the assertion near the top
of the file) fails at `CAD_TOLERANCE_EPS=1e-12`:

> d = 0.05: the wall certifies at 3.850263672224892e-10

Reproduced on `origin/main` at the 2026-10-13 head, and in the PR gate's
1e-12 row of PR 5021, which does not touch the offset fit. The row pins
that at 1e-12 the fit stops at the cap (the closed
`a-saddle-walls-offset-fit-stalls-short-of-the-default-eps`). Something
on main moved that outcome: the likely candidate is f026b3643 ("one stop
rule (crate::descent) for the offset fit and limb 2's subdivision"). The
question is whether certifying at 1e-12 is the new correct behaviour. If
it is, re-baseline the row. If it is not, the stop rule regressed.
