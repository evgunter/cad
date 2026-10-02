---
id: carrier-walk-has-no-crossing-row-for-spiric-or-spline-edges
kind: issue
title: The in-plane carrier walk has no crossing row for a spiric or spline edge, so a planar face bounded by one refuses containment near that edge
status: open
opened: 2026-10-01
priority: P2
cost: M
---

Filed by TANG's re-measurement of `work/tang/arc-aware-point-in-loop.md`
(2026-10-01): that row's item 3 remainder, left once its circle and
ellipse halves were answered.

`splitting::containment::point_in_carrier_loop` and `carrier_loop_side`
cross lines on their chords and circle and ellipse arcs on their conics
(`ConicArc::of` in `carrier_loop`). A `Spiric` or `Nurbs` edge has no
crossing row. `carrier_loop` holds it as `LoopEdge::Unrowed { center,
reach }`: the spiric's arc as a ball around its midpoint sized by its
speed bound, and the spline as its control hull's ball. The walk answers
only along a ray that definitely misses every such ball. A point inside
one, or a point every scheduled ray from which could meet one, gets the
walk's typed `PointInLoopError::Uncrossable` (the loop, the first edge a
ray was abandoned on, and its carrier), and each caller names it:

- `ContainError::Uncrossable` from `boolean::contain::contfp`, surfaced
  as `BooleanError::ArcLoopContainmentUnsupported` by `boolean::reduce`
  and `boolean::ops`, and as `CensusUnsupported` by the census;
- `PointInSolidError::EdgeCarrierUnsupported` from
  `boolean::solid_contain::point_in_face`. Through `point_in_solid` it
  reaches the join's role probe (`boolean::shell_witness::complex_side`)
  as `BooleanError::Containment`, because `inconclusive` does not list
  it. Tier 3's planar-face witness (`certified_in_face`) instead discards
  the candidate;
- `SplitJoinError::RingHoming(Uncrossable)` from `chord_join::rehome_rings`;
- `ValidationError::RingNestingUndecided` from check 9's `ring_nesting`.

**The body class is reachable.** Built bodies have planar faces bounded by
a spiric. `crates/sweep/tests/pis_arc_capped_poses.rs`,
`a_spiric_bounded_face_refuses_only_within_its_reach`, finds one on the
shelled vessel's cavity and pins `EdgeCarrierUnsupported` at the spiric's
midpoint. Nobody has measured whether a public boolean reaches the
refusal, which needs a containment probe that lands inside the spiric's
ball. Check 9 does reach it on built bodies: `sweep`'s
`shell7_dump_corpus` and `shell7_r1_diff_corpus` validate shelled bodies
on which four ring-vertex queries refuse this way (measured 2026-10-02).
`shell7_dump`'s tier-3 lines carry no `RingNestingUndecided` once check 9
reports the refusal, so another vertex of each such ring places it, and
no verdict there rests on the refusal yet.

**What a fix needs**: a crossing row for each carrier, folded into
`carrier_walk` beside the conic row: a certified ray × spiric root count
in the section's plane, and a ray × spline count by subdivision against
the control hull. How the callers name the walk's refusal is a separate
question, answered by `work/cleave/carrier-walk-none-is-answered-four-ways.md`.
