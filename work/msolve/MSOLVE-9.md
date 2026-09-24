---
id: MSOLVE-9
kind: unit
title: A mate frame that names a face of the part and resolves at evaluation through the reach road; A11's inputs sentence revised
status: dispatched
opened: 2026-09-19
branch: msolve/9-from-face
priority: P0
cost: D
pr: 2934
---


Spec: `docs/MSOLVE-9-SPEC.md`. Ev's ruling (F) on `[ev]` PR 2256:
`MateFrame` gains a `FromFace { face, reference }` arm resolved at
evaluation through `face_pose`, on MSOLVE-6's reach road (the cached
part's own product and name table answer it in part coordinates); the
solve runs over resolved frames and its algorithm is unchanged. The
unit revises A11 rule 5's inputs sentence in `ASSEMBLY.md` — the
wording rides this unit's `[ev]` PR and waits for Ev; the unit
dispatches after that merge, after MSOLVE-8 (it rides its frame
witness). LIB's façade and Python half follows by announcement.

Ratified: Ev's word on `[ev]` PR 2895 (2026-09-20, "lgtm"), merged at
`5530c0633`; the spec and A11 rule 5's inputs sentence are on main.
Dispatches from main after MSOLVE-10 merges (both units rewrite
`mate/solve.rs`; the lane branches from a main that holds the door's
`admit_mate` rather than merging it later).

## Closed (2026-09-20, MSOLVE-9 lane)

Landed on `msolve/9-from-face`. `MateFrame` is `Authored(AuthoredFrame)`
| `FromFace(FaceFrame)`; `MateReach::face_pose` answers the face's
canonical pose off the cached product in the part's own coordinates
(`CacheReach`, `PartReach`; `RefusingReach` refuses `PartUnresolved`);
the solve resolves each side before the coset table reads it
(`mate/solve.rs` `resolve_side`) and forms the datum's lever term over
the resolved origins; replay declines the face as it declines the
rider (`Maintain::reach`). The rows, `crates/editor-core/tests/
msolve9_from_face.rs`: A1 `a1_the_mate_follows_the_edited_face`; A2
`a2_the_reachs_face_pose_is_the_parts_own_face_frame_bit_for_bit`,
`a2_every_analytic_carrier_resolves_to_face_pose_bit_for_bit`,
`a2_the_sense_bit_is_not_folded_and_axis_sense_alone_decides`,
`a2_a_nurbs_face_refuses_no_canonical_frame_typed`;
`a_tied_face_refuses_ambiguous_at_the_door`;
`a_face_frame_under_a_dual_evaluation_refuses_unpinned`;
`a_vanished_name_refuses_no_such_name_at_the_door_and_at_evaluation_never_at_load`;
`an_unresolvable_part_faults_in_the_resolvers_voice`; A4
`a4_the_key_moves_under_an_edit_to_the_faces_part_and_holds_under_one_outside_it`;
the wire,
externally tagged under the spec's 2026-09-24 amendment,
`both_arms_round_trip_and_a_stray_key_on_either_refuses` and
`an_untagged_frame_refuses_whichever_arms_keys_it_carries`; C5
`c5_every_tracked_document_loads_on_the_tagged_wire_and_re_saves_identically`
(no tracked document carries a mate, so none moved; closes
`msolve-9-spec-prescribes-an-untagged-wire`);
and in `msolve10_door_admission.rs` the counting row
`a_from_face_side_asks_face_pose_once_per_side_at_the_door` and
`a_logged_from_face_insert_replays_with_no_store_and_loads`. The
viewer's tool authors `FromFace` (`mate_tool_flow`, `review_gui4_r1`,
`review_gui4_r2`, `assembly_walk`, `story_assembly`); the tour's stand
seats its posts on their cap faces and its migration walk shows the
shelf following the shortened posts; Python has `MateFrame.from_face`,
`mate_face_unresolved`, `face_refusal_tag` and the payload's `face`
(`test_assembly_author.py::TestMateFrameFromFace`). A face frame is
its name alone (the review fix pass dropped the authored reference: its
roll is the carrier's). Residue filed:
`from-face-frame-under-an-analysis-lane-refuses-unpinned` and
`a-face-frame-cannot-turn-its-roll`.
