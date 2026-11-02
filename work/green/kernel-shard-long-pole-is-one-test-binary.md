---
id: kernel-shard-long-pole-is-one-test-binary
kind: issue
title: the kernel test shard's critical path is sweep's one 277k-line test binary (389-544 s, one rustc), and the app shard's is editor-core's lib then its test binary
status: closed
opened: 2026-11-02
closed: 2026-11-02
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

## Measured (GREEN latency lane, 2026-11-02, PR 6351)

Every variant was built in the same job, from the same restored cache,
TIER=all. CPU-s is user+sys of the cargo process tree.

| variant | kernel wall (s) | kernel CPU-s | app wall (s) | default-eps tests, kernel / app (s) |
|---|---|---|---|---|
| A: as on main | 882; repeat 840 | 3439; 3286 | 788; repeat 779 | 92.6 / 35.6 |
| B: every `--test` crate at opt 0 | 372 | 1428 | 466 | 117.4 / 37.3 |
| **B2: integration binaries (`tests/all.rs`) at opt 0** | **539** | **2053** | **593** | **99.7 / 35.7** |
| C2: sweep's `all` split in two | 921 | 3618 | — | — |
| C3: sweep's `all` split in three | 1002 | 3893 | — | — |

These are from run 41431857124 (kernel on an EPYC 7763, the slow mode).
Run 41425616061 got a faster runner and gave A 667 s against B 281 s.

- **The split loses.** Each half of `sweep`'s `all` took 521 and 558 s,
  close to the whole binary's 507–539 s. A half instantiates nearly the
  same kernel generics as the whole, so C2 is +81 s wall and +332 CPU-s,
  and C3 is worse. `test-aggregation.sh`'s rationale (each binary pays
  again to monomorphize the kernel) still holds, and the gate is
  unchanged.
- **Where the binary's time goes.** Under `-Ztime-passes` locally
  (diagnosis only) the front end is about 35 s of 410. The rest is
  codegen: `codegen_to_LLVM_IR` 175 s and `LLVM_passes` 331 s.
  The binary instantiates 45 MB of code itself, against 19.5 MB it takes
  from the workspace rlibs. 22 MB of the 45 MB is kernel generics at the
  scalars the suites drive: `Sym<f64>` 6.1, `Dual` 5.1, `Sym<Interval>`
  3.5, `Interval` 3.4, `f64` only 1.2. Share-generics (on at opt 1)
  already serves the `f64` code from the libraries.
- **Taken: B2** (`scripts/rustc-test-crates-opt0.sh`, PR `green:
  integration-test binaries build at opt-level 0`). On the slow runner
  it gave kernel −343 s and app −195 s. Five tests crossed the 1 s
  backstop at opt 0 and joined the slow set.
- **Not taken: B** (also opt 0 for every lib's unit-test harness). It is
  −170 s more on the kernel and −127 s more on the app. But a harness
  compiles its crate's own code again, so its tests ran that code
  unoptimized: 14 fast-set tests crossed 1 s on the slow runner (0.15 →
  1.11 s at worst), and 104 sat at or above 0.5 s, against 34 for A.
- `codegen-units=256` on the binary alone was 303 s against 286 locally,
  so it is not a lever.
- **The next pole** under B2 is `topo`'s lib-test harness (288 s, last to
  finish). That is `work/topo/topos-unit-tests-recompile-the-whole-crate-on-every-topo-closure-pr.md`.
