---
id: main-red-spiric-arc-interval-fuzz-fails-at-1e-6-on-seed-0x139cdf42d954efca
kind: issue
title: spiric_arc's random_arcs_at_interval fuzz fails at 1e-6 on seed 0x139cdf42d954efca
status: closed
closed: 2026-11-23
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

## Fixed (GERM, `germ/spiric-arc-fuzz-ceiling`)

The failure was the abandon ceiling (`abandoned * 5 <= asked`), never
a wrong answer: 0x139cdf42d954efca set aside 34 of 127 rays and
0xa73f99cde9ddae65 39 of 158 at 1e-6; both are 0 of 127 and 9 of 158
at 1e-9, 0 and 1 at 1e-12. Over 10,000 seeds at 1e-6 the old ceiling
failed 15.6% of them (p50 2.5%, p99 48%, max 100%), with no wrong
answer anywhere. At 1e-9 the maximum was 19.6% and at 1e-12 14.4%,
both just under the ceiling.

Every abandon is a refusal of the spiric × plane door's line × torus
ladder: an escalated ladder margin (54%), the shifted-line count's
`Unsettled::Tangency` (46%), and a few resolvent or touch refusals.
The per-ray rate depends on the case's thinnest feature (`r`, `R − r`,
`R − r − |o|`) measured in `√(ε·R)`: 12–29% below `√(ε·R)`, 1% at
1–10, 0.2% past 100. The f64 lane gives the same rates as the
Interval lane, ray for ray. This is the documented `√(ε·R)` width of
the ladder's tangency band (`line_torus_roots`, "the band is wide"),
which takes in a whole oval at 1e-6 whose tube is a few hundred ε
thick. It is the honest cost of the band, not a reader setting aside
rays it should decide. With 4 cases, all of them may land thin.

The ceiling now reads only rays of cases whose thinnest feature is at
least `10·√(ε·R)`, and fails past one third. Over 1000 seeds per row,
those rays are set aside at 0.5%, 0.3% and 0.2%, with a per-seed
maximum of 10%, 11% and 15%. At 1e-6, about 10% of seeds draw no such
case; these stand down loudly. The wrong-answer assertion is
untouched. The always-abandon mutant reds both lanes at every ε on
both seeds, and 500 fresh seeds per ε replay green.
