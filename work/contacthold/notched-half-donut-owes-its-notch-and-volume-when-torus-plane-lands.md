---
id: notched-half-donut-owes-its-notch-and-volume-when-torus-plane-lands
kind: issue
title: When the torus x plane boolean is admitted, the notched half donut owes a point_in_solid Out at its notch and a volume of 3pi^2/8
status: parked
opened: 2026-10-08
priority: P3
cost: E
blocked_on: [c5-plane-torus-cone-cylinder-arms]
---

Found by CONTACT-11's review (Style 6).

`crates/sweep/tests/contact11_torus_chart_l.rs` cuts a half donut
(`R = 2`, `r = 1/2`, revolved by π about `y`) with the bar
`[0, 3] × [0, 1] × [−3, 3]`. Both torus walls are left L-shaped in the
chart. Today `topo::subtract` refuses the pair with
`GermFrameUnsupported { a_kind: Torus, b_kind: Plane }`, and the row
pins that refusal.

**When the plane × torus arm lands** (the trigger,
`c5-plane-torus-cone-cylinder-arms`), the pin goes red. The row then
needs to assert:

- `point_in_solid` answers `Out` at a point in the notch: inside the
  removed tube, `x > 0`, `y > 0`;
- the result's volume is `3π²/8`, the half donut's `π²/2` less the
  quarter-turn upper half tube's `π²/8`.

The L faces' containment is guarded by `solid_contain::chart_polygon_box`
(CONTACT-11), so a wrong `In` here would be a regression of that check.
