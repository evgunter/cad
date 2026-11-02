---
id: restrict-derivative-widens-on-a-few-ulp-knot-span
kind: issue
title: On a knot span a few ulps wide, SpanRow::restrict_derivative is up to 1.85x wider than restrict-then-difference; derivative_net first, then restrict, may condition better
status: open
opened: 2026-11-01
priority: P3
cost: M
---


## What was found (SLIVER's review, 2026-11-01)

`SpanRow::restrict_derivative` (`crates/geom-core/src/spline/blossom.rs`)
differences the blossom across the span's own knots `U_s` and
`U_{s+1}`. That is well conditioned when the span is wide, and only as
well conditioned as `U_{s+1} − U_s` when the knot vector itself holds
two values a few ulps apart. SLIVER's reviewer measured the new door up
to 1.85× wider than the restrict-then-difference it replaced on such a
span. No fixture has one: the arc-spine and S-duct knots are spans of
about 1/16 and 1/12.

## A fourth shape to measure

Take the span's derivative net first (`derivative_net`, or the Bézier
segment's hodograph as the SSI section's `slope_over` does, in
`crates/geom-brep/src/ssi/section.rs`), then restrict it. Measure it
against both on few-ulp knot spans, and keep whichever is no wider on
either input class.
