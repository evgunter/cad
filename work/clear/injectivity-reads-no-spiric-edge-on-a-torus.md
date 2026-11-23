---
id: injectivity-reads-no-spiric-edge-on-a-torus
kind: issue
title: The injectivity check reads no spiric edge: a plane meeting a torus off its axis (the klein elbow's shelled caps) refuses Departure at the root, Structural on both sides
status: closed
opened: 2026-10-31
priority: P1
cost: H
closed: 2026-10-31
---


## Finding

Found by CAVITYDIAG (#6105); the full analysis is in
`shell-cavity-gate-over-refuses-three-valid-bodies`.

Offsetting the klein elbow's meridian caps along their normals moves
them off the torus axis. Each cap then meets the torus in a **spiric**
section, a quartic, instead of a circle.
- The plane side reads only lines and conics.
- The torus side reads only harmonic rows.
- Both are therefore `Structural`, and the pair refuses `Departure` at
  the root, with 0 splits, at both scalars.

Pinned: `the_klein_elbows_cavity_refuses_at_its_spiric_caps`.

## The design problem

Present the torus face beside a spiric edge. The edge is the zero set
of `ψ = (S − o)·n` in the torus chart. Read it as a graph over one axis
per cell, split at its turning points. Departure then uses the cap's own
plane as the separator, and the plane side needs nothing.

This is a new presentation arm: H, with a DUAL review.

Blocks SHELL-4.
