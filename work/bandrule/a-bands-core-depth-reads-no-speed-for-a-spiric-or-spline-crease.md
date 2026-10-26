---
id: a-bands-core-depth-reads-no-speed-for-a-spiric-or-spline-crease
kind: issue
title: blend::reach's core depth reads a crease edge's speed from speed_hi, which answers None for a spiric or spline, though speed_range bounds every kind
status: closed
opened: 2026-10-25
priority: P3
cost: E
closed: 2026-10-26
---

Found by SPLICE's sweep in the PR that widened `carrier_touch` to
`speed_range` (branch `splice/touch-doors`).

`sweep::blend::reach::band_reach`'s `depth_in` closure reads a
carrier's speed bound from `geom::carrier_speed::speed_hi`, which
answers the line and the conics alone, so a spiric or spline crease
edge reads no core depth and is "not certified" (its comment: "a
carrier with a speed bound (a line, a circle, an ellipse) is read
piecewise; any other is not certified"). `curve_in_core` needs only
an upper bound on `|C′|` over the span, and
`geom::carrier_speed::speed_range(c).1` gives one on every kind (a
spiric's from `spiric_period_rate_bounds`, a spline's from its control
net, certified at `Interval`). Widening the read would let a band's
cup over a spiric or spline crease read its core depth.

Unmeasured: no fixture in the tree builds a band whose crease is a
spiric or spline edge. Not changed in SPLICE's PR, which kept to the
boolean's doors.

## Closed (2026-10-26, easy-fixes lane)

`crates/sweep/src/blend/reach.rs` `band_reach`'s `depth_in` closure now reads `geom::carrier_speed::speed_range(c).1.get()`, an upper bound on every kind. A spiric or spline crease therefore reads its core depth through `curve_in_core`. A poisoned sup cannot certify there: a NaN piece radius never passes `sure` and runs out `EDGE_PIECES` to `None`. On the line and the conics the sup is the same value `speed_hi` gave. Still unmeasured on a spiric or spline crease, because no fixture builds one (the row's own note). Verified: `cargo nextest run -p sweep --test all -E 'test(/band|reach|prism|leaning/)'`, 512 passed.
