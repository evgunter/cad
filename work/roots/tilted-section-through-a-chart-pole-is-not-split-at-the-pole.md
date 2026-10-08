---
id: tilted-section-through-a-chart-pole-is-not-split-at-the-pole
kind: issue
title: The join mints a tilted section arc that runs through a chart pole without a vertex there, where a pole on a face is a vertex of it
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [validity-refuses-an-interior-chart-singularity, tilted-section-near-a-chart-pole-refuses-arc-near-pole]
---


## Measured

Found in the review of PR 3817 and re-measured on its fix pass: two
unit balls, both `y`-poled, centres `c` and
`c + 1.4·(√(1 − 0.7²), 0.7, 0)`, whose radical circle passes exactly
through A's north pole. The join mints the section arc through the
pole with no vertex there, and the pcurve mint refuses
`Pcurves { Certify { ArcNearPole } }` (the near-pole sibling is
`pcert/tilted-section-near-a-chart-pole-refuses-arc-near-pole`).

By Ev's ruling recorded at
`restfront/validity-refuses-an-interior-chart-singularity` — a chart
singularity inside a face is a vertex of it — and by the same token
inside an edge: the chord should carry a vertex at the pole, as a
revolved ball's meridians end there.

## What a fix owes

Split a section chord at any pole of the divided face's chart its arc
passes through (a closed-form root: the circle meets the pole point),
so the arc's pieces each have a chart image; the row above, under
every op, against the lens closed form.
