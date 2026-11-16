---
id: a-turned-link-a-kilometre-off-reads-its-own-rays-in-band-on-interval
kind: issue
title: A turned link a kilometre off reads its own rays in band of its own planes on Interval at the finest band
status: open
opened: 2026-11-12
priority: P3
cost: M
---



Found by the valid-bodies lane (PR 6878) and its reviews. Priority P3:
the readings refuse `InBand`, which is sound, and only on `Interval` a
kilometre off at ε ≤ 1e-12; but it is why several unit rows cannot pin
a `Rest` on that leg.

## What

A tangent link whose sectors' planes are not the world's coordinate
planes, built about a point a kilometre off the origin, reads its own
rays in band of its own planes and wedges when the scalar is `Interval`
and the band is the finest (`CAD_TOLERANCE_EPS=1e-12`): the enclosure
of a ray's point, at coordinates of order 10³, is as wide as the band's
zero there, so `link_plane` and `link_wedge` read nothing
(`metric::sign_as` in `crates/topo/src/link.rs`). The link then refuses
`TouchUnread::InBand` before any reading one order down is taken.

The rows that meet it, each of which states it and works round it:

- `link::second_order_rows::turned` (its doc): the whole-plane rows are
  stated in a frame whose plane spans the apex's `y` and `z`, because
  spanning `x` and `y` "the plane's own cut rays read against its own
  wedges, on `Interval` at the finest band, in band (`link_wedge`)".
- `link::lens_rows::v` and `link::apex_rows::v` (their docs): the same
  choice of frame, for the same reason.
- `link::apex_rows::a_nappe_off_every_plane_is_the_thorn_or_needle_it_bounds`,
  "two whole turns": accepts `InBand` on that leg.
- `link::lens_rows::a_sheet_on_a_planar_face_along_a_line_is_the_void_its_bend_opens`
  (`turned_in_band`): a ridge's end turned about the floor's normal by
  0.3, 2 or 5 rad reads `InBand` there where it rests on every other
  leg, and so do four of its refusals (a ball's or a cylinder's tangent
  plane for the floor, a hollow ridge, a ball's octant). Unturned it
  reads exactly.

So on that leg a turned link's `Rest` is not witnessed by any unit row.
The real bodies' rows run in `f64`, where the reading is exact.

## The fix

Either the reading's own: read a link's rays about its point, in
coordinates relative to it, so an enclosure's width is the direction's
and not the position's. Or the rows': build the turned fixtures from
exactly representable directions, so the enclosures are points.
