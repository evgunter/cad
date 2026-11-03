---
id: test-shards-link-examples-nextest-never-runs
kind: issue
title: the test shards link 22 example binaries nextest never runs: 166 unit-s, 43 s of kernel-shard wall on one runner
status: open
opened: 2026-11-02
priority: P3
cost: E
---


## Found (GREEN latency lane, 2026-11-02, PR 6324)

`cargo nextest run` with no target flags uses cargo test's default
selection, which builds each selected package's examples. nextest never
runs them, since every example is `test = false`. The kernel shard builds
18 of them (sweep 12, step-export 2, step-import 2, stl 1, topo 1), the
app shard pncad's 3 and viewer's featureless 2, and the viewer job
viewer's 3. `cargo test --tests` (unit graph via `-Z unstable-options
--unit-graph`) drops exactly those, plus `pncad-py`'s cdylib and
doctest-only units, which nextest does not use either.

Measured on PR 6324, which trialled `--tests` on every nextest call in
the test and viewer jobs:

- `--timings`: the examples were 166 of 2,720 unit-s in the kernel
  shard (run 41402856622) and 8 of 1,727 in the app shard.
- The same runner built the examples on top of a `--tests` build in
  43 s (kernel), 4 s (app) and 5 s (viewer job), in run 41408102092.

It was not taken. It saves at most about 1 min of one shard, and its
coverage half costs back some of that. The lint job's clippy strips
`-p viewer` on closure runs without the toolkit flag, so viewer's two
featureless examples would need a `cargo clippy -p viewer --examples`
there.

## Owed

Take it with a shard rebalance, or with the long-pole row
(`kernel-shard-long-pole-is-one-test-binary`), where the freed cores
land on the critical path. The flag goes on every nextest call in a job
(build, each eps row, the slow set, `nextest list`), or a later call
builds what the first one left out.
