---
id: quad-lanes-length-bound-charges-an-edge-for-its-whole-carriers-fastest-stretch
kind: issue
title: quad_lane's length_bound charges an ellipse and a spline edge for the whole carrier's fastest stretch
status: open
opened: 2026-11-16
priority: P3
cost: S
---


Filed by the `sectarms/planar-spline-winding` fix pass (PR 7028), from
its sweep for the class it fixed in the planar winding sum.

## What

`length_bound` in `crates/topo/src/props/quad_lane.rs` is the
quadrature lane's own spelling of an edge's length bound, read as the
extent gate's perimeter lever. Two of its arms charge the edge for
speed it does not run at:

- an ellipse arc is `speed_hi × span`, the larger semi-axis times the
  span, which over-states an arc about the end of the long axis by up
  to `a/b`;
- a B-spline rim is `speed_upper_bound() × span`, the sup over the
  whole carrier, so one fast knot span elsewhere inflates every edge on
  the carrier.

The planar winding sum had the same two arms
(`geom_brep::props::closed_segment_term`, `SegmentQ`). Measured there
before its fix: 85× the length on a non-rational cubic with one fast
span, 3100× on a rational cubic with weights over three decades, 87× on
a 100:1 ellipse's slow end, and a real region 21 µm to 5 mm wide
decided `Zero` at ε = 1e-6. The spiric arm of both was already
piecewise
(`work/germ/the-spiric-winding-lever-decides-real-pinched-loops-zero.md`).

Not measured here: what the extent gate does with an over-large
perimeter, so whether this is a wrong answer or only a refusal.

## Done when

`length_bound`'s conic and spline arms read
`geom::carrier_speed::length_hi` over the edge's own interval (the home
the winding sum's lever now reads), or the gate is shown to read the
lever one-sidedly and the doc says which side.
