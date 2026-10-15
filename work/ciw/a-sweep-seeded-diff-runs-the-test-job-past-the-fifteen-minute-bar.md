---
id: a-sweep-seeded-diff-runs-the-test-job-past-the-fifteen-minute-bar
kind: issue
title: A diff that seeds sweep runs the per-PR test job for over 21 minutes, past the 15-minute bar
status: open
opened: 2026-10-11
needs_ev: true
refs: [5001, 5011]
cost: M
priority: P3
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

## Measured, 2026-10-13 (CI wall-time lane)

Step times are from `get_workflow_job`; per-test times are from the
job logs' tails.

| PR | run / job | build | default ε | ε 1e-6+1e-12 | slow set | whole `test` |
|---|---|---|---|---|---|---|
| 4954 | 38360973529 / 115175058576 | 20m34s | 9m30s | 16m56s | **55m12s** | 1h43m |
| 4954 | 38380828910 / 115241360546 | 16m02s | 7m21s | 12m58s | 43m18s | 80m |
| 4944 | 38366610178 / 115193568567 | 19m27s | 4m24s | 9m28s | 2m49s | 37m |
| 4944 | 38373999542 / 115217941704 | 19m58s | 4m27s | 9m33s | 2m50s | 37m |
| 5001 | 38436425383 / 115434032795 | 13m20s | 2m32s | 1m57s | 2m59s | 21m |

**Why 4954's slow set took 55 min and 4944's took 3.** It was not
the seeds, sharding or the runner. #4954's own diff (the quotient
bound, 092b3022) made the SSI enclosures rebuild every cell's
Bernstein quotients on every read.
- `encl_curved_loft_shell::shelling_the_curved_loft_refuses_at_a_walls_fit`
  ran 1926 s / 1530 s under 4954, against 60 s under 4944. It ran
  alone for its last 12–15 min.
- `r1_lane0_e2e::the_f64_seam_answers_every_public_door` ran
  821 / 651 s against 17 s.
- The geom-brep slow rows took 470–1000 s each.
- Locally, at one thread, the shell row took 35.3 s before #4954 and
  1692 s after.
- #5001 memoises the whole-cell nets (284 s). The cut-net remainder is
  `work/nurbs/a-cut-cell-nets-quotients-cost-the-walk-eight-times-its-pre-4954-time.md`.

**Queueing.** None in the four brief runs: every job started within
2 s of creation. #5001's run queued 4.5 min for the filter and 19 min
for `test` (9 runs queued, 13 in progress at the time).

**The build floor.** A local cold build of 4954's 17-crate scope, with
deps warm and the ci profile, takes 18m19s, against 16–20 min hosted.
It runs 337 units, 3871 cpu-s, at parallelism 3.52 on 4 cores, so it
is throughput-bound with the editor-core chain as its tail.

Units by cpu-s:

| unit | cpu-s |
|---|---|
| editor-core lib | 658 |
| sweep `all` | 537 |
| topo lib-test | 531 |
| topo `all` | 294 |
| editor-core `all` | 287 |
| topo lib | 208 |
| mesh `all` | 159 |
| editor-core lib-test | 145 |
| mesh lib-test | 128 |

A deep-kernel PR's build alone is past the 15-min bar. No arrangement
of its test steps brings the job under it.

**Tour suite** (`demos`, 4944, job 115193568829): 25m19s.
- The release build takes 15m55s; the `demo-tour` crate's own
  codegen is about 9.5 min of that.
- The tests take 8m08s: 94 tests, 1881 cpu-s on 4 threads. The three
  `eps_regression::tour_runs_green_at_*` rows run 338–448 s each.

## Taken and declined

- **Taken: #5001.** Memoising the whole-cell nets takes the shell row
  from 1692 s back to 284 s locally. The cut-net remainder is on
  NURBS's slate.
- **Tried, not kept: the tour's own crate at opt-1**
  (`[profile.release.package.demo-tour] opt-level = 1`).
  - Locally, 4 cores: the `demo-tour` units drop from about 553 s to
    463 s of build wall, and from 1825 to 1682 cpu-s.
  - The tests run in 381 s at opt-3 and 384 s at opt-1; all 94 pass.
  - Hosted (#5010, `demos` job 115486892202): the tour step goes from
    24m04s to 23m32s, about 45 s saved.
  - That is under the 3-min bar a profile override has to clear.
  - The cost is the crate instantiating the kernel's generics, not
    optimising them.
- **Declined: caching the workspace crates across runs.**
  - Over 3 days, 115 of main's 571 first-parent commits changed `src/`
    in geom-core, geom, geom-brep or topo. That is 88% of the commits
    that changed any crate's `src/`, about 1.6 an hour, and each one
    invalidates nearly every workspace artifact.
  - Keeping the entry fresh costs a near-full build per main push, at
    about 0.9 GB per save against the 10 GB budget.
    `docs/CI-MINUTES-2026-08.md` declined the same trade.
- **Deferred: splitting `test` into two package-group jobs.**
  - Estimate from the timings: a kernel shard of 2616 cpu-s and an app
    shard of 1560 cpu-s, chain-bound; the job goes from about 37 min to
    22–25 min.
  - It costs a job slot per code PR while runs already queue for 19 min.
  - To be re-measured with the queue included.
- **Filed:** `work/topo/topos-unit-tests-recompile-the-whole-crate-on-every-topo-closure-pr.md`.
- **The bar itself is Ev's.** A deep-kernel PR's cold build is 16–20 min
  on 4-vCPU hosted runners, so no arrangement of its tests brings it
  under 15. The `[ev]` PR states the choice.

## A persistent incremental target, measured

Local, 4 cores, ci profile, 4954's 17-crate closure, one target
directory, in sequence:

| build | wall | workspace crates recompiled |
|---|---|---|
| cold, `CARGO_INCREMENTAL=0` (deps warm) | 18m19s | all |
| cold, `CARGO_INCREMENTAL=1` | 21m26s | 19 |
| a one-function body edit in `geom-core/src/spline/knots.rs` | 11m10s | 16 |
| then a one-line edit in `topo/src/readback.rs` | 8m43s | 11 |
| no change | 2 s | 0 |
