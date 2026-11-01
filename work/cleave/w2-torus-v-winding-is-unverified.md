---
id: w2-torus-v-winding-is-unverified
kind: issue
title: W2's premise on a torus face periodic in v is unverified: chart_bound lifts u only, and nothing shows the walk's winding composes the v channel
status: closed
opened: 2026-10-24
priority: P3
cost: M
closed: 2026-10-24
---


## What

W2 (`boolean/section_cert.rs`, via `ChartCache::describes`) reads
`chart_boundary(..).is_ok()` as "every closed curve in the face's interior
lifts with zero winding". A torus is periodic in BOTH channels. One of CHART's
region designers (2026-10-24) noted that `chart_bound` never lifts `v`, and could
not tell whether the loop walk's `Winding` checks the `v` channel. If it does
not, a torus face whose loops wind once in `v` (a full meridional band) could pass
W2 with a section circle that wraps it, and the result would be a silent wrong
boolean.

**Unverified: measure first.** Build a torus face bounded by two `v`-wrapping
loops, and check whether `chart_boundary` refuses it. If it does not, find out
whether W2 clears a section that wraps the band. (CHART orchestrator)

## Closed

Closed 2026-10-24: checked, and not a hole. `ChartPeriods::of` answers a
`v` period on the sphere and the torus, the walk decides both channels, and
`chart_boundary` requires `Winding::is_zero`, the identity deck in `u` AND
`v`. A torus face winding in `v` refuses `LoopWraps`, and W2 reads that
refusal as "does not describe". (CHART orchestrator, on a designer's check)
