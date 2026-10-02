---
id: ruled-cap-meter-reads-a-co-requested-ring-at-its-stored-circle
kind: issue
title: blend: the ruled cut-off's cap meter reads a ring requested in the same call at its stored circle, not its trim
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## Finding (by reading; not measured)

`ring_clearance_pass`'s arm (c) (`crates/sweep/src/blend/surgery.rs`)
meters every edge a convex ruled cut-off leaves on its transverse cap
against the region enclosing the sliver it removes, reading each edge
through `stored_piece` — its STORED place. A ring of that cap can be a
LADDER rim requested in the same call (a bore through the D-rod, its
cap rim filleted with the creases); its own carve moves the cap's
boundary to its trim circle, and the meter reads the ring before the
move. Arms (a) and (b) widen such a ring through `effective`, and the
support-boundary walk now reads a co-requested edge at its trim
(`co_requested_trim`); arm (c) has neither.

Why it has not been seen: the support-boundary walk meters the cap's
outer cycle against the ladder rim's trim circle exactly, so that trim
disc lies inside the cap. A disc inside the cap whose radius is at
least the band's radius `r` cannot reach the sliver — the sliver is
exactly the part of the corner no such disc covers. A 90° rim's trim
radius is `ring + r ≥ r`. The gap is an OBTUSE rim (a countersink's
cone, setback `r·tan((π − θ)/2) < r`) whose trim disc is smaller than
`r` and sits inside the corner, with its stored ring clear of the
sliver's enclosure and its trim not.

Found while sweeping for `support-boundary-meter-reads-a-co-requested-edge-at-its-stored-place`.

## What the taker owes

Either build that countersink-in-a-corner case and see whether it
carves wrong, or read a co-requested ring at its trim circle in arm
(c) (`co_requested_trim` already answers "where does this edge bound
this face after its carve"), pinned by the row that reds without it.
