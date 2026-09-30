---
id: klint-dev-probe-fires-after-going-dark
kind: issue
title: k-lint (dev-probe) fires 35 flags on its first nightly run after moving off the per-PR gate
status: open
opened: 2026-09-29
priority: P0
cost: M
---

**A red nightly is a red main, and the orchestrator owns it**
(`docs/prompts/implementer-discipline.md` §2). This is that row.

## What fired

Nightly run 36561506133 (2026-09-29, head `aae5716bf`), job
`k-lint (dev-probe)`, exit 2:

```
k-lint: GATE FAILED — the margin distribution changed: 35 margin(s) crowd a decision
             boundary that the committed baseline says should be empty.
```

Per-row, from the job log:

| eps row | samples | rule 1 (undecided/invalid) | rule 2 (near a threshold) | rule 3 (below a floor) |
| --- | --- | --- | --- | --- |
| `k-eps-1e-6.csv` | 1,260,729 | 9 | 0 | 0 |
| `k-eps-1e-9.csv` | 1,260,737 | 9 | 0 | 0 |
| `k-eps-1e-12.csv` | 1,260,749 | 9 | 8 | 0 |
| TOTAL | 3,782,215 | 27 | 8 | 0 |

Every sample is `classified` (0 `symbolic_zero`, 0 `sign_gated`, 0
`registered`).

**The shape is informative and should steer the measurement.** Rule 1
is **9 at every eps row** — eps-INDEPENDENT, so those nine are not
crowding a threshold, they are margins that came back undecided or
invalid. Rule 2's eight appear **only at 1e-12**, which is the
eps-coupled behaviour rule 2 is for.

## The instrument had gone dark, which is the other half of the story

- `dev-probe` lived in **`ci.yml`** (the per-PR gate) until the CI cut
  `49d5b2aee` ("ci: cut the per-PR gate for latency; move the rest to
  nightly", 2026-09-28) moved it to `nightly.yml`.
- The nightly runs of **2026-09-22 through 2026-09-28 contain ZERO
  `k-lint` jobs**; the 2026-09-29 run has five, of which `dev-probe` is
  the one that failed.
- **No main commit between 2026-09-25 and 2026-09-28 carries a
  `dev-probe` check run at all** (scanned via the commits' check-runs
  API), so it was not running on merges either.

So this is not "the distribution moved last night". It is an instrument
that had not run over main for an unknown window, running again and
firing. The window is the first thing to measure: **when did
`dev-probe` last actually execute and pass**, and what merged since.

The baseline constants themselves are not ancient — `BASELINE_FLOOR_MARGIN`
and friends were last touched at `c3012ed35` (2026-09-16, instr u12).

## What is NOT established, and must not be asserted

A candidate mover is **ENCL's PR 3418** (merged 2026-09-29, seam-noted
on this slate): `Decide::sign_within` now returns `Decided { sign,
margin }` on **every** outcome, `MarginDiag` became opaque, and
`terminal_sliver` is decided at classify time. If more outcomes now
carry a reporting margin, the probe CSV can record margins it never
recorded before — and newly-visible `Invalid` margins would land in
rule 1 exactly as observed, eps-independently, with no geometry having
moved.

**That is a hypothesis with a mechanism, not a finding.** It fits the
eps-independence of rule 1 and nothing else has been checked. It could
equally be that the sample population is unchanged and nine real
undecided margins appeared. Measure it; do not inherit it.

## The one forbidden move

From the lint's own output, and `implementer-discipline` §3: **do not
change geometry to get under the threshold.** A fired lint is evidence
about the margin distribution. The recourses, in the lint's order, are
to re-derive the baseline and thresholds per `docs/K-REPORT.md`'s "M7
addendum (2026-08-07): the large-K lint's floor refresh", or — if
re-derivation is not warranted — to demote the row to advisory with a
recorded justification.

Note the lint's recourse text still names `ci.yml + local-scripts/ci-local.sh`
as the pair that must not drift; after `49d5b2aee` the hosted row lives
in `nightly.yml`, so that sentence is stale and is part of what this
row fixes.

## Refs

Nightly run 36561506133. CI cut `49d5b2aee`. Baseline constants at
`c3012ed35`. Runbook: `docs/K-REPORT.md`.
