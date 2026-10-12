---
id: an-ellipse-arcs-span-is-levered-by-its-global-speed-floor
kind: issue
title: An ellipse arc's span check is levered by the ellipse's global speed floor (its minor semi-axis), so a 24-band arc on a steep flank reads 3.4 bands and escalates
status: open
opened: 2026-10-12
---


Found by FLUSH's two-pinch proof lane (PR 4769). That PR stops
`boolean::join::find_match` from refusing on a pair order that cannot
change the matching. With that refusal gone, the `k = 10` pose of
`crates/sweep/tests/band_apart_partners_on_a_steep_ellipse.rs` reaches
the Euler layer's certification and refuses there, in every op, at
ε 1e-9, 1e-6 and 1e-12 alike:

```
Join(Euler(Certification { error: Escalated { check: ParamSpan, ..
  margin: Value(3.394113434751489e-9), band: (1e-9, 1e-8),
  predicate: Some("interval_span_forward") } }))
```

`geom_brep::certify`'s check 2 (`span_at_floor`, in `certify.rs`) meters
an ellipse arc's parameter span at the ellipse's smaller semi-axis:
`Curve3::Ellipse { major, minor, .. } => span_at_floor(major.abs().min(*minor))`.
That is the curve's global speed floor. The arc here is the section
arc across the pose's finger, at `x = k/√2` on an ellipse of semi-axes
`k = 10` and `1`. Its span is `24ε / (k sin φ)` = 3.39ε radians, and
its speed there is `√(k² sin² φ + cos² φ)` ≈ 7.1. So the arc is 24ε
long, but the margin reads 3.39ε × 1 and lands in the band.

The bound is sound: it is a lower bound on the arc length. It is
loose by the ratio of the speed over the span to the global floor,
which is up to the aspect `k`. Any short arc on a steep ellipse's flank
pays it.

## Done when

An ellipse arc's span is metered at a speed floor over the span itself:
the smaller of the speeds at its ends, or the curve's minimum if the
span passes a major vertex. The steep-ellipse row's `k = 10` pose then
reaches its next door. Re-pin it in
`band_apart_partners_on_a_steep_ellipse.rs`
(`stops_where_expected`), which names this file.
