---
id: vtxfac-tangent-sector-should-descend-per-bound
kind: issue
title: vtxfac's declared-Tangent sector should descend per bound once the band-edge arc split is fixed
status: parked
opened: 2026-10-01
blocked_on: [an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band]
priority: P3
cost: E
---

## What

`vtxfac::classify_vertex_on_face` lumps a declared-`Tangent` sector
whose two bounds read `On` WHOLE (`sectors::tangent_lump`: the
transverse direction's second-order side, both bounds). The per-bound
reading (`sectors::tangent_relative_side` per bound, as the v-v door
`recl_sectors` reads a declared-`Tangent` record) is the finer one: a
bound riding the tangency locus stays `On` and the other takes the side
its carrier curves to. `vtxfac`'s `resolve_on_entries` resolves an
isolated `On` from its neighbours and refuses only consecutive ones, so
one `On` bound is within what the door handles.

Per bound refuses today only because of a defect. On the one fixture
that reaches the arm (`crates/sweep/tests/m9_3_zip.rs`,
`a_tangent_curved_sector_on_a_face_lumps_whole`), the second bound is
the arc, cut to a 2.1e-8 m sliver by the band-edge split
(`work/hone/an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band.md`),
so its second-order margin is inside the zero band and it reads `On`
too: two consecutive `On`s, and all three ops refused
`ClassificationInvariant` (measured with a temporary per-bound build).
With the split fixed the arc should read its own side (`Out`), leaving
one `On`.

The split between the two forms is per door, not `vtxfac` against
`recl`: `recl` itself reads per bound at the v-v door and whole-sector
at the e-e door (`recl.rs`'s flanking pairs, `tangent_lump`).

## When the blocker closes

1. Switch `vtxfac`'s declared-`Tangent` branch to per bound and run the
   fixture above: the three ops should hold their answers.
2. Reconcile with
   `work/contact/boolean-conic-side-code-zero-is-first-order.md`'s
   prescription (descend to second order at a first-order Zero for
   every curved bound, keep `On` only for a second-order tie): the
   per-bound reading here is that prescription for declared pairs, so
   the two should land as one rule.
3. Rewrite the site comment above the branch, which states why it is
   whole-sector today.

Filed from PR 3747's style review (the TANG m9-3 residues fix pass).
