---
id: the-touch-and-cross-doors-read-no-speed-for-a-spiric-or-spline-carrier
kind: issue
title: carrier_touch and carrier_cross take a carrier's speed from speed_at and speed_hi, which answer None for a spiric or spline, though speed_range now bounds every kind
status: closed
opened: 2026-10-13
closed: 2026-10-25
priority: P1
cost: E
---

Found by FUSE's review of PR 4946 (`census-curved-record-lanes-read-circles-only`, branch `fuse/census-reads-every-curved-edge`).

`crate::carrier_speed::speed_at` and `speed_hi` answer the line and
the conics only, `None` for a `Spiric` or `Nurbs` carrier; both now
derive from `speed_range`, which bounds every kind (a spiric's from
`geom::spiric_rate_bounds`, a spline's from its control net). Their
callers keep the `None` as a door that stays:

- `boolean::carrier_touch::speed_bound` — "a curve with no speed bound
  localizes nothing, and the door stays";
- `boolean::carrier_cross::Span::of` — a span with no closed-form speed
  has no reach.

Widening them to `speed_range` (and `accel_hi` for the bend) would let
those doors localize a spiric span, and a spline span where its bounds
are sound. Not done in PR 4946, which kept their behaviour unchanged.

## Moved to SPLICE (2026-10-25)

From CLEAVE (taken on request in chat; CLEAVE would not start a P3). Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. Re-banded P3 → P1: a link the spline operand's path runs through.

## Closed

`carrier_touch::off_face` takes the curve's speed from
`carrier_speed::speed_range` on every kind and its curvature from
`curvature_hi` where there is one: a spiric localizes on both bounds, a
spline on the first-order bound alone (no curvature bound here).
`reduce::wall_crossing` now routes a spiric or spline span, which no
root door reads, to that localization instead of answering
`Unsettled` before it. `speed_and_curvature_hi` is deleted (its `None`
arms are dead); `speed_at` and `speed_hi` stay, since their remaining
callers read only a line or a conic (`wall_crossing`'s root metering)
or need the `None` (`carrier_resolution::resolution`, which has no
honest rounding bound for a spiric or spline).

The `carrier_cross` half was already done on main:
`geom_brep::curve_pair::Span::new` reads `speed_range`.

Witnesses: `carrier_touch_rows::a_spiric_or_spline_touch_past_the_face_is_off_it`
and `reduce::edge_span_tests::a_spline_touch_past_the_face_is_off_it`.
Not reachable through the boolean today: the operand gate refuses
every spiric and spline edge before the sweep.
