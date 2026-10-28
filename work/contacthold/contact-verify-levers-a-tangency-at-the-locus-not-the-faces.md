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

## The edge extent's new home (OFFSET, 2026-10-27)

`geom_brep::edge_extent` no longer exists (OFFSET, PR 5875). Its home is `geom_brep::extent`, beside the reach family, as three functions: `edge_extent_inf` (≤ the edge's diameter `D`: the chord, the minor-axis folds and the sampled pairs; the lever of a reading whose Zero only refuses), `edge_extent_sup` (≥ `D`; a reading whose Zero is acted on and whose definite signs only refuse) and `edge_reach_from` (the edge's farthest distance from a point of it; a reading that acts on every sign, read from that point). `contact_verify::tangent_table` now levers at `edge_extent_sup(carrier, t0, t1, chord)`: its parallel reading's Zero verifies the contact and a definite sign contradicts. The faces'-reach question this row asks is unchanged.
