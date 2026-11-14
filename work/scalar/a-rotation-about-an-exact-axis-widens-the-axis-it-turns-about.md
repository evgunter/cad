---
id: a-rotation-about-an-exact-axis-widens-the-axis-it-turns-about
kind: issue
title: RigidMotion::rotation about an exact axis stores that axis's own component a bracket wide: about exact y at Interval, v.y reads [0.98255, 1.01745] inside a UnitVec3
status: open
opened: 2026-11-14
priority: P2
cost: E
---


Found by BOXFINISH's M3 measurement (2026-11-14, main `4c7404c2`; probe
row `m3_turned_plane_door` on `boxfinish/probe-switch-off`, commit
`bba45062`, never merged).

`RigidMotion::rotation` about the exact y axis by θ = 30° ± 1°, at
`Interval`, stores its second axis as `v.y = [0.98255, 1.01745]`, held
in a `UnitVec3`. A rotation about y fixes y: the component is exactly 1
for every θ in the box. Inferred, not traced: the entry is computed as
`c + (1 − c)·y²`, which interval arithmetic reads as two unrelated
brackets. The other two axes' width is the rotation's own.

What closes it: a rotation's entries along its own axis are the axis's,
exact, at every scalar; a row at `Interval` that fails on today's code.
