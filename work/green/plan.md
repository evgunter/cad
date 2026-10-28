# GREEN — the plan

Main green and the per-PR gate fast. Opened 2026-10-28 by a
priority-seam cut across the infra group (`work/green/log.md`).

## The slate

**26 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | lane |
|---|---|---|---|
| P0 | `pr-ci-wall-clock-root-cause-measured` | M | latency |
| P0 | `the-demos-job-is-pr-cis-long-pole-on-its-release-build` | M | latency |
| P0 | `a-sweep-seeded-diff-runs-the-test-job-past-the-fifteen-minute-bar` | M | latency |
| P1 | `main-reds-editor-core-slow-set-and-the-k-probe-sweep` | H | slow-set reds |
| P1 | `tess-lint-gate-red-on-main-23-findings` | M | tess-lint |
| P1 | `main-red-tilted-cut-upper-moves-in-the-thread-count-digest-at-default-eps` | E | small reds |
| P1 | `nightly-rustdoc-gate-red-on-main` | E | small reds |
| P2 | `tests-red-under-all-features-never-run-by-ci` | E | gate trust |
| P3 | `pick-oracle-sweep-anti-vacuity-fails-one-seed-in-1300` | E | gate trust |
| P3 | `pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1` | C | rides with the above |
| P3 | `random-integer-rays-search-trips-at-eps-1e-6-on-one-run` | C | rides with the above |
| P3 | `cert10-strict-gap-floor-gates-on-a-varying-seed` | E | gate trust |
| P3 | `gate-selftests-run-in-no-ci-job` | E | gate trust |

## Lanes

Five lanes, all dispatchable at once: they touch different ground,
except that the latency lane and the gate-trust lane both edit
`.github/workflows/`, which is coupled only where a new step adds to
the per-PR wall-clock. The gate-trust lane therefore puts any new
step (the gate self-tests, an `--all-features` test row) on the
nightly, or measures it against the latency lane's budget first.

- **latency** (P0). Done when the nightly's `PR CI latency (alarm)`
  reads a median under 20 min, with a measured path toward Ev's 15.
  The 10-24 measurement makes `demos` the critical path (its release
  build, and one 513 s row, `tour_runs_green_at_eps_1e_12`); the
  10-11 one makes a sweep-seeded `test` job 21.5 min. Measure today's
  split before choosing levers. No lever may hide a red: a row moved
  off the PR gate still runs nightly.
- **slow-set reds** (P1). The 24 editor-core slow-set rows and the
  k-probe sweep's two aborts. Bisect one row per file before blessing
  anything; re-derive receipts on the day.
- **tess-lint** (P1). Read the 23 findings scene by scene before any
  baseline re-cut.
- **small reds** (P1). `tilted_cut_upper`'s ulp move and the nightly
  rustdoc job.
- **gate trust** (P2–P3). Four cheap rows that make the gate lie in
  either direction.

## Exit

The track closes when the nightly is green on main for the rows on
this slate and the latency alarm is green. No exit walk.
