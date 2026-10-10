---
id: torus-germ-pairs-have-no-section-frame
kind: issue
title: A torus wall against a sphere, a cylinder or another torus reaches the join and refuses GermFrameUnsupported: no frame names its section
status: open
opened: 2026-10-04
priority: P1
cost: H
design: true
refs: [c5-plane-torus-cone-cylinder-arms, cylinder-sphere-germ-pair-has-no-join-lane]
---

Found by the cylinder × sphere frame lane's sweep of the germ pairs that
reach `FrameError::NoArm` (branch `join/cylinder-sphere-frame`, 2026-10-04).

## Measured

Poses (`kind_pair_sweep_probe`, run on this branch and not committed:
shapes at the origin, B moved by a rotation then a translation, ∪ in both
member orders): a drum `cyl(0.5, 1.0)` (radius 0.5 about `z`,
`z ∈ [−1, 1]`, `common::germ_pair`), a ball of radius 0.5 revolved about
`y`, a donut revolved about `y` (major 0.6, minor 0.2), a frustum, and a
brick `[−0.4, 0.4]³`; the six placements are rotation 0 then
`t = (0.45, 0.1, 0.05)`; 1.5707963 rad about `x` (a hair short of a right angle), `(0.3, 0.2, 0.1)`; 0.7 rad about
`x`, `(0.1, 0.35, 0.2)`; 1.1 rad about `(0.3, 1, 0.2)`,
`(0.42, −0.15, 0.3)`; 0.4 rad about `y`, `(0.6, 0, 0)`; 0.9 rad about
`(1, 1, 0)`, `(0.05, 0.1, 0.02)`.

`GermFrameUnsupported` from `pair_section_frame`'s catch-all, by germ pair
(the count of the 12 runs per ordered shape pair, both orders summed):

| shapes | germ pair | lines |
|---|---|---|
| ball × donut, donut × ball | Sphere × Torus | 22 |
| drum × donut, donut × drum | Cylinder × Torus | 19 |
| donut × donut | Torus × Torus | 6 |
| brick × donut, donut × brick | Plane × Torus | 21 (the plane × torus half is GERM's `c5-plane-torus-cone-cylinder-arms`) |

The remaining donut lines escalate (`ArcCylinderRoots`, `PierceCurvature`)
or stop at `CurvedPierceUnsupported` before the join. No cone pair was
in this measurement: it stopped at the operand gate then, and since the
cone joined the roster (VERBS-CONE U7) a cone germ against a curved face
is documented to refuse at its frame (`GermFrameUnsupported`; the Cone
bullet of `BooleanError::CurvedPairUnsupported`'s docs), unmeasured here.

## What a fix has to supply

A frame per torus pair, or a proof that none exists for a section shape
(a torus section can have up to four loops, and a loop need not turn
monotonically about any one axis). This is a different construction from
the cylinder × sphere frame, whose loops are graphs over a circle of one
operand. Past a frame, each pair meets the lane door
(`cylinder-sphere-germ-pair-has-no-join-lane`).

## The radial hole stops here too (GERM, 2026-10-10)

Measured on `germ/radial-hole-through-a-tube`, which gives the section
certificate its square-wall arm (a cylinder whose axis is square to the
torus axis). The donut (`R = 2`, `r = 0.5`, revolved about `y`) minus a
rod along `x` of radius `0.2`, through both walls of the tube
(`x ∈ [1, 3]`), through the inner side (`[1, 2]`), through the outer side
(`[2, 3]`), and off the axis (`[1, 3]` at `(y, z) = (0.1, 0.15)`): every
torus × wall pair of the section report answers `Ok` (each was
`Err(Reach)` on main), and ∖ refuses `GermFrameUnsupported { a_kind:
Torus, b_kind: Cylinder }` at the join. Pinned by
`crates/sweep/tests/germ_radial_hole.rs`
`a_radial_hole_through_the_tube_stops_at_the_germ_frame`, which turns red
the day this frame lands. The section there is the arm's: in the
meridian plane normal to the rod, its trace circle against the tube
circles, each loop lying over a run of that circle with two points
(`±t` along the rod) over each of its points (`section_cert.rs`
`torus_square_cylinder`), which may be the frame's starting point for
this one pose.
