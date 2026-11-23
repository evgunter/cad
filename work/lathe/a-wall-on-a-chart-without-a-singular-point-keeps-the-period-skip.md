---
id: a-wall-on-a-chart-without-a-singular-point-keeps-the-period-skip
kind: issue
title: Two one-band walls on one cylinder or torus record stay two faces across their shared rim: the merge keeps the period-closure skip there
status: open
opened: 2026-11-23
priority: P2
refs: [a-section-loop-inside-a-one-band-curved-face-refuses-the-interior-loop-guard]
cost: M
---


Found by the LATHE interior-loop lane (2026-11-23, PR 7440).

## What

`merge_faces`' `merge_group` closes a curved run through its wrap edges
only where the survivor's chart has a singular point (a sphere's poles,
a cone's apex, a spindle sheet's apexes): there a wrap edge is the
face's seam kept to its tip, and a second edge into that tip goes by
`kemr`. On any other curved chart (cylinder, torus) every same-face
duplicate the absorption leaves still skips `PeriodClosure`, the wrap
edges included.

So two one-band walls on one record meeting along a full rim stay two
faces at rest, against `[topology]`'s maximal faces: the union of two
stacked full-turn collars of one radius
(`crates/sweep/tests/a_peg_in_a_two_segment_bore_moves_only_its_wall.rs`,
"the bore's wall is two faces") is the measured case. With the skip
lifted for wrap edges on every curved chart, the bore glued into one
wall face and built valid (measured on the PR's branch before the
narrowing), and that row's fixture lost its premise.

## Open

Whether a cylinder or torus run lifts the skip for its wrap edges too,
which needs the wrap pieces of the members to chain into one seam from
rim to rim (one record, one chart, so one azimuth), and a re-built
fixture for the peg row, whose subject is one record worn by two
faces.
