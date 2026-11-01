---
id: a-dipping-round-refuses-at-a-run-boundary-on-its-coplanar-bottom-sector
kind: issue
title: A quarter round dipping into a slab (an overlapping tangency) refuses every op: the glue door has no class for a tangency that claims no material sides, so C8 refuses the undeclared curved sector
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [boolean-conic-side-code-zero-is-first-order, an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band]
design: true
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
boundary lands beside it on the bottom sector. The union refused
`TangentSlitArmUnbuilt` with the auto-declared `Tangent` pair then; that
refusal is retired (`contacthold/face-face-touch-row`, PR 4921), and the
union answers one solid (the PR 4831 section below).

This is not the arc's reading (PR 4575's descent leaves it, the lump
keys on first-order ties), and not
`work/honeroot/an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band.md`,
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

## Weighed (2026-10-12)

A designer pair (fork log row 118) converged in one round: the glue
door's tangency verdict claims no material sides (`GlueClass` becomes
`Rest | Continuation | Tangent`; `Contact(Tangent)` and `Seam` collapse
into `Tangent`), and every consumer derives the side it needs. That
changes C4's seam text, so it went to Ev as an `[ev]` PR. The build then
meets this row's original `sectors::along` refusal, the remainder.
Build note: the fixture carries a second wedge-0 edge at (x = 3, z = 0)
lying in the slab's bottom face; a slab spanning z ∈ [−1, 1] isolates
the rim.

## Built: the claim-free glue (PR 4831); the remainder is `along`

PR 4831 glues the dip's wall `Tangent`. On a slab z ∈ [−1, 1]
(`the_dip_on_a_deep_slab_unions_into_one_solid`) ∪ answers one solid
(80), tier 3 clean; slab ∖ round, round ∖ slab and ∩ refuse
`sectors::along` "a germ sector with both edge bounds on the partner face".
The original fixture reaches the same refusal for those three, and its ∪
escalates `Corner(Straight)` at the round's own wedge-0 corner in the
slab's bottom face.

Traced: at the rim vertex the bottom sector (Rest lump) and the wall
sector (tangent lump, In) share the rim bound; one lump overwrites the
other's class there, so the run's transition lands on a lumped sector with
both bounds On. Taking the germ from the bound the neighbour's lump took
over (the rim edge as cell, its ray as direction, in `vtxfac`'s `germ_of`)
reaches `chord_join::bool_planar_chord_spec`'s "tangent plane×wall germ
pair in the boolean join" frontier; naming the planar neighbour's face
instead makes the join try to divide the wall ("curved section chord
outside the split/wall lane"). The remainder needs a tangent-line chord arm
on the planar side of the boolean join (the split lane has one,
`chord_join`'s `SectionCase::Tangent`), with the germ attribution above.

## A second fixture: a bore inside a disc, caps coplanar (PR 4831's fix pass)

A 6 × 6 plate with a bore of radius 1 about (1, 0) against a disc of
radius 2 about the origin, both z ∈ [0, 1]: the walls touch along the
ruling through (2, 0) with their outward normals opposed and the crescent
shared, the overlapping tangency C7 says builds. The pair glues `Tangent`,
and all four ops refuse `Join` ("8 section ends found no partner") at
ε 1e-6, 1e-9 and 1e-12 (`review_4831_probes::p1_a_bore_inside_a_disc_touching_its_wall`,
which pins it).

Its vertex shape is this row's: at the ruling's end vertices the disc's
cap sector (a continuation lump with the plate's cap) and its wall sector
(a tangent lump, `In`) share the cap's circle edge as a bound. Measured:

- the same pair with the disc run past the plate (z ∈ [−1, 2]), so no
  cap is coplanar, answers every op at its closed form
  (`p1_a_bore_inside_a_tall_disc_touching_its_wall`);
- the aligned postures with coplanar caps answer: the nested touch (`p6`)
  and the bore in a bigger bore (`p2`);
- with the disc's or the bore's seam vertex turned off the ruling, the
  ops that do not refuse `Join` refuse `CurvedPierceUnsupported` at the
  crossing layer.

The end refusal differs from the dip's (`Join` rather than
`sectors::along`) and the mechanism is not traced to a line; filed here
because the vertex shape is the same, a coplanar lump beside a tangent
lump on one bound.
