---
id: derivative-bernstein-row-over-a-sub-interval-has-three-homes
kind: issue
title: The Bernstein row of a derivative over a sub-interval is spelled three times: SpanRow::restrict_derivative, the SSI section's slope_over, and derivative_net / Grid::derived
status: open
opened: 2026-11-01
priority: P3
cost: M
---


## What was found (SLIVER's review, 2026-11-01)

One mathematical object, the Bernstein row of a piece's first
derivative over a sub-interval of its span, has three spellings:

- `SpanRow::restrict_derivative` (`crates/geom-core/src/spline/blossom.rs`),
  which differences the blossom across the span's knots. It is read by
  `NurbsSurface::rect_jet` and `NurbsCurve3::span_deriv`.
- `slope_over` (`crates/geom-brep/src/ssi/section.rs`), which forms the
  segment's hodograph and then `restrict_segment`s it.
- `derivative_net` / `Grid::derived`
  (`crates/geom-core/src/spline/derivative.rs`), which difference a net
  scaled by `span_derivative_scale` over the grid's stated ends.

## What a fix needs

One home, chosen by width on the callers' real inputs (see
`clear/restrict-derivative-widens-on-a-few-ulp-knot-span`), with the
others routed through it.
