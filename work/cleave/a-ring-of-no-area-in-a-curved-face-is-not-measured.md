---
id: a-ring-of-no-area-in-a-curved-face-is-not-measured
kind: issue
title: The Boolean refuses to part an edge lying inside a curved face with neither end on its boundary: the zero-area ring the parting leaves there is not measured
status: open
opened: 2026-10-26
priority: P1
cost: M
---



Found by the CLEAVE lane on `cleave/riders-read-every-edge-in-a-face`,
building `a-boolean-edge-lying-in-a-curved-face-is-not-read`. P1 for
verb breadth: a box cut from a pipe whose corners touch the wall is
ordinary geometry.

## What

The pipe `x² + y² ≤ 1`, `z ∈ [−2, 2]`, less the box
`|x| ≤ √.91`, `|y| ≤ .3`, `|z| ≤ .3`
(`sweep/tests/verbs_ga_r2_probes.rs`,
`r2_a_box_with_on_carrier_rulings_builds_every_op_undeclared`): the
box's four edges along `z` are rulings of the pipe's wall, and the
void's two faces at each leave it into the wall's material, so D1
parts each ruling into two coincident edges. Neither end of a ruling
lies on the wall's boundary (the box's corners rest in its interior),
so the parting is a ring of no area in the wall (`slit::part`,
`kfmrh_minting`). Built, the result gate refuses it `ResultInvalid`
(`VolumeUncomputable`, `DegenerateFace` on the wall): the curved face's
measure parses its boundary into rims and reads a two-edge ring of no
extent as degenerate (`geom-brep/src/props/curved.rs`
`require_extent`). So `p ∖ b` refuses `EdgeInFace` (`CurvedRing`)
before the parting (`slit::part_riders`) at every ε. Main ships the one
edge along each ruling there, which tiers 3 and 3′ pass
(`work/restread/tier-3-prime-passes-an-edge-lying-along-a-curved-face-of-its-own-solid.md`).

A ring of no area in a plane face is measured, and an edge with an end
on the curved face's boundary parts as a slit of it and measures
(`sweep/tests/edges_lying_in_a_face_part_wherever_they_lie.rs`,
`a_keel_along_a_cylinder_floors_ruling_parts_in_every_op`).

## Fix

Let the curved face's measure read a ring of no area as contributing
nothing (its two edges cancel), as the plane lane does, then drop the
`CurvedRing` refusal and pin `p ∖ b` building at `π·4 − 2√.91·.36`
with two coincident edges along each ruling.
