---
id: which-fragment-of-a-divided-face-holds-a-segment-is-spelled-three-ways
kind: issue
title: Which fragment of a divided face holds a segment is spelled three ways: the shared lineage, finish's descendants, and segment_curve's first half
status: open
opened: 2026-10-04
---

Found by PR 4008's fix pass (JOIN), on review r2's style finding that
three functions answer one question: which fragment of a face that
chord `mef`s divided holds a segment.

## The spellings

- `crate::chord_join::lineage` (`chord_join.rs`): a face and every
  fragment `ChordJoiner`'s rows divide off it, one pass in mint order.
  `boolean::join`'s `wall_region` and `boolean::rest`'s
  `fragment_holding` read through it since PR 4008.
- `boolean::finish`'s `descendants` (`finish.rs`): the same set over
  rows in ANY order (a fixpoint), because the pierce-weld pass chains
  the join's rows with its own weld rows. It is a strict generalisation
  of `lineage`, and one of the two could be the other.
- `ChordJoiner::segment_curve` (`chord_join.rs`): reads the face of the
  segment's FIRST half only (`halves.0`'s loop) and asks nothing of the
  second. The join's `wall_region` checks both halves; whether
  `segment_curve` can be handed halves on two faces is unmeasured.

## The shape of a fix

One helper that names the face holding both halves among a face's
lineage, rows in any order, and the three callers reading through it.
`finish.rs` and `chord_join.rs` are cleave's and hone's ground, so
PR 4008 shared only the lineage between the two join-side callers.

## Evidence (PR 3985, 2026-10-04)

`boolean::join`'s `wall_region` is gone: it chose the region whose
azimuth window the planar-side chord read, and PR 3985
(`reach/arc-from-pairing`) retired that window — the chord takes the
arc its pairing names. `rest::fragment_holding` and
`chord_join::lineage` remain.

## Built (branch cleave/fragment-lineage)

- **`chord_join::Lineage`** is the one lineage: a face and every face
  the fragment rows divide off it, rows in any order (the fixpoint
  `finish`'s `descendants` was). `Lineage::holding_both` names the face
  of the lineage holding both ends of a segment through
  `chord_join::sole_common_face`, moved there from `boolean::sectors`.
  `rest::fragment_holding` reads `holding_both`; `finish::weld_pinches`
  reads `Lineage::contains` as `pinch_site`'s admission.
  `chord_join::lineage` and `finish::descendants` are gone.
- **`JoinPlan` names the face holding both halves** (`face_holding`),
  read by `segment_curve`, `ChordJoiner::join` and the boolean's
  both-chords-skipped refusal (`JoinPlan::face`). Halves on two faces
  refuse typed there: `SectionInvariant`, "a join's two halves sit on
  two faces, and no chord joins loops of two faces".
- **Measured** (topo + sweep, default profile, 4531 tests, instrumented):
  - `segment_curve` WAS handed halves on two faces: 64 calls in 6 sweep
    tests (`join2_r1_probes::unions_through_ring_vertices_…`,
    `join2_r2_probes::a_like_far_ends_tie_…`, `…a_channel_whose_arm_ends_…`,
    `…an_island_of_ring_vertex_segments_…`,
    `reach_continuation::a_tangency_in_the_middle_of_an_edge_…`,
    `rest_nested_strut::a_pinch_apex_meeting_one_vertex_…`). Every one
    was refused already: 37 by `chord_spec` computed in the first half's
    face (`SectionInvariant`, the tangent-germ frontier words), 27 by the
    join's `mekr` (`Euler(NotSameFace)`). With `face_holding` the same
    6 tests, and no others, refuse at the plan; all 4531 pass.
  - The one-pass mint-order lineage and the fixpoint agreed on every
    call (16 test threads: 13 through `rest`, 3 through the weld): every
    current caller hands rows in mint order.
