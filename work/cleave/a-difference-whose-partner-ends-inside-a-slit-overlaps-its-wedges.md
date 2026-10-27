---
id: a-difference-whose-partner-ends-inside-a-slit-overlaps-its-wedges
kind: issue
title: A difference whose partner's edge along a slit or ring ends inside it refuses LineWedges Overlap
status: open
opened: 2026-10-25
priority: P2
cost: M
refs: [revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet]
---


Found by the claims review of PR 5544
(`cleave/boolean-closes-a-slits-face`, row
`a-boolean-closes-a-slits-face-across-its-line`).

## What

An operand `y` carries a slit or zero-area ring in face F, and the
other solid `x` has an edge along the slit's line that ends inside it.
`x − y` builds coincident edges on the line, from the slit's tip to
where `x`'s edge ends, whose material wedges overlap, and the result
gate refuses `LineWedges { fault: Overlap }` (`boolean/ops.rs`, `gate`).
Where `x`'s edge runs the slit's whole length, or past it, the same
difference builds sound.

Witnesses, from the review's probe (the plate less the lying void of
`a_ring_crossed_along_its_line_places_its_germs_apart.rs`; the slab
less the keel vee of `a_keel_on_the_floor_subtracts_as_a_slit.rs`), at
rest and turned:

- ring-groove, short: a box straddling the top less a pyramid lying in
  it, its reflex edge along the ring to 0.3 of the ring's 0.5, `x − y`;
- keel ring and wall to wall, both slit origins (the Boolean's and the
  split's), the box under and around less the wide vee, the vee
  `(2.2, 3.4)` (`half`) and, wall to wall, `(1.7, 2.3)` (`part`), `x − y`.

Before PR 5544 the ring-groove and keel-ring cells refused
`SlitLineCrossed` `Closes` in `recl`; they now reach the gate. The
wall-to-wall cells were read on PR 5544's head only.

## Fix

The re-pairing of the sheets on a line where coincident edges meet, the
material deciding the edges (`crates/topo/README.md`, tier 3), in the
join: here, where `x`'s edge ends inside the slit, the slit's two edges
part at that end into the stretch `x` crosses and the stretch it does
not. Then pin these cells building.

## Note (2026-10-26, branch `cleave/flush-partners-at-a-slit-build`)

The box `[−1, 5]² × [−1, 0.5]` less the wide vee along `(1.7, 2.3)` or
`(2.2, 3.4)` of the keel, less the slab with the trench closed at both
ends or open at one, `x − y`, joins these cells there: the closing now
parts the keel across the vee's stretch (`slit::part_mid_slit`), where
it refused `SlitFaceClosesMidSlit` on main, and the stretches beyond
refuse `LineWedges` `Overlap`, at ε default, 1e-6 and 1e-12, at rest,
turned and flipped and turned. Pinned (Boolean's and split's slit, the
closed keel, `(1.7, 2.3)`) by
`a_keel_on_the_floor_subtracts_as_a_slit::a_face_closed_across_part_of_the_slit_refuses_typed`.

