---
id: a-cut-by-a-rod-tangent-from-inside-records-its-material-crescent-as-a-tangent-contact
kind: issue
title: A cut by a rod tangent to a face from inside records the material crescent as a tangent contact, which the gate now refuses as contradicted
status: open
opened: 2026-11-18
priority: P1
cost: M
---


Found by `embed/records` (2026-11-18), when `finish` began confirming
the records it is handed.

## What

The plate `[-2, 2]² × [0, 1]` less a rod of radius 0.5 centred
`(0, 1.5)`, over `z ∈ [0.25, 0.75]`, so the rod's wall is tangent to
the plate's `y = 2` face from inside
(`crates/sweep/tests/face_face_touch.rs`,
`a_bore_tangent_from_inside_refuses_on_the_tangent_record_its_cut_wrote`).
The result is one solid pinched along `x = 0` with its material in the
crescent beside the line, which owes two cusp edges (D1 tier 3).

`topo::subtract` writes a face–face record of the plate's face and the
bore's wall, class `Tangent`. The census reads that pair as an overlap
(`ContactContradicted { record: ContactPair { class: Tangent }, verdict:
Tangent(Row { row: Overlap, margin 0.25 }) }`): a material crescent is
no contact.

Before the gate read records the cut built, and the body shipped with
the false record and without its cusp edges; the tests of that body
read it with no record and pinned `CuspEdgesMissing`. The cut now
refuses `BooleanError::ResultInvalid` on the record it wrote, at three
poses (`the_void_and_material_pinches_read_alike_at_every_pose`).

## Owed

- The cut should not record a tangent contact where the two faces bound
  material between them; whether it then builds (minting the cusp
  edges) or refuses typed for the missing edges is the lane's to
  settle.
- `a_solid_missing_its_cusp_edges_leaves_a_solid_in_its_bore_cleared`
  was deleted with this change: its fixture (the bored plate with a
  cube floating in the bore) is built by that cut and cannot be staged
  through a door. What it pinned, that a solid's missing cusp edges do
  not leave the census's other solid pairs unexamined, has no row
  until the cut builds again or the fixture is built by hand.

## A row this defect takes with it (2026-11-22)

`sweep face_face_touch::a_solid_missing_its_cusp_edges_leaves_a_solid_in_its_bore_cleared`
was deleted with PR 7124: it united this cut's result with a cube
standing in the bore and pinned that tier 3′ reports the plate's
missing cusp edges and nothing of the cube. Its property (a missing
cusp finding in one solid does not stop the census reading the other
solid pairs) is pinned nowhere now. The cut is the only door that
builds a closed body missing its cusp edges, and it refuses. The
kernel's own cusp rows (`census::same_solid_tangent_lines`) stand on
open sheet fixtures whose placeholder faces the cross-solid arm cannot
bound: the same two walls with a cube grafted in the bore answer
`CuspEdgesMissing` and a `CensusUndecidable { MayOverlap }` of the two
solids, so that fixture does not hold the property either (tried in
the review fix pass). The row comes back with the fix here: once the
cut mints the two cusp edges, or ships the crescent unrecorded, the
united body exists again.

