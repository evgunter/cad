---
id: pr-ci-latency-median-is-32-minutes
kind: issue
title: PR CI's median wall-clock is 27-33 minutes against the 15-minute bar and the alarm's 20, and no live row holds it
status: closed
opened: 2026-11-21
priority: P2
cost: M
branch: green/pr-latency-2
closed: 2026-11-22
pr: 7364
---


The nightly's `PR CI latency (alarm)` job: "median PR CI wall-clock 27.2 min exceeds 20 min" on the scheduled 11-21 run (44870651337), and 32.5 min on the dispatched 17:00Z run (44927630239). The bar is 15 minutes, typically under 10 (`memories/orchestration-model.md`, Friction). `ciw/latency-cut` and GREEN's earlier wall-clock rows are closed, so nothing holds it. 11-15 measured 20.8 minutes; it has grown since.

Owed: measure where the median's minutes go now (queue vs run, which job is the long pole, which PRs), and cut it.

## Measured (2026-11-21, `green/pr-latency-2`)

The rise since 11-15 is queue, not run time: over 400 completed PR runs 11-20 12Z..11-21 the median run is 26.7 min wall, 17.1 of it active and 7.8 queued, against 15.7 / 15.4 / 0.1 on 11-15 (`pr-runs-queue-behind-the-twenty-job-cap` has the table and the demand split). The longest jobs: python 28 min (its suite step 24, against an opt-level-0 wheel), demos 17, mesh-budget 12, the test shards 13, lint 12 (its gates step 7.6, `the-gates-step-rescans-the-tree-once-per-gate`).

The cut here: the python and mesh-budget jobs build at the test job's opt-level 1 and restore `build-default` (their own keys were never saved: `pr-only-jobs-rust-cache-is-never-saved`). Together they were 30% of 11-21's PR job-minutes (17,800; 25% of all workflows' 21,800), and the measured cut saves about 13% of PR job-minutes (10% of all). Measured on the PR's own run (44989289861, cache hit on `build-default`): python job 15.3 min (wheel build 2 → 7.5 min at opt-level 1, suite 1056 → 344 s for the same 961 tests), mesh-budget 7.5 min (integration run 354 → 137 s, unit 50 → 14 s, build unchanged at 3m46s).

## Closed (2026-11-22, PR 7364)

Measured: the median's rise since 11-15 is queue behind the 20-job cap, not run time (active 15.4 → 17.1 min, queue 0.1 → 7.8). PR 7364 cut demand, not the median: python 28 → 15 min and mesh-budget 12 → 8 min on the test profile, about 13% of PR job-minutes. A gate (`scripts/gates/build-default-env-agrees.sh`) holds the shared cache key's env blocks equal. What remains is `pr-runs-queue-behind-the-twenty-job-cap` (the queue, and the alarm question), `the-gates-step-rescans-the-tree-once-per-gate` and `pr-only-jobs-rust-cache-is-never-saved`.
