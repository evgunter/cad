---
id: gate-selftests-run-in-no-ci-job
kind: issue
title: No CI job runs any scripts/gates/*.sh --selftest, so a gate's selftest can sit red on main for weeks unseen
status: closed
opened: 2026-10-26
priority: P3
cost: E
closed: 2026-10-28
---


Split out of `loop-boundary-selftest-cannot-plant-an-arm-fragment`
(closed 2026-10-26, tracker sweep), whose second finding this was.

## What

`ci.yml`'s `gates (scripts/gates)` step runs every `scripts/gates/*.sh`
**plain** (`bash "$g"`, no `--selftest`), and no other job runs a gate's
self-test: at `bb25fa4f` the only `--selftest` invocations in
`.github/workflows/` are `dual-review-log.py`, `slow_set.py`,
`check-slow-set-names.py`, `check-fast-set-times.py` (`ci.yml`),
`doc-gate.sh`, `criterion-emit.py` (`nightly.yml`) and three render
scripts (`render.yml`). Instrument: `grep -n selftest .github/workflows/*.yml`.

## Why it is a row

Every gate in the directory argues that "a guard never shown to fire is
not a guard", and each gate's `--selftest` is what shows it. Nothing
shows that the self-tests themselves still pass. The instance:
`loop-boundary-discards.sh --selftest` failed on a clean main from at
least 2026-10-06 (`212ccfa1`) to 2026-10-26 and was filed twice, by two
lanes that happened to run it; CI was green throughout.

## Shape

A step (per-PR when `scripts/gates/` changes, or nightly) that runs
`bash "$g" --selftest` for every gate. Two things to measure first: the
total wall time of all self-tests (some plant whole fixture trees), and
whether every gate accepts `--selftest`. The wiring is in
`.github/workflows/*`, which is CIW's ground, so it lands by announced
seam.

## Closed (2026-10-28)

`nightly.yml` gains a `gate self-tests (scripts/gates)` job. It runs
`scripts/run-gates.sh --selftest`, the same runner `ci.yml`'s `lint`
job now runs plain, so the roster (every `scripts/gates/*.sh` but
`lib.sh`) is read in one place. Each gate gets its own log group with
its wall time, and the job fails with an `::error::` line for each gate
that exits non-zero or that exits 0 without printing its own
`<gate> selftest OK` line (a gate that ignored the flag would scan the
repo plainly and pass). It runs nightly only, so an ordinary PR's wall
clock is unchanged. `record` waits on it.

Measured first, on `main` at `4beeeb05`:

- **Every gate accepts `--selftest`.** All 30 gates parse it through
  `lib.sh`'s `gate_parse_args`, and each one's last log line is its own
  `selftest OK: …` sentence, so none silently ran plain.
- **All 30 pass on main.** None is red today, so there is no finding to
  fix or file.
- **Wall time.** About 181 s locally, on a box shared with a running
  build. The slowest were `probe-suite-census` (50 s), `bounds-allowlist`
  (31 s), `viewer-module-kinds` (19 s) and `certification-doors` (18 s).
  On a hosted runner, measured through a temporary push-triggered copy of
  the job: **the step took 1 min 41 s and the whole job 2 min 10 s**,
  green.

**A per-PR self-test step was considered and declined, on latency
grounds.** Even scoped to diffs that touch `scripts/gates/`, it would
add about two minutes to the `lint` job of those PRs, against a gate
already over its 15-minute bar. A gate edit that breaks its own
self-test is seen the next night rather than on its PR, and the
instance this row was filed on (a self-test red for 20 days) is the
kind a nightly catches.
