---
id: the-snowman-pole-cut-row-sits-on-the-one-second-bar
kind: issue
title: sweep snowman::a_pole_to_pole_cut_of_a_ball_wedge_builds sits on the fast set's one-second bar and reds a PR whose diff is doc comments
status: closed
opened: 2026-11-19
closed: 2026-11-22
priority: P4
cost: E
---


Filed by the SCALAR orchestrator, 2026-11-20. Same class as
`a-fast-set-germ-row-sits-on-the-one-second-bar-and-reds-a-docs-only-commit`.

PR 7154's head `c2318dba83` changes a README, a fork-log row and one
doc comment in `crates/geom-core/src/real.rs`. Its `test (kernel)` job
(run 44695147428) failed on the timing bar alone:
`sweep::all snowman::a_pole_to_pole_cut_of_a_ball_wedge_builds took
1.059 s (existing test: 1.0 s)`. The same branch's earlier head was
green on the same row. The row belongs in the slow set
(`.config/nextest.toml`), or needs to be made faster; the failed job
was re-run.

## Closed (2026-11-22, green/fast-set-margin)

`snowman::a_pole_to_pole_cut_of_a_ball_wedge_builds` is already in the
slow set (1.033 s on PR 7208's hosted run, in `.config/nextest.toml`).
The class is `the-one-second-bar-reds-prs-off-their-path-several-times-a-day`'s,
which PR 7397 decides.
