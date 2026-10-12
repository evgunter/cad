---
id: tier-3-passes-a-face-whose-loop-crosses-itself-at-a-repeated-vertex
kind: issue
title: Tier 3 passes a face whose loop crosses itself at a vertex it visits twice
status: closed
opened: 2026-10-06
priority: P1
cost: M
closed: 2026-10-07
refs: [4240, the-corner-slice-arm-is-silent-on-cones-nurbs-faces-and-first-order-ties]
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

## Reachability

Before JOIN's `zip::split_cones` (main 52cbed0c), a one-shot
`subtract(plate, U)` of leaning wedges built the crossed shape without
a mutant: [18, 41, 25] at k = 2, and at k ≥ 3 three wedges [27, 66, 40],
four [27, 72, 45], three notches past the plate's edge [21, 57, 34].
JOIN's sweep rows passed such bodies as SOUND, measured on eb1fae63:

- `join_pierce_runs_sweep::an_island_pinched_twice_to_its_holes_ring_dies_at_each_crossing`: 1 of 6;
- `join_pierce_runs_sweep::two_pinches_in_one_op_are_each_crossed`: 4 of 6;
- `join_pierce_strut_facing::a_wide_run_builds_in_every_op`: 8 of 12;
- `join_pierce_strut_facing::two_edge_runs_build_in_every_op`: 3 of 18;
- `join_pierce_strut_facing::two_out_runs_at_the_corner_build_in_every_op`: 12 of 18.

Tiers 3 and 3′ and the volume passed on every one of those bodies, and
a later boolean on such a body could refuse `ClassificationInvariant`.
`split_cones` splits the point per cone instead, and PR 4129's rows
(the plate against the union in five poses, the P − U grids at three
and four holes) build it with disjoint corners. No route on main is
known to reach the crossed shape now. The ring-order mirror mutant
above still does, so the tier-3 gap stands.
`topo::test_support::meeting::corners_disjoint` is the planar check,
shared.

## Closed (branch `join/tier3-pinch-checks`)

JOIN's corner-slice row built this check as check 9's corner arm
(`validate.rs`, `pinch_corner_errors`, refusing `PinchCornerCrossed`).
Corners are grouped by point key, so a vertex a loop visits twice is
read along with several vertices on one point. Curved faces are read in
the tangent plane at the point, as asked here. The arm's silences are
filed as `the-corner-slice-arm-is-silent-on-cones-nurbs-faces-and-first-order-ties`.

The witness above no longer reaches a crossed body on main `fe26bdfe`.
With `vtxfac::ring_order`'s struts mirrored, `holes_meeting_at_a_vertex`'s
wedges either refuse at the union (`PierceRunsNested`, every order that
folded three or four wedges before the plate) or build with disjoint
corners (`corners_disjoint` holds and tier 3 is clean). The new pin is a
hand-built crossed face instead,
`validate::tests::check_9_refuses_a_corner_crossing_its_face_at_a_pinch`.
