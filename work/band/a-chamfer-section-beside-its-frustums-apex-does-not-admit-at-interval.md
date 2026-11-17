---
id: a-chamfer-section-beside-its-frustums-apex-does-not-admit-at-interval
kind: issue
title: A chamfer section beside its frustum's apex does not admit at Interval
status: open
opened: 2026-11-17
priority: P3
cost: M
---

Found by the band-stripfru lane's fix pass (PR 7050), following a reviewer probe.

A rounded rectangle (`[−1, 1] × [−0.6, 0.6]`, corner radius `R = 0.3`, 1 thick), every edge chamfered at a setback `s = R − gap` just under the arcs' radius. Each frustum's plane trim is then a circle of radius `gap` about the arc's axis, and the generator at each junction ends `gap·√2` from the cone's apex.

Measured with `Interval` setbacks (`crates/sweep/tests/band_closed_mixed_chain_shapes.rs::rounded_outlines_at_their_limits_chamfer_whole`, and a local probe over the gaps):

- ε = 1e-9:
  - gaps `1e-2` and `1e-3` build;
  - gaps `1e-4` down to `1e-7` refuse `UnsupportedGeometry` "a junction's section does not certify as the shared row of its two bands".
- ε = 1e-6:
  - gaps `1e-2` to `1e-4` build;
  - gaps `1e-5` and below refuse at the clearance screen (its margin is the gap, in band).
- ε = 1e-12: every gap refuses at the section, as every chamfer section does there (`every-mixed-rims-section-escalates-its-touch-reading-at-interval-at-eps-1e-12`).
- `f64`: builds at gap `1e-7` at ε = 1e-9, and refuses at the section at ε = 1e-12.

The geometry is exact: the generator stays on one nappe, clear of the apex by `gap·√2`. Why the admission stops where it does is not measured. Two things change as the gap shrinks: the cone's apex-relative chart image of the generator, and the row's lever.

The row pins today's behaviour both ways; closing this row flips the Interval half of that pin.
