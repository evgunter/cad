---
id: the-filleted-drum-at-interval-refuses-on-a-straddling-rate-at-its-tangent-seam
kind: issue
title: With the sliver refused, the filleted drum at Interval refuses across the circle its fillet band shares with its top plane: its pairs split on a rate enclosure that straddles zero
status: open
opened: 2026-11-17
priority: P1
cost: M
---


## Finding

From the sliver measurement
(`the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause`,
its `## Measured` section, 7), main at `362cd72fd`.

`a_filleted_drum_certifies_across_its_fillet` at `Interval` holds only
through `present_chart`'s sliver: 16 `Zero` decisions, each on a cell
that lies past the fillet band's end (minor angle π/2, the circle the
band shares with the top plane), where the band's chart line is placed
over an enclosure 2.1e-14 rad wide and the sweep's cut falls inside it.
Each of those pairs certifies with neither face read.

With the sliver refused and the departure margin not scaled by the
rectangle's span, the drum refuses `Uncertified` (faces 1 and 7, edge
12, depth 40; 15,200 pairs): 11,512 pair visits split on a rate
enclosure that straddles zero, 11,275 of them on both faces, and 321
certify by the graph separator. The band meets the top plane
tangentially along that circle. At `f64` the line is one value, no such
cell arises, and the drum holds.

## Three causes offered, not yet told apart

The fork's three designers read the same numbers three ways. A trace
decides between them (asked of the measurement lane, the row's section
9):

- **The line's placement.** The band's chart line on the shared circle
  is an enclosure at `Interval` where the construction fixes it at
  exactly π/2. Placed from the construction, the far-side cell does not
  arise. The same mechanism refused the drum's wall rim until
  `a-fillet-band-rim-phase-outruns-the-touch-concession-at-interval`'s
  fix placed that rim from the sketch frame.
- **The separator's direction.** `graph_of` and `planes_through` take
  their direction through `heading`, read at the middle of each
  rectangle. A rectangle with no width has its middle on the edge, so
  the direction is noise and both rates straddle.
- **No separator crosses a tangent seam.** Against either face's own
  carrier the rate is zero on a seam where the faces meet smoothly. A
  separator through the edge normal to the faces' common tangent plane
  (here the cylinder through the circle about the drum's axis) would
  read both at first order.

## What closing it takes

The drum at `Interval` holds with `present_chart`'s sliver refused,
pinned so. It gates that sliver's retirement
(`the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause`).
`teapotlid`, `hollowring` and `hollowtorus` refuse the same way once
the touch concession's `few_ulps` is also 0 (the measurement's section
8): the touch is not this row's, but the reading that closes this row
is the first thing to try there.
