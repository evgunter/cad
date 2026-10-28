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
`bash "$g" --selftest` for every `scripts/gates/*.sh` except `lib.sh`,
prints each gate's wall time in its own log group, and fails at the end
with an `::error::` line for each red gate. It runs nightly only, so an
ordinary PR's wall clock is unchanged. `record` waits on it.

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

Not done: a per-PR step for diffs that touch `scripts/gates/`. A gate
edit that breaks its own self-test is seen the next night rather than
on its PR. The per-PR gate's `lint` job is the latency lane's ground
this week, and the instance this row was filed on (a self-test red
for 20 days) is the kind a nightly catches.
