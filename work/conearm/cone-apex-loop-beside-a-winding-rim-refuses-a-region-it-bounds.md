---
id: cone-apex-loop-beside-a-winding-rim-refuses-a-region-it-bounds
kind: issue
title: A cone face with a loop through the apex and a rim that winds the axis refuses, though it bounds one region
status: open
opened: 2026-10-19
priority: P4
cost: E
---

Found in review of PR 5333 (`flux/cone-closure-and-per-term-whole`,
NOTE-1), pinned there by
`periodic_chart_closure::a_winding_rim_beside_a_loop_through_the_apex_refuses`.

`cone_face_read` (`crates/geom-brep/src/props/curved.rs:1821`) refuses a
face with a loop through the apex beside a rim that winds the axis
(`LoopsWindChart`, the check at `curved.rs:1862`). Two such faces do
bound one region:

- the slit cap of slant 1 (generator out, whole rim, generator back)
  with the rim at slant 2 wound against it: the band `[1, 2]`;
- the sector `[0, 1] × [0, 1]` with that rim: the cap of slant 2 less
  the sector.

With the check removed both measure their closed forms exactly, at
`f64` and at the interval scalar (the band `π·sin α·3 = 4.5185`, the
cap less the sector `5.7849`, on the cone of half-angle ½), and the
other traversals refuse `RingWoundWithFace` or `HolesExceedFace`. So the
refusal is conservative: these faces are not what a body's cone faces
are stated as, but a widening could read them. Main refused them too
(`ConeLoop{props_cone_ring_contractible}`), so nothing narrowed.

The repair, if one is wanted: read a loop through the apex as winding
nothing and let the apex puncture take the balance (`−Σk`), as it does
for loops off the apex, then pin both faces' values.
