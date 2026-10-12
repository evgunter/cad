---
id: an-ellipse-arcs-span-is-levered-by-its-global-speed-floor
kind: issue
title: An ellipse arc's span check is levered by the ellipse's global speed floor (its minor semi-axis), so a 24-band arc on a steep flank reads 3.4 bands and escalates
status: open
opened: 2026-10-12
---


Found by FLUSH's two-pinch proof lane (PR 4769). A draft of that PR
let `boolean::join::find_match` take an in-band order between mutual
pairs on conic germs. With that, the `k = 10` pose of
`crates/sweep/tests/band_apart_partners_on_a_steep_ellipse.rs` got past
the pair order and refused at the Euler layer's certification, in
every op, at ε 1e-9, 1e-6 and 1e-12 alike:

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

The PR as merged frees the order only for straight germs, so the row
stops at `bool_join_nearest` again
(`work/flush/conic-germs-pair-order-is-unargued.md`). This finding has
no shipped reproducer until that row lands. The arithmetic below
stands on its own.

The bound is sound: it is a lower bound on the arc length. It is
loose by the ratio of the speed over the span to the global floor,
which is up to the aspect `k`. Any short arc on a steep ellipse's flank
pays it.

## Done when

An ellipse arc's span is metered at a speed floor over the span itself:
the smaller of the speeds at its ends, or the curve's minimum if the
span passes a major vertex. The steep-ellipse row's `k = 10` pose then
reaches its next door, once the conic pair order is free too.
Re-pin it in `band_apart_partners_on_a_steep_ellipse.rs`
(`stops_where_expected`). The floor must stay a lower bound in `f64`:
an end speed read in floating point can round above the true
infimum, so take it with a downward margin of a few ulps, or from an
enclosure.
