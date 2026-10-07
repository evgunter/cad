---
id: sheet-clip-admits-a-cylinder-at-its-stored-origin-by-an-unlevered-tilt
kind: issue
title: sheet_clip admits a cylinder by its stored origin's offset and an unlevered tilt gate
status: open
opened: 2026-10-07
priority: P2
cost: M
refs: [cylinder-offsets-read-at-a-stored-origin-off-the-reach]
---


## What

`sheet_clip` (`crates/sweep/src/blend/reach.rs`) classes a surface as
`Kind::Cylinder` when `near(off(*origin)) && near(tilt(*axis))`: the
cylinder's STORED origin (any point of its axis) measured off the band
torus's canonical axis `(o, a)`, and a tilt gate `tilt.hi() <=
band.zero()` with no lever. A tilt in the zero band moves the origin's
offset by the tilt times how far along the axis the origin is stored,
and nothing levers the tilt at the extent the clip is consumed over.
Whether a wrong `Kind::Cylinder` can make the sheet enclosure unsound
(rather than only returning `None`) decides the priority; read, not
probed.

The fix's shape is `cylinder-offsets-read-at-a-stored-origin-off-the-reach`'s
(PR 4118 and its sibling): read the offset at the foot of the consumed
region on the axis, and decide the tilt through a named margin levered
at that region's extent from the foot.

Found by `tang/cylinder-offsets-at-the-reach`'s sweep.
