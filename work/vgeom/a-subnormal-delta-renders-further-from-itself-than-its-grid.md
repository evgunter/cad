---
id: a-subnormal-delta-renders-further-from-itself-than-its-grid
kind: issue
title: display_budget: a subnormal δ (4.94e-321 mm) renders as 5e-321, further from it than the render's own grid; red on main
status: closed
opened: 2026-11-25
closed: 2026-11-25
pr: 7775
branch: vgeom/field-seed
priority: P2
cost: E
---


`crates/viewer/tests/display_budget.rs`,
`no_delta_renders_as_a_number_a_delta_cannot_be` (slow set), fails on
main (reproduced on 53116fdf43 by the PR 7764 review, and on the lane's
own main checkout): the `for end in [5.0e-324, MIN_POSITIVE, 1e3]` end of
its sweep panics with "δ 4.94e-321 mm renders as 5e-321, further from it
than the render's own grid". It reads only `DisplayTolerance::render_in`.
The cause is not established; a candidate is 76c79d89a5 (2026-11-23, "the
deciding figures are positional"). It reds the gate of every PR whose
diff edits `display_budget.rs`, and the nightly.

Assigned to `vgeom/field-seed`, which is on the δ field's render; it
says whether the render or the test's grid claim is wrong at a
subnormal.

## Closed

Closed (2026-11-25, PR 7775, `vgeom/field-seed`). The test's grid claim
was wrong and the render was right. The test read `render_in`'s text as
a millimetre float and bounded its error by that float's gap. The told
rule's floor is the spacing of the canonical floats at δ, in metres
(`geom_core` `told.rs`, `Float::of`). At δ = 5e-324 m that spacing is
all of δ, a thousand millimetre-float gaps, and `5e-321` mm types back
through the millimetre text door as δ exactly.

The row was masked until PR 7291 made the told render read the
once-rounded number of the unit (its commit series includes 76c79d89a;
not bisected to one commit). Behind it, the same reading also failed at
`COARSEST_DELTA`, whose render is a number of millimetres no float
holds.

The row now reads the render in metres through `UnitDef::canonical`.
The one genuine residue (the coarsest δ's nearest grid point reads back
as a δ the door refuses) is
`a-told-delta-at-the-top-of-the-type-reads-back-as-one-the-door-refuses`.

