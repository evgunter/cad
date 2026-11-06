---
id: main-red-the-k-probe-sweep-cannot-compile-geom-brep-under-probe-alone
kind: issue
title: K roster revalue blocked: scripts/k_probe_sweep.sh stops at m4_pr8_k_probe's corpus dump (key splits since PR 6341, <unnamed> samples since PR 6376)
status: dispatched
opened: 2026-11-05
priority: P2
cost: E
refs: [main-reds-editor-core-slow-set-and-the-k-probe-sweep]
branch: onecopy/main-reds-k-and-m10
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

## Narrowed by PR 6560 (2026-11-06)

Items 1 and 2 are done: geom-brep (and geom, geom-core) carry the self dev-dependency enabling `test-support`, held by `scripts/gates/test-support-self-edge.sh`; `ray_wall_margin_twins` is re-pinned per ε (movers PR 5781, 6349, 6482) on `work/onecopy/ray-wall-twins-read-f31-angle-renames-as-margins-that-must-scale.md` and `work/flush/ray-wall-twins-fire-different-predicates-at-1e-6.md`.

**What remains: item 3, the revalue.** The sweep now runs its whole plain half and stops at its first corpus dump, `m4_pr8_k_probe` at 1e-6, on two reds owned elsewhere:
- the probe/f64 key splits, off their pin since PR 6341: `work/tally/probe-lane-content-and-naming-keys-split-from-f64-on-every-corpus-document.md`;
- 138 `<unnamed>` samples since PR 6376 (`geom_core::running::Rounded::divisor`'s raw `sign_within`): `work/green/main-red-slow-rows-uncovered-at-b2337e9e.md`.

Once both are green, run `scripts/k_probe_sweep.sh <dir>` and the nightly's k-lint steps against it.
