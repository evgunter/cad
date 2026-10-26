---
id: the-touch-and-cross-doors-read-no-speed-for-a-spiric-or-spline-carrier
kind: issue
title: carrier_touch and carrier_cross take a carrier's speed from speed_at and speed_hi, which answer None for a spiric or spline, though speed_range now bounds every kind
status: closed
opened: 2026-10-13
closed: 2026-10-25
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

## Closed

`carrier_touch::off_face` takes the curve's speed from
`carrier_speed::speed_range` and its curvature from `curvature_hi` on
every kind. `accel_hi` and `curvature_hi` now bound a spline too, from
`NurbsCurve3::accel_upper_bound` and its speed floor, and return a
value rather than an `Option`. The bound is not finite where the
spline is not `C¹` (a knot of multiplicity at least its degree:
`accel_upper_bound` reads `KnotStructure::crease_runs`), or where its
speed floor certifies nothing. The second-order bounds then clear
nothing. Review of PR 5715 found the crease: a cornered degree-1 net
had read curvature zero
(`carrier_touch_rows::a_spline_with_a_corner_crossing_the_face_is_not_off_it`,
`curve_pair::meet_rows::a_spline_with_a_corner_meets_a_line_past_it`). `curve_pair::walk` reads the same two bounds, so a spline pair
there now stops at its tangency floor instead of the band.
Without the spline curvature bound, the first-order walk ran out of
`PIECE_BUDGET` at ε = 1e-12 (CI on this PR's first head).
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
