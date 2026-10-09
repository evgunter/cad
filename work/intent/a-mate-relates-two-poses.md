---
id: a-mate-relates-two-poses
kind: issue
title: D10 stage 3 PR B: a mate reads two pose variables and its kinds are its primitive; MatePrimitive, MateFrame, FrameBase, the clocking rider and PlanarRest.offset retire into pose definitions
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [poses-are-variables, a-mate-reads-face-variables]
refs: [intent-stage3-is-built]
---

INTENT stage 3, PR B. Spec: `docs/INTENT-STAGE3-SPEC.md` §3.

`Mate { a, b, sense, class }`: `a` and `b` are pose variables of one kind, and the kinds are the primitive (FORK-S3-3). `Alignment`, `MateFrame`, `FrameBase`, `MatePrimitive`, the clocking rider, `PlanarRest.offset`, `MateFrame::authored` and `table_gap` (`mate.rs`) retire into pose definitions. The spanning tree and gauges still decide what places until C. Every MSOLVE fixture's `SolvedPoses` is bit-equal.

Closes `a-clocking-rider-is-levered-unreduced`, `a-face-frame-cannot-turn-its-roll`, `a-face-base-puts-its-reference-on-local-y`, `mate-primitive-unit-variants-load-from-a-null-payload` and `a-mate-frame-axis-is-decided-against-a-length-band`. It subsumes MSOLVE-15 (#3681).

FORK-S3-3 was weighed with FORK-S3-2 and S3-5 as FORK-S3M (fork log row
97, PR 4326), and this unit builds on that answer. A mate is
`{ on, to }`, two pose variables of one kind, and the kind is the
primitive. `on` is a pose read off the copied shape's geometry and `to`
one read off geometry of the space the copy joins (a face's plane, a
carrier's axis or centre, a standoff along a plane's normal): there is
no `FrameBase::Part`, no part frame to read, and no reader of a
carrier's reference direction (a face reads as a plane, FORK-S3P round
10). What a mate leaves is set by values (FORK-S3M rounds 6-8), each a
`Length` or `Angle` variable charted on the two bodies' own coordinates,
so the `Offset` target carrying in-plane numbers retires. Today's mates
written against a part frame, authored vectors and world-gauge offsets
are absolute coordinates; the migration restates each as mates over
geometry plus values, each value computed from today's pose so no bit
moves, and otherwise drops it and names it in its report. There is no
`sense` operand: the sense is `Flip { pose }`, a construction (an
involution the door normalises to one side), and `Flip` has no `Point`
arm; a `Frame` "opposed" becomes a `Flip`ped plane mate plus values,
which retires `opposed()`'s hidden half-turn about `x`. A clocked
coaxial (today's `Coaxial` plus rider, residual `Prismatic`) migrates
to an `Axis`–`Axis` mate plus a slide and a spin value. That needs the
`Cylindrical` residual's order-free chart in the shared `Subgroup` in
this unit; the spec's Q6 line ("a `Plane`–`Plane` mate through the
axis") pins the slide and is wrong.
