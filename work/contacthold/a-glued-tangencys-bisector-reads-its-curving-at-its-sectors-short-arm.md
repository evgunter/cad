---
id: a-glued-tangencys-bisector-reads-its-curving-at-its-sectors-short-arm
kind: issue
title: A glued tangency's bisector bound reads its curving at its sector's short arm, and escalates where its face reaches far
status: open
opened: 2026-10-12
priority: P3
cost: M
---

## What

The sliver family (`crates/sweep/tests/review_4831_probes.rs::p10_the_sliver_family`):
a disc of radius 2 about the origin, z ∈ [0, h], nested on a disc of
radius 1 about (1, 0), z ∈ [−1, 1 + 2h], internally tangent along the
ruling through (2, 0). Every op answers at its closed form, both orders,
except a narrow window of `h` at each ε where every op refuses
`Escalated` on `tangent_sector_order2` (which way a face curves away
from a face it touches): h = 1e-4 at ε 1e-9, h = 3e-6 at ε 1e-12. (At
ε 1e-6, h = 1e-5 escalates earlier, a vertex h from a face read in
band; below it the extrusion itself refuses.)

The escalating reading, traced: at the fat disc's bottom vertex (2, 0, 0)
the thin disc's wall is split along its ruling at z = 0 and z = h, so its
sector there spans the half-turn between a bound of reach h (up the
ruling) and one of reach 1 (down it), and is bisected. The bisector
points across the ruling (−y) and, as a bisector, is levered at its
sector's arm, h (`sectors::Reach::Bisector`). `recl::recl_sectors`' tangent
arm reads each bound over its own reach
(`sectors::tangent_relative_side`), so the bisector's sagitta
½·κ_rel·h² lands in band for h ≈ √(2ε·[1, 10]/κ_rel) and escalates; below
that window it reads Zero, the bound stays `On`, and the edge engine
resolves it.

## Why it is a defect

The face the bisector crosses reaches far past h (it is the thin disc's
wall, the whole half-turn), so its departure off the shared tangent
plane is definite at any reach that face actually has. The arm is the
sector's shorter bound, a property of the ruling's split, not of the
cross direction. A bisector's reach that measured the face along the
bisector, not the sector's shorter chord, would read the side.

## History

PR 4831's first fix pass made this family refuse
`ClassificationInvariant` ("surviving record with an On code after
srecledges") wherever the fat disc's ruling edge was too short for its
second-order departure to show, because the tangent descent read every
bound at the sector's arm. The second fix pass reads each bound over its
own reach, as the first-order side code does; the kernel-bug kind is gone
from the family at every sampled ε, and this window is what remains.
