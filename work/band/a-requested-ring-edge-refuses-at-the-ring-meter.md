---
id: a-requested-ring-edge-refuses-at-the-ring-meter
kind: issue
title: blend: a requested straight edge of a support's ring refuses at the ring meter, though its strip may clear
status: open
opened: 2026-10-06
priority: P3
cost: M
---


## Finding

A straight edge in a RING of a planar support face can be requested on
its own: a pocket whose outline has a tab reaching into it has a mouth
edge at the tab's tip between two corners whose three edges are all
convex, so predicate 6 classifies both ends `EndFace` and the local
carve would cut the band off in the tab's side walls.

It refuses `RingClearance` at arm (a) of `surgery::ring_clearance_pass`
instead. Arm (a) meters every ring of an open link's support, piece by
piece, against the link's UNBOUNDED trimline, requiring the whole ring
beyond it; the ring that carries the requested edge always reaches
behind its own trimline (its own edge, read at the trim through
`co_requested_trim`, sits at margin zero, and the ring's far side lies
behind the edge), so the request refuses however small the band.

Witness: `band_planar_cut_off_meters::a_requested_ring_edge_refuses_at_the_ring_meter`
(margin −1.6 on the tabbed pocket, both verbs).

Conservative, not unsound: nothing is built. But the strip such a band
removes lies inside the tab, between the two rims, and arm (d)
(`surgery::strip_clearance`), which bounds the strip by its stations,
is the meter that fits it — it walks only the outer cycle today
because arm (a) refuses every requested ring edge first (stated at
`strip_clearance`).

## What would close it

Meter a ring that carries a requested edge as arm (d) meters the outer
cycle — each other edge of the ring against the strip's bounding
region between its stations — and leave arm (a)'s unbounded trimline
for rings that carry none; then turn the witness row into a build at
its closed form.

## Findings (implementer)

Measured on `origin/main` before the change: the witness refused
`RingClearance` at arm (a), both verbs, as stated. After it the tab's
tip builds at the prism closed form (`verb.section() × 1`), tier 3,
naming totality, both verbs
(`band_planar_cut_off_meters::a_requested_ring_edge_builds_at_its_strip`).

The reach meter (`blend/reach.rs`) agrees: it passes the clear tab
and the clear spike. With arm (d)'s ring walk switched off, the
crossed spike reaches the reach meter's `screened` premise check and
fails loud there (`SurgeryInvariant`, "a support's vertex inside the
strip its band replaces"), so arm (d) is what keeps that premise for
a ring between the screen's samples; its doc now says so.

Sibling filed: `a-requested-ruled-ring-edge-refuses-at-the-ring-meter`
(a RULED link keeps arm (a) on every ring, since arm (d) meters planar
strips alone).
