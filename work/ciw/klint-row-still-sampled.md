---
id: klint-row-still-sampled
kind: issue
title: The k-lint unification row is still drawn 1-in-5 after the lane/eps un-sampling
status: open
opened: 2026-09-04
parent: reinstate-full-configuration-runs
---

## The finding

`reinstate-full-configuration-runs` un-samples the lane and the eps row on
Ev's 2026-09-04 authorisation. It leaves the **third** sampled dimension
alone: `k-lint (gate)`'s five feature unifications
(`scripts/ci-filter.py`, `KLINT_ROWS`) are still drawn one per run from the
head SHA, salted independently, pinned only for a `tools/` diff.

**Disclosed here rather than only in a PR body**, because "the hosted gate
samples the configuration matrix" is now true of exactly one dimension and
a reader who stops at the un-sampling will not know that.

## Why it was not un-sampled in the same unit

The cost shape is different, and that is the whole of it. The eps rows are
one archive replayed under a different `CAD_TOLERANCE_EPS` — three legs, no
extra compile. The five k-lint rows are five COMPILES of demos/tour and the
kernel crates that share almost no artifacts: `--release` and dev are
different profiles, and `budget` and `probe` are opt-in features gated at a
module boundary, so each is its own fingerprint for every crate that sees it
(the argument is written out at `KLINT_ROWS`). The measurement that priced
un-sampling — `docs/CI-MINUTES-2026-08.md`, 2026-09-04 — covered the lane and
eps and did NOT cover this, and Ev's authorisation was given against the
lane/eps reasoning.

Median `k-lint (gate)` job over the same window: 120 s (n=71). Running all
five is not five times that (the cache lane is keyed per profile and some
steps are shared), which is exactly why it needs measuring rather than
assuming.

## What it costs today, stated because two ratified review outcomes named
## these rows as unconditional

MIN-1's per-triangle certificate falsifier rides `dev-budget`;
`crates/sweep/tests/k_report.rs` and docs/K-REPORT.md's "on every building
merge" ride `dev-probe`. Both are 1-in-5, both sites say so, and both remain
so after the lane/eps change.

## The disposition this wants

Measure the five rows on the 4-vCPU runner, then either un-sample (if the
cost is of the same order as the lane/eps change) or record why the k-lint
dimension stays sampled where the other two did not. It is a measurement
first and a decision second, which is why it is an issue and not a unit.

## No longer a cost-shape deferral: the remaining sampled row has a measured miss (2026-09-04)

Filed from PR 1805 (code-quality Track T), folded into
`work/ciw/f3-recosting-on-a-public-repo` when that PR was closed as
superseded.

**`#1756 → #1775`: `k-lint (gate)` reported green with `demos tour fmt +
clippy` skipped**, because the drawn row did not carry it. A real failure
sat behind a green row name, found later and repaired by a separate PR.

That changes what this item is. When it was written, k-lint was scoped
out of `reinstate-full-configuration-runs` on **cost shape** — five
feature unifications compiled, against one archive replayed for the
lane/ε rows — with no known cost to leaving it sampled. There is now a
known cost, on the record, on the only dimension the hosted gate still
samples.

It also means the sentence "the hosted gate samples the matrix" is still
true of this repository, and every reader who learns from PR 1823 that
sampling is gone will be wrong about this row.

What is owed is the same measurement PR 1823 made for lane/ε: what do
five unifications cost on the 4-vCPU public runner, in job-minutes and in
critical-path wall clock, against a `k-lint` row that currently draws one.
The answer may still be that sampling is right here — the cost shape
argument was not wrong, it was just uncontested. It is contested now.
