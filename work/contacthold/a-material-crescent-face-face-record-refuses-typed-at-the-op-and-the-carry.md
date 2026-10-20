---
id: a-material-crescent-face-face-record-refuses-typed-at-the-op-and-the-carry
kind: issue
title: Until the cusp-edge arm exists, an op or carry that would land a face–face record whose crescent reads material refuses typed, not ContactContradicted
status: open
opened: 2026-10-20
priority: P2
cost: M
refs: [a-bore-tangent-to-a-plates-face-mints-a-face-face-record-the-census-contradicts, a-material-crescent-tangent-line-builds-two-coincident-cusp-edges]
---


Ev's decision on the pinch fork (`[ev]` PR 5395, 2026-10-20) makes a
material-crescent pinch along a line two cusp edges, never a face–face
record (`a-material-crescent-tangent-line-builds-two-coincident-cusp-edges`).

## What

Today plate ∖ an internally tangent rod builds and mints a face–face
row; the op's own 3′ then refuses `ContactContradicted`
(`contact_tangent_overlap`, through `boolean::tangent_table_at_rest`),
and so does every later op that carries the row. The verdict is right
and its name is wrong: nothing about the record is contradicted, the
op built the wrong form. A carry that lands a record whose crescent
turns material (the mould, `box ∖ bridged rod`) refuses the same way.

## Owed

Until the cusp-edge arm is built, the op refuses at its glue decision,
and the carry where it lands the record, with a typed refusal naming
the unbuilt arm (the `RimCuspArmUnbuilt` / `TangentSlitArmUnbuilt`
pattern): the face–face tangency's crescent reads material, so the
line owes two cusp edges. The census's at-rest reading of such a
record (a material crescent along a line inside one solid) reports the
missing edge as the op's defect, not `ContactContradicted`. Pins: the
bore row's probes (both A∖B rows, every scale and ε row) and the round-3
mould.


## The census variant (2026-10-20)

`contacthold/same-solid-discovery` (the unit
`the-census-discovers-no-same-solid-tangent-touch-through-face-interiors`)
added `ValidationError::CuspEdgesMissing { a, b, witness }`: the
census's report of a material crescent along a tangent line through two
faces of one solid with no edge on it, the op's defect. Its discovery
raises it for a pair no record names. The record's own confirm
(`census::confirm_face_pair`, through `boolean::tangent_table_at_rest`)
still answers `ContactContradicted` on `contact_tangent_overlap`:
plate ∖ the internally tangent rod (`sweep`
`face_face_touch::a_bore_tangent_from_inside_with_no_edge_on_its_line_is_missing_its_cusp_edges`'s
fixture), validated with the op's own records, refuses
`ContactContradicted { a: Face(5), b: Face(9) }`; validated with none,
`CuspEdgesMissing` on the same pair. This row's census half is that
confirm reporting `CuspEdgesMissing` for a same-solid pair, and its op
half can name the same defect.
