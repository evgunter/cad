---
id: the-band-columns-twin-read-back-has-no-witness
kind: issue
title: the walk's band columns read a lifted image back through the sphere's twin, and no body reaches that arm
status: open
opened: 2026-11-21
priority: P3
cost: M
refs: [the-pole-to-pole-band-reads-its-columns-off-the-area-vector-beside-the-lift]
---

Found by PR 7303's review.

`mesh::walk::band_columns` reads each meridian of a rimless pole-to-pole
band onto the branch its lifted image sits on. Where the lift moved the
image onto a sphere chart's twin sheet (`LiftedRow::lift.twin`, the
involution `(u, v) ↦ (u + π, π − v)`), it reads the image back through the
twin, `entry_u − π`, before taking the column. No body in the mesh or
sweep suites reaches that arm: every lifted row the lift-turns measurement
saw had `twin: false` (2637 band reads).

The arm is guarded: a column more than a quarter turn from its image
refuses `TessellateError::LiftedColumnOff`, so a wrong reading there
refuses rather than meshes the wrong half. But what it answers on a real
twin is unmeasured. A witness is a rimless sphere band whose lift crosses
onto the twin: `topo::pcurves`' singular turn joins two edges through the
twin where their images lie half a turn apart at a pole
(`half_turn_off`), for example a band whose meridians' images are stated
on opposite sheets. Build one through the public doors, mesh it, and
check its volume against props.


