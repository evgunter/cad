---
id: the-gates-step-rescans-the-tree-once-per-gate
kind: issue
title: scripts/gates takes 7.3-7.6 min on every PR run, docs-only included: 64 gates rescan the tree independently, 1358 CPU-s on 4 vCPU, and lint is the pole of half the runs
status: closed
opened: 2026-11-21
priority: P2
cost: M
closed: 2026-11-23
---


## Found (GREEN PR-latency lane, 2026-11-21)

`lint`'s `gates (scripts/gates)` step (`scripts/run-gates.sh`) is 7.6 min median over 332 PR runs 11-20 12Z..11-21 (6.6 on 11-15), p90 10.7. It runs on every PR run, docs-only ones included: on the 106 docs-only runs of that window (filter + lint + gate ok, no build) the lint job is 7.6 min and the gates step 7.3 of it.

Job 136904187370 (11-21): 64 gates, their own timings summing to 1358 s on a 4-vCPU runner, i.e. the parallel run is CPU-bound. The heaviest are `bare-rounding-allowlist` 80 s, `escalation-question-marks` 57, `cover-kind-match` 55, `reporting-margin-door` 46, `interval-square-allowlist` 45, and a dozen more at 36–42 s; each is a single-threaded scan that re-reads the shipped tree on its own.

`lint` is the critical-path job of 189 of the 400 runs in that window (python 89, the test shards 72): the median run's active time (queue removed) is change filter 1.3 + lint ~12 min.

## Options (not chosen)

- One shared read of the tree (a cached file list and stripped-code view the gates read) — the per-gate 35–45 s floor looks like the same walk repeated.
- Split the gates into their own job beside the rust lints: lint's pole drops from ~12 to ~8.5 min, at about one more job setup per run (more demand in the queue that already dominates).
- Skip a gate on a diff that cannot move its subject (docs-only): a change to what CI runs, argued per gate, since some read Markdown.

## Closed

Done by PR 7439, which closed CIW's duplicate P0
(`work/ciw/the-lint-jobs-gates-step-scans-the-tree-64-times-for-5-to-11-minutes-on-every-pr`):
the gates step went from 5:22–10:54 to 1:06 on a hosted runner (run
45069292031) through one shared read of the tree, the first option above.
