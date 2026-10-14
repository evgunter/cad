---
id: param-speeds-is-a-second-per-kind-speed-list
kind: issue
title: offset_axial::param_speeds is a per-kind speed list beside crate::carrier_speed, and calls itself the one spelling
status: closed
branch: fuse/the-localization-core-2
pr: 5030
closed: 2026-10-13
opened: 2026-10-12
priority: P3
cost: E
---



Found by the review of PR 4782 (`cleave/one-carrier-reads-ellipses`).
That PR moved the boolean's per-kind speed into the neutral
`crates/topo/src/carrier_speed.rs` (`speed_at` and `speed_hi`), now
read by `carrier_cross`, `reduce::wall_crossing` and
`carrier_touch::speed_bound`. It left this copy for its owner.

## What

`offset_axial::param_speeds` is documented as "the one spelling of the
parameter-to-metre lever per kind". It gives a floor and a ceiling for
a circle (the radius, both ways) and a spiric (`geom::spiric_rate_bounds`),
and `None` for a line or an ellipse. That makes it a second per-kind
speed list beside `carrier_speed`, which reads lines, circles and
ellipses but not spirics.

## Direction

Fold the two together in one home:
- give `carrier_speed` a floor (an ellipse's smaller semi-axis) and
  the spiric's bounds;
- have `param_speeds` read it, keeping its own `None` where its
  periodic arithmetic has no line or ellipse arm.

## 2026-10-13, closed: PR 5030

`param_speeds` now reads `crate::carrier_speed::speed_range`, which
bounds every kind, the spiric included. Its doc no longer claims the
spelling. It keeps its own `None` for a line, an ellipse and a spline.

The ellipse half of that `None` is a dimension gap rather than a choice:
an ellipse's window decides as metres. That is filed as
`work/offset/an-ellipse-edges-offset-window-decides-radians-as-metres.md`.
