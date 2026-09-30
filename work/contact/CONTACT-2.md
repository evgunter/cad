---
id: CONTACT-2
kind: unit
title: the Planar join lane measures a conic edge against its section plane: the axis-coincident box lap stops reaching an unreachable invariant
status: closed
opened: 2026-09-25
priority: P0
cost: H
branch: contact/2-axis-lap
closed: 2026-09-26
pr: 3250
---


Carries `axis-coincident-lap-trips-the-planar-join-invariant`. Spec:
`docs/CONTACT-2-SPEC.md`. A survey found that the Planar arm's premise
("the operand gate promises every carrier planar") is stale: the gate
has admitted conic edges since M5 PR 9.

Review tier: **single, full**. The diff is small, but minting or skipping
a chord wrongly yields a wrong body.

## Closed

PR 3250. The `JoinLane::Planar` lane now carries the partner germ
plane and decides a Circle or Ellipse between edge with the midpoint
test the split lane uses. Both lanes share one arm. The invariant is
deleted, `Nurbs` refuses typed, and role resolution gains the
`EdgeOnCarrier` anchor.

The row's pose, the axis lap, still refuses. Its refusal is now
`Join(UnpairedLooseEnds)`, the edge-in-face defect that reproduces
unchanged on an all-planar prism at the base. It is filed at
`work/zip/an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`.
The spec had said no other invariant may fire. The orchestrator
accepted this because that defect is pre-existing, is out of reach of
this arm, and is pinned and filed.

What now builds, and was refused on main:
- full-length flats in every pose;
- the short sunk rod;
- BAND's bottom-entry blind D pocket, whose volume equals the analytic
  value to the last bit.

The top-entry pocket is unchanged, and ZIP's row carries it.

Single full review. It found no MAJOR. The fix pass pinned the review's
witness that the older chord-midpoint anchor is unsound on curved
edges (`axis_lap.rs` `a_chord_midpoint_probe_reads_both_loops_alike`,
ZIP's row) and gave the edge midpoint one home
(`EdgeCurve::mid_point`). The remaining copies are REACH's row
`edge-midpoint-evaluation-is-copied-at-each-site-that-needs-a-point-on-an-edge`.
