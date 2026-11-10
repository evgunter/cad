---
id: an-end-torus-link-cut-off-in-its-meridian-plane-is-unbuilt
kind: issue
title: A torus link at an open chain's end refuses, though its cut-off in a plane through its axis is a circle
status: open
opened: 2026-11-10
priority: P3
cost: M
refs: [an-open-tangent-chain-of-mixed-links-refuses-at-the-open-carve]
---


Filed by the band-openmix lane (2026-11-10), measured while building the open mixed chain.

An open chain admits a torus link only between two junctions (`OpenBand::admit`, `crates/sweep/src/blend/admit.rs`). A torus link at an open chain's end refuses `UnsupportedChain`: "an open chain's link is neither plane–plane, nor a ruled cylinder pair, nor a torus between two junctions with links on other supports". The witness is a half-disc's arc requested alone, `crates/sweep/tests/band_open_mixed_chain.rs::a_torus_link_at_an_open_chains_end_refuses_at_the_open_door`.

The ratified cut-off rule already decides one case of it: the band ends in the end face's plane section of it. When the end face is a plane containing the torus's axis (a meridian plane), that section is the minor circle, a stored kind. A D-shaped outline's arc requested alone meets its diameter's wall like that. Any other end plane cuts a spiric quartic with no stored carrier, and it stays refused (D3). A torus band's mitre against a neighbour, and a corner patch over a curved support, stay refused as well.

Building the meridian cut-off takes:
- an `EndCut` over a torus band;
- a sliver meter for the region between the circle and the old vertex;
- the end face's carve with a circular trim on the curved mate.

No consumer has needed it yet: the bracket's chains (`a-boss-on-a-plate-meets-it-at-a-mixed-convexity-corner-no-run-out-rounds`) carry their torus links between junctions.
