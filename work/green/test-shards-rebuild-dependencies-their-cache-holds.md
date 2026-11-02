---
id: test-shards-rebuild-dependencies-their-cache-holds
kind: issue
title: the test shards rebuild about 30 dependencies on a full cache hit, because cache-prime builds --workspace and each shard unifies features differently
status: open
opened: 2026-11-02
priority: P3
cost: E
---


## Found (GREEN latency lane, 2026-11-02, PR 6324)

Every sampled `test (kernel)` and `test (app)` job restores
`v0-rust-build-default-…` with `full match: true`. The build then still
compiles crates.io dependencies: num-traits, rand, getrandom, tempfile,
spade, num-bigint, proptest, serde, syn, serde_derive, chrono, cgmath,
ruststep and the truck-* stack (job 125071003587, a closure kernel
shard).

`cache-prime` builds `cargo test --no-run --workspace`. Each shard
selects a subset of packages, and cargo unifies features over that
subset, so some dependencies hash differently (#5957 found the same for
the workspace's own libs).

- `--timings`: dependency units were 64–84 unit-s in the kernel shard
  and 34–35 in the app shard (runs 41402856622, 41405638031).
- They finish within the first ~40 s, beside the workspace's first libs,
  so the wall cost is small.
- The runner-minutes are paid on every code run.

## Owed

Prime the cache with each shard's own selection (one `cargo test
--no-run` per shard scope at TIER=all, into the same key), or pin the
shared dependencies' features workspace-wide. Measure the first
dependency `Compiling` line to the first workspace one, before and
after.
