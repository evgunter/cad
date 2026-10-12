---
id: pair-classes-refuses-partners-flush-along-an-edge
kind: issue
title: pair_classes refuses partners that lie flush along an edge, where planar flush faces decide In
status: open
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
  is `In`;
- a plane face and a curved one tangent along the edge, where the
  edge is `On` at second order.

The first-order half-planes cannot tell them apart. No suite row
reaches either. The site row `partners_parting_by_the_band`
(`boolean::offer_rows`) executes only the in-band arm.

## The shape to give

Read the flush case where every matched half-plane is a plane face's,
and the material lies on opposite sides of each match: there the
partners' materials cover the edge's neighbourhood, so it is `In`.
Same-side matches overlap the materials, which a body at rest does not
hold; that is an invariant. A curved face in a match keeps the refusal,
or reads the bend. This needs the sector to carry its face's shape:
`BoolSector` does not carry it today.
