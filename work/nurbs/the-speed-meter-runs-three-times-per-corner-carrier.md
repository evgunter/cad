---
id: the-speed-meter-runs-three-times-per-corner-carrier
kind: issue
title: The integral speed meter runs about 3× per corner carrier per build; the piece assembly makes an Interval loft build 3.7–5.5× main
status: open
opened: 2026-10-10
priority: P3
cost: E
---

Measured by PR 4540's fix pass, at head `1dbd945611`.

`NurbsCurve*::speed_lower_bound` now always runs the piece assembly. It has to: `Real` is comparison-free, so a skip gated on the value cannot be written. That makes an `Interval` loft build slower than main:

| loft | main | head |
|---|---|---|
| 40 sections, degree 3 | 4.5 ms | 16.6 ms |
| 24 sections, degree 5 | 3.6 ms | 20.0 ms |

The meter is called 12 times per build: each corner carrier is read three times, by certify's span arm, by `topo::split` and by `param_rate`. Memoising the meter per carrier would cut that by about 3×.

The alternative is `INTEGRAL_METER_SPLITS` = 8, at half the cost and half the reach. The orchestrator prefers memoising, since it costs no reach.

Measure first: decide whether a cache keyed on the carrier fits the functional style, or whether threading the meter's value through the three callers is cleaner.
