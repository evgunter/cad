---
id: test-shards-build-the-kernel-libraries-twice
kind: issue
title: the two test shards each build the shared kernel libraries, about 600 job-min a day: build them once and hand them to both shards
status: open
opened: 2026-10-28
priority: P0
cost: M
parent: pr-ci-wall-clock-root-cause-measured
---


## What

#5941 splits the per-PR `test` job into an `app` and a `kernel` shard.
Both shards build the kernel libraries the app crates depend on, so a
code run pays that build twice: about +600 job-min a day by the
latency lane's estimate, about 2 points of a 20-slot day on top of the
28% PR runs use. The queue is empty now (p90 0.1 min), but at 45-74%
of capacity in early October the p90 queue reached 9-13 min, so the
headroom is worth keeping (the user, in chat, 2026-10-28).

## Owed

1. Measure the duplicated part: with a hosted `cargo build --timings`
   per shard, find how many of each shard's build minutes are the
   kernel libraries both shards compile.
2. Prototype a three-job layout: one job builds the shared libraries
   once and hands its `target/` to both shards (as an artifact or a
   run-keyed cache), and each shard builds only its own test binaries.
   Cargo decides a workspace crate is stale from source mtimes, and a
   fresh checkout resets them, so restore mtimes from git in the shard
   jobs (or the shards rebuild everything). Measure the handoff
   overhead: job setup plus `target/` upload and download.
3. Keep it if it saves job-minutes without adding more than about a
   minute to the median wall-clock. Otherwise rebalance the shards to
   shrink the shared part, and record the numbers here either way.
