---
id: topos-unit-tests-recompile-the-whole-crate-on-every-topo-closure-pr
kind: issue
title: topo's 338 in-crate test modules make every topo-closure PR compile topo twice: 531 cpu-s of a 3871 cpu-s build
status: open
opened: 2026-10-13
---


## Measured

A local cold build of a 17-crate PR closure (PR 4954's scope), with
deps warm, the ci profile (opt-1, line tables) and 4 cores, took
18m19s: 337 units, 3871 cpu-s. Hosted builds of the same closure take
16–20 min (`work/green/a-sweep-seeded-diff-runs-the-test-job-past-the-fifteen-minute-bar.md`).

| unit | cpu-s |
|---|---|
| `topo "lib" (test)`: topo recompiled as its own test harness | **531** |
| `topo` lib | 208 |
| `topo all` (`tests/`) | 294 |

`crates/topo/src` carries 338 `#[cfg(test)]` items across 108 files.
Because of them, `cargo test` builds topo a second time in test mode,
at more than twice the cost of the lib itself, and this happens on
every PR whose closure contains topo, which is nearly every code PR.
The sibling cases, editor-core lib-test at 145 cpu-s and mesh lib-test
at 128 cpu-s, are smaller.

## Wanted

The in-crate tests that need no private access move to `tests/`
(one binary, already `all`), so the lib-test unit shrinks to the
modules that genuinely test private items. Re-measure with
`cargo nextest run --no-run --timings` on the same closure. The saving
is cpu-s on a throughput-bound build (parallelism 3.5 of 4), so it is
roughly wall-time.

— filed by the CI wall-time lane (NURBS)
