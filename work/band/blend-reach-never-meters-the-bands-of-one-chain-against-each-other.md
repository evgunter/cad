---
id: blend-reach-never-meters-the-bands-of-one-chain-against-each-other
kind: issue
title: blend: the reach never meters two bands of one chain, or of two chains that meet, against each other, anywhere — not only at the vertex they share
status: open
opened: 2026-10-07
priority: P2
cost: M
---


Found sweeping for `blend-reach-skips-every-face-at-a-chain-vertex`'s
class (excused by chain, where the excuse holds only near a vertex).

`band_reach` in `crates/sweep/src/blend/reach.rs` meters another band's
surface against a reach only for reaches that are `apart`: no chain in
common and no shared vertex between their chains (`meet`). The excuse
is that bands meeting at a corner or a turn touch there by construction,
which the corner predicate and the surgery judge. But the skip is by
chain, not by neighbourhood: two links of one chain, or of two chains
that share a vertex, whose bands come together AWAY from that vertex
(a chain that runs back alongside itself, a U whose arms close in) are
never metered against each other.

The same holds for a chain's own supports: `excluded` skips every
support of every link of the chain for each link's reach. A far link's
support that enters this link's reach inside the strip its own band
replaces would be cleared by that band's `replaces` region anyway, so
what is left is the band-against-band case above.

No body is known to reach it past the support screen
(`fillet3_face_clearance` refuses two strips crossing on one support,
e.g. `blend_band_reach_chain_ends::a_cut_off_face_at_another_links_end_is_metered_against_the_link`
whose door refuses at the screen); a witness should be built first,
likely two links of one chain with no common support whose bands
approach. The fix would meter the band surfaces of links of one chain
against each other with each shared vertex's neighbourhood taken out.
