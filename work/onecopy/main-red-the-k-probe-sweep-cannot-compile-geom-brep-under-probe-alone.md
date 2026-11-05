---
id: main-red-the-k-probe-sweep-cannot-compile-geom-brep-under-probe-alone
kind: issue
title: main red: scripts/k_probe_sweep.sh no longer compiles geom-brep's tests under --features probe alone (Reach::fixture is behind test-support since 72f10792), and ray_wall_margin_twins' pin now stops on geom predicates
status: open
opened: 2026-11-05
priority: P2
cost: E
refs: [main-reds-editor-core-slow-set-and-the-k-probe-sweep]
---


Found by `onecopy/polish-meters` (PR 6537), which was asked to revalue the K roster.

## What

`scripts/k_probe_sweep.sh` builds geom-brep's tests with `--features probe` alone. Since `72f10792` they no longer compile that way: `Reach::fixture` sits behind `test-support`, and test code calls it.

With `test-support` added locally, the sweep then aborts on `sweep::ray_wall_margin_twins`'s pinned red. That pin now stops on `azimuth_near_cut_side` and `period_near_mark`, which are geom's predicates, where it once stopped on `curve_pair`'s.

That stop was not checked at base, for lack of disk. So whether PR 6537 or main moved it is unmeasured.

The closed row `main-reds-editor-core-slow-set-and-the-k-probe-sweep` greened the earlier instance.

## Why it recurs

Nothing in CI runs the K probe sweep. So a feature-gate move breaks it silently. That is the same root as `work/ciw/no-ci-row-runs-the-suite-at-a-non-default-k.md`.

## What closing it takes

1. Make the script pass the features its targets need, read from one place rather than spelled in the script.
2. Re-measure the ray_wall_margin_twins stop at main and at PR 6537. Re-pin, or fix, the row.
3. Revalue the K roster.
