---
id: a-needle-pose-at-the-zeros-edge-refuses-on-the-basin-far-from-the-origin
kind: issue
title: A needle pose whose gap sits at the zero's edge refuses on the basin at 1e3 from the origin, where at the origin it meets once
status: open
opened: 2026-11-09
---

Found by PR 6638's FULL review; measured again by `onecopy/transverse-range`.

## What

The needle pair (`order_rows::needle_pair`), with the near-circle turned +3e-6 about the long ellipse's tangent at its far vertex, has a `span_gap` of 9.63e-13: 0.96 of the zero at band 1e-12/1e-11. At the origin it reads one meeting in both orders. Moved by 1e2 it still does. Moved by 1e3 along (1, −0.7, 0.3) or along (0.6, 0.8, 0), both orders refuse `Unsettled::Meter { curve_pair_pose_basin }`.

The noise mint is not the cause: PR 6638's base refuses it too. At 1e3 a coordinate rounds at about 1.1e-13, a tenth of the zero, so a gap 4% inside the zero is within a few roundings of its edge. The window does not certify there, and the polish meets the needle's near-parallel tangents.

## Owed

Measure whether the window's off or on readings fail at the zero's edge (a reading resolution), or whether the plane root's window is placed differently far from the origin. Then decide the window, or state the pose as the band's resolution. A gap within a coordinate's rounding of the zero is the band's to escalate, not the basin's to refuse.
