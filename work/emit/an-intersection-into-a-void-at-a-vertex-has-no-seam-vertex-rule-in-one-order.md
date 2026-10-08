---
id: an-intersection-into-a-void-at-a-vertex-has-no-seam-vertex-rule-in-one-order
kind: issue
title: An intersection whose operand crosses into a void at one vertex refuses SeamVertexParentage in one operand order and names in the other
status: open
opened: 2026-10-07
priority: P2
cost: M
---


Filed by TANG's touch-only re-read lane (PR 4256's second fix pass).

## What

A pyramid standing on its apex at a point, crossing the apex of
another pyramid that holds a pyramidal void at the same apex, names
`x ∩ y` and refuses `y ∩ x` with `NamingError::SeamVertexParentage`.
The other five ops name. The refusal is the emitter's missing-rule
word, raised from a named shape (`crates/editor-core/src/names/emit.rs`,
`SeamVertexParentage`), so no rule covers this seam vertex's
parentage in that operand order.

It is not the boolean's classification. On main, where the vertex is
read by vertex-vertex pairs alone, the bare case refuses the same way,
with only the arch's rows recorded. On PR 4256, the edge classes at the
vertex are layered over both partners, and are checked against an
analytic germ (`crates/topo/tests/a_vertex_read_again_classes_every_edge.rs`).
It still refuses the same way.

## Witnesses

`crates/editor-core/src/names/emit_topo.rs`,
`touch_reread_rows::a_vertex_read_twice_names_its_result_in_every_op`,
at every pose of `meeting::poses`, `y ∩ x` only:
- "over a void in the bare arch": `corners(50, 0.7, 0.5)`, standing
  at `MEET`, against the arch `corners(60, 0.5, 0.4)` less
  `nest(arch, 0.7)`. This is on main too (07fb2b6ce).
- "over the void in the arch": the same void, the arch united with the
  plate first. It builds from PR 4256 on, and was refused on main.
- "over the void, the island in it": the same, with an island in the
  void.
- "over a quad void in a quad arch" and "over a quad void in a bare quad
  arch": a quadrilateral arch over `bearing(40|80, 0.45|0.25, 0.5)`, less
  `nest_polygon(arch, 0.7)`. The plated one is from PR 4256 on; the bare
  one refuses on main too. Their `x ∪ y` refuses `Emission` "seam vertex
  parentage underdetermined", which is WIRE's
  `a-legal-declared-union-reaches-the-seam-vertex-parentage-residue-emission`.

All at ε = 1e-9 (`Tol::witness`).

The rows accept this refusal on those cells, citing this item.
