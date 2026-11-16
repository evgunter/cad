---
id: the-lune-reading-depends-on-the-corner-fans-quarter-turn-split
kind: issue
title: A lune is read in one pose and refused in another: corner_fan's quarter-turn split against pair's mouth on a ray
status: open
opened: 2026-11-12
priority: P2
cost: M
---


Found by the remainder-refuses lane (PR 6854). Priority P2: no measured
body reaches it today, but it makes a valid body's verdict depend on
where its edges are placed rather than on its point set. Since PR 6854
the difference is a refusal against a rest, not a skip against a rest.

## What

`link::corner_fan` (`crates/topo/src/link.rs:3066`) turns a face's
corner fan at quarter turns from its edge. `second_order::pair`
(`crates/topo/src/link/second_order.rs:622`, the `along` check at
`:643`) finds a lune only where the sector running on past the mouth
has a ray of its fan exactly at the mouth.

So with that sector's edge turned off a quarter turn from the other
sector's, the mouth falls inside one of its wedges. The pair is then no
candidate, its sheets stay on each other, and the point refuses
undecided. The same geometry with the edge placed otherwise reads `Rest`.
`crates/topo/README.md:1461` states this ("in that pose only"). The
`a_rim_flush_…` body with its seam moved is the known case.

## The fix

Before pairing, cut the running-on sector's fan at the other sector's
rays, as `Link::pieces` cuts each sector at every ray of the link. Then
the mouth always lies on a ray of its fan, whatever the edge's pose. Pin
the fix with the `a_rim_flush_…` body turned so.
