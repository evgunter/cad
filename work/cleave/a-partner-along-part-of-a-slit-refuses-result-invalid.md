---
id: a-partner-along-part-of-a-slit-refuses-result-invalid
kind: issue
title: A Boolean with a partner whose edge runs along part of a slit refuses ResultInvalid DescriptionNotAdjacent or ScaffoldAtRest
status: closed
closed: 2026-10-27
pr: 5812
opened: 2026-10-25
priority: P2
cost: M
---


Found by the claims review of PR 5544
(`cleave/boolean-closes-a-slits-face`, row
`a-boolean-closes-a-slits-face-across-its-line`). The refusals are the
same on main before it.

## What

The slab `[0, 4]² × [0, 1]` less a keel vee
(`a_keel_on_the_floor_subtracts_as_a_slit.rs`) carries the keel as a
slit or ring of its floor, from either origin (the Boolean's slit and
the split's). A partner vee or holed box whose edge runs along part of
the keel's line, not its whole length, refuses at the result gate with
a kernel defect, at rest and turned:

| slit | partner | op | refusal |
|---|---|---|---|
| ring, open | wide vee `(1.7, 2.3)` (`part`) | `x ∪ y` | `ResultInvalid` `DescriptionNotAdjacent` |
| ring, open | wide vee `(1.7, 2.3)` | `y ∪ x` | `ResultInvalid` `ScaffoldAtRest` |
| open | wide vee `(2.2, 3.4)` (`half`) | `x ∪ y`, `y ∪ x` | as above |
| ring, open | box less the wide vee, `part` | `x − y` | `ResultInvalid` `DescriptionNotAdjacent` |
| open | box less the wide vee, `half` | `x − y` | `ResultInvalid` `DescriptionNotAdjacent` |

`DescriptionNotAdjacent` is an edge described against faces it does not
bound; `ScaffoldAtRest` a scaffolding edge left in the result. Each is a
construction the join leaves half done where the partner's edge ends
on the slit's line, not a reading of the input.

## Fix

Find where the partner's end on the line leaves the scaffold or the
stale description (the zip's seam pairs at that end, likely), build it
as the material decides, and pin these cells building.

## Note (2026-10-26, branch `cleave/boolean-closes-a-slits-face`)

The `x − y` cells with the box less the wide vee (ring `part`; open
`part` and `half`; both slit origins; at rest) refuse
`SlitFaceClosesMidSlit` on PR 5544's head, not `ResultInvalid`
`DescriptionNotAdjacent`: the zip glues the floor to itself across the
stretch of the keel the vee runs along, a strut whose ends both continue
into the slit's edges, and the output stage refuses to part the slit
there (`boolean/slit.rs`, `close_struts`). The fix is that parting:
`kemr` of the strut, splitting the slit's loop in two, each piece then
read as its own slit or ring. Pinned by
`a_keel_on_the_floor_subtracts_as_a_slit::a_face_closed_across_part_of_the_slit_refuses_typed`.

## Closed

2026-10-27, PR 5812. The unions build, every cell of the table, at ε
default, 1e-6 and 1e-12, at rest, turned and flipped and turned, as
the closed form says, tier 3 and 3′, their edges on the keel the
material's creases. Their cause is the ringed plate's
(`a-ring-partner-inside-the-ring-unions-result-invalid`): the zip
leaves the vee's stretch of the keel an edge of the floor on both
sides, and the closing deletes it from the seam edges, parting the
keel with `kemr` (`boolean/slit.rs`, `part_mid_slit`). Pinned by
`a_keel_on_the_floor_subtracts_as_a_slit::a_partner_along_part_of_the_keel_unites_with_the_slit_in_either_order`.

The `x − y` cells with the box less the wide vee part the keel too, and
refuse `LineWedges` `Overlap`, typed, beyond the vee's stretch: that is
`a-difference-whose-partner-ends-inside-a-slit-overlaps-its-wedges`
(P2), which carries them.
