---
id: tier-3-passes-a-face-whose-loop-crosses-itself-at-a-repeated-vertex
kind: issue
title: Tier 3 passes a face whose loop crosses itself at a vertex it visits twice
status: open
opened: 2026-10-06
priority: P1
cost: M
---

## What

Found on branch `tang/holes-meeting-at-a-vertex`.

A face whose loop passes one vertex several times must visit its
corners there in angular order about the face's normal. Otherwise two
of the face's corners at that vertex overlap, and the loop crosses
itself there. `topo::validate_geometric` does not see this.

The witness: `crates/topo/tests/holes_meeting_at_a_vertex.rs`, three
leaning wedges folded before the plate. Hang the pierce's ring struts
in the mirror of `vtxfac::ring_order` (reverse every strut after the
first). Each of the 6 orders that fold three wedges before the plate
then builds a top face whose ring crosses itself at (1.5, 1, 1). Tier 3
reports `Ok(())` on all 24 bodies, with the closed-form volume. The
test's own check, `corners_disjoint`, refuses those 6.

The next boolean on such a body is where it surfaces. With four
wedges, folding the fourth after the crossed body refuses
`ClassificationInvariant { "two crossing germs of a vertex pair lie
along one direction in one sector entry" }`.

## The shape to give

A tier-3 check: at every vertex a face's loops pass more than once,
the corners there are angularly disjoint about the face's outward
normal at that point. `corners_disjoint` in that test is the planar
spelling. A curved face needs it in the chart's tangent plane at the
vertex.

## Reachable on main

A one-shot `subtract(plate, U)`, where U is the union of leaning
wedges meeting at a point of the plate's top, builds the crossed shape
without a mutant (PR 4129's review). This is the zips' representation
of holes meeting at a point (`zip::split_across`, `finish::pinch_site`),
JOIN's open design question
`work/join/two-representations-of-holes-meeting-at-a-point.md`.

- k = 2, on main: [18, 41, 25].
- k ≥ 3 is refused on PR 4129's head (`PinchOfManyHolesInOneRing`).
  Only the gate-dropped mutant builds it: three wedges [27, 66, 40],
  four [27, 72, 45], three notches past the plate's edge [21, 57, 34].

Tiers 3 and 3′ and the volume pass on every one of those bodies. A
later boolean on such a body can refuse `ClassificationInvariant`.

JOIN's sweep rows pass such bodies as SOUND, measured on eb1fae63:

- `join_pierce_runs_sweep::an_island_pinched_twice_to_its_holes_ring_dies_at_each_crossing`: 1 of 6;
- `join_pierce_runs_sweep::two_pinches_in_one_op_are_each_crossed`: 4 of 6;
- `join_pierce_strut_facing::a_wide_run_builds_in_every_op`: 8 of 12;
- `join_pierce_strut_facing::two_edge_runs_build_in_every_op`: 3 of 18;
- `join_pierce_strut_facing::two_out_runs_at_the_corner_build_in_every_op`: 12 of 18.

Whether tier 3 should refuse the k-rings form waits on that JOIN
question. `topo::test_support::meeting::corners_disjoint` is the planar
check, shared.
