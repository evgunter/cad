---
id: check-7s-volume-reading-is-loose-over-a-box
kind: issue
title: Check 7's volume reading encloses a slab of volume [0.95, 1.05] as [-1.62, 4.13] over a ±0.05 box, and reads only below width 0.03
status: open
opened: 2026-11-14
priority: P1
cost: M
---


Found by BOXFINISH's M1 measurement (2026-11-14, main `b363f5fa`, the
switch's box arm off on `boxfinish/probe-switch-off`).

Where no aimed ray reads, check 7 reads a solid's side off its volume
(`positive_volume_exact`). Over the slab of depth 1 ± 0.05 (true volume
in [0.95, 1.05]) the reading is `Enclosure { -1.617, 4.129 }`, and over
the derived-frame cube `[-5.163, 15.214]`. It reads only at a box width
of 0.03 or less, and a finish that falls to it costs 254 to 787 ms at
`Sym<Interval>`, against 50 to 110 ms where the rays read. Where the
width enters is not traced.

It is not behind the switch's deletion: once the rays read
(`the-side-reader-cannot-read-a-solids-orientation-over-a-box`,
`the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box`)
the slab class does not reach it. It still holds every body the rays
cannot read: a selection holding a NURBS face casts no ray
(`cast_aims`' `uncrossable`), so a loft always goes to the volume.

## Re-banded P1 (2026-11-15, M4): it holds the switch's deletion

With the frames built (PR 6972) and the switch's box arm off, a slab
boxed wider than ± 0.215 of its depth still loses every aimed ray and
then the volume reading (`OrientationUndecided { reading: Volume {
Enclosure{-0.9699, 2.9699} }, aimed: 6, set_aside: 6 }` at depth
`[0.75, 1.25]`): 40 refusing finishes across the box studies, one
extra bisection level, and two red rows (m10_8's decision pin; m10_3's
`the_certification_width_is_no_longer_bounded_by_epsilon`, which
settles at a half-width of 0.215 where it wants about 1). The stored
normals are exact now; per ray the refusals are the caps'
`side_at_infinity_winding` (`{-3.827, 13.758}`, `{-1.412, 8.154}`,
raised by `turned_inside` / `crosses_by_turn`), `side_at_infinity_edge`
(`{0, 0.177}` to `{0, 1.227}`) and `side_at_infinity_advance`
(`{-1.199, 0.927}`). `delete-the-box-scalar-sweep-leaf-switch` is
parked on this row, so it takes that row's band.
