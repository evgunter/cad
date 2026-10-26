---
id: a-partner-along-part-of-a-slit-refuses-result-invalid
kind: issue
title: A Boolean with a partner whose edge runs along part of a slit refuses ResultInvalid DescriptionNotAdjacent or ScaffoldAtRest
status: open
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
