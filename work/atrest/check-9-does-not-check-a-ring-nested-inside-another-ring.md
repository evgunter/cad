---
id: check-9-does-not-check-a-ring-nested-inside-another-ring
kind: issue
title: tier 3 accepts a face whose ring lies inside another ring of the same face: check 9 nests each ring against the outer loop only
status: open
opened: 2026-09-25
priority: P1
cost: D
---


## Finding

Check 9 (`crates/topo/src/validate.rs`, `ring_outer_contact` +
`ring_nesting`) decides each ring against its face's OUTER loop only
(`ring_nesting(body, face.outer, ring, …)`). Nothing asks whether one
ring of a face lies inside ANOTHER ring of it — a ring in a region the
face does not cover, which check 9's outer test cannot see because
that region is inside the outer loop.

## Repro (measured 2026-09-25, `band/ruled-d-hole-ring-crease`)

A keyhole through-hole: block `[−1, 1]² × [0, 1]` with profile rings
(a) the keyhole — disc R = 0.5 about the origin plus the slot
`[√0.21, 0.8] × [−0.2, 0.2]`
(`crates/sweep/tests/review_band_ruled_ring_probes.rs`, `keyhole_block`)
— and (b) a bore `profile::circle((0.4623, 0.204), 0.001)`. Extruded;
tier 3 `Ok`. `fillet_edges` on the two disc/slot creases (`rod_creases`)
at r = 0.1 carves both. The bore sat in material the convex band
removes, so on each cap the bore's ring now lies inside the keyhole
ring's (rounded) region. `validate_geometric` returns `Ok(())`, and
`mass_properties` still subtracts the bore: `ΔV` = −2·A exactly
(`keyhole_cut(0.1)`), off from the true body by π·1e-6.

The upstream defect that produces this body is
`work/band/ruled-cut-off-leaves-a-cap-ring-inside-the-removed-sliver`;
this row is that tier 3 does not catch it.

## What the taker owes

A ring-vs-ring nesting arm in check 9 (two disjoint rings of one face:
neither inside the other), on the instruments check 9's nesting arm
already has, with this fixture as its row — or the gap stated in check
9's banner beside the ones
`check-9-nesting-arc-parity-and-no-walk-wait-on-the-arc-aware-walk`
carries. (The keyhole ring is arc-bearing, so its class may be that
item's `ArcParity`/`NoWalk` too.)
