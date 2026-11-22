---
id: pr-only-jobs-rust-cache-is-never-saved
kind: issue
title: ci.yml's lint, topo-release, demos and tools jobs save their rust-cache only on main, where they never run, so every PR run compiles their dependencies cold
status: open
opened: 2026-11-21
priority: P3
cost: E
---


## Found (GREEN PR-latency lane, 2026-11-21)

`ci.yml`'s `lint` (key `lint`), `topo-release` (`topo-release`), `demos` (`demos`) and `tools` (`tools`) jobs restore with `save-if: ${{ github.ref == 'refs/heads/main' }}`. None of them runs on a push to main (`cache-prime` is the only job a main push runs), so their keys are never written: the actions cache holds no `v0-rust-{lint,topo-release,demos,tools}-*` entry, and the jobs' logs on 11-21 read `No cache found` (jobs 136904187370, 136907995748, 136951401840; demos 136-series, whose `ci-tour` build then takes 6m44s cold). `python` and `mesh-budget` were the same until `green/pr-latency-2` moved them onto `build-default`. `viewer` restores the nightly's `viewer-toolkit` key and is warm.

So every PR run of those four compiles its dependency graph from scratch, and the `rust-cache` step costs a lookup for nothing.

## Weigh before fixing

- Priming them means building each job's graph on main pushes (runner-minutes on every merge, in the same 20-job pool PRs queue for: `pr-runs-queue-behind-the-twenty-job-cap`).
- The actions cache is at its 10 GB limit (10.73 GB in 16 entries on 11-21), so a new key evicts another; the nightly's k-lint keys alone are 0.5–1.1 GB each.
- Cheaper where it fits: point a job at a key that already exists with the same `CARGO_*` env (as `python` and `mesh-budget` now restore `build-default`). `lint`'s clippy and rustdoc are check-mode and doc units, which no existing key holds.
