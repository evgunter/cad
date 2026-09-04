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

* **+14.9 job-minutes on a TIER=closure run** (61 of 71) and **+18.9 on a
  TIER=all run** (10 of 71), derived by summing the configuration-dependent
  jobs at their per-tier medians. A pooled, tier-blind +15.4 was published
  first and is corrected here: this unit's own gate run (`33853141826`,
  TIER=all, green, twelve `test (…)` jobs) came in at **54.0 job-minutes
  against a TIER=all sampled median of 30.1 — +23.9**, above even the
  per-tier derivation, and n=1 against a median is not like-for-like but the
  derivation is a floor rather than a forecast. A second un-sampled run
  (`33854219517`, also green, also twelve legs) came in at 44.4 job-minutes,
  so the two bracket the derivation: +23.9 and +14.3, mean +19.1.
* **+283 job-minutes/hour** at this window's 18.4 code-tier runs/hour;
  **+161/hour** at PR 1796's 10.3/hour over a 14.45-hour window. The two
  lanes agree on the per-run figure (+15.4 against +15.6) and differ only in
  how busy an hour is.
* **Critical path**: every run now takes the interval archive's path (366 s
  median against 273 s scoped), so a run that would have drawn `default`
  gains the difference and one that would have drawn `interval` gains ~0. The
  added eps legs start together behind an archive already being built. The
  gate run's wall was **619 s against a TIER=all sampled median of 437 s
  (+182 s)** — again above the derivation, and again n=1.

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
