---
id: contact-verify-levers-a-tangency-at-the-locus-not-the-faces
kind: issue
title: contact_verify's tangency table levers a declared contact's second order at the locus's own extent, not its faces'
status: open
opened: 2026-10-15
priority: P3
cost: M
---


Filed by TANG U-lever (`tang/u-lever-the-wedge-is-levered-at-the-faces-extent`).

`topo::boolean::contact_verify`'s `tangent_table` caps every lever arm
of a declared contact's jet schedule at the locus's own extent
(`geom_brep::edge_extent(carrier, t0, t1, chord)`): the orientation read
`contact_tangent_opposed` and the second-order
`contact_tangent_second_order`. The edge certifier and the must-carry
rule now lever the same pair at the farther face's extent
(`topo::wedge_reach`), so a short declared contact between large faces
reads its sagitta at its own length here and at the faces' extent once
it is an edge. `tangent_table`'s public callers
(`contact_pair_verdict`, `tangent_pair_relation`,
`tangent_locus_relation`) take surfaces, not faces, which is why it was
left: the faces would have to be threaded in. This is issue 1439's
sibling (the hand-rolled second-order spelling `tangent_second_order`'s
docs name).
