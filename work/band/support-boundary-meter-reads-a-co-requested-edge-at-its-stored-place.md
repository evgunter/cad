---
id: support-boundary-meter-reads-a-co-requested-edge-at-its-stored-place
kind: issue
title: blend: the rim support-boundary meter reads an outer-boundary edge that is itself requested in the same call at its stored place, not its trimline
status: open
opened: 2026-10-01
priority: P2
cost: M
---


## Finding (by reading; not measured)

`ring_clearance_pass`'s support-boundary walk
(`crates/sweep/src/blend/surgery.rs`, arm (b)) meters each outer-boundary
edge of a closed rim's supports at its STORED place. When that edge is
itself requested in the same call — another rim's arc on a shared
support (two annulus rims on one wall or cap carve together), or an
open link's edge in a ladder host's outer cycle (a single call may blend
the box edges and the rims together) — its own carve moves that
boundary to its trimline, and the meter reads the gap before the move.
The ring meter answers the same question for RINGS by widening a
requested ring to its trim (`effective`); the outer-boundary walk has no
such widening, so those pairs are metered exactly only by predicate 2's
screen, which subtracts both setbacks but reads at sample stations.

On a shared revolution wall the two rims are coaxial latitudes whose
samples share azimuths, so the screen is exact there; an open link in a
ladder host's outer cycle has no such argument.

Found while generalising that walk to annulus hosts and mates
(`band/annulus-host-outer-metered`).

## What the taker owes

Measure whether a ladder host whose outer-cycle line is requested
alongside the rim can pass the screen off-sample and carve wrong; if so,
meter a requested boundary edge at its trimline on that face, as
`effective` does for rings.

## Measured

The ladder case the finding names does not carve wrong: arm (a) meters
an open link's trimline against every ring of each of its supports,
widening a requested ladder ring to its trim (`effective`), so the box
edge beside a ladder rim is read exactly whatever the screen samples
(`band_co_requested_boundary::a_ladder_hosts_co_requested_box_edge_is_metered_exactly_by_the_ring_arm`:
a hole whose stations miss the near side, the screen passes at
`r = 0.26` and the ring arm refuses at `0.5 − 2r`).

Two coaxial rims on one shared wall are exact at the screen: the wall's
seam meridian puts a vertex of each rim on one azimuth, a station of
both
(`two_coaxial_rims_on_a_shared_wall_are_read_exactly_by_the_screen`:
gap `1.0`, margin `1 − 2r`).

An open PLANE–PLANE link cannot sit in an annulus host's outer cycle:
every vertex of an open chain is a joint (valence 2) or a corner whose
three edges are all requested, so walking that cycle from the link
reaches the host's seam foot, which refuses. A RULED link can — it ends
at transverse caps, whose cap edges are unrequested — and it carved
wrong: a rod on a washer's bottom face, requested with the bore rim,
put the rod's trimline 0.044 inside the rim's trim circle at
`r = 0.25` and returned a tier-3-valid body whose bottom face boundary
crosses itself
(`a_ruled_link_co_requested_with_an_annulus_rim_is_metered_at_its_trimline`,
red before the fix).

## Closed

The support-boundary walk reads an outer-boundary edge requested in
the same call at its trim on that face (`co_requested_trim`): an open
link's trimline over its stored window projected onto it, a co-requested
rim arc at its rim's whole trim circle. The sibling in arm (c) is
`ruled-cap-meter-reads-a-co-requested-ring-at-its-stored-circle`.
