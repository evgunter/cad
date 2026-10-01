---
id: point-in-solid-reads-in-band-against-a-face-plane-far-from-the-face
kind: issue
title: point_in_solid reads witnesses in-band against a face's plane where the point is far from the face itself, so a shell or wedge whose witnesses all lie near such planes refuses (ShellWitnessExhausted) where the answer is clear
status: open
opened: 2026-10-01
priority: P2
cost: M
---


Unmeasured beyond what two lanes saw. The hypothesis has not been checked.

- PR 3716's review traced the `m5_pr8_bvh_diff` grazing fixture: a
  corner read in-band against operand B's top plane while sitting 4 m
  from the face itself. PR 3716 now treats that reading as
  inconclusive, and another witness decides.
- After PR 3716, `offer_rows`' `wide_wedge_with_a_far_vertex_{union,intersect}`
  refuse `ShellWitnessExhausted` with 0 witnesses on the boundary and
  26 in-band. Before, they refused `Containment`. Why every witness of
  that wedge reads in-band was not investigated. The guess is the same
  grazing against an infinite plane.

What to establish first is where `point_in_solid`'s in-band margin
comes from: the face's carrier plane, or the face as bounded. A
reading against an unbounded carrier, for a point outside the face's
extent, would explain both cases.
