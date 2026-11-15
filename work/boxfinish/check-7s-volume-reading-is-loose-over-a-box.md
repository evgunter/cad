---
id: check-7s-volume-reading-is-loose-over-a-box
kind: issue
title: Check 7's volume reading encloses a slab of volume [0.95, 1.05] as [-1.62, 4.13] over a ±0.05 box, and reads only below width 0.03
status: open
opened: 2026-11-14
priority: P2
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
