---
id: reinstate-full-configuration-runs
kind: unit
title: Reinstate full configuration runs in place of the lane/eps sampling draw
status: review
opened: 2026-09-04
refs: [1796]
pr: 1823
branch: ciw/reinstate-full-runs
---

## The ask, and who authorised it

**Ev, in chat, 2026-09-04:** *"feel free to reinstate full runs instead of
sampling"*, with the reasoning *"CI is weakened right now because of sampling
only certain configurations to run … undoing that sampling now that actions
minutes are much cheaper is probably a good idea regardless of if we still
need the fix described in [PR 1796]"*.

This unit implements a ratified decision. It does not re-open one.

## What is being undone

Since 2026-08-22 a hosted run has gated **one point** of {default features,
`interval`} x {default eps, 1e-6, 1e-12}, drawn deterministically from the
head SHA (`scripts/ci-filter.py`, `_sample`; the wiring is
`.github/workflows/ci.yml`'s `test` / `test-interval` jobs and the
`lane != …` conditions). A green run meant green *at the point it drew* —
about one run in six per point — not at the other five.

The premise was a scarce billed resource: `docs/CI-MINUTES-2026-08.md` opens
with the Actions allowance being consumed faster than the work justified. The
repository went public on 2026-09-03; standard-runner minutes are free and the
runner is 4 vCPU / 16 GB (was 2 / 7). The sampling argument was sound and it
was never free; the thing it bought stopped having a price.

## Scope

**In:** the lane and the eps row. `LANE=both` and `EPS=all` on every run;
ci.yml fans the three eps rows out as matrix legs over each archive — twelve
`test (…)` jobs against today's two. `_forces_interval` (the
`interval-transcendentals/` lane pin) goes with the draw it pre-empted.

**Out, and stated rather than left to be noticed:**

* **The k-lint unification row is still drawn 1-in-5.** That dimension is
  five separate COMPILES of demos/tour and the kernel crates sharing almost
  no artifacts, not one archive replayed under a different env var, so the
  cost argument above does not carry over to it and it was not re-costed.
  Filed as `klint-row-still-sampled`.
* **The interval-only selection stays reverted.** Its 2026-08-22 reversal
  was forced by the lane draw; the draw is gone, so its original premise
  holds again — but restoring it would REDUCE what a run gates, which is the
  opposite direction from this unit. Filed as
  `interval-only-selection-premise-restored`.
* **Un-sampling does nothing about a composition no run ever compiles** (PR
  1796's subject), and nothing about a check that runs nowhere.

## The measured cost

Population: every `CI` run created 2026-09-04T04:00Z–07:52Z with
`conclusion` ∈ {success, failure}: 155 runs, 71 code-tier. Per-job wall time
from the jobs API; nothing is billed, the repository is public.

* **+15.4 job-minutes per code-tier run** (24.5 → ~40), derived by summing
  the configuration-dependent jobs at their medians: 1436 s un-sampled
  against 512 s expected over the 50/50 draw.
* **+283 job-minutes/hour** at this window's 18.4 code-tier runs/hour;
  **+161/hour** at PR 1796's 10.3/hour over a 14.45-hour window. The two
  lanes agree on the per-run figure (+15.4 against +15.6) and differ only in
  how busy an hour is.
* **Critical path +57 s on the half of runs that would have drawn `default`,
  +0 on the other half** — every run now takes the interval archive's path
  (368 s median against 278 s). The added eps legs cost ~0 wall: they start
  together behind an archive already being built.

The derivation, the population and the confound that rules out a
run-total comparison are in `docs/CI-MINUTES-2026-08.md`, in the block that
supersedes the 2026-08-22 sampling section.

## Territory

`scripts/ci-filter.py` is **S-TCOST's** (`work/tcost/program.md` `keep_out`),
not CIW's; CIW owns `.github/workflows/*`. Ev's authorisation covers the
change, and the PR announces the edit to S-TCOST by name rather than letting
the fence warning stand unexplained. No open S-TCOST item assumes the draw
exists; one PARKED item — `skip-eps-battery-by-observing-oncelock` — has its
premise RESTORED by this change and is worth more than it was.

## Exit

The workflow's own run shows twelve `test (…)` jobs across two lanes and
three eps rows, `ci-filter.py --selftest` asserts the two dimensions are not
sampled, and the prose that documented the sampling as live no longer does.
