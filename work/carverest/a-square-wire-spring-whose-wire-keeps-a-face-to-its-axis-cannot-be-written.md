---
id: a-square-wire-spring-whose-wire-keeps-a-face-to-its-axis-cannot-be-written
kind: issue
title: a square-wire spring whose wire keeps a face toward its axis cannot be written: every sweep carries the rotation-minimizing frame
status: open
opened: 2026-11-03
priority: P2
cost: M
design: true
---

## Finding

It should be possible to write an ordinary square-wire compression
spring: one whose wire keeps a face toward the coil's axis all the way
up, so the wire twists about its own path as it climbs. Today it is
not. This row records the need and decides nothing about how it is
met.

Every path sweep carries the author's start frame by the
rotation-minimizing frame (`crates/sweep/README.md:266`
`[sweep.frame]`), which turns about no tangent. On a helix of angle
`α` that frame rolls against the helix's axis by `2π·sin α` a turn.
It is the shape of a rod bent into a coil without twisting it, not the
shape of a coiled spring. The README states the spring as out of the
law's reach (`crates/sweep/README.md:320`–`324`: "a frame held to
something other than the path, which is placement vocabulary and not
this law"). The author owns only the start placement
(`demos/tour/src/projectbox.rs:127` `spring`, which hands
`sweep_body` a start frame and nothing else), so no spelling of the
call yields the spring.

The tour's spring shows it. The projectbox cell's square wire rolls
about 196° over its six turns. Its outer face turns to a corner around
the second and fifth turns and comes back about 16° off at the top. In
the FreeCAD lane's thumbnail (`demos/renders-freecad/projectbox.png`)
the spring's four wire edges spiral across its faces. At that size the
spring reads as broken. It is not: its STEP export imports into OCC
valid (6 faces), and the OCC mesh at FreeCAD's default deviation is a
clean coil (checked 2026-11-03).

The need was noticed when the frame law was weighed
(`work/carve/sweep-frame-is-a-minimal-rotation-from-the-start-tangent.md:121`–`125`,
"Consequences to show Ev"; `work/carve/log.md:599`) and parked "on
the D10 hold". No row carried it until this one.

## Suggested demo: a spring comparison cell

Take the spring out of the project box and give it its own montage
cell that shows the two springs side by side:

- the spring the sweep makes today: no twist about its path, so its
  wire rolls against the axis and looks odd;
- the normal spring: its wire twists about its path and keeps a face
  toward the axis.

The comparison shows what the frame law does in one look, at a size
where the wire is wider than its outline. Inside the project box the
spring is too small for that (`demos/tour/src/projectbox.rs:391`, a
few pixels wide at the section's camera). The project box then stops
carrying the spring's walls (its Pappus–Guldin volume, its lowest
corner on the boss top, the retired round-wire wall 1, and the
section passing over it), and those move to the new cell.
`demos/README.md:83` describes the cell and changes with it.

The second spring cannot be built until the frame question is
answered, so the cell lands with the answer, or lands first with the
twist-free spring alone.

Moved from `work/issues/` to `work/carverest/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.
