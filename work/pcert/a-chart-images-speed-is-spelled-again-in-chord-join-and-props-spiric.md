---
id: a-chart-images-speed-is-spelled-again-in-chord-join-and-props-spiric
kind: issue
title: A chart image's speed bound is spelled again outside Pcurve::speed_bound: topo's chart_v_du and props::spiric's span_rates
status: open
opened: 2026-11-14
---

Filed by TESSLANE's `one-chord-rule-over-each-boundary-images-speed-bound`,
whose fix pass found it. Not scheduled.

## What

`geom_brep::Pcurve::speed_bound` (`pcurve_cache/speed.rs`) is a chart
image's per-axis speed bound, in `f64`. Two readers spell parts of it
again, at their own scalar:

- `topo::chord_join::chart_v_du` bounds a harmonic image's `v` variation
  as `(√(pa.y² + pb.y²) + |pl.y|)·Δt`, generic over the scalar, with a
  `PCERT_RADIAL_PAIR_MUTANT` site; its comment says it re-spells
  `geom_brep::harmonic` because those helpers are crate-private.
- `geom_brep::props::spiric::span_rates` bounds a spiric wall image's
  azimuth rate over a window as `√(S² − r²)/ρ_lo`, in interval
  arithmetic. The speed bound reads the same module's window rates per
  piece (`props::spiric::image_rates`), so the two share their pieces'
  reading but not their door.

## Fix

A speed bound at the scalar each reader needs (generic, or at
`Interval`), read by both; or a statement of why each must stay its own.
