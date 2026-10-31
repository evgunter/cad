---
id: a-family-scalars-cone-pose-too-wide-for-the-tube-bounds-refuses-rather-than-bisecting
kind: issue
title: at a parameter-family scalar, a plane×cone arc whose box is too wide for its section's tube bounds refuses NoTubeEnclosure, which the drive does not bisect
status: open
opened: 2026-10-28
priority: P3
cost: M
---


Found by `a-classified-pairs-section-states-the-bounds-that-retire-the-tube`.
`geom_brep::intersect::tube_bounds` reads every datum through its certified
bracket, so at a parameter-family scalar a stated bound holds for every
member of the box. Where the box is too wide for the bounds (the floor, the
moat or the Miranda square fails over it), `edge_nurbs::analytic_rung3`
owes the tube as before; for a plane×cylinder, ×sphere or ×plane pair the
tube decides at the scalar and an undecided box bisects, but a cone has no
tube enclosure, so the edge refuses `AnalyticRung3Refusal::NoTubeEnclosure`,
which carries no undecided arm (`AnalyticRung3Refusal::decision` is `None`
for it). The drive takes that as the node's verdict, where a smaller box
could have certified. The remedy is a waiver that fails over a box it could
pass at a point to answer undecided at an enclosure scalar, as limb 2's
`Stalled` does.
