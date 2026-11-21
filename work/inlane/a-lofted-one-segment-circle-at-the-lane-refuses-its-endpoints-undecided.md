---
id: a-lofted-one-segment-circle-at-the-lane-refuses-its-endpoints-undecided
kind: issue
title: A loft over a one-segment circle at Interval refuses ArcSweepUndecided(Endpoints): the skin reads whether a full turn ends where it starts off the distance between its one lane-wide vertex and itself
status: open
opened: 2026-11-20
priority: P3
cost: E
---



Found by PROFILE's `circle-lowers-to-one-segment` (branch `profile/circle-lowers-to-one-segment`), which lowers `circle()` to one full-turn segment at one vertex.

## What

`editor-core`'s `e6_drive_over_a_loft_at_interval.rs` `with_no_box_an_arc_section_refuses_and_a_rotated_copy_places` lofts a `circle` section at `Interval`, the profile elaborated at the lane so its vertex is a few ulps wide. With the two-arc circle the loft refused `SkinError::Structure(CarrierWeightNotAPoint)` (C6's interim until S4/S5). With the one-segment circle it refuses one decision earlier:

`Skin(ArcSweepUndecided { section: 0, which: Endpoints, margin: Enclosure { lo: 0.0, hi: 4.44e-16 } })`

`sweep::skin::segment_curve` decides whether an arc "ends where it starts" from `a.distance(b)`'s enclosure. For a full turn `a` and `b` are the loop's one vertex (D1: `b = a`), so the chord is zero for every value it stands for; but the distance of a wide enclosure from itself is the decorrelated `[0, 4.4e-16]`, which is not definitely zero and not definitely positive.

## What closing it takes

The full turn is structural: the section's validated loop has one vertex holding an arc (`profile::is_full_turn`). The skin can read that where it builds the segment (`skin.rs` `vertex_segment`'s caller) instead of re-deciding it from the endpoints. `Real` carries no `PartialEq`, so `segment_curve` itself cannot test `a == b`; the fact has to come from the loop. Once read, the pose reaches `CarrierWeightNotAPoint` as the two-arc form did, and the row re-pins to it.

The witness row is re-pinned to the refusal it now meets, naming this item.
