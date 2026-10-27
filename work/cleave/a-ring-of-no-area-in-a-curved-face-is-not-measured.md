---
id: a-ring-of-no-area-in-a-curved-face-is-not-measured
kind: issue
title: The Boolean refuses to part an edge lying inside a curved face with neither end on its boundary: the zero-area ring the parting leaves there is not measured
status: parked
opened: 2026-10-26
priority: P1
cost: M
blocked_on: [a-curved-faces-measure-reads-a-ring-of-no-area-as-degenerate]
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

Two steps, split by territory. The measure reads the ring:
`work/measuredoor/a-curved-faces-measure-reads-a-ring-of-no-area-as-degenerate.md`
(MEASUREDOOR, the measurement door, owns it), which this row is parked on.
Then CLEAVE drops the `CurvedRing` refusal (`slit::part_riders`) and
re-pins what each moved test owes back:

- `verbs_ga_r2_probes.rs` `r2_a_box_with_on_carrier_rulings_builds_every_op_undeclared`,
  `p ∖ b`: builds at `π·4 − 2√.91·.36`, two coincident edges along each
  ruling, its census and its eight v-f rests re-read on the parted form
  (main pinned `(10, 18, 12, 2)` and `[0, 8]` on the one-edge form);
- `a_ruling_through_a_box_corner_edge.rs`
  `a_wall_through_a_box_edge_builds_every_op_but_the_ring` and
  `a_post_through_the_near_wall_piece_refuses_the_ring`: `w ∖ b`
  builds, and the two v-f rests at the corner edge's ends land on the
  wall piece that holds them (what the tests checked before PR 5793,
  under their old names `…_records_the_rest_on_the_piece_that_holds_it`
  and `…_keeps_the_rests_on_their_piece`); both get their names back;
- `a_ruling_lying_on_a_wall.rs`
  `a_prism_edge_on_the_tubes_wall_builds_every_op_undeclared`, off a
  seam ruling inside one face, `t ∖ b`: builds, with the v-f rests at
  the edge's ends it pinned on the one-edge form.
