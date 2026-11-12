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
spline chart as a `General` row (`edge_nurbs::general_chart_image`), so a
face on a RATIONAL spline chart can now carry a `General` row at rest.
Its volume reaches `props::quad::trimmed` `trimmed_patch_face_rounds`
(near :1515), which refuses any rational net:

    QuadratureUnsupported { what: "a RATIONAL chart reached the trimmed
    lane — the chord integrand is the polynomial ⟨S, S_u×S_v⟩, and a
    quotient's is not polynomial in the chord parameter …" }

(PR 6532 dropped the message's closing claim, "the rational chart
carries IsoArc rims only on this head", which no longer held.)

## Witness

`topo::test_support::cyl_wall_sheet_tilted` swapped onto a rational ring
net in two sub-arcs carries such a face: its bottom rim, a circle or an
ellipse no row holds, mints a `General` row on the rational chart
(`crates/topo/tests/a_bending_carrier_on_a_spline_wall_takes_the_general_lane.rs`).
That body is a sheet, so it has no volume and does not reach the lane.
No shipped body does today. dm1's l-bracket did on PR 6532's first head,
until PR 6528 gave its rim held end weights (`Pcurve::IsoArc`'s `ends`)
and the rim took its row. A closed body on a rational wall whose rim no
row holds would reach the lane; nothing here built one.

## Fix

A rational chart in the trimmed lane: the integrand
`⟨S, S_u×S_v⟩` is a quotient, so either a rule with a certified
remainder for it, or a polynomial reparameterization of the face's
region.
