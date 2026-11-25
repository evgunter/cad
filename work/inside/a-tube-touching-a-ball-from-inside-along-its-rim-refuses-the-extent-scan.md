---
id: a-tube-touching-a-ball-from-inside-along-its-rim-refuses-the-extent-scan
kind: issue
title: A tube inside a ball whose rim lies on the sphere: ball minus tube refuses at the parting (EdgeInFace Curved, then CurvedFace), where it is two or three solids touching along the rims
status: parked
priority: P1
cost: M
opened: 2026-10-09
refs: [4399, 7602, 7481]
blocked_on: [the-partings-curved-face-gate-refuses-a-division-its-reason-does-not-cover, a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs]
---

Found by JOIN's tube-on-a-ball lane (branch `join/tube-ending-on-a-ball`),
probing the tube that ends on the ball from inside.

## What

A `z`-axis tube of radius `a` inside the ball of radius `√2` about the
origin (`sweep::test_support::ball_poled_z`), with one or both rims on
the sphere: `z ∈ [−z0, z0]` or `z ∈ [0, z0]`, `z0 = √(2 − a²)`, for
`a ∈ {1, 0.7}`, upright or tilted. Every op in both member orders
refuses `FallbackExtentUnsupported { "the sphere's section circle runs
near the plane face's boundary …" }` at every ε row.

The tube touches the sphere along its rim and crosses nothing, so the
reduction mints no null pair and the no-crossings path takes the pose.
Its sphere extent scan (`boolean::ops::sphere_extent_scan`, the
`Surface::Plane` arm) cuts the sphere by the tube's end disc's plane,
which gives the disc's own boundary circle, and `face_boundary_meets`
reads it as running near the boundary, as it is.

The tube standing on the ball from outside (`z ∈ [z0, z0 + len]`)
reaches the crossing layer as an ON event and builds every op
(`crates/sweep/tests/a_tube_ending_on_a_ball.rs`).

## Why it is not a one-line fix

`tube ∪ ball` is the ball and `tube ∩ ball` the tube, but `ball ∖ tube`
pinches along the rim: the material on either side of the tube's wall
meets the sphere at the rim circle only. So a fix decides what the
boolean does with a touch whose difference is a pinch along a curve,
which is contact semantics, and the D10 hold
(`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`) may gate it.

## Witness

`crates/sweep/tests/a_tube_ending_on_a_ball.rs`'s probe family
(`the_probe_family_ships_no_wrong_body`) carries these poses as
`inside, both rims`, `inside, one rim` and `inside, one rim tilted`, at
radii 1 and 0.7. It asserts only that they build sound or refuse typed.
A tube that ends on a half ball from inside, its wall leaving through
the flat face, crosses something and builds (the same family).

## The D10 hold has lifted (2026-11-03)

INTENT closed on 2026-10-30 with D10's umbrella row (#6103,
`docs/doc-ledger/intent-leaves-the-tracker.md`), so the hold that
"may gate it" above no longer stands. What the row waits on is its own
design question (what the boolean does with a touch whose difference
pinches along a curve), weighed by a designer pair before a lane
builds it.

## 2026-11-25 — weighed: not a design fork (INSIDE orchestrator)

Two designers weighed the row independently and agreed after one round.
The semantics are ratified, so nothing goes to Ev:
`[topology.conventions]` ("pieces that only touch … are distinct
solids", PR 3901) and Ev's PR 5967 ruling (two coincident arcs are two
cells with an edge–edge record). So `tube ∪ ball` is the ball,
`tube ∩ ball` the tube, `tube ∖ ball` empty, and `ball ∖ tube` is three
solids (both rims) or two (one rim) with one edge–edge record per rim,
at the ball's volume less the tube's. The planar twin already builds
(`a_cavity_resting_its_keel_on_the_floor_opens_through_it_as_a_ring`).

**The premise moved.** Measured on main `133ca95e02` (a designer's
probe, not committed): since PR 7602 the extent scan no longer refuses.
Its plane arm asks the section certificate first, and W4 clears the rim
off its recorded vertex touch. ∪ and ∩ build in both orders,
`tube ∖ ball` is Empty, and only `ball ∖ tube` refuses, with
`EdgeInFace { gap: Curved }` in the output stage's parting.

**What it waits on.** The parting (`boolean/slit.rs`), HELDRECORDS's
ground: PR 7481 (`heldrecords/split-on`) lifts `Curved`; then the
`CurvedFace` gate must narrow to the arms that leave a zero-area ring or
a slit, and gain a division arm for a closed edge inside one face
(`work/heldrecords/the-partings-curved-face-gate-refuses-a-division-its-reason-does-not-cover.md`,
filed today). This row is then a witness: re-pin the six inside poses
SOUND with their solid counts and records. The probe family pins
nothing beyond "sound or typed" today.

Filed off-question on HELDRECORDS's slate:
`a-fallback-subtract-labels-its-result-voided-before-the-parting-opens-the-cavity`,
`the-fallback-hands-insert-hollow-voids-strict-containment-evidence-for-a-touching-shell`.
