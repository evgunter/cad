---
id: the-pr-tour-suite-builds-at-opt-level-1
kind: unit
title: the per-PR tour suite builds at opt-level 1: release's opt 3 doubles the build and stops generic sharing
status: closed
opened: 2026-11-01
closed: 2026-11-01
priority: P1
cost: E
refs: [the-demos-job-is-pr-cis-long-pole-on-its-release-build, 6179]
pr: 6187
branch: claude/gallant-heisenberg-0bj07v
---

## Found (2026-11-01)

Ev asked why the `demos (tour + wild)` job on PR 6179 (run 41259154748,
job 124708883754) took 23.9 min. It was not a regression. Eight demos
jobs from 10-30 to 11-01 ran 23.3–24.3 min (median 23.6 after 10-31),
against GREEN's closing 22.8 / p90 23.7. Each job's `demos tour suite`
step spent 16.5–17.3 min on the release build, then 4.8–5.3 min on
nextest, where `eps_regression::tour_runs_green_at_eps_1e_12` alone sets
the wall.

PR 6179 touched only `crates/viewer/`, but its `crates/viewer/Cargo.toml`
edit (an `[[example]]` entry) is a member-manifest change. That is
TIER=all (`scripts/ci-filter.py:1402`), which runs every row, demos among
them.

**The split this row's parent read was misattributed.** It took the time
from `Compiling demo-tour` to `Finished` as the tour's own build. Cargo
prints `Compiling` when a unit starts, and `editor-core` starts early and
runs alongside. `cargo test --release --no-run --timings` on one 4-core
box (kernel from scratch, `CARGO_INCREMENTAL=0`):

| unit | start → end | duration |
|---|---|---|
| `editor-core` | 58 → 695 s | 636 s |
| `demo-tour` lib | 167 → 545 s | 378 s, beside editor-core |
| `demo-tour` test binary `all` | 695 → 857 s | 162 s, after editor-core |

So the critical path is editor-core, then `all`. `all` re-instantiates the
scenes' and the kernel's generics for `f64` because rustc does not share
generic instantiations across crates at opt-level 2 or 3. It does at 0,
1, `s` and `z`; `docs/PERF-SCAN-2026-08.md` said opt-level 0 only, and is
corrected in this unit.

## Done

`demos/tour/Cargo.toml` gains `[profile.ci-tour]` (inherits release, so
debug-assertions and overflow-checks stay on, at `opt-level = 1`). ci.yml's
`demos tour suite` runs `cargo nextest run --cargo-profile ci-tour --profile
ci`. The nightly's tour rows and the render lanes keep `--release`.

The same box, the same commands, build then the `ci` nextest profile:

| profile | build | tests | total |
|---|---|---|---|
| release (opt 3) | 857 s | 195 s | 17.5 min |
| opt 1 | 379 s | 221 s | 10.0 min |

At opt 1, editor-core takes 309 s, the tour lib 29 s and `all` 5 s.
88/88 pass at both levels, the bit-pinned rows included: an optimization
level changes no floating-point result in Rust.

Why opt 3 was there: the profile's header argued for an optimized build
rather than a dev one, because the root's per-package opt-2 overrides do
not reach a detached workspace and a dev build would tessellate at opt 0.
That argues against opt 0, not for opt 3. The other claim in that header,
"the ONLY release build of the kernel in CI", stopped being the reason
once the profile re-armed the assertions: with them on, the tour checks
what a dev build checks. The shipped configuration (assertions off) is
`corrupt input (release profile)`'s row.
