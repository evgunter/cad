---
id: e6-driver-k-probe-reds-at-eps-1e-6
kind: issue
title: E6 driver K-telemetry row reds at eps=1e-6 (nothing certified, nothing to sample) - pre-existing on main, never drawn by the klint axis
status: open
opened: 2026-08-31
github: 1342
refs: [1290]
---

## From GitHub issue 1342

Opened 2026-08-31; 0 comments.

## What

`m10_3_driver_k_probe_interval::k_report_driver_dump` — the E6 driver's
K-telemetry dump row, run by `scripts/k_probe_sweep.sh` inside the
`k-lint (gate)` job's "K-telemetry probe sweep" step — **panics at
`CAD_TOLERANCE_EPS=1e-6`**:

```
crates/editor-core/tests/m10_3_driver_k_probe_interval.rs:117:5:
nothing certified, nothing to sample
```

The row drives two ε-relative fixtures (`slab(1.0, eps/16)` and
`slab(20*eps, 40*eps)`, `documents()` at `:93`) with
`KProbe::CertifiedMidpoints` and asserts `!v.certified().is_empty()`.
At ε = 1e-6 the driver certifies no leaf, so there is nothing to sample
and the row reds. It passes at the default ε.

## It is pre-existing on `main`, and it is a silent-coverage case

Measured (2026-08-31): reverting `crates/` wholesale to `origin/main`
and re-running the same command reproduces the panic byte for byte —

```sh
git checkout origin/main -- crates/
CAD_TOLERANCE_EPS=1e-6 cargo test -p editor-core --features probe,interval \
  --test all -- m10_3_driver_k_probe_interval:: --ignored --nocapture
# ... nothing certified, nothing to sample
```

It has never been seen because the step has never run on `main`: the
last six `main` CI runs all record `k-lint (gate)` as **`skipped`**
(the job is its own sampled axis, `klint_row`). It surfaced on PR #1290
only because that PR's change set draws the k-lint tier. Third-plus
face of the class recorded in `memories/agent-lane-operations.md` — a
green job name over a step that never ran.

## Suggested disposition

The E6 unit owns the fixtures and the assert. Either the slabs need to
scale so the driver has something to certify at a coarse ε, or the row
needs an honest ε-scoped skip (loud, not silent) saying the population
does not exist there. Not fixed in #1290: it is another unit's fixture,
and #1290's own suites are green at every lane and ε locally.

## Home

M10: the row is `crates/editor-core/tests/m10_3_driver_k_probe_interval.rs`, inside M10's `crates/editor-core/tests/m10*` territory glob, and the E6 interval subdivision driver is M10's charter.
