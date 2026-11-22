---
id: check-4s-fold-reads-a-crease-where-a-face-curves-back-inside-the-band
kind: issue
title: check 4's fold reads a crease where a face curves back inside the band
status: open
opened: 2026-11-22
priority: P1
cost: H
design: true
---

## Finding

From EMBED's design fork `embed-wedge` (fork-log row 304, PR 7456,
2026-11-22); all three designers reached it independently and none
disputed the arithmetic. Measured in part on `origin/embed/wedge-measure`.

Check 4 levers an edge's wedge at the fold
(`geom_brep::folded_lever_arm`: the least of the faces' reach and their
curvature radii). Take a cylinder of radius 1e-3 m and a plane through
one of its rulings at a sine of 1e-4:

- the plane cuts the cylinder a second time 2r·sin θ ≈ 2e-7 m away;
- between the two cuts the faces part by at most r·θ²/2 ≈ 5e-12 m;
- moving the plane 5e-12 m removes the edge: to within ε there is no
  edge;
- check 4 reads a crease: 1e-4 × 1e-3 = 1e-7, above the band.

Measured (`injective::wedge_measure_rows::the_sliver_through_sign_on`
on that branch): `folded_lever_arm` = 1e-3 and the class at reach 1 is
`Corner`.

The fold takes a face to turn away over its radius. The first-order
angle governs the gap only out to about θ/κ (here 1e-7 m), with κ the
curvature toward the other face, so the fold over-reads by about 2/θ.
At a sine of 5e-7 (the third row of PR 7329's reviewers' table) the
fold happens to be right. Where the curved face bends away from the
other the gap keeps growing, and what fails there is the edge's
position and class under a sub-ε move, not the far gap.

## Design question

What meter says an edge's angle is real on a curved face: one
candidate is the lesser of sin θ × reach and the second-order parting
sin²θ / (2κ), which is `tangent_second_order`'s quantity read before
the class is settled. A repaired meter has to say which of the two
failures above it protects against. Weigh it with
`check-4-levers-a-wedge-at-the-farther-faces-run-…`: the embedding
question reads check 4's class from the verdict
(`work/embed/the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict.md`),
so a class that is wrong here is read wrong there.
