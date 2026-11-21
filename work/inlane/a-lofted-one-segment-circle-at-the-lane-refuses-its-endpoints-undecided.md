---
id: a-lofted-one-segment-circle-at-the-lane-refuses-its-endpoints-undecided
kind: issue
title: A loft over a one-segment circle whose vertex is lane-wide would refuse ArcSweepUndecided(Endpoints): the skin reads whether a full turn ends where it starts off the distance between its one vertex and itself
status: open
opened: 2026-11-20
priority: P3
cost: E
---



Found by PROFILE's `circle-lowers-to-one-segment` (branch `profile/circle-lowers-to-one-segment`), which lowers `circle()` to one full-turn segment at one vertex.

## What

`sweep::skin::segment_curve` decides whether an arc "ends where it starts" from `a.distance(b)`'s enclosure. For a full turn `a` and `b` are the loop's one vertex (D1: `b = a`), so the chord is zero for every value that vertex stands for; but at an enclosure scalar the distance of a wide enclosure from itself is decorrelated, `[0, w]`, which is neither definitely zero nor definitely positive, and the skin refuses `ArcSweepUndecided { which: Endpoints }`.

Measured once, then moved out of reach: on the branch's first head `editor-core`'s `e6_drive_over_a_loft_at_interval.rs` `with_no_box_an_arc_section_refuses_and_a_rotated_copy_places` (a `circle` section lofted at `Interval`, the profile elaborated at the lane) refused `Skin(ArcSweepUndecided { section: 0, which: Endpoints, margin: Enclosure { lo: 0.0, hi: 4.44e-16 } })`, the vertex widened by the lane's `sin_cos(0)`. The branch then held a phase that is exactly zero as `+x`, the vertex became a point, and the row reads `CarrierWeightNotAPoint` again (C6's interim until S4/S5). A full turn whose vertex is itself wide (a `circle_split(1)` at a non-zero phase, or a circle about a boxed centre) is not measured.

## What closing it takes

A witness with a lane-wide full-turn vertex, then read the full turn structurally: the section's validated loop has one vertex holding an arc (`profile::is_full_turn`), which the skin can read where it builds the segment (`skin.rs` `vertex_segment`'s caller) instead of re-deciding it from the endpoints. `Real` carries no `PartialEq`, so `segment_curve` itself cannot test `a == b`; the fact has to come from the loop.
