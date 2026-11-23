---
id: main-red-spiric-arc-interval-fuzz-fails-at-1e-6-on-seed-0x139cdf42d954efca
kind: issue
title: spiric_arc's random_arcs_at_interval fuzz fails at 1e-6 on seed 0x139cdf42d954efca
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [7501]
---

Filed by the CIW orchestrator, 2026-11-23, from PR 7501's gate (run
45184032071, `test (kernel)`, the ε = 1e-6 row).

`topo splitting::spiric_arc::fuzz::random_arcs_at_interval` failed on
fuzz seed `0x139cdf42d954efca` (effort 1). Replay it with
`CAD_FUZZ_SEED=0x139cdf42d954efca CAD_TOLERANCE_EPS=1e-6`. The run's
summary line read "127 far asked, 34 rays abandoned, 40 near asked, 0
refused past band, 0 on-arc refused, 0 cuts the mint refuses".

PR 7501's only change in `topo/src` was a rustfmt reformat of
`ring_path.rs`, so the seed is the finding and the PR is not its
cause. `spiric_arc` moved three times in the day before the run
(`1d9b888752`, `85bfd81ce6`, `fef5755439`, all GERM's). The seed varies
per run, so a pass on another PR's gate does not clear this row: replay
the seed.

Band P1: a varying-seed row reds other programs' PRs at random
(`work/ciw/a-pr-gate-runs-varying-seed-fuzz-rows-that-fail-on-main.md`).
It is not P0, because no wrong answer is shown yet.

What would close it: the seed replays green at 1e-6, or the failure is
explained and the seed pinned with its owning row.
