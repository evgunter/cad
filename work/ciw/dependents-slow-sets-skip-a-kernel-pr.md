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
after the full-suite rows, on scheduled runs only. That workflow runs
`scripts/nightly-attribution.py`; its docstring is the method.

- **Newly red.** A test is newly red at an ε row when it fails in this
  run's junit and passed in the latest earlier scheduled `main` nightly
  that reported it there, searched back seven runs. A test that a leg was
  running when it hit its budget counts as unreported on both sides, and
  this run's are listed as `leg stopped`.
- **Retry.** Each newly red test is retried once at the bad sha from the
  run's own test archive (one nextest call per ε row). A pass is `flaky`.
- **Range.** Every first-parent commit between the baseline and this
  run's sha; none is dropped by path (tests read `work/` and `docs/`).
  One commit is the answer with no probe.
- **Bisect.** Each probe archives one test target (`cargo nextest archive
  -p <crate> --test/--lib`) at the PR gate's opt-level 1, into the
  `target/` rust-cache restored from `build-default`. It runs that
  binary's open tests in one nextest call per ε row. A failure counts
  only when its junit type matches the nightly's (a panic). A crash,
  abort, timeout, absent test or failed build makes the commit untestable
  for that test. A rustc killed for memory is retried at `-j 2`.
- **A culprit is confirmed.** C is named only when, in the probe
  configuration, the test passed at C^1 and failed at C, and a re-run of
  both from the kept archives agrees. A bisect that narrows to an end
  probes that end, so a red the probe configuration does not share comes
  back as `range … (probe config disagrees at the bad end)` (or good
  end), never as a culprit. The docstring states the monotone, no-flake
  assumption.
- **Order and bounds.** The binary with most open tests goes first and
  finishes before the next. Once C is confirmed, other binaries probe C
  and C^1 first. The limits:
  - 12 builds and 60 minutes, counted from the script's start; the step
    is cut at 70 and the job at 90;
  - 900 s of tests per nextest call;
  - a test that took over 300 s at the bad sha, a range over 200
    commits, and anything past the 25 cheapest tests get a range with
    the reason.

  Verdicts reached before an error are kept. The script exits 0 and
  reports its own failure as `attribution failed: <why>`, and a final
  step says so if the step itself died.
- **Output.** `test | eps | verdict` in the job summary and in the
  `nightly-attribution` artifact (`.md`, `.json`).
- **Gates.** `scripts/gates/build-default-env-agrees.sh` holds the job's
  env to cache-prime's. The script's `--selftest` runs in
  `gate-selftests` and covers:
  - the leg-stopped rule;
  - work/-only commits staying in the range;
  - the confirm path;
  - both end disagreements;
  - an abort not counting as a failure;
  - C and C^1 tried first;
  - verdicts kept after an error.

**Evidence.** Scheduled run 44680731754 (11-20, `8039ba06`). Its 11-19 and 11-18
runs uploaded no junit, so the baseline is 11-17 (`cdee7461`): 29 newly
red tests over 172 first-parent commits. Dry runs of 11-17 and 11-21
found 25 and 37 newly red tests over 157 and 152 commits.

Bisected locally on `topo
ring_path::spline_fuzz::a_path_across_a_carried_loop_reads_the_membership_parity`
@ 1E-06, whose culprit GREEN's U6 found by hand
(`main-red-seven-sweep-and-topo-rows-red-at-1e-6-since-11-17`: green at
7052^1, red at 7052):

- **The first version** (merges only, no confirmation) named **#7052
  (6c2430e888)** in 7 probes.
- **The current version** archived and ran in the real nextest-archive
  mode. #7080, `b916398a3f` and `823c0fa4c9` failed, #7049 passed, and
  `f50b58e2f3` failed: 5 builds of 5.5 to 10 minutes, each archive 25 MB.
  That narrowed the range to `9c47bfbe57..f50b58e2f3`, which holds
  #7052.
- The box's shared disk then filled from other lanes' build
  directories: rustc "No space left on device" at #7052, then a failed
  `git checkout`. The run reported
  `range 9c47bfbe57..f50b58e2f3 (5 commits; stopped by an error (CalledProcessError))`,
  so the bisect stopped on the error and kept the range it had reached.
- A third attempt hit a full disk at its fourth build. The confirmed
  culprit path (C^1 and C re-run) has not run on a real build; the
  selftest covers it.

The workflow cannot be dispatched from a branch (GitHub dispatches only
default-branch workflows). actionlint 1.7.7 with shellcheck is clean.

Residue: `the-nightly-attribution-writes-its-verdicts-to-no-row` (P3).
It holds writing the `main-red` row, the culprit-PR comment and the
capacity limit. (CIW lane)
