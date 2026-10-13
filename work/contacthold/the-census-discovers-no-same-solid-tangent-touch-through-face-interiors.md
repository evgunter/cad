---
id: the-census-discovers-no-same-solid-tangent-touch-through-face-interiors
kind: issue
title: One solid pinched along a tangent line through two faces' interiors passes tier 3′ with no record: the census has no same-solid face–face discovery
status: open
opened: 2026-10-12
priority: P1
---


Found building the face–face row (`contacthold/face-face-touch-row`):
the "C9's same-solid discovery" step the bridge row's Weighed section
names (`a-bridge-union-fuses-a-declared-tangent-rest-into-one-shell-with-an-edgeless-contact`).

## What

`sweep` `face_face_touch::a_rod_on_a_plate_records_its_touch_and_a_bridge_lands_it`'s
fixture: a rod resting on a plate's side face along a ruling through
both faces' interiors; the union is two solids with the face–face
record. Union that result with a box bridging the rod's top to the
plate's, handing it NO records (`topo::union`):

```
solids=1 shells=1 v=18.533794219 kinds=[] t3=true 3'(records)=Ok(()) 3'(none)=Ok(())
```

One solid, its rod wall still on the plate face along z ∈ [0.25, 0.6],
no edge on the ruling, no record, and `validate_pseudomanifold` passes
it. Carrying the first union's face–face record, the same op lands it
and 3′ passes too, so the census confirms the record but never asks for
it. The census's face × face reading is the cross-solid backstop
(`census::sweep_cross_solid_backstop`), and "same-solid distinct-key
curved pairs are not detected; they are their constructor's obligation"
(`crates/topo/README.md` C2). Here the constructor was handed an
operand already missing its record, so nothing in the chain owes it.

## Owed

The census pairs a solid's own faces on distinct carriers that touch
along a tangent line through both interiors (the two-carrier lane it
already reads for a record, `boolean::tangent_pair_at_rest`) and
refuses one no record names, `UnrecordedContact`. Then the plain bridge
above refuses 3′ and the carried one passes.
