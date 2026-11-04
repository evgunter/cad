---
id: a-rational-spline-face-with-a-general-row-has-no-volume-lane
kind: issue
title: A face on a rational spline chart bounded by a General row refuses its volume at the trimmed lane, which reads polynomial charts only
status: open
opened: 2026-11-04
priority: P2
cost: H
---


Filed by `pcert/curved-general-image` (the row
`a-curved-carriers-degree-one-general-image-ends-as-a-kernel-bug`).

## What

The pcurve mint now images a circle no row holds, and an ellipse, on a
spline chart as a `General` row (`edge_nurbs::curved_chart_image`), so a
face on a RATIONAL spline chart can now carry a `General` row at rest.
Its volume reaches `props::quad::trimmed` `trimmed_patch_face_rounds`
(near :1515), which refuses any rational net:

    QuadratureUnsupported { what: "a RATIONAL chart reached the trimmed
    lane — the chord integrand is the polynomial ⟨S, S_u×S_v⟩, and a
    quotient's is not polynomial in the chord parameter … the rational
    chart carries IsoArc rims only on this head" }

The phrase "IsoArc rims only on this head" no longer holds.

## Witness

`step-import`'s dm1 (`tests/fixtures/wild/stepcode/dm1-id-214.stp`),
pruned to the l-bracket (`r1_dm1_probe::the_l_bracket_alone_adopts_its_reversed_slit`),
and whole in `tier_gate::every_corpus_import_passes_the_shared_gate`.
Wall `#382` is rational, and its arc rim, which no row of its split
one-span sub-patches holds, now mints its `General` row. The import
then refuses `TierInvalid { VolumeUncomputable { QuadratureUnsupported } }`
at every ε, where before this head it refused at the mint. Both rows
are re-pinned on that message.

## Fix

A rational chart in the trimmed lane: the integrand
`⟨S, S_u×S_v⟩` is a quotient, so either a rule with a certified
remainder for it, or a polynomial reparameterization of the face's
region. Until then the refusal's text should say what it is, not what
the head mints.
