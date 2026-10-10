---
id: a-requested-ruled-ring-edge-refuses-at-the-ring-meter
kind: issue
title: blend: a ruled band's requested edge in a planar support's ring refuses at the ring meter, though its strip may clear
status: open
opened: 2026-10-07
priority: P3
cost: M
---



## Finding

Arm (a) of `surgery::ring_clearance_pass` meters every ring of an open
link's support against the link's UNBOUNDED trimline. For a PLANAR
link whose requested edge lies in a ring, that ring is now arm (d)'s
(`surgery::strip_rings`, `surgery::strip_clearance`), metered against
the strip between the edge's stations. A RULED link keeps arm (a) for
every ring, because arm (d) meters planar strips alone: so a ruled
band's requested edge lying in a ring of its planar support (a blind
cylindrical groove's mouth line, read at its own trimline through
`co_requested_trim`, sits at margin zero) refuses `RingClearance`
however small the band, by the same mechanism the planar row
`a-requested-ring-edge-refuses-at-the-ring-meter` closed.

Not measured: no fixture builds the blind groove yet, so the first
step is a witness row (both verbs) confirming the refusal and that
nothing earlier — `open/ruled.rs`'s plan gates, predicate 6 at the
groove's end walls — stops it first.

Conservative, not unsound: nothing is built.

## What would close it

A witness first. Then meter that ring against the ruled band's strip
on the plane — bounded by its two caps, as `RuledPlan` holds them —
the way arm (d) meters a planar strip, and leave arm (a)'s unbounded
trimline for rings that carry no requested edge.
