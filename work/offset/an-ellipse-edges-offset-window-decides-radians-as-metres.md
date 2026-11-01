---
id: an-ellipse-edges-offset-window-decides-radians-as-metres
kind: issue
title: offset_axial's forward window and carried-read anchor lever circle and spiric parameter gaps by their speed, but decide an ellipse's unlevered: radians read as metres
status: open
opened: 2026-10-13
priority: P3
cost: E
---

Found by FUSE (branch `fuse/the-localization-core-2`) while folding
`offset_axial::param_speeds` onto `carrier_speed::speed_range`.

`param_speeds` returns speeds for a `Circle` and a `Spiric` only. Its
two readers fall back to an unlevered margin otherwise:

- `forward_window`: `Margin::levered(span, floor)`, else `Margin::of(span)`;
- the carried-read anchor before it: `offset_axial_edge_carried`, else
  the read stands.

An `Ellipse` edge's parameter is an angle, like a circle's, so on that
path its window `t1 − t0` decides as a length in metres. That breaks
D4's dimension rule (`docs/predicate-dimension-audit.md`). On a large
ellipse a span forward by a few band widths of metres reads as within
the band, and on a small one the reverse.

**Reachability is unverified.** `normal_of` in the rim reauthoring
accepts an ellipse, so an ellipse rim (a cylinder cut by a tilted plane)
looks movable. No row exercises it. The fix is probably to answer
`speed_range`'s bounds for the ellipse too, and the `Conic` floor and
ceiling are already there. The turn-keeping reading `keep_turn` would
then apply to it as well, which is the part to check.
