---
id: MSOLVE-9
kind: unit
title: A mate frame that names a face of the part and resolves at evaluation through the reach road; A11's inputs sentence revised
status: open
opened: 2026-09-19
branch: msolve/9-from-face
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
`a2_a_nurbs_face_refuses_no_canonical_frame_typed`; the reference rule
`a_carried_reference_beside_an_authored_one_refuses_and_no_reference_is_pinned`;
`a_vanished_name_refuses_no_such_name_at_the_door_and_at_evaluation_never_at_load`;
`an_unresolvable_part_faults_in_the_resolvers_voice`; A4
`a4_the_key_moves_with_the_face_and_holds_otherwise`; the wire
`both_arms_round_trip_and_a_stray_key_on_either_refuses`; C5
`c5_every_tracked_document_reads_its_frames_as_authored_and_re_saves_identically`;
and in `msolve10_door_admission.rs` the counting row
`a_from_face_side_asks_face_pose_once_per_side_at_the_door` and
`a_logged_from_face_insert_replays_with_no_store_and_loads`. The
viewer's tool authors `FromFace` (`mate_tool_flow`, `review_gui4_r1`,
`review_gui4_r2`, `assembly_walk`, `story_assembly`); the tour's stand
seats its posts on their cap faces and its migration walk shows the
shelf following the shortened posts; Python has `MateFrame.from_face`,
`mate_face_unresolved`, `face_refusal_tag` and the payload's `face`
(`test_assembly_author.py::TestMateFrameFromFace`). Residue filed:
`from-face-frame-under-an-analysis-lane-refuses-unpinned`.
