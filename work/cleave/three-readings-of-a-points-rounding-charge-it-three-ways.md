---
id: three-readings-of-a-points-rounding-charge-it-three-ways
kind: issue
title: A conic point's rounding is read three ways: carrier_resolution (relative to the window), implicit::sample_rounding and ring_path::resolution (absolute)
status: open
opened: 2026-10-14
priority: P3
cost: M
---


Found by the review of PR 4810 (Q1).

## What

Three functions bound the rounding of a point on a carrier, each its
own way:

- `geom_brep::carrier_resolution::resolution` (PR 4810): a section
  carrier's own resolution over a parameter window, charged relative to
  the window (`u·speed_hi·(32·reach + τ)`), leaving the coordinates'
  rounding at the model's extent to the band's `K` headroom (Q1). The
  join's pin reading admits on it.
- `geom_brep::implicit::sample_rounding`: one sampled residual's
  rounding over a whole conic, charged at `|C₀| + speed_hi`, the point's
  distance from the world origin.
- `topo::ring_path::resolution` (same name): `4ε` times the farthest
  stored point's distance from the origin, for a reading between points
  that should coincide.

They answer neighbouring questions: what a carrier amplifies, and what
coordinates resolve at the model's extent. But nothing says which a
reader wants, and two of them charge from the world origin.

## Owed

One home that names the two terms (the coordinates' floor, the carrier's
amplification) and lets each reader take the ones it needs. Rename
`ring_path::resolution` so one name means one quantity.
