---
id: role-resolution-interior-tiers-certify-only-planar-region-faces
kind: issue
title: Role resolution probes a curved edge at its chord midpoint, which is on neither flanking region and reads both loops alike (SectionLoopMixed); its region-interior tiers read a curved region through the planar-only face_plane
status: open
opened: 2026-09-25
priority: P0
cost: H
---


Found by CONTACT-2 (PR 3250) and its review.

`boolean/join.rs` `resolve_roles_geometric` decides which loop of a
completed section polygon is the IN copy by probing the regions flanking
the seam against the other operand, tier by tier (`Anchor::Vertex`,
`Anchor::EdgeMidpoint`, `Anchor::EdgeOnCarrier`, `Anchor::RegionInterior`,
`Anchor::RegionVertexChord`). Two defects sit in it.

## 1. `Anchor::EdgeMidpoint` is unsound on a curved edge (reproduced)

The tier probes each region edge's CHORD midpoint (`lerp` of its ends)
uncertified, as if it lay on the region. For a curved edge it does not:
a rim semicircle's chord midpoint is the circle's centre, a point on
neither flanking region, and its one verdict is taken by both loops.

**Reproducer**, `crates/sweep/tests/axis_lap.rs`
`a_chord_midpoint_probe_reads_both_loops_alike`: rod `r = 0.5` about `z`
over `z ∈ [0, 4]` (an extruded circle) minus a cutter extruded over
`z ∈ [−1, 5]` from the half-plane `y ≥ 0` with a half-rod bump
`r = 0.1` on the axis, bulging either way. Every rod vertex is
`OnBoundary`, so the chord-midpoint tier decides, and both loops read
the centre alike: `Join(SectionLoopMixed)` for both bulge signs, on
CONTACT-2's head and on its base. The loud guard fires, so no wrong
body ships, but a legal pose refuses as a kernel invariant.

Swapping the chord midpoint for the on-carrier one is not the fix by
itself: CONTACT-2 tried it and three rows moved to the refusal in §2
(`curved_mergedoor`
`floating_and_mid_bore_pegs_refuse_at_the_zip_seam_chord_today` and
`consumed_side_of_the_pair_is_gone_and_one_record_ships`,
`r1_probes_m9_3` `probe_partial_engagement_never_silent`). There the rim
lies ON the other body's bore wall, so the on-carrier point reads
`OnBoundary`, and today the off-region chord midpoint decides them —
by the same unsound read. CONTACT-2 kept the tier and added
`Anchor::EdgeOnCarrier` after it.

## 2. The region-interior tiers are planar-only

`Anchor::RegionInterior` and `Anchor::RegionVertexChord` certify a
candidate interior to its region face through `point_in_face`, taking
the projection normal from `solid_contain::face_plane`, which answers
only for a `Plane` and returns `PointInSolidError::KindUnsupported`
otherwise. A curved region reaching them refuses
`Containment(KindUnsupported { kind: Cylinder })`, and
`KindUnsupported`'s docs say the containment door HAS a cylinder arm.
Reached on CONTACT-2's tree before `Anchor::EdgeOnCarrier` landed by
`rod − brick((-1, 1), (0, 1), (-1, 5))`, and by the three rows above
with the chord-midpoint tier replaced; no row reaches it today.

## What the taker owes

A sound curved-edge anchor — the chord-midpoint tier fenced to straight
edges, with the rows it decides today resolved by a certified point (a
chart-space `point_in_face` for curved regions answers §2 as well) — or
a typed refusal naming the gap. The reproducer row flips when it lands.
