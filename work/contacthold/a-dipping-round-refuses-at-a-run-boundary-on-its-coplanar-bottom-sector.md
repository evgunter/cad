---
id: a-dipping-round-refuses-at-a-run-boundary-on-its-coplanar-bottom-sector
kind: issue
title: A quarter round dipping into a slab refuses its differences and intersection at sectors::along, a run boundary on its coplanar bottom sector
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [boolean-conic-side-code-zero-is-first-order, an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band]
---


## Finding

From PR 4575 and both its reviews.
`crates/sweep/tests/an_arc_tangent_at_a_boolean_vertex.rs`'s
`the_untilted_dip` (print-only; run
`cargo test -p sweep --test all the_untilted_dip -- --ignored --nocapture`)
prints the untilted `dip`: a quarter round whose bottom face rests
on the slab's top (z = 1) up to the rim x = 2, and whose cap arcs leave
the rim vertices tangent to the top and bend down into the slab, so the
two overlap in a lens of volume `4 − π`.

Undeclared (the boolean verifies and declares the wall's tangency
itself), slab ∖ round, round ∖ slab and slab ∩ round refuse
`ClassificationInvariant { what: "a germ sector with both edge bounds
on the partner face" }`, raised in `sectors::along`. The sector is the
round's coplanar BOTTOM sector at the rim vertex (its edges, the bottom
profile edge and the rim, both lie on the slab's top): the wall touches
the top along the rim and lumps whole as a tangent sector, and a run
boundary lands beside it on the bottom sector. The union refuses
`TangentSlitArmUnbuilt` with the auto-declared `Tangent` pair.

This is not the arc's reading (PR 4575's descent leaves it, the lump
keys on first-order ties), and not
`work/hone/an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band.md`,
whose refusal is `vtxfac::pierce_germ_dir`'s "pierce transition on a
coplanar sector" at the lumped wall sector. Forcing the arc's every
reading to its far vertex moved this refusal to that one, by unlumping
the wall. The answer wants a run boundary on a lumped coplanar sector to
take its germ from the tangency locus, not from the sector's bounds.
