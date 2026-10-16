---
id: the-test-jobs-build-time-candidates-measured
kind: issue
title: Where the test job's cold build time goes, and which levers move it (measured)
status: closed
opened: 2026-10-14
refs: [5011, 5104]
cost: E
priority: P4
closed: 2026-10-15
---


The question (Ev on #5011): what makes the cold build of the deep crates
take 16–20 min on 4 cores, and which cheap changes would cut it. The
scope already measured in
`a-sweep-seeded-diff-runs-the-test-job-past-the-fifteen-minute-bar` is
not redone here.

## Method

All numbers come from this lane's box: 4 cores, 15 GB, main `416607159e`.
- **Env:** ci.yml's test-job env (opt-level 1, `line-tables-only`,
  `TEST_STRIP=debuginfo`), `CARGO_INCREMENTAL=0` as rust-cache sets it,
  and a private target dir.
- **"Cold":** every workspace crate's `src/lib.rs` and `tests/all.rs` is
  touched with the dependencies warm. That is the hosted shape: rust-cache
  serves the deps, and a deep-kernel diff invalidates the workspace.
- **Scope:** `--workspace`.
- **Noise:** the box is noisy run to run. One base build took 1172 s and
  another 1368 s. Every comparison below is therefore a same-session
  pair, and a lever only counts when it beats the noise in both pairs.

## Where the time goes

Base cold rebuild: **1172 s wall, 4166 cpu-s in workspace units**
(88 units).
- By kind: test targets (`all` binaries + lib-tests) 2916 cpu-s, libs
  1123, examples 127.
- Top units (cpu-s): editor-core lib 739, topo lib-test 567, sweep `all`
  463, editor-core `all` 335, topo `all` 318, topo lib 230, editor-core
  lib-test 201, mesh `all` 164, mesh lib-test 142, geom-brep `all` 122.

Phases: `-Ztime-passes` (via `RUSTC_BOOTSTRAP=1`) on the editor-core lib,
the head of the critical chain.

| phase | s (wall) |
|---|---|
| total | 274 |
| LLVM_thinlto | 143 |
| LLVM_passes | 84 |
| codegen_to_LLVM_IR | 37 |
| generate_crate_metadata | 28 |
| monomorphization collector | 21 |
| typeck + borrowck | 11 |
| macro expansion + resolve | 1 |
| link | 1 |

**The frontend and the linker are not where the time is. The backend
is, and half of the backend is crate-local ThinLTO.** Cargo's default
`lto = false` runs that pass at opt-level ≥ 1 with more than one codegen
unit. `docs/PERF-SCAN-2026-08.md` lists `lto` among the non-levers as
"pure compile-time cost". That is true of turning LTO on, but it misses
the pass the default already runs.

## Candidates, ranked by measured saving

"Gate" says whether the candidate changes what the per-PR gate verifies.

| # | candidate | measured | cost / risk | gate | quick PR? |
|---|---|---|---|---|---|
| 1 | `CARGO_PROFILE_{DEV,TEST}_LTO=off` in the test job + cache-prime | build **−291 s / −302 s** (two pairs); workspace cpu-s 4166 → 3024 (−27%). Tests **+9–18%** (per-test sums ×1.18, uniform). Whole job seeding geom-brep: **2191 → 1966 s (−225 s)** | two env lines per job; the cache key changes once; runtime cost grows with test volume, break-even about 27 min of test steps | none | **yes: #5104** |
| 2 | opt-level 0 for test builds | editor-core lib proxy 639 → 158 cpu-s (with lto off) | tests ≈ 3× slower at opt-0: `docs/perf-data/opt-level/` (opt-0 E = 4840 s vs opt-1 ≈ 1600 s), the nightly calibration that set opt 1 | none, but the job gets far slower | no: decided by the calibration lane |
| 3 | lib-test double compile (each crate builds its lib twice, once as `--test`) | topo lib-test 567, editor-core 201, mesh 142, geom-brep 92, geom-core 72 cpu-s (base) | moving `#[cfg(test)]` modules out of the heavy libs is per-crate work | none | no: topo's is filed (`work/topo/topos-unit-tests-recompile-the-whole-crate-on-every-topo-closure-pr.md`); the rest are the same shape, below that bar |
| 4 | editor-core lib's own size (739 cpu-s, critical chain) | `docs/GENERICS-BUILD-COST.md` (2026-08) put 62% of workspace LLVM IR in editor-core's serde derive; not re-measured here | a persistence-layer refactor | none | no: not cheap |
| 5 | examples built by `nextest --no-run` / `cargo test` | 18 example binaries, 127 cpu-s ≈ **36 s** wall at the measured parallelism | dropping them loses their link check (clippy `--all-targets` still type-checks them) | drops a check | no: under 2 min |
| 6 | `codegen-units=16` (with lto off) | build 956 s (vs 881 with lto off alone); tests ×1.64 slower | — | none | no: worse on both |
| 7 | `codegen-units=16` (ThinLTO kept) | editor-core lib proxy 1036 → 912 cpu-s, wall unchanged (297 → 302 s) | — | none | no: under 2 min, and noise-level on the chain |
| 8 | `debuginfo=0` instead of `line-tables-only` | editor-core lib proxy not resolvable above box noise (lto-off pair: 639 vs 875 cpu-s, the wrong sign) | loses file:line in failing-row backtraces | none | no: not shown to beat noise |
| 9 | linker (lld / mold) | none to take: 1.97's x86_64-linux default is already rust-lld (`.comment` of a test binary: `Linker: LLD 22.1.6`); link is 1 s of 274 on the biggest lib | — | none | no |
| 10 | duplicate dependency versions (rand ×3, syn ×3, itertools ×3, nalgebra ×2, …), proc-macros, build.rs | 0 on the measured path: deps come warm from rust-cache, and no workspace member has a build.rs | — | none | no |
| 11 | test-binary count / link time | already one `all` binary per crate (`autotests = false`); 1 s link per binary | — | none | no |

Already measured and declined in the 15-minute-bar row, not redone: a
cross-run cache of the workspace crates, test-job shards, demo-tour
opt-1, and incremental builds.

## Found on the way

`geom-brep`'s `a_rigidly_mapped_wall_reads_the_seated_chart_speeds_floors_and_tube`
fuzzer went red on 2 of this lane's 6 runs, on two seeds, at the base
profile too. Filed:
`work/ssiarith/a-rigid-map-moves-an-enclose-chart-margin-past-its-rounding-allowance.md`.

— build-time lane (NURBS), 2026-10-14

## Hosted, after #5104 (2026-10-14)

The `test` job's `build` step, in ci.yml pull_request runs from 07:07Z to
14:43Z. "Before" means created before main's cache-prime for the new key
finished (push run 38555354191, 11:32Z, full dependency build). "After" means created after it. Runs
whose build step was cut short by a cancel are excluded.

| scope | before: n, median | after: n, median | Δ |
|---|---|---|---|
| `--workspace` | 30, 1246 s | 13, 820 s | **−426 s** |
| T11 + geom-brep | 11, 1185 s | 10, 904 s | −282 s |
| T11 (editor-core … topo) | 11, 1137 s | 5, 671 s | −466 s |
| all scopes | 60, 1206 s | 31, 871 s | **−335 s** |

These are medians over different PRs at different trees, so runner
variance and the day's merges ride in them. The direction and size match
the local pair (−291 / −302 s).

Two branches have a `--workspace` run on each side, so the whole `test` job can be compared:
- pcert/spline-pair-any-carrier: 3339 s → 1821 s;
- band/concave-bar: 2716 s → 2051 s.

Their test steps got *faster*, not about 9% slower as the local pair
predicts. Each pair's two runs test different trees, so the test-step
cost of lto off is not resolvable from hosted data. The build saving is.

## Closed

#5104 (merged 2026-10-14) took the one cheap lever, `lto = off` in the test job. Every other candidate in the table is declined, decided elsewhere, or filed on its own program (topo's lib-test double compile, SSIARITH's rigid-map fuzzer). Nothing here is still owed. (CIW orchestrator)
