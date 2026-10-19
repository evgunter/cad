---
id: pair-classes-refuses-partners-flush-along-an-edge
kind: issue
title: pair_classes refuses partners that lie flush along an edge, where planar flush faces decide On
status: closed
closed: 2026-10-15
opened: 2026-10-12
priority: P3
cost: M
refs: [pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided]
---

Filed by `pair-classes-keeps-per-pair-rows-where-its-layering-is-undecided`.

## What

`crates/topo/src/boolean/vtxfac.rs` `along_partners` reads an edge on
two or more partners' boundaries as `On` where some half-plane
(`sectors::half_planes`) is decidedly apart from every other partner's.
Where every half-plane meets another's, the partners lie flush along
the edge, and it refuses `Escalated` with
`BooleanDecision::PartnerFaces` and the decided zero's margin.

Two cases are refused alike there:

- two plane faces glued flush, material on both sides, where the edge
  is `In` (superseded: `On`, see Closed);
- a plane face and a curved one tangent along the edge, where the
  edge is `On` at second order.

The first-order half-planes cannot tell them apart. No suite row
reaches either. The site row `partners_parting_by_the_band`
(`boolean::offer_rows`) executes only the in-band arm.

## The shape to give

Read the flush case where every matched half-plane is a plane face's,
and the material lies on opposite sides of each match: there the
partners' materials cover the edge's neighbourhood, so it is `In`
(superseded: `On`, see Closed). Same-side matches overlap the
materials, which a body at rest does not hold; that is an invariant
(superseded: a typed refusal, see Closed). A curved face in a match keeps the refusal,
or reads the bend. That needs each matched sector's face shape:
`BoolSector` carries its `face` key, so the shape is a lookup on the
body that holds it (`FaceShape::of`), which `pair_classes` does not
take today.

## Closed

**The flush case reads `On`, not `In`** (orchestrator ruling, D4). The
shape above was built as written first, and the polygon-cone fuzzer
(`sectors::cone_fuzz`,
`the_polygon_cone_reader_never_contradicts_the_exact_oracle`, seed
`0x334ea383a21209f7`, effort 1) refused it on two pair probes:

- ε 1e-9, `OnTheFaces` (a partner in the cone's void touching its
  face): `In` where the exact oracle reads `On`. The oracle reads any
  direction on a partner's face as `On`.
- ε 1e-12, glued faces turned ~3e-7: `In` agrees with the truth, but a
  2.6e-23 m deviation flips it, so it is decided in band.

Glued faces are coincident only to within the zero band: a sub-band
tilt parts them by a sliver whose boundary the edge lies on, so only
`On` is decided there. With `On`, the fuzzer passes and the witness's
cells build to the same bodies as with `In`.

`along_partners` now takes the partners' face shape (`ShapeOf`, read on
the partners' body in `boolean::mod`), and `half_planes` returns each
half-plane with its sector. Where every half-plane meets another's at a
decided zero, and every meeting is of two plane faces bounding material
on opposite sides, the edge is `On`. Plane faces on one side are a
refusal too, `Escalated(PartnerFaces)`: a lump thinner than the band
glued on a face can make such a match, so a valid input may reach it
(review of PR 5207). A curved face in a meeting keeps the
`PartnerFaces` refusal, as does a chord in band of zero.

**Public witness** (`a_vertex_read_by_two_sector_passes`,
`an_edge_through_two_blocks_glued_faces_builds_in_every_op`, slow set,
and its per-PR pose `…_builds_at_rest`): two blocks in face contact,
their corners at `MEET`, against a pyramid standing there whose edge
runs through the contact face's interior and out of the blocks. On main
all 18 cells (3 pyramids, every op, both slots) refused
`Escalated(PartnerFaces)`. Now every cell builds, sound by the file's
`builds` (tier 3, tier 3′ with the blocks' own contacts in their glued
plane, the naming's classes, the material and inclusion–exclusion) at
every pose, and the union names the edge through the glued faces `On`.

Unit rows (`vtxfac::tests::a_touching_edge_on_two_partners_boundaries_reads_their_faces`)
pin the glued `On` at ε 1e-9 and, turned 3e-7, at ε 1e-12, and the
one-side, curved-face and in-band-chord refusals. No public pose is known for a plane against a tangent
curved face along an edge.

