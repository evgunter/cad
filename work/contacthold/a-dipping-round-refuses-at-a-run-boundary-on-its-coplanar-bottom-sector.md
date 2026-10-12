---
id: a-dipping-round-refuses-at-a-run-boundary-on-its-coplanar-bottom-sector
kind: issue
title: A quarter round dipping into a slab (an overlapping tangency) refuses every op: the glue door has no class for a tangency that claims no material sides, so C8 refuses the undeclared curved sector
status: parked
opened: 2026-10-10
priority: P2
cost: M
refs: [boolean-conic-side-code-zero-is-first-order, an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band]
design: true
blocked_on: [4723]
---


## Finding

From PR 4575 and both its reviews.
`crates/sweep/tests/an_arc_tangent_at_a_boolean_vertex.rs`'s
`the_untilted_dip` (print-only; run
`cargo test -p sweep --test all the_untilted_dip -- --ignored --nocapture`)
prints the untilted `dip`: a quarter round whose bottom face rests
on the slab's top (z = 1) up to the rim x = 2, and whose cap arcs leave
the rim vertices tangent to the top and bend down into the slab, so the
two overlap in a lens of volume `4 − π`.

Undeclared (the boolean verifies and declares the wall's tangency
itself), slab ∖ round, round ∖ slab and slab ∩ round refuse
`ClassificationInvariant { what: "a germ sector with both edge bounds
on the partner face" }`, raised in `sectors::along`. The sector is the
round's coplanar BOTTOM sector at the rim vertex (its edges, the bottom
profile edge and the rim, both lie on the slab's top): the wall touches
the top along the rim and lumps whole as a tangent sector, and a run
boundary lands beside it on the bottom sector. The union refuses
`TangentSlitArmUnbuilt` with the auto-declared `Tangent` pair.

This is not the arc's reading (PR 4575's descent leaves it, the lump
keys on first-order ties), and not
`work/hone/an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band.md`,
whose refusal is `vtxfac::pierce_germ_dir`'s "pierce transition on a
coplanar sector" at the lumped wall sector. Forcing the arc's every
reading to its far vertex moved this refusal to that one, by unlumping
the wall. The answer wants a run boundary on a lumped coplanar sector to
take its germ from the tangency locus, not from the sector's bounds.

## Re-measured on main b59055db63 (2026-10-12): the refusal moved, and its premise is gone

`the_untilted_dip` now refuses all four ops `CurvedBooleanUnsupported
{ face: <the wall>, kind: Cylinder }`, raised by the C8 guard in
`vtxfac::classify_vertex_on_face` ("a CURVED on-carrier sector is
opened by a VERIFIED declaration and by nothing else"); `sectors::along`
is no longer reached. A first-parent bisect from PR 4575's merge
(aeab3e2ae5, which reproduces the finding above) puts the move at PR
4604's merge (758bd7acb4, `contacthold/at-rest-cell-pairs-census`).

What 4604 changed: the C4 `Tangent` table now reads the material
crescent (`contact_verify::tangent_table`, `contact_tangent_overlap`),
so a `Tangent` CONTACT whose two materials overlap between the sheets
is contradicted. The dip's wall and the slab's top are exactly that:
tangent along the rim, normals opposed, and the lens `4 − π` between
them is in both materials. A probe in `glue::tangency` shows the glue
door (`glue::decided_declarations`) now offers no class for the pair:
`Contact` refuses `contact_tangent_overlap`, `Seam` refuses
`seam_senses_aligned`. With no glue class the wall sector is an
undeclared curved on-carrier sector, and C8 refuses it.

So the finding above was reached only through a false auto-declaration:
before 4604 the door glued the pair as a `Tangent` contact it is not,
which is also why the union refused `TangentSlitArmUnbuilt` (#131's
doubled cusp of two bodies touching along a line), a refusal that does
not describe this union, where the wall inside the slab simply goes.
The run boundary on the coplanar bottom sector needs the wall sector
lumped `In`, which is the overlap itself, so no non-overlapping posture
reaches `along`'s refusal either: the upright round on the wide slab
(the wall curving away) answers at every op.

**Open question, ahead of `along`:** an *overlapping* (internal)
tangency between operands is a coincidence the kernel infers from
values, so under D10 it is recorded at the glue door; the door's
vocabulary (`GlueClass`: `Rest`, continuation, seam, `Tangent`
contact) has no class for it, and C7's second-order lump
(`sectors::tangent_lump`) opens only for a pair the door decides. The
two shapes in view are a door verdict for a tangency that claims no
material sides (opening the lump, whose definite `In`/`Out` reading
uses no declaration), or reading the lump in `vtxfac` straight off the
witness locus, which infers a coincidence outside the one door. Only
once one of them lands does the run boundary this row names become
reachable again, and the intersection would then meet MATE-7's unbuilt
wedge-0 cusp along the rim (`RimCuspArmUnbuilt` family).

## Parked (CONTACTHOLD orchestrator, 2026-10-12)

The re-measure turns the row into a design question: how an
*overlapping* (internal) tangency between operands is decided, at the
glue door as a new class (A) or read off the witness locus in `vtxfac`
(B), and what its intersection's wedge-0 rim becomes. It lands on C7's
tangency paragraph, which `[ev]` PR 4723 is rewriting for the external
touch, so it waits on that ruling and then goes to a designer pair with
the re-measure above as its evidence.
