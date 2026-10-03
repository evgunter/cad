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
