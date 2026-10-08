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
needs_ev: true
---

INTENT stage 3, PR B. Spec: `docs/INTENT-STAGE3-SPEC.md` §3.

`Mate { a, b, sense, class }`: `a` and `b` are pose variables of one kind, and the kinds are the primitive (FORK-S3-3). `Alignment`, `MateFrame`, `FrameBase`, `MatePrimitive`, the clocking rider, `PlanarRest.offset`, `MateFrame::authored` and `table_gap` (`mate.rs`) retire into pose definitions. The spanning tree and gauges still decide what places until C. Every MSOLVE fixture's `SolvedPoses` is bit-equal.

Closes `a-clocking-rider-is-levered-unreduced`, `a-face-frame-cannot-turn-its-roll`, `a-face-base-puts-its-reference-on-local-y`, `mate-primitive-unit-variants-load-from-a-null-payload` and `a-mate-frame-axis-is-decided-against-a-length-band`. It subsumes MSOLVE-15 (#3681).

FORK-S3-3 was weighed with FORK-S3-2 and S3-5 as FORK-S3M (fork log row
97, PR 4326), and this unit builds on that answer. A mate is
`{ on, to }`, two pose variables of one kind, and the kind is the
primitive. `on` is a pose read off the copied shape's geometry and `to`
one read off geometry of the space the copy joins (a face frame, a
carrier's axis or centre, their offsets): there is no `FrameBase::Part`
and no part frame to read. Today's mates written against a part frame,
authored vectors and world-gauge offsets are absolute coordinates; the
migration restates each over geometry where the geometry carries it,
and otherwise drops it and names it in its report. There is no `sense` operand: the sense is
`Flip { pose }`, a construction (an involution the door normalises to
one side), and `Flip` has no `Point` arm. A `Frame` "opposed" becomes an
explicit `turn/2` in an `Offset`, which retires `opposed()`'s hidden
half-turn about `x`. Offsets read through `Offset`/`InFrame`, so a
`turn/2` there must evaluate to an exact half-turn, or B's
unmoved-poses check moves an ulp. A clocked coaxial (today's `Coaxial`
plus rider, residual `Prismatic`) migrates to `Axis`–`Axis` plus
`Direction`–`Direction` over the two reference directions, one turned by
the clock angle. That needs `Direction`'s arm (translations and spin
about it, dim 4) in the shared `Subgroup` in this unit; the spec's Q6
line ("a `Plane`–`Plane` mate through the axis") pins the slide and is
wrong.
