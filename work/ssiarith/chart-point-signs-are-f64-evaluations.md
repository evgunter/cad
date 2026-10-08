---
id: chart-point-signs-are-f64-evaluations
kind: issue
title: the chart lane's point signs are f64 evaluations read as certified, so a zero within rounding of a window corner or cut can take a sign
status: open
opened: 2026-10-03
priority: P3
cost: M
---

## Found (limb-3 fix pass, branch `ssi/limb3-one-arc`, 2026-10-03; a review of PR 3999 named it)

`NurbsBoxes::rect_box` (`ssi/enclose.rs`) reads a rectangle's box as
`S(mid) ⊕ S_u·[−h_u, h_u] ⊕ S_v·[−h_v, h_v]`. Its midpoint term is
`eval_in_span` at the caller's scalar, crossed with
`Interval::from_certified`. On the `f64` lane that is a point evaluation
with its rounding dropped: over a degenerate rectangle (a point) the
"enclosure" of `φ` is the `f64` value, and a sign read off it is
certified only up to the evaluation's rounding.

Two readers take such point signs as certified:

- **The exhaustiveness sweep** (`exhaust::sweep_chart_plane`), on a cell
  that collapses toward a point. This is the convention in place before
  the one-arc proof.
- **Limb 3's one-arc walk** (`ssi/one_arc.rs`):
  - `edge_runs` reads a monotone run's end signs, and `boundary_zeros`
    reads the joint signs at cuts and corners.
  - `holds_zero` reads the two ends of its linking line.
  - A zero within rounding of one of those points can be read with a
    definite sign rather than as unknown. A run's count, or a joint's,
    would then be decided on rounding noise.

**The case that meets it:** the corner clip of
`m5_pr7_ssi::a_plane_clipping_a_walls_corner_traces_the_clip_however_short`.
- The pcurve is the 45° line `u + v = 0.2`, so its padded windows' anti-diagonal corners lie exactly on the locus.
- `φ` at those corners is zero up to rounding.
- The walk handles an unknown sign there (the joint rule), but nothing
  makes the sign unknown when the `f64` evaluation lands a few ulps off
  zero.

## What is open

Make the point evaluation an enclosure on the `f64` lane: evaluate at the
interval scalar, or widen by a certified rounding bound. Then a point's
sign is certified or unknown, never rounding noise. Both readers move
together.
