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
chain, not by neighbourhood.

Where it lives in production: the battery breaks a chain at every turn
(`battery.rs` `broken_at_turns`) before the reach reads
`verdict.chains`, so a multi-link chain spans valence-2 joints only,
and two links that turn into each other are two chains that `meet`.
Chains that do not meet are metered against each other: a box's
top-front edge turning down the front-right edge into the bottom-right
one, whose first and last bands overlap below `h = 1`, refuses at the
meter and at the door
(`blend_band_reach_chain_ends::the_bands_of_chains_that_meet_at_turns_are_metered_against_each_other`).

What is left is two bands of one chain, or of two chains that meet,
coming together away from the vertex they share. Every chain the door
builds today is plane–plane, so its links are straight: one chain's
links are collinear through their joints, and two straight chains
through one vertex approach only at it, where the mitre or the corner
patch is the judge. The residue becomes reachable when a chain can
curve back on itself — mixed line–arc chains and open arc chains, which
the door refuses today (`UnsupportedChain`; see
`blend-reach-takes-an-open-arc-link-over-the-whole-turn`). Build the
witness then: a G1 chain whose arms close in, or two such chains
sharing a vertex.

The same holds for a chain's own supports: a link's reach skips every
support of every link of its chain. A far link's support that enters
this link's reach inside the strip its own band replaces would be
cleared by that band's `replaces` region anyway, so what is left is the
band-against-band case above.

The fix would meter the band surfaces of links of one chain, and of
chains that meet, against each other with each shared vertex's
neighbourhood taken out.
