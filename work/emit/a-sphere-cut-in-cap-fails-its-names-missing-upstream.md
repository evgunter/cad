---
id: a-sphere-cut-in-cap-fails-its-names-missing-upstream
kind: issue
title: A document's lens ∩ tilted brick (a sphere cut-in cap) fails EMIT with MissingUpstream naming the lens
status: open
opened: 2026-10-07
---


## The finding

A document of two revolved balls (the unit ball, and radius 0.8 at
height 1.4, each a full revolve of a semicircle about y), their
`Intersect` (the lens), and a 0.4 × 0.3 × 0.4 block rigidly turned so
its near face lies 0.985 from the origin toward latitude 68°, azimuth
50°, then `Intersect(lens, brick)`: the lens names (15 rows), and the
cap node fails `Naming(MissingUpstream { node: <the lens> })`, raised
at `names::defer::upstream_name`. Measured on `fuse/curved-join` with
the curved join switched off as well as on, so it does not come from
the join. The same pair builds at the kernel (`sweep`'s `snowman`
rows), so the gap is the emitter's: an entity of the lens the cap's
emission asks a name for is not one the lens's table holds.

## Why it matters

This is the one construction known to leave a conventional vertex (the
cap circle's, after its chart-meridian cut is glued away), so the
naming half of `work/fuse/curved-joinable-vertices-are-left-unjoined`
has no document that reaches it until this names.
