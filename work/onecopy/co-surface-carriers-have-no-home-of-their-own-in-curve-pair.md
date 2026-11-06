---
id: co-surface-carriers-have-no-home-of-their-own-in-curve-pair
kind: issue
title: Two carriers on one surface have no co-surface reading in curve_pair; a lane can hand them to meet, whose 3D gap is identically zero ([ev]: C5 sentence + OnSurface type)
status: dispatched
opened: 2026-11-06
priority: P2
cost: M
refs: [region-landings-read-a-leg-against-an-edge-by-hand-rows-beside-meet]
---


From the off-surface landings fork (2026-11-06). The two designers agreed on this. The draw is on `analysis/design-fork/onecopy-landings-off-surface`.

## What

Two curves known to lie on one surface record have a 3D gap that is identically zero. So `curve_pair::meet`'s contact stage decides nothing well-posed for them: what it reads is the construction's rounding against the band.

- At Interval it escalates once the coordinate scale nears 1e3 m at ε 1e-12. The enclosure gap is about 3.4e-15 × scale, measured on a probe.
- Nothing in the types stops a caller from handing such a pair to `meet`. PR 6491's first route did exactly that.

The region doors now read a plane-section crossing instead (the row it refs).

## What closing it takes

1. **A type for a curve on a surface.** `OnSurface<'s>` or `Section` holds a span plus the surface record it lies on. It never converts implicitly to a bare `Span`.
2. **A co-surface arm in `geom_brep::curve_pair`.** It shares `transverse`'s harmonic generator and reads the surface's component in place of `contact`. Then the region doors and the sphere cut-in (`apply_cut_ins`) read it by type.
3. **The rule in the design doc.** This extends C5's table, so it is an `[ev]` PR. Until then, add a clarifying sentence to `[one-way.readings]` or C5 (also `[ev]`): two carriers on one surface meet as a crossing of a plane section, or in the chart, never as a 3D curve-pair gap.

It earns its place with a second consumer. The sphere cut-in is one.
