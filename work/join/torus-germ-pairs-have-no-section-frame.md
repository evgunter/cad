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
or stop at `CurvedPierceUnsupported` before the join. Every cone pair
stops earlier, at the operand gate (`CurvedPairUnsupported`), so no cone
germ pair reaches the frame.

## What a fix has to supply

A frame per torus pair, or a proof that none exists for a section shape
(a torus section can have up to four loops, and a loop need not turn
monotonically about any one axis). This is a different construction from
the cylinder × sphere frame, whose loops are graphs over a circle of one
operand. Past a frame, each pair meets the lane door
(`cylinder-sphere-germ-pair-has-no-join-lane`).
