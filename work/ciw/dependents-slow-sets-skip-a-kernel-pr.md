---
id: dependents-slow-sets-skip-a-kernel-pr
kind: issue
title: The nightly attributes each newly red slow or ε row to its merge, so no lane bisects a main red by hand
status: closed
opened: 2026-10-16
closed: 2026-11-22
branch: ciw/nightly-attribution
priority: P1
cost: M
---

Ev ruled on PR 6731 (2026-11-12) that a newly red slow test is bisected
by the nightly and written up as a row. No PR runs a crate's whole slow
set (items 1 to 3 of the policy below this row's history). Nothing
implements it yet: neither `.github/workflows/nightly.yml` nor `scripts/`
mentions an attribution. Its only bisection aid is the `sha` input at
`nightly.yml:16`, which someone has to drive by hand.

**What it would save.** From 2026-10-22 to 11-22, 157 `main-red-*` rows
were opened across `work/`. About 140 of them are slow or non-default-ε
rows that the breaking PR's gate did not run. Many were bisected by hand:
ONECOPY's `main-reds-triage` and `slow-set-reach`, GREEN's `goldens`, and
SCALAR. A bisect takes about 7 to 13 probes at about 6 min each
(`work/ciw/log.md`, 2026-11-10). Lanes also hit reds on their merge base
and had to prove them inherited.

**The cost.** It costs nothing on PRs and adds runner time to the
nightly only. That time is at most five bisects a night, each probe
rebuilding one test binary.

**What it subsumes.** The per-PR canary that catches most of these reds
before merge is `a-stored-data-change-runs-none-of-the-slow-rows-that-read-it`.
This row keeps the policy and the counts already recorded here, and
feeds that row's list.

What would close it: a nightly job that bisects up to five newly red slow
or ε rows over the first-parent merges between two nightly heads and
writes or appends the `main-red` row the policy specifies.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): - The row holds Ev's 11-12 policy: the nightly attributes each newly red
slow test to its merge. That is not implemented. `nightly.yml` has
only a manual `sha` input (line 16), and no script bisects or writes
rows.
- It costs no PR latency and replaces hand bisects (7 to 13 probes at
about 6 min each). About 130 of the month's 157 main reds are of the
kind it would attribute.
- Reform at `reform/dependents-slow-sets-skip-a-kernel-pr.md`, retitled
to the attribution. Its counts feed the P1 canary list.
- 2026-11-22 — **Re-banded P2 → P1 by the CIW orchestrator.** Ev already ruled for it (PR 6731), it costs no PR time, and it removes the hand bisect from most of last month's ~130 main reds of this kind. It is the zero-PR-cost half of `a-stored-data-change-runs-none-of-the-slow-rows-that-read-it`, and the one to build first.

## Closed

The scheduled nightly now attributes its newly red tests itself.
`nightly.yml`'s `attribution` job calls `.github/workflows/nightly-attribution.yml`
after the full-suite rows. It runs on scheduled runs only. That workflow
runs `scripts/nightly-attribution.py`:

- **Newly red.** A test is newly red at an ε row when it fails in this
  run's junit and passed in the latest earlier scheduled `main` nightly
  that reported it at that row. Earlier nights are searched back seven
  runs, so a night whose build died is stepped over. A red test with no
  earlier report is counted, not attributed.
- **Retry.** Each newly red test is retried once at the bad sha from the
  run's own test archive. A pass is reported as `flaky`.
- **Attribution.** The range is the first-parent commits between the
  baseline sha and the run's sha. Commits that touch only `work/` or
  `docs/perf-data/` are dropped (58 of 157 kept on 11-15..11-17). One
  commit left is the answer with no probe. Otherwise it bisects. Each
  probe builds one test target (`cargo test --no-run -p <crate>
  --test/--lib`, PR-gate env, `build-default` cache restored) and runs
  only the open tests of that binary, each at its own ε. A commit where
  the target does not build, the test is absent or it times out is
  skipped for that test. A rustc killed for memory is retried at `-j 2`.
  One binary's bisect finishes before the next starts, so a spent budget
  leaves whole answers.
- **Bounds.** 12 probes, 70 minutes, a 300 s per-test cap at the bad sha,
  a 200-commit range cap and a 25-test cap; past a bound the verdict is
  the range with the reason. The job has `timeout-minutes: 90` and
  cannot redden the run. Every step continues on error, and the script
  reports its own failure as `attribution failed: <why>` and exits 0.
- **Output.** `test | eps | verdict` in the job summary and in the
  `nightly-attribution` artifact (`.md` and `.json`). A verdict is
  `introduced by #NNNN (sha)`, `range a..b (… bisect stopped: why)` or
  `flaky (passed on retry at the bad sha)`.
- **Gates.** `scripts/gates/build-default-env-agrees.sh` now holds the
  attribution job's env to cache-prime's. The script's `--selftest`
  runs in the nightly's `gate-selftests`.

**Evidence.** Dry run against scheduled run 44680731754 (11-20, `8039ba06`).
The script stepped over the 11-19 and 11-18 runs, which have no junit,
to the 11-17 baseline (`cdee7461`). It found 29 newly red tests over 71
commits. Real run, locally, on the `topo
ring_path::spline_fuzz::a_path_across_a_carried_loop_reads_the_membership_parity`
@ 1E-06 row:

- The retry at `8039ba06` failed.
- Seven probes followed (#7094, #7070 and #7055 fail, #7050 and #7043
  OOM-killed on this shared box so skipped, #7049 pass, #7052 fail).
- Verdict: **introduced by #7052 (6c2430e888)**, the merge
  GREEN's U6 bisected by hand (`main-red-seven-sweep-and-topo-rows-red-at-1e-6-since-11-17`,
  "TubeStraddles near the node", green at 7052^1, red at 7052).

The OOM fallback came out of that run. The workflow could not be
dispatched from the branch: GitHub dispatches only workflows on the
default branch. actionlint 1.7.7 with shellcheck is clean.

Residue: `the-nightly-attribution-writes-its-verdicts-to-no-row` (P3).
It covers writing or appending the `main-red` row, commenting on the
culprit PR, and the one-group-a-night probe capacity. (CIW lane)

