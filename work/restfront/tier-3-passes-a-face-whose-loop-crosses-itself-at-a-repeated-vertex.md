---
id: tier-3-passes-a-face-whose-loop-crosses-itself-at-a-repeated-vertex
kind: issue
title: Tier 3 passes a face whose loop crosses itself at a vertex it visits twice
status: open
opened: 2026-10-06
priority: P3
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
