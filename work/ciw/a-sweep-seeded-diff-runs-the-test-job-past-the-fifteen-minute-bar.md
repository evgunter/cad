---
id: a-sweep-seeded-diff-runs-the-test-job-past-the-fifteen-minute-bar
kind: issue
title: A diff that seeds sweep runs the per-PR test job for over 21 minutes, past the 15-minute bar
status: open
opened: 2026-10-11
---

## Measured

PR 4684 (`nurbs/main-slow-set-reds-since-4553`, head `c25f38a643`) touches:
- `crates/sweep/tests/`;
- `crates/step-import/tests/`;
- `crates/topo/src/props.rs`.

Its CI run 38174765730, job `test`, ran from 21:04:14Z to 21:25:48Z, which is **21.5 min**. Every other job finished in under 5 min. The bar is 15 min at most and typically under 10 (`memories/orchestration-model.md`, "Friction is a finding").

The likely cost is the slow set of the crates the diff seeds, sweep's thread-count digest above all. That digest's f64 and Sym walks run in one test, at four threads, each with its own quadrature. This has not been confirmed per test: the job's per-test timings are the next thing to read.

## Wanted

The seeded slow set fits the bar, for example by sharding the seeded slow rows into their own job, or by moving the heaviest digest rows to the nightly with a register. CIW's call.

— (NURBS orchestrator)
