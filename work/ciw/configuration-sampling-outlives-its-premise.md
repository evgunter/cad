---
id: configuration-sampling-outlives-its-premise
kind: unit
title: The lane/eps draw was bought with billed minutes: price un-sampling, and say what the red record can and cannot attribute to it
status: open
opened: 2026-09-04
---


Opened 2026-09-04 by CIW unit 8's lane, out of Ev's comment on PR 1796:

> i think an important missing piece of this puzzle is that CI is
> weakened right now because of sampling only certain configurations to
> run; i'm not sure how much of the reds on main are from that vs
> conflicts but undoing that sampling now that actions minutes are much
> cheaper is probably a good idea regardless of if we still need the fix
> described in this pr

and Ev's authorisation in chat the same day: *"feel free to reinstate
full runs instead of sampling"*. **The change is authorised; this file
carries the measurement and the fences, and a separate lane owns the
edit.**

## The premise that died

Configuration sampling landed 2026-08-22 (`docs/CI-MINUTES-2026-08.md`,
*configuration sampling, and the draft skip*). A code-tier run used to
execute every point of {default features, `interval`} × {default eps,
1e-6, 1e-12}; it now gates **one**, drawn from the head SHA. The saving
it booked was **billed minutes**, and the repository has been public
since 2026-09-03, so standard-runner minutes are not billed.

## The price of un-sampling, measured (unit 8's reading, 2026-09-04)

Population: 220 completed `pull_request` runs (conclusion success or
failure; cancelled excluded), `2026-09-03T15:20Z`–`2026-09-04T05:47Z`,
149 code-tier.

**The shape first, because it is not a 6× multiplier.** `eps` is read at
runtime (`geom-core/src/tolerance.rs`) and only `interval` is a distinct
build graph, so six points are **two builds and twelve test jobs**
(ci.yml's *BUILD ONCE PER COMPILE MODE*). Measured medians:

| row | median | n |
|---|---|---|
| `build + archive (default)` | 309 s | 66 |
| `build + archive (interval)` | 369 s | 96 |
| one `test (…)` job, default lane | **46 s** | 128 |
| one `test (…)` job, interval lane | **58 s** | 186 |
| `clippy` | 66 s | 66 |
| `clippy + doc-tests (interval)` | 114 s | 96 |

* today, one gated point: 344 + 106 + 94 ≈ **544 job-seconds**
  (lanes weighted by their observed 59/41 split);
* un-sampled: 678 + 624 + 180 = **1482 job-seconds**;
* **Δ ≈ +938 job-seconds = +15.6 job-minutes per code-tier run**, taking
  the median run from **24.4 to ~40 job-minutes**;
* at **10.3 code-tier PR runs/h**, **+161 job-minutes an hour ≈ +2.7
  mean concurrent jobs**, against a measured mean of 4.3, p90 12, peak
  36, and a queue delay of 3 s median / 25 s p99;
* **wall clock ≈ +22 s on the median run (~5 %)**: un-sampling does not
  lengthen the critical path, it makes the *slower* lane mandatory —
  interval-lane runs already end their last test row at a median 464 s
  against the default lane's 385 s, and 442 s is today's mixed median.

**It is affordable.** Roughly 3× the runner load of restoring the push
gate and a twentieth of its latency cost.

## What the red record can and cannot attribute to the draw

Five recorded main-red instances (the enumeration and its search method
are in `f3-recosting-on-a-public-repo`): two compositions, one
tier-scope, one **`k-lint` row sampler**, one configuration with no CI
row at all. **Zero are attributable to the lane/eps draw**, and both
compositions are point-independent, so un-sampling would not have caught
either any sooner.

**That is not an argument against un-sampling and must not be quoted as
one.** A defect at a point nothing drew is invisible until something
draws it, so the recorded population is biased against exactly the class
in question. The record cannot answer Ev's "how much of the reds on main
are from that vs conflicts"; the reason it cannot is the reason the draw
is worth undoing. The ground for the change is the price above and the
exposure below, not the red history.

**Exposure**: a merge is gated at one of six points, and each point
gates between 9 % and 31 % of code-tier runs (interval/1e-12 31 %,
default/1e-12 22 %, interval/default 15 %, interval/1e-6 13 %,
default/default 10 %, default/1e-6 9 %; lane marginal 59 % interval).
Those are gating frequencies, not a claim about the hash — a lane can
also be *asked for* by trailer, and `CONFIG_SOURCE` separates drawn from
requested in a run's log, which this reading did not open.

## Fences

- **The edit is not this file's.** A lane is dispatched to reinstate
  full runs; this item is the numbers and the argument.
- **`scripts/ci-filter.py` is S-TCOST's territory**
  (`work/tcost/program.md` `paths:`), and the draw lives there
  (CONFIGURATION SAMPLING). CIW owns `.github/workflows/*` and the CI
  posture question. Any edit to the filter is S-TCOST's to make or to be
  announced to them.
- **`k-lint`'s 1-of-5 unification sampler is a SECOND, independent
  sampler** and is not in the price above. It is also the only sampler
  with a recorded red to its name
  (`work/m10/probe-census-red-interval-cfg-gate`: *"k-lint samples 1-of-5
  rows per run, which is why it surfaces intermittently"*). Its job
  median is 127 s with a p75 of 347 s and a max of 1039 s, so
  un-sampling it is a materially different sum and wants its own
  reading. Do not fold it in silently.
- **Downstream, and worth landing together**: branch protection requires
  status checks *by name*, and the sampled matrix's names are computed
  (`test (eps = ${{ needs.filter.outputs.eps }}, 1/2)`). Un-sampling
  makes those names constant, which is a precondition for the merge
  queue that `f3-recosting-on-a-public-repo` now recommends.
