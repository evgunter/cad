---
id: profile-validation-reads-its-pairs-through-the-plane-instance
kind: unit
title: profile validation's pair pass reads the plane instance, and seg's pair reader is deleted
status: parked
opened: 2026-11-22
priority: P1
cost: H
blocked_on: [the-seg-differential-agrees-with-the-plane-instance]
---


PLANAR's sixth unit; runs beside unit 7 once the differential agrees.

## The ruled text

C5 (`crates/geom-brep/README.md`): the co-surface arm's readers include
*"profile validation's pair pass"*; in a sketch plane two segments meet
by the plane instance. `docs/DESIGN.md` Layering, the `profile` row:
validation *"reads how two of its segments meet ... through
`geom-brep`'s curve-pair reading"*. PR 7290: `seg`'s pair reader is
deleted only after the differential agrees. Not part of it: the joint
pass (`seg::joint_tangency`, `junction_turn`, `coincident` and the
turn at an authored vertex) and the fillet constructions.

## What changes (named in the tree, 2026-11-22)

- `crates/profile/src/validate.rs` `judge_pair` reads each pair through
  the instance (the sketch-plane constructor, the lowering unit 5
  wrote) and decides only from the answer: a meeting at the shared end
  of both adjacent spans is excused; a shared run is
  `NonSimple { Overlap }`; a point meeting is `NonSimple { Crossing |
  Touch }` or `TangentialContact` by the meeting's kind; a refusal is
  the escalation at `SegmentPair`.
- `contact_at_shared_vertex` and its `ContactAtSharedVertex` gate go if
  nothing else reads them.
- Deleted from `crates/profile/src/seg.rs`: `pair_contacts`, `joint`,
  `gaps_apart`, `touches_only_near_the_shared_end`, `collinear_overlap`,
  `line_line`, `line_arc`, `arc_clear_of_carrier`,
  `segment_clear_of_circle`, `end_clear_of_circle`, `arc_clear_of_circle`,
  `arc_arc`, `carrier_pair`, `candidates`, `end_in_stretch`,
  `carrier_side`, `end_touches`, `touches`, `touches_from`, `CKind`,
  `Contact`, `PairOutcome`, `pair_contact_tests`, and the trim readers
  `line_span` and `arc_span` if `ray_crossings` (unit 7) no longer needs
  them; `seg_reach_fuzz.rs` reads the instance. The differential retires
  with them; its corpus rows stay as rows on the instance.
- `crates/profile/src/gate.rs`: the gates only those functions read go
  (`ChordSide`, `LineSpan`, `ArcSpan`, `CarrierLineCircle`,
  `CarrierCirclesChord`, `CircleEndClear`, `CircleExtremePastArc`,
  `CircleFootPastSegment`, `CircleSide`, `CollinearOverlap`,
  `ContactAtSharedVertex`, `SegmentGap`, as the lane's grep finds them);
  a gate the joint pass also reads (the `CarrierCircles*` rows through
  `joint_tangency`) stays.

## Done when

- `profile`'s suite, `sweep`'s and `editor-core`'s profile corpora are
  green; every moved golden or pin (`m4_pr6_eps_diff`, the `m10_*`
  interval pins, `revalue_corpus`, `sym11_exact_channel_rows`,
  `sym_9_retry_interval`, `m10_bulge_renders.txt`) is re-baselined with
  the reason in the PR.
- The five span-clear witnesses of
  `validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc`
  (claimed, `parent:` this unit) validate.
- No function in `profile` decides how two segments meet (grep for the
  deleted names, and for `Circles` reads outside the joint pass and the
  fillet constructions).
- `crates/profile/README.md` and the `seg` module doc describe the pass
  as it is (the ruled text is already in C5 and the Layering row).

## First check, and what to do if it fails

**How a reader's refusal becomes a profile refusal.** Every profile
escalation is minted from a `Gate` (`crate::gate`, D4 ¶1 (ii): *"one
[`Gate`] per `k_stats` name this crate decides under"*), and
`ProfileError` and `StructureRefusalKind` carry `gate: Gate`. The
instance refuses with `MeetFault` and its own `k_stats` names
(`curve_pair_pose_extreme` and the rest). Whether the profile refusal
carries a gate per reader row, one gate for "the pair's meeting" with
the reader's diagnostic as its source, or no gate, is not settled by the
ruling; it is a question in the cut's PR. Stop until it is answered.
Also check `pncad-py`'s bindings for the gate names (LIB's): a removed
variant is a Python-visible change, announced on LIB's log first.

## Leaves for later

- The ray walk: `both-planar-ray-walks-share-one-driver-beside-the-reader`.
- The ovoid's ellipse-arc segment (PROFILE's `an-ovoid-cannot-be-authored`,
  part 1) lands after this unit: its pair rows (line × ellipse,
  circle × ellipse, ellipse × ellipse) are then the instance's arms, and
  the lowering's exhaustive match takes its one new arm.
- The pair pass's pruning (PROFILE's log, 2026-11-20: 60 teeth of 8
  pieces is about 115,000 pairs per evaluation and tier) is PROFILE's row
  to file; this unit's PR says what the instance costs on it.

## Rows

Closes `validate-reads-in-band-carriers-before-spans-in-line-line-arc-arc`
(claimed). With unit 3 closes
`check-9-arm-5-is-a-second-copy-of-seg-planar-edge-pair-intersector`.
Closes PROFILE's `an-adjacent-pairs-shared-vertex-is-recomputed-as-a-root`
(the instance divides a held meeting's root out of its equation, C5)
and the curved half of `a-widened-shared-vertex-escalates-the-pair-pass`;
both are live on PR 7264, so the lane closes them in its own PR. Changes
PROFILE's `circle-pair-home-is-planar-by-name-and-hands-out-an-undecided-half-chord`
(seg's five-way classifier goes; arc_fillet's stays) and
`computed-points-read-uncharged-where-err-means-no-contact`.

## Territory

`crates/profile/*`: PROFILE. `crates/editor-core/tests/*` and
`crates/sweep/tests/*` goldens: announce on PROFILE's log. `pncad-py`:
LIB.

## Review

**Dual**, difficulty **H**: the sketch door's acceptance moves for every
profile, the refusal vocabulary changes, and the re-baselines must be
judged.
