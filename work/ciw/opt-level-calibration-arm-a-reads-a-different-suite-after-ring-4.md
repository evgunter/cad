---
id: opt-level-calibration-arm-a-reads-a-different-suite-after-ring-4
kind: issue
title: The opt-level calibration's free arm reads a renamed build job and a different suite from RING-4 on
status: open
opened: 2026-09-24
priority: P4
cost: E
refs: [ring-4-interval-feature-dropped]
---

## What

`scripts/opt-level-calibrate.py`'s free arm (arm A) reads `a` from the
`build + archive` job's archive step and `E` from the `test (eps = …)` legs
of recent gate runs. RING-4 folded CI's lane axis: the job is renamed
(`build + archive (default)` → `build + archive`, and `ARCHIVE_JOB` moved
with it), and the archive it times and the suite the legs execute are now
the WHOLE suite at every scalar — what the interval lane built and ran —
where they were the `f64`-only default lane's.

So the arm's population steps at RING-4's merge: runs before it no longer
match `ARCHIVE_JOB` and are skipped (a run from before the rename reads as
"not a code-tier run"), and runs after it measure a larger compile and a
longer execution. The script's own drift cadence (>20 % moves a
recalibration) will likely fire on the step. Nothing is wrong with either
reading; what is missing is a line in the calibration history that says the
subject changed on that date, so the step is not read as an opt-level
effect.

## Evidence

- `scripts/opt-level-calibrate.py`: `ARCHIVE_JOB`, `sample_run`.
- `.github/workflows/ci.yml`: the `build` job's name and the `test` job's
  two named `(interval)` steps, both from RING-4.
