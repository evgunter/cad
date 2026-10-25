---
id: the-touch-and-cross-doors-read-no-speed-for-a-spiric-or-spline-carrier
kind: issue
title: carrier_touch and carrier_cross take a carrier's speed from speed_at and speed_hi, which answer None for a spiric or spline, though speed_range now bounds every kind
status: dispatched
opened: 2026-10-13
priority: P1
cost: E
branch: splice/touch-doors
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
