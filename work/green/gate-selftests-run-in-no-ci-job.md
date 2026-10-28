---
id: gate-selftests-run-in-no-ci-job
kind: issue
title: No CI job runs any scripts/gates/*.sh --selftest, so a gate's selftest can sit red on main for weeks unseen
status: open
opened: 2026-10-26
priority: P3
cost: E
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
