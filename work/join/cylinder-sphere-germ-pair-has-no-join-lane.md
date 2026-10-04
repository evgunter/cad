---
id: cylinder-sphere-germ-pair-has-no-join-lane
kind: issue
title: A transverse cylinder wall x sphere germ pair has its section frame and no chord lane at the join (CurvedBooleanUnsupported): its section is a space quartic, and every lane rides a plane
status: open
opened: 2026-10-04
priority: P1
cost: H
design: true
refs: [cylinder-sphere-germ-pair-has-no-section-frame, skew-cylinder-germ-pair-has-no-section-frame]
---

Found by the cylinder × sphere frame lane (branch
`join/cylinder-sphere-frame`, 2026-10-04): the frame is the door before
this one.

## Measured

`crates/sweep/tests/cylinder_sphere_frame.rs` pins eight poses, each
under ∪, ∩ and both differences in both operand orders (48 runs): a ball
against a radius-0.5 drum `z ∈ [−1, 1]` crossing its wall in one loop
(centre on the wall, inside it, outside it, a turned chart) or two (a
ball reaching past the far side, one with a turned chart), and the
tilted drum cut's lower part against the two balls straddling its rim
(radius 0.2 at `(0.5, 0, 0.35)`, radius 0.1 at `(0.45, 0, 0.3)`). Every
run passes the section frame and the matcher, which consumes every germ
and joins only neighbouring sites along each loop
(`every_segment_joins_neighbouring_sites_along_its_loop`), and then
refuses

```
CurvedBooleanUnsupported { operand: A, face: <the wall or the sphere>,
                           kind: Cylinder | Sphere }
```

at `boolean::join::bool_connect`'s lane dispatch, the `(a_s, b_s)`
catch-all. The rows hold each run to its volume (an independent oracle,
the slices' closed-form areas integrated) the day it builds.

## What a fix has to supply

A chord lane for a non-planar section. Every lane the dispatch has
(`GermLane::{Planar, PlaneWall, WallPlane, Radical}`) lays both sides'
chords in one plane, and a wall face's ring lane closes its island along
that plane (`RingClosure::Wall`). The transverse cylinder × sphere
section is a space quartic: the chord between two germ sites is an arc
of it, minted as a fitted curve (`geom_brep::ssi::cylinder_sphere_ssi`
marches and fits this pair; `Pcurve::Fitted` certifies at rest), with
its pcurve on each face's chart, and the ring lane needs an island
closing that is not a plane section. How a fitted chord between two
germ sites is minted and certified, and how the island closes, is a
design question; the skew cylinder pair reaches the same gap once it has
a frame (`skew-cylinder-germ-pair-has-no-section-frame`).
