---
id: a-mate-relates-two-poses
kind: issue
title: D10 stage 3 PR C: a mate equates two poses of one kind and holds no number; values set the freedoms mates leave; MatePrimitive, MateFrame, FrameBase, AxisSense, the rider and PlanarRest.offset retire
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [poses-are-variables, a-placement-is-the-bundle-of-mates]
refs: [intent-stage3-is-built]
---

INTENT stage 3, PR C. Spec: `docs/INTENT-STAGE3-SPEC.md` §4. Built on FORK-S3M (fork log row 97, PR 4326).

A mate is `{ on, to }`, two pose variables of one kind, and the kind is the primitive. `on` is read off the copied shapes' geometry. `to` is read off geometry of the space the copy joins, or reads the world through a projection of its frame (spec Q7). There is no part frame to read, and no reader of a carrier's reference direction. A mate holds no number. Its sense is `Flip` on one side. A standoff is `Standoff`, a construction on the target.

A value sets one freedom the mates leave, a slide or a spin, to a `Length` or `Angle` variable. It is charted on the two bodies' own coordinates as the placement carries them, as a function of the relative pose alone, and zero is always valid. A value on a lone point mate's rotation refuses `NoChart`. A mate taking a freedom a value sets refuses `Overconstrained`.

Retired: `Alignment`, `MateFrame`, `FrameBase` (with B's `World` arm), `MatePrimitive`, `AxisSense`, the clocking rider, `PlanarRest.offset`, `MateFrame::authored`, `table_gap`, and the `Offset` target carrying in-plane numbers.

The migration restates each mate over geometry plus values, each value computed once from today's solved pose. A clocked coaxial becomes an `Axis`–`Axis` mate plus a slide and a spin value, so the `Cylindrical` residual's chart lives in the shared `Subgroup`. A side written against a part frame or an authored vector is restated where a face or carrier pins the same coset, and otherwise dropped and named. A world mate is restated as a world read (`WorldRead`, spec Q7: the world's frame, planes, axes or origin with `Standoff`/`InFrame`/`Flip` inside the mate, never a variable). Where the copy's geometry offers no pose of the coset, the migration refuses to regenerate and names it, because dropping a world mate would empty the product.

Closes `a-clocking-rider-is-levered-unreduced`, `a-face-frame-cannot-turn-its-roll`, `a-face-base-puts-its-reference-on-local-y`, `mate-primitive-unit-variants-load-from-a-null-payload`, `a-mate-frame-axis-is-decided-against-a-length-band`, `a-mate-frame-is-written-in-the-reading-instances-coordinates` and `placement-step-slots-are-spelled-three-ways`. It subsumes MSOLVE-15 (#3681).

C ends B's migration interim, in which a placement's constraints are today's mate payload with its numbers.

Waits on A (`poses-are-variables`) and B (`a-placement-is-the-bundle-of-mates`, the placement that owns values).
