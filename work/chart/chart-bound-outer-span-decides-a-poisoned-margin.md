---
id: chart-bound-outer-span-decides-a-poisoned-margin
kind: issue
title: chart_bound_outer_span classifies a NaN margin on three shapes: 9 rule-1 k-lint flags per eps row
status: open
opened: 2026-09-29
priority: P1
cost: M
---

## What

`ChartBound::assembled` (`crates/topo/src/chart_bound.rs`, the `SPAN`
const) classifies the outer loop's chart-`u` excess over the chart
period:

```rust
decide(SPAN, u_arm.meter(hull.u_max - hull.u_min - p), band)
```

On three shapes that margin is **NaN**, so the classifier reports
`Invalid` and the K sweep records `outcome=invalid` with `margin=NaN`.
Nine such rows appear at **every** eps row — three shapes x three
samples each, identical at 1e-6, 1e-9 and 1e-12, which is what a
poisoned margin looks like: no threshold is involved.

From nightly run 36561506133 (head `aae5716bf`), job `k-lint
(dev-probe)` (job id 109383464088):

| shape | predicate | rows/eps | recorded margin |
| --- | --- | --: | --- |
| `corpus/boss_union` | `chart_bound_outer_span` | 3 | `NaN` |
| `demo/bossplate` | `chart_bound_outer_span` | 3 | `NaN` |
| `demo/lily_walls` | `chart_bound_outer_span` | 3 | `NaN` |

## Why this is a defect and not a stale threshold

k-lint's rule 1 is outcome-based and has no threshold to re-derive:
`Reason::Invalid` is "a poisoned one — a defect wherever it appears"
(`tools/k-lint/src/lib.rs`, `Reason::rule`), and the same doc records
that nothing offers a demotion for rule 1, because demoting it would
demote the ERROR-DESIGN E6 re-open trigger. So neither of
`docs/K-REPORT.md`'s two recourses reaches these nine.

## It is new, and the window is 19 hours

The last execution of `k-lint (dev-probe)` before the failure is run
36449836735 / job 109022798206, 2026-09-28T16:16:28Z. It read
**3,789,703 samples over the three rows and flagged rule 1 zero
times** — so on that tree the whole sweep contained no `invalid` row
at all. The failing run read 3,782,215 samples and flagged nine per
row. Between the two, ~85 pull requests merged to main (the window
runs from 2026-09-28T16:16:28Z to the nightly's head at
2026-09-29T11:24:07Z); this row does not name a culprit inside it.

## The design tension worth settling with it

`docs/predicate-dimension-audit.md`'s `chart_bound_outer_span` row says
the check is "Refusal-only and one-sided: a DEFINITE positive refuses
the description, `Zero`/in-band/poison let it stand." So `chart_bound`
tolerates a poisoned margin by design, while k-lint treats any
`invalid` outcome as a defect. Both cannot be right for this
predicate. Either the margin must never be poison here — the arm or
the hull that feeds it is the thing to fix — or the audit's tolerance
needs an argument that survives rule 1, which today it does not.

## Home

CHART — `crates/topo/src/chart_bound.rs` is chart's territory
(`scripts/work.py territory`). Found by the PROPS k-lint baseline unit,
which measured but does not fix kernel geometry.
