---
id: notched-half-donut-owes-its-notch-and-volume-when-torus-plane-lands
kind: issue
title: When the torus x plane boolean is admitted, the notched half donut owes a point_in_solid Out at its notch and a volume of 3pi^2/8
status: open
opened: 2026-10-08
priority: P3
cost: E
blocked_on: [point-in-solid-refuses-an-l-shaped-torus-face]
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

## The volume landed; the notch waits on containment (GERM U-T3, 2026-10-12)

The plane × torus chord for the meridian and concentric circles
(`germ/plane-torus-chord-circles`) builds the notched half donut. The
row is rewritten as `a_notched_half_donut_answers_its_volume`: the
result is `3π²/8` and valid at tier 3. `point_in_solid` on it refuses
`PartialTorusFace` at every point, the notch included, because both
torus walls are L-shaped in the chart, which `torus_chart_trim` reads in
neither of its classes; the row pins that refusal. The notch's `Out` is
re-parked on that containment arm,
`work/inside/point-in-solid-refuses-an-l-shaped-torus-face.md`.

## Fired: the notch reads (RESTREAD, 2026-10-18)

`restread/torus-region` closed the containment arm this row is parked
on (`point-in-solid-refuses-an-l-shaped-torus-face`): a torus face is
read from its boundary edges (`boolean::torus_region`).
`contact11_torus_chart_l.rs` `a_notched_half_donut_answers_its_volume`
now asserts `Out` at the notch `(√2, 0.25, −√2)`, `In` under it and on
the other quarter's top, `Out` past the half turn, and the analytic
membership at a 400-point lattice over the tube, beside the volume
`3π²/8` the row already held. Both obligations above are met. Set `open`,
its trigger having fired; closing it is CONTACTHOLD's.
