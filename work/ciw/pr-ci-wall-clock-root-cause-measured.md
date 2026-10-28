---
id: pr-ci-wall-clock-root-cause-measured
kind: issue
title: PR CI wall-clock rose from 9 to 33 min because the test job's own steps tripled, with TIER=all runs tripling in share, not because of queueing
status: open
opened: 2026-10-16
priority: P3
cost: E
---


The question (Ev): what is the root cause of the rise in per-PR CI time? Nightly 61's `PR CI latency (alarm)` read a median of 32.6 min (p90 52.1, n=256).

## Answer

The rise is execution time on the critical path, not queueing. It has three parts:

1. **The `test` job's test steps grew 5×.** They went from about 3 to 21 min median (sure).
   - The suite grew. `#[test]` count went from 10,390 to 14,872.
   - Tests ≥ 1 s accumulated faster still. The slow-set name count went from 141 to 512.
2. **Its `build` step doubled**, from 9 to 16–19 min median (sure that it doubled; likely that code growth is why). #5104 has taken back about 3 min.
   - The critical-chain crates' `src/` doubled in 15 days: topo 140k → 293k lines, geom-brep 42k → 92k, editor-core 90k → 131k.
3. **TIER=all runs went from 14% to 46% of completed runs** (sure).
   - A TIER=all run builds `--workspace` and runs the whole suite at 1e-6 and 1e-12 (`EPS_EXTRA=all()`).
   - The tier mix alone accounts for about 7 of the 30 min. The per-tier growth accounts for about 22.

Ruled out:
- **Volume.** Runs per day did not rise: 600 on 10-01 at a 13 min median, against 400 on 10-13 at 38 min.
- **Queueing in the median.** The run-to-first-job and critical-job queues have a median of ≤ 0.1 min on every day. The p90 queue reaches 9–13 min on the busiest days (10-09..10-13). That is a second-order effect of the larger job-minutes per run.

## The trend

The statistic is the alarm's own: completed `pull_request` runs of ci.yml with conclusion success or failure, `updated_at − created_at`, over the 24 h before 05:30Z. Rows marked `[n]` are the nightly's printed line, read from the job log. The other rows are recomputed from `list_workflow_runs`. The recompute reproduces the printed lines within 0.5 min (e.g. 10-15: 32.7 / 52.1 / n=259 against 32.6 / 52.1 / n=256). The alarm job first exists in nightly 39 (09-29).

| 24 h to 05:30Z | n | median | p90 | cancelled (superseded) |
|---|---|---|---|---|
| 09-26 (before latency-cut) | 188 | 62.7 | 141.6 | 185 |
| 09-29 | 201 | 13.6 | 19.2 | 43 |
| 09-29 11:24 `[39]` | 247 | 12.1 | 15.8 | |
| 09-30 | 262 | 11.2 | 16.1 | 65 |
| 10-01 | 289 | **7.7** | 16.9 | 64 |
| 10-01 11:39 `[45]` | 407 | 9.9 | 18.9 | |
| 10-02 | 540 | 14.1 | 21.2 | 132 |
| 10-03 10:27 `[49]` | 380 | 17.0 | 23.8 | |
| 10-05 | 112 | 19.4 | 25.7 | 38 |
| 10-07 11:49 `[53]` | 373 | 23.9 | 33.6 | |
| 10-09 `[55]` | 266 | 22.8 | 35.5 | |
| 10-11 `[57]` | 356 | 30.9 | 46.8 | |
| 10-12 | 359 | 24.6 | 39.0 | 105 |
| 10-13 | 382 | 25.9 | 41.4 | 112 |
| 10-14 `[60]` | 329 | **36.4** | 71.5 | |
| 10-15 `[61]` | 256 | 32.6 | 52.1 | |
| 10-16 `[62]` | 99 | 33.5 | 49.3 | |

`latency-cut` (09-28) took the median from about 60 min to 8–14 min. The median then rose steadily rather than in one step: it crossed the 20-min alarm on 10-07, passed 30 on 10-11, and peaked at 36 on 10-14.

Volume by UTC day, all `pull_request` runs:

| day | runs | success+failure | cancelled | branches | median wall |
|---|---|---|---|---|---|
| 09-29 | 338 | 276 | 62 | 106 | 11.1 |
| 09-30 | 325 | 258 | 67 | 99 | 9.1 |
| 10-01 | 599 | 492 | 107 | 154 | 13.2 |
| 10-02 | 569 | 449 | 120 | 159 | 15.8 |
| 10-06 | 532 | 434 | 98 | 128 | 22.2 |
| 10-10 | 439 | 313 | 126 | 119 | 27.2 |
| 10-11 | 510 | 391 | 119 | 115 | 28.4 |
| 10-12 | 510 | 401 | 109 | 142 | 24.5 |
| 10-13 | 410 | 304 | 106 | 116 | 37.9 |
| 10-14 | 391 | 315 | 76 | 129 | 31.3 |
| 10-15 | 144 | 122 | 22 | 57 | 33.8 |

About 20–25% of runs are superseded (cancelled) every day. The alarm excludes them, and so does everything below.

## The decomposition

**Sample.** 30 completed runs a day were drawn at random (seeded) from success/failure runs, 09-28..10-16, 549 runs of the post-cut shape. For each run the sample takes `list_workflow_jobs` (latest attempt) with step timestamps. Every sampled run is attempt 1.

**Windows.** "Before" is the trough, 09-29..10-01 (n=90). "After" is 10-13..10-15 (n=90). The entries are median / mean in minutes.

| component | over | before | after | 10-16 |
|---|---|---|---|---|
| **run wall** (`updated − created`) | all runs | 8.5 / 8.9 | **38.8 / 35.1** | 21.1 |
| queue: run created → first job started | all runs | 0.1 / 0.3 | 0.1 / 0.8 | 0.1 |
| `change filter` | all runs | 0.6 | 0.9 | 0.8 |
| queue: critical job created → started | all runs | 0.0 / 0.2 | 0.0 / 1.1 | 0.0 |
| critical job's duration | all runs | 7.2 / 7.8 | 35.3 / 31.9 | 20.0 |
| `gate ok` after the critical job | all runs | 0.1 | 0.1 / 0.4 | 0.1 |
| `test` job | runs it ran in (56 / 73) | 11.4 | **38.0** | 24.8 |
| · setup (checkout, toolchain, cache restore, nextest) | | 0.4 | 0.4 | 0.5 |
| · build | | 9.1 / 8.0 | **16.1 / 16.9** | 14.5 |
| · default ε | | 1.1 | **5.3** | 4.8 |
| · extra ε (1e-6 + 1e-12) | | 0.8 / 1.5 | **9.2 / 8.6** | 1.7 |
| · slow set | | 0.8 / 1.1 | **6.5 / 8.0** | 5.3 |
| · doc-tests | | 0.1 | 0.2 | 0.1 |
| `demos` (tour + wild) | runs it ran in | 12.4 | 25.1 | 25.0 |
| `python` | runs it ran in | 3.6 | 10.8 | 10.9 |
| `lint` | runs it ran in | 2.5 | 4.7 | 3.2 |

- `test` is the critical job in 50–90% of each day's sampled runs. `lint` is the critical job on most of the rest, which are the docs-tier runs.
- `demos` (25 min) is under `test` (38 min) after the rise, so it is off the critical path. Before the rise, `demos` (12 min) sat beside a `test` of 11–16 min.

**By tier.** The tier is inferred from which jobs ran. "docs/aux" means `test` was skipped. "all" means `demos`, `tools`, `python` and `step import` all ran: on TIER=all every direct-path row is forced on, and on TIER=closure each runs only when its own subject is in the diff. "closure" is the rest.

| tier | share before → after | wall | test job | build | default ε | extra ε | slow set |
|---|---|---|---|---|---|---|---|
| docs/aux | 38% → 19% | 1.9 → 3.9 | — | — | — | — | — |
| closure | 48% → 36% | 12.0 → 37.4 | 11.2 → 31.2 | 8.4 → 15.8 | 1.1 → 4.4 | 0.4 → 2.6 | 0.3 → 6.0 |
| all | 14% → 46% | 19.0 → 45.6 | 15.9 → 43.5 | 10.5 → 16.2 | 1.6 → 6.1 | 3.5 → 13.3 | 2.0 → 6.6 |

**Reweighting the tier mix.**
- The after-window's per-tier wall-clocks drawn under the before-window's tier mix give a median of **30.1** min.
- The before-window's per-tier wall-clocks under the after-window's mix give **15.1** min.
- Against 8.5 → 38.8, about three quarters of the rise is per-tier growth and a quarter is tier mix.
- On a TIER=all run the extra-ε step is two more full passes of the suite, so it scales with the default step: 3.5 ≈ 2 × 1.6, and 13.3 ≈ 2 × 6.1.

## What grew inside the steps

**Test time.** One TIER=all `test` job on each side, from the job logs. The tool returns only the last 5000 lines, which hold the end of the 1e-12 pass and the slow set:

| | 36891624023 (09-30), job 110468710965 | 38670267649 (10-15), job 116211764790 |
|---|---|---|
| last ε pass: nextest summary | 10,679 tests, 112 s | 14,119 tests, 443 s |
| tail rows ≥ 1 s (not in the slow set) | 10 rows, 38 of 136 cpu-s | **46 rows, 445 of 548 cpu-s** |
| heaviest rows | sweep `pis_cut_cavity` 12 s | sweep `review_d2_adv_probes::d2_no_input_reaches_a_panic` 94 s, `::d2_reached_variants` 85 s, `verbs_germarms2::every_pose_of_the_family_builds_at_its_closed_form` 45 s, topo `m6_2_fitted_at_rest::*` 15–20 s |
| slow set | 120 tests, 186 s; geom-brep 270, editor-core 268, step-import 91 cpu-s | 179 tests, 328 s; editor-core 770, topo 526 cpu-s |

- The test count grew 32% while the pass's wall-clock grew 4×. The difference is ≥ 1 s rows inside the `ci` profile's fast set, with single rows at 85–94 s.
- The slow set's name count tells the same story: `.config/nextest.toml` held 141 test names at main on 09-30, 512 on 10-14 and 551 now.
- Every name added there is an edit to `.config/nextest.toml`, which forces TIER=all. That is the link between the test-time growth and the tier-mix growth. See `a-slow-set-edit-to-nextest-toml-runs-the-whole-suite` (36 of 257 merges in a week) and `predicate-dimension-audit-doc-forces-the-whole-suite` (22 of 257), both filed on the CIW orchestrator's branch.

**Build.** Workspace lines on main, `crates/*/src` and `crates/*/tests`:

| main at | src lines | tests lines | `#[test]` |
|---|---|---|---|
| 09-28 (`8a6c8db3e8`) | 506,691 | 625,731 | 10,390 |
| 10-01 | 573,350 | 658,346 | 11,258 |
| 10-05 | 661,506 | 735,729 | 12,598 |
| 10-09 | 751,146 | 805,562 | 13,907 |
| 10-13 (`b3209dc5a1`) | 809,267 | 851,491 | 14,872 |
| 10-16 (`95c77e152a`) | 845,432 | 880,286 | 15,442 |

Per crate, `src/` 09-28 → 10-13:
- topo: 139,624 → 292,644;
- geom-brep: 42,390 → 91,508;
- editor-core: 90,029 → 131,162;
- geom-core: 38,746 → 56,617;
- sweep: 22,343 → 35,282 (its `tests/` 120k → 215k).

These crates are the build's critical chain (`the-test-jobs-build-time-candidates-measured`). The TIER=all build always builds `--workspace`, so its scope did not change, yet its median went from 10.5 to 16.2 min. That makes code growth, not a widening scope, the likely cause.

The cache did not go cold. The rust-cache restore step took 9–15 s (median 12–14 s) on every day's sample, with no run under 9 s.

**Job-minutes.** Mean job-minutes per completed run rose from 15–19 (09-29..10-01) to 41–75 (10-13..10-15). At 300–400 runs a day that is 45–74% of a 20-slot day, before counting cancelled, push, nightly and render runs. That is why the p90 queue appears on busy days.

## Blind spots

- **The tier is inferred, not read.** The `change filter` log names `TIER` and `CARGO_SCOPE`, but read one at a time it costs one call per run. One run (38631873868, inferred "all") was read: `TIER: all`, `CARGO_SCOPE: --workspace`, `eps_extra=all()`. A closure diff that touches `demos/` and `tools/` and seeds pncad-py and step-export would be misread as "all"; that combination is not expected to be common. docs and aux are merged.
- **Closure scope (PKGS size) is not measured.** Part of the closure-tier build growth (8.4 → 15.8) could be wider closures rather than bigger crates. The TIER=all build, which has a fixed scope, says code growth is at least most of it.
- **Per-test times cover only the last 5000 log lines** (the tool's cap). The default-ε step's per-crate split is therefore not measured; only the 1e-12 pass's tail and the slow set are. Two runs are not a distribution.
- **A cache hit cannot be told from a stale fallback.** A restore-key fallback (a partial hit with stale deps) restores in the same 12 s and would put dependency compiles into `build`.
- **Medians do not add up.** The component medians do not sum to the wall median. Means do approximately, which is why both are shown.
- **The sample is 30 random runs a day.** The tier shares carry about ±9 points of sampling error a day, so the 3-day windows are used for the attribution.
- **10-16 is a partial, quiet day** (41 runs by 07:00Z), so its column is shown but not used.

## Method

Run lists come from `GET /repos/evgunter/cad/actions/workflows/ci.yml/runs?event=pull_request&created=<day>` with manual paging. The `gh` CLI was used for the bulk calls; these are the same REST endpoints as the GitHub MCP `actions_list` and `actions_get`. Job lists come from `/actions/runs/<id>/jobs?filter=latest`. Logs come from the GitHub MCP `get_job_logs`, because the proxy refuses the log-blob redirect for `gh`. The alarm's statistic was recomputed with the nightly's own selection and arithmetic, `.github/workflows/nightly.yml:661-698`.

— CIW latency measurement lane, 2026-10-16
