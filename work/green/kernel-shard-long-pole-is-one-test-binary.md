---
id: kernel-shard-long-pole-is-one-test-binary
kind: issue
title: the kernel test shard's critical path is sweep's one 277k-line test binary (389-544 s, one rustc), and the app shard's is editor-core's lib then its test binary
status: open
opened: 2026-11-02
priority: P2
cost: M
---


## Found (GREEN latency lane, 2026-11-02, PR 6324)

`cargo nextest run --no-run --timings` on two TIER=all runs of PR 6324
(a temporary measurement step, since removed), one 4-core hosted runner
per shard:

| run | shard | build end | the longest units (start → duration, s) |
|---|---|---|---|
| 41402856622 | kernel | 867 s | `sweep` `all` 323 → 544; `topo` lib-test 318 → 430; `topo` `all` 373 → 399; `topo` lib 20 → 238 |
| 41405638031 | kernel | 638 s | `sweep` `all` 248 → 390; `topo` lib-test 230 → 346; `topo` `all` 248 → 226 |
| 41402856622 | app | 752 s | `editor-core` lib 61 → 402; `editor-core` `all` 491 → 261 |

- **Kernel shard.** The build ends when `sweep`'s `all` test binary ends.
  `crates/sweep/tests/` is 277k lines aggregated into that one binary
  (622 `#[path]` mounts), against 40k lines of `sweep/src`. rustc's
  front end is serial, and over the last 100–370 s of the build that
  unit runs with fewer than four units beside it.
- **App shard.** Two serial units in a chain: `editor-core`'s lib (402 s)
  and then its `all` binary (251k test lines, 261 s).
- Over the 170 completed PR runs of 2026-11-01 18:33Z → 11-02 06:25Z,
  `test (kernel)` is the last job in 28 of the 34 runs in the top 20%
  of wall-clock. Its build step is median 11.8 / p90 13.9 min at the
  closure tier, out of a 14.2 / 17.0 min job.

## Owed

`scripts/gates/test-aggregation.sh` holds every member to at most one
`[[test]]` target, on #179's measurement that each extra target cost a
codegen+link of the whole rlib graph. Splitting `sweep`'s `all` in two,
and possibly `editor-core`'s, trades one more link for a front end run on
two cores. It is a change to that gate's invariant, so it needs:

1. a hosted `--timings` A/B on one runner, with the split's unit-s and
   wall against the single binary's;
2. the gate's rule restated as a per-crate limit, with the rationale
   updated in its header.

The measurement has to be paired. The same build on two hosted runners
ran 540–663 s or 821–868 s (TIER=all kernel build, 23 runs, bimodal), so
a cross-run comparison mostly measures which runner it got.
