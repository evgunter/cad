---
id: latency-cut
kind: unit
title: Cut the per-PR gate to latency; the nightly holds the rest
status: open
opened: 2026-09-28
needs_ev: true
---


The per-PR gate is sized for latency and job-minutes; `nightly.yml` holds
everything else. `.github/workflows/ci.yml`, `.config/nextest.toml` and
`scripts/ci-filter.py` carry the selection; this page carries the evidence.

## Why

- **The queue is the free plan's 20-concurrent-job cap.** On 2026-09-25,
  322 PR runs across 85 branches used 84% of the day's 20-slot capacity;
  PR CI was 93% of it (nightly, render and work-status under 1%). Completed
  code runs took 50 min median and 140 min at p90 of wall-clock; 32 took
  over 2 h. 23% of the day's minutes went to runs a newer push cancelled.
- **A code PR run was ~27 jobs and 100–130 job-minutes**, so the pool
  sustained about 9–12 PR runs an hour. Without queueing, a run took ~21 min
  (36 min before the tour suite moved to nextest on 09-26).
- **Where the minutes went** (median execution, 09-24..26): k-lint
  release-default 13–28, k-lint dev-probe 14, render scene inputs 9, build +
  archive 8, rustfmt/rustdoc/wasm32 7.4, test legs 4.3 (p90 12.4; the 1e-12
  legs 13–16), render gui 4.2, k-lint release-budget 4.2, python 3.6.
- **Test time is a Pareto tail.** 4 vCPU, opt-level 1, default eps: 9,811
  tests, 503 s wall. The 205 tests at ≥ 1 s hold 86% of it; the other
  ~9,600 run in ~70 s. Source-tree guards and proptests together are under
  2%.
- **The build is the floor.** Cold: 649 s, with a ~600 s critical path of
  editor-core's lib (292 s) then editor-core's `all` test binary (294 s).
  The read reach pinned editor-core and topo into every closure, so every
  PR paid that path.

## What caught bugs (CI history 2026-07-16 → 09-28)

1,135 red runs across 12,881 runs. About 20 real defects, hand-verified:
nextest behaviour rows (~15) and fuzz rows (4, three of them P0s), plus the
k-lint dev-budget certificate falsifier (the `nurbs_face_bound` P0).
Everything else caught only bookkeeping: rustdoc (124 reds), rustfmt (39),
every clippy variant (~380), every discipline gate, ruff, the tracker lint,
the k-lint probe/K-telemetry/budget rows, render drift, the python suite
(censuses and stub parity), and ~114 census/roster/ledger tests inside
nextest. 52 reds were CI testing CI. INFRA: billing (161 runs), the render
lanes' missing merge ref (92), ~25 others. No test at ≥ 1 s has a recorded
catch.

**The eps-only reds** (default row green, 1e-6 or 1e-12 red): 72 runs. ~50
were tests that were not eps-portable (an eps literal, absolute slack, a
golden embedding epsilon), ~12 the rational-quadrature family working as
designed, ~7 real eps-specific defects. They concentrate in step-import,
geom-brep (`props::quad`), profile (`review_s2`), topo (boolean band rows),
and probe/golden rows — which is the rule the gate's extra eps rows key on.

## What the gate runs

- `test`: one job, no archive hand-off. The default-eps suite under the
  `ci` profile (the slow set filtered out); the slow set of the crates the
  diff seeds; the 1e-6 and 1e-12 rows for seeded step-import, geom-brep,
  profile and topo and for any crate whose probe or golden files changed;
  doc-tests.
- `lint`: rustfmt, clippy `--all-features` over the closure (viewer's app
  feature only when the viewer job runs), every `scripts/gates/*.sh`,
  payload-rung check, ruff, the tracker lint.
- Direct-path rows, each only when its subject is in the diff: viewer (app
  feature, lavapipe, wasm32 clippy), python, topo release corruption, mesh
  budget falsifier (when mesh is built), step import (freecad), demos,
  tools, interval.
- Pushes to main run only the cache primer.

## What moved to the nightly

The whole suite at every eps row with the slow set; doc-tests; the topo
per-op scalpel; wasm32 check; all five k-lint rows; the render lanes (which
now commit their re-baselines from the nightly); python, topo release, step
import and interval, ungated.

## What was deleted

- 50 slow tests with no recorded catch (verdicts per test: the PR's
  worktree commit; shapes were studies, measurement ceilings, thread-count
  invariance walks, and adopted one-off review probes), plus three sweeps cut
  ~10x.
- The CI-testing-CI scripts: the hosted/local parity checker and its
  siblings (status capture, render-lane parity, install wrappers, cache-prime
  parity, run-job aggregator, reach selftest, gate roster), the test-cost
  reporters, the opt-level calibrator, the demotion selector, and the local
  mirror `local-scripts/ci-local.sh` + `gate.sh`.
- The read reach in `ci-filter.py`: a tree-wide guard runs when its own
  crate is in the closure, and nightly.

## Expected shape

A leaf-crate PR: ~5 jobs, dominated by its own compile. A deep kernel PR
still compiles editor-core (~10 min cold on hosted runners, since the
workspace crates do not fit the 10 GB cache) — the remaining lever for that
is a persistent build directory (a self-hosted runner), which is Ev's call.
