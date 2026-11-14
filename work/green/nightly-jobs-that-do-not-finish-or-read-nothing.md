---
id: nightly-jobs-that-do-not-finish-or-read-nothing
kind: unit
title: Nightly jobs that do not finish, stop at their first red, or read no test
status: dispatched
opened: 2026-11-14
priority: P1
cost: M
---

From nightly 43371638371 (main `e36675c1`, 2026-11-13) and the run before it (43161026932). These are jobs, not tests: each one hides results rather than reporting a red.

**Difficulty letter:** M. **Review tier:** single review, FULL — It changes what the nightly runs, and a job can drop tests and read as a faster run, so the review carries correctness claims (FULL). Both logged before any draw (2026-11-14).

## What is wrong

- `suite under --all-features (default eps)`: on 11-13 its build step was still running when the job ended at 62 minutes (step cap 60), and the logs return 404. It took about 32 minutes on 11-12. `nightly-all-features-build-hung-once-with-a-test-binary-running` (this slate, P3) names an earlier hang of the same step.
- `full suite (eps 1e-12)`: 92 minutes on 11-13 and 118 on 11-12, against a 120-minute cap. One test, `sweep::all germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`, is 1953 s of it (`work/tcost/the-tilted-rods-other-spins-row-runs-past-two-hours-at-1e-12-locally`); `spline_subtract_intersect_census::every_op_of_a_spline_operand…` is 1072 s. A leg that times out reports nothing for any 1e-12 row.
- `k-lint (dev-probe)`: `bash -e` stops at the first failing probe suite (`assemble_one_local_battery`'s gather row, `work/onehome/main-red-the-gather-records-one-sample-tier-3-prime-does-not`), so the rest of the sweep, `k_probe_brackets` and both k-lints have not run since 11-08.
- `Any pins under the first-refusal mutant`: "any-first-mutant: the control run read no test (the build failed?)", while the log shows 21 of 21 pins passing.
- `viewer app feature`: fail-fast, 784 of 1565 tests not run behind the two `door_at_rest` reds (`nightly-reds-default-eps-behaviour-rows` greens those; whether the job should stop at its first red is this unit's).

PR 6731 (`onecopy/slow-set-reach`, open) makes the nightly attribute each newly red slow test to its merge; read it before touching `nightly.yml`, since the two are coupled.
