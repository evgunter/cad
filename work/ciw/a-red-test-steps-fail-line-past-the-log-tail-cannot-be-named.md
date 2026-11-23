---
id: a-red-test-steps-fail-line-past-the-log-tail-cannot-be-named
kind: issue
title: A red test step whose FAIL line falls outside the last 5000 log lines cannot be named from a session
status: closed
opened: 2026-10-14
closed: 2026-10-15
priority: P3
cost: M
branch: ciw/readable-reds
pr: 5206
refs: [5104, an-all-tier-diff-with-no-crate-seed-runs-no-slow-row]
---


## Found

PR #5104 at 3a366af7, run 38541955556, job 115778948969 (`test`):
step 8 "test (default eps)" was red, and the later steps (ε 1e-6/1e-12 over
the whole workspace, doc-tests) were green. Neither the lane nor its
orchestrator could name the failing row:

- **The tail is the wrong part.** The GitHub MCP `get_job_logs` returns at most the last 5000 lines of a
  job. A TIER=all `test` job prints about 14 000 `PASS` lines per suite
  step, so step 8's `FAIL` and `Summary` lines fall outside that tail.
  The tail holds only the 1e-12 step.
- **The full log is unreachable.** The full log archive
  (`get_workflow_run_logs_url`) is served from
  `results-receiver.actions.githubusercontent.com`, and the session's
  egress proxy refuses that host with a 403.
- **No re-run is possible.** `rerun_failed_jobs` returns 403 "Resource not accessible by
  integration", for both the lane and the orchestrator.
- **No annotation names it.** The check run's annotations say only "Process completed with exit
  code 100".

The row did not reproduce: four local default-ε runs at that head passed
13972/13972, and a later merge of main ran green. So the red was never
attributed. If it was a fuzzer, its seed is lost, and a fuzz counterexample that cannot be
read is one that cannot be pinned (`docs/prompts/implementer-discipline.md` §8).

## Wanted

Every nextest step writes what failed where a session can read it
without the log tail:
- either append nextest's failure list (test name plus the `[fuzz] … seed=` line)
  to `$GITHUB_STEP_SUMMARY`;
- or emit one `::error title=<test>::<seed / first panic line>`
  annotation per failure, readable through the check-run annotations API.

Both survive any later step's output volume.

Related: `an-all-tier-diff-with-no-crate-seed-runs-no-slow-row` (same PR,
the other gap it exposed: the slow set is skipped when a TIER=all diff
seeds no crate).

— build-time lane (NURBS), 2026-10-14

## 2026-10-15 — again on PR 5192 (TANG)

The failing row's `FAIL [0.087s] (9818/14131)` sat about 50 tests before
the 5000-line tail that `get_job_logs` serves, and the run log's blob
host refuses the session's proxy (403). The panic was read only after
pushing a temporary `failure-output = "final"` on the `ci` profile,
which moved the output into the closing summary, and reverting it in
the next commit.

## Closed

Every failing nextest row is now named where a session can read it.
The `ci` profile in `.config/nextest.toml` writes `junit.xml`
(`[profile.ci.junit]`) and sets `failure-output = "immediate-final"`.
`scripts/nextest-failures.py` runs after each nextest invocation and reads
the report's path from that table. It deletes the report once read, so a
later run that dies cannot re-report a stale one. For each failing test it
writes the name, the panic's location and message (backtrace dropped) and
its first four `seed=` lines (then a count of the rest) to the step summary
and to `$RUNNER_TEMP/nextest-failures.txt`. A final `if: failure()` step
prints that file, so the rows close the job log. The first two failing
tests of each invocation also get an `::error` annotation, and one more
annotation counts the rest: GitHub keeps 10 error annotations per step, and
nightly's ε loop runs three invocations in one step. A nonzero exit with no
failing row is recorded, naming the missing report path when there is none.
Any exception inside the script is a `::warning` and exit 0, so a step under
`bash -e` keeps nextest's exit code and its remaining ε rows.

The script covers every nextest step of ci.yml's `test` job (default ε,
the 1e-6/1e-12 loop, the slow set) and nightly.yml's `full suite` job.
The nightly job now runs `--profile ci --ignore-default-filter` rather than
the default profile. That is the same whole suite, with two differences: the
junit report, and `failure-output = "immediate-final"` in place of the
default `immediate`, so a failing test's output is printed both when it fails
and again at the end of each run.
