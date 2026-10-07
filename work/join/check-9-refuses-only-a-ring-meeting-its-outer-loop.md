---
id: check-9-refuses-only-a-ring-meeting-its-outer-loop
kind: issue
title: Check 9 refuses a ring meeting its face's outer loop, but not two rings meeting, which one vertex per cone makes always wrong
status: open
opened: 2026-10-06
priority: P2
cost: M
refs: [a-pinch-no-kept-face-can-cross-refuses]
branch: join/tier3-pinch-checks
---


## What

Check 9 (`validate.rs`, `ring_outer_contact_about`) refuses a ring that
meets its face's outer loop (`RingMeetsOuter`). It runs no check between
two rings of one face. Under the ruling (Ev, PR 4057), a face's two loops
meeting at a point is always a crossing: one vertex per cone gives a
figure-eight hole as one loop through two vertices, never two rings
touching. Ev's ruling says check 9 may then refuse every meeting of two
loops of one face.

The pinch unit (branch `join/pinch-one-vertex-per-cone-build`) did not
widen it. The mesher refuses a ring touching another loop at a pinch
(`mesh planar::pinch_rows::a_ring_touching_its_outer_loop_at_a_pinch_refuses_typed`,
the same sector rule for two rings). The unit's batteries record 0 such
refusals, so the widening would refuse nothing they build.

## The shape to give

Run `ring_outer_contact_about`'s contact arms over every pair of rings
of a face, with a typed `RingMeetsRing` refusal (a new
`ValidationError` variant, threaded through `pncad-py`'s tags). Pin it
on a hand-built face with two rings touching at a vertex.

## Built (branch `join/tier3-pinch-checks`)

Check 9 runs `ring_outer_contact_about`'s contact arms over every pair
of rings of a face, with the earlier ring in the outer loop's seat.
A contact refuses `ValidationError::RingMeetsRing { face, ring, other,
contact }`, where `contact`'s `outer_*` fields name `other`'s
entities. An in-band margin refuses `RingContactEscalated`, whose doc
now covers either pair. The arms' residue is check 9's own: ellipse,
spiric and NURBS edges, and arms 4 and 5 off a plane. Whether one ring
lies inside another is not asked here; that stays
`restfront/check-9-does-not-check-a-ring-nested-inside-another-ring`.

Threaded through `pncad-py`: the tag `ring_meets_ring` (and
`pinch_corner_crossed`, `pinch_corner_escalated` for the corner arm),
`ring_contact_kind` read off its contact, and the binding census.

Pinned by `validate::tests::check_9_refuses_two_rings_touching_at_a_vertex`:
a lamina whose face holds two triangular rings meeting at the origin
refuses one `RingMeetsRing { Vertex }`, and the same rings 0.5 apart
pass.

**Measured** with the corner arm (same batteries, same result): 0 lines
moved.
