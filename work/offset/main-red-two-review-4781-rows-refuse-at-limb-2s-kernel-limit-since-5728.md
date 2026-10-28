---
id: main-red-two-review-4781-rows-refuse-at-limb-2s-kernel-limit-since-5728
kind: issue
title: Main red: review_4781's r1 and r2 touch rows refuse at 1e-12 as limb 2's KernelLimit on a cylinder since PR 5728
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Found by GREEN's full-suite pins lane (`green/full-suite-pins`,
2026-10-28) in nightly run 40637814390 (main `a1cfd1cfdb`), job `full
suite (every eps row, slow set included)`; re-run on main `91c83c38`.
Red at `CAD_TOLERANCE_EPS=1e-12` only.

## What

Both rows stop on the same refusal:

- `sweep` `review_4781_r1_probes::review_4781_touch_pairs_every_op`
  (`crates/sweep/tests/review_4781_r1_probes.rs`, the fixture's `unwrap`);
- `sweep` `review_4781_r2_probes::r2_every_same_side_touch_pose_answers_as_a_touch`
  (`crates/sweep/tests/review_4781_r2_probes.rs`, the same).

```
Op { source: Certification { error: AnalyticRung3(KernelLimit {
  operand: Cylinder, bound: 3.667082637548463e-12,
  margin: MarginDiag(Value(3.667082637548463e-12, None, ZeroWord(0))),
  width: 4.320099833421409e-12, extent: 2236.067977499794 }) } }
```

A cylinder fixture at extent 2236 cannot get its certified distance
bound under ε = 1e-12: the class of
`main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728`.

## Old values

The night before (run 40455870959, main `fb476030a9`) both rows were
already red at 1e-12, further on and differently:

- r1: `box on seam, tall box s=1e3 … Union: REFUSED PointInFaceRefused
  { operand: A, face: FaceKey(3v1), refusal: RayExhausted }` (PR 5664,
  `work/charttail/main-red-ringed-curved-doors-reds-four-sweep-rows`);
- r2: `pc corner +y ×1e3, A ∪ B: 3′ on its records: Err([CensusEscalated
  { … predicate: Some("curve_pair_pose_offset") } …])` (PR 5162,
  `work/cpair/a-same-side-touch-pose-escalates-the-curve-pair-extreme-at-1e-12`).

So the kernel limit now stops each row at a fixture build, before the
reds those rows own.

## Merge

BISECT

## Pinned

Each row is pinned whole at 1e-12 with
`test_utils::kernel_limit::pinned_row`, as PR 5935 pinned its five. The
pins go red the day the rows build past the fixture; closing this row
removes them, and the charttail and cpair reds behind them are then
measured again.

## What closing it takes

The question of the five-row row: whether limb 2's bound on a cylinder
at extent 2236 is an accepted kernel limit at 1e-12 (then these rows
take a typed refusal at that ε, or a smaller fixture) or a regression.

Reproduce: `CAD_TOLERANCE_EPS=1e-12 cargo nextest run -p sweep --profile
ci --ignore-default-filter -E 'test(/^review_4781_r[12]_probes::/)'`.
