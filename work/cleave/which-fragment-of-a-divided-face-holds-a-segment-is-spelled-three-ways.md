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

- **`boolean::fragments`** is the home: `Lineage` (a face and every face
  the fragment rows divide off it, rows in any order: the fixpoint
  `finish`'s `descendants` was) and `sole_common_face`, moved from
  `boolean::sectors`. `Lineage::holding_both` names the face of the
  lineage holding both ends of a segment. `rest::fragment_holding`
  reads it, `finish::weld_pinches` reads `Lineage::contains` as
  `pinch_site`'s admission, and `sectors::tangent_face` and
  `rest::mirror_edges` read `sole_common_face`.
  `chord_join::lineage` and `finish::descendants` are gone. Unit rows
  pin rows listed out of mint order and a lineage holding two shared
  faces.
- **`JoinPlan` reads the face once** (`JoinPlan::face`, the first
  half's), and `first_chord`, `segment_curve`, `ChordJoiner::join` and
  the boolean's both-chords-skipped refusal read the plan's.
  `chord_join::he_loop`/`he_face` are the one half-edge → loop/face
  reader (`splitting::join::he_face`, `place_pending`'s `loop_of`,
  `cut_core`'s and `join`'s reads retired into them).
- **Halves on two faces are reached** and left to today's refusals
  (ruled on PR 4131's review): 64 `segment_curve` calls in 6 sweep
  tests, refused by the curve's lane on the first half's face (37,
  `SectionInvariant`) or the join's `mekr` (27, `NotSameFace`). The
  half-selection defect behind them is
  `work/join/a-boolean-match-takes-a-half-from-a-sector-on-a-face-its-ends-do-not-share.md`.
- The one-pass mint-order lineage and the fixpoint agreed on every call
  measured; every caller hands rows in mint order.
