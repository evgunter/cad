---
id: a-material-crescent-face-face-record-refuses-typed-at-the-op-and-the-carry
kind: issue
title: Until the cusp-edge arm exists, an op or carry that would land a face–face record whose crescent reads material refuses typed, not ContactContradicted
status: open
opened: 2026-10-20
priority: P2
cost: M
refs: [a-material-crescent-tangent-line-builds-two-coincident-cusp-edges]
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

## Merged in: `an-at-rest-shared-crescent-reads-as-a-contradicted-contact-not-an-interference-finding` (2026-11-25)

Folded in at the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`): the same mechanism, fixed by the same change. Its title was: At rest, a shared crescent on a two-carrier face-face record reads ContactContradicted rather than an interference finding. Its body follows, headings demoted.

### What

census reads a shared crescent (both materials fill the thin region beside a tangent locus) on a two-carrier face-face record as ContactContradicted { Tangent }; D10's Assertions paragraph makes an overlap between two copies an interference finding. Decide which the census reports.

Found by a designer of the overlapping-tangency fork (fork log row 118), read from the code at 63288134b8, not measured.

### Note (PR 4831)

Untouched by the claim-free glue build: the census confirm
(`tangent_table_at_rest` → `contact_verify::tangent_locus_relation`) still
reads the crescent as the `Tangent` contact's contradiction. The ruling on
PR 4775 places the crescent check there and calls a shared crescent
interference; which refusal the census reports is this row's.

### Answered (Ev, `[ev]` PR 5395, 2026-10-20)

Between two distinct solids a shared crescent is interference (C6), not
`ContactContradicted`: the census reports it as an interference
finding. Inside one solid, along a line it is a missing edge (the op's
defect), and at a point touch it is legal (C4).

Moved from `work/contacthold/` to `work/tang/` on 2026-11-25, when CONTACTHOLD closed (`docs/doc-ledger/contacthold-leaves-the-tracker.md`); id unchanged. The cleanup re-read the row against main at `6003877ad` and found it live.
