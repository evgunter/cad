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

## Traced: three changes, each of which closes it

The fork's three designers read the numbers three ways. The
measurement lane ran each as a variant on its own, on top of the
refusing configuration (the sliver row's `## Measured`, section 9).
**All three hold the drum at `Interval`:**

| variant | verdict | pairs |
|---|---|---:|
| as measured (sliver refused, margin un-scaled) | refuses | 15,200 |
| **the line placed as a point**: the band's chart line on the shared circle given its `f64` value `[x, x]` in `placed` | holds | 559 |
| **the separator's direction from the chart's derivative across the zero side**, not from the rectangle's middle (`heading`, side and past-side zeros) | holds | 683 |
| **a separator across the seam**: the cylinder through the shared circle about its axis, tried before the graph | holds | 665 |

For scale: the drum at `Interval` on main, with the sliver, takes 567
pairs, and at `f64` 150.

- **With the line a point the far-side cell does not arise**: no
  sliver decision is taken at all, and the touch reads the cell
  outside, as at `f64`. The line's enclosure, 2.1e-14 rad where the
  construction fixes π/2, is where the refusal starts. The same
  mechanism refused the drum's wall rim until
  `a-fillet-band-rim-phase-outruns-the-touch-concession-at-interval`'s
  fix placed that rim from the sketch frame.
- **With the heading or the seam cylinder the thin rectangle is still
  presented, and is read**: 124 and 110 sliver-width rectangles arise,
  and the pairs certify (by the graph separator, or by the seam
  cylinder) or split and certify below.
- **What separates the 321 visits the graph certified from the 11,512
  it did not** is the graph separator's rate alone: decided on both
  faces where it certified (enclosures centred about 2.4 half-widths
  from zero), and under a tenth of a half-width from zero where it
  split. The cells, their widths and their place against the seam are
  the same in both groups. The trace did not record the direction
  itself; the heading variant holding is the evidence that the
  direction is what differs.

So this is not one missing reading but two places to act: where the
line is placed (it removes the cell), and how a rectangle without
width is read (it reads the cell). The first acts where the refusal
starts. The second is what any family whose line is honestly an
enclosure will need.

## What closing it takes

The drum at `Interval` holds with `present_chart`'s sliver refused,
pinned so. It gates that sliver's retirement
(`the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause`).
`teapotlid`, `hollowring` and `hollowtorus` refuse the same way once
the touch concession's `few_ulps` is also 0 (the measurement's section
8): the touch is not this row's, but the reading that closes this row
is the first thing to try there.
