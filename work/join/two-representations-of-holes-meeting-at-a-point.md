---
id: two-representations-of-holes-meeting-at-a-point
kind: issue
title: Two representations of holes meeting at a point: k rings on one vertex (the zips) against one ring visiting the point k times (the sequential subtract)
status: open
opened: 2026-10-06
priority: P1
cost: H
design: true
---


## The question

The kernel builds holes that meet at one point of a face in two shapes:

- **k rings, each through one shared vertex.** This is the zips' shape.
  `zip::split_across` crosses a pinch the zips would fuse twice by
  `mev` then `kemr` across two corners of one ring.
  `finish::pinch_site`'s `Joint::Hole` welds two pierces on one ring the
  same way. Both are documented as "two holes meeting at the point, one
  shape at rest" (65a09d4a), and the mesher handles it
  (`TessellateError::PinchWedge`).
- **One ring visiting the point k times.** This is what the union of
  the same members builds, and the sequential subtract. Its corners at
  the point are the face's sectors between the holes, and the
  sequential subtract keeps a separate vertex per visit.

TANG found the fork on PR 4129 and is not acting on JOIN's
representation. Which shape is canonical is JOIN's call.

## Evidence

**Corner geometry.** At a vertex a ring passes twice, with disjoint
corners (in₁ → out₁) and (in₂ → out₂), the four directions run out₁,
in₁, out₂, in₂ round the vertex. The `kemr` leaves corners (in₁ → out₂)
and (in₂ → out₁), each of which sweeps both originals. The k-rings form
therefore always has corner sectors that overlap at the vertex.
`topo::test_support::meeting::corners_disjoint` reads this.

**JOIN's rows already build that form.** Crossed bodies among those
that `common::differential::outcome` passes as `SOUND`, measured on
eb1fae63:

| row | crossed |
|---|---|
| `join_pierce_runs_sweep::an_island_pinched_twice_to_its_holes_ring_dies_at_each_crossing` | 1 of 6 |
| `join_pierce_runs_sweep::two_pinches_in_one_op_are_each_crossed` | 4 of 6 |
| `join_pierce_strut_facing::a_wide_run_builds_in_every_op` | 8 of 12 |
| `join_pierce_strut_facing::two_edge_runs_build_in_every_op` | 3 of 18 |
| `join_pierce_strut_facing::two_out_runs_at_the_corner_build_in_every_op` | 12 of 18 |

**The two-face `kef` crossing builds it too.** With the one-ring
crossing and `Joint::Hole` refused, which was a measurement only and has
since been reverted, `two_pinches_in_one_op_are_each_crossed` still
built 2 of its ops crossed. So the question is about the representation
as a whole, not about one arm.

**Tiers 3 and 3′ cannot see it.** The overlapping sectors pass both
tiers and the volume. That is RESTFRONT's
`tier-3-passes-a-face-whose-loop-crosses-itself-at-a-repeated-vertex` (P1).

**The next boolean refuses on it.** The plate `[0,3] × [0,2] × [0,1]`
less the union of two leaning wedges whose footprints meet at
(1.5, 1, 1), in one boolean (`topo::test_support::meeting`), builds the
k-rings form, [18, 41, 25]. A later boolean on that body can refuse
`ClassificationInvariant { "two crossing germs of a vertex pair lie
along one direction in one sector entry" }`. The plate less each wedge
in turn builds the other form, and further booleans build on it.

**PR 4129's narrowing.** Three or more holes on one ring are refused
typed at the zip (`PinchOfManyHolesInOneRing`) rather than crossed into
k rings. The rows are in
`crates/topo/tests/holes_meeting_at_a_vertex.rs`,
`the_plate_against_the_holes_union_builds_sound_or_refuses_typed_in_every_op`.

## A candidate

Keep the ring whole and the point as two vertices on it, v and v′, as
the sequential subtract builds: the split's `mev` without the `kemr`,
with the pair's zip leaving the copies apart. Then retire
`PinchOfManyHolesInOneRing`, since a ring can then pass any number of
copies.
