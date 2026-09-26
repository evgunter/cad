---
id: the-session-box-is-built-by-hand-in-two-builders-and-inline
kind: issue
title: A box through the session (rectangle template, then extrude) has two private builders and more inline copies
status: closed
closed: 2026-09-26
opened: 2026-09-25
priority: P4
cost: E
pr: 3285
---


## Finding

- **Where**, measured at `5fe3dff36` plus the insert-door fix pass:
  a box authored through the SESSION — `SessionOp::AddProfile` with
  the chrome's `ProfileShape::Rectangle` template, then
  `SessionOp::AddExtrude` over it — is a named builder twice and
  inline elsewhere:
  - private builders: `combine_ops.rs` `boxed(session, [w, d, h])` and
    `blend_authoring.rs` `boxed(session, side)` (the cube case of the
    first), each preceded by `common::xy_frame_in`;
  - `story_assembly.rs` `author_box_part` (the same pair, then saved
    as a part);
  - inline, the template followed by an extrude within nine to
    eighteen lines: `combine_ops.rs`
    (`a_duplicate_is_not_measured_off_a_picture_older_than_the_document`),
    `creation_ops.rs` (`a_bracket_block_authors_saves_reloads_and_undoes`,
    `the_op_vocabulary_exceeds_the_chrome_templates_and_that_works`,
    `a_boss_is_authored_on_a_picked_face`), `docm1_face_frame.rs`
    (`a_pick_whose_node_an_undo_took_away_is_refused_as_gone`) and
    `story_authoring.rs`'s rook, four times.
  The home would be `common` beside `xy_frame_in` / `instance_in`: a
  `box_in(session, plane, [w, d, h]) -> (profile, extrude)`. The
  story rows may reasonably stay longhand — a story suite reads as the
  user's sequence of gestures, and whether a door keeps that readable
  is this row's per-site question.
- **Not members**: the eight other template sites draw a profile and
  stop (seat-kind rows, refusal rows, `creation_ops`' template-shape
  oracle, `profile_edit`'s `authored`).
- **Importance**: low. No oracle rides on the builder.
- **Instrument, and its blind spot**: `git grep -n
  'ProfileShape::Rectangle' -- crates/viewer/tests` (19 sites), each
  checked for an `AddExtrude` within the next twenty lines. A box
  whose extrude is further away, whose profile is a `Path` or
  document-door rectangle, or that is authored through the document
  door (`common::rectangle` + `Node::Extrude`, e.g. `common::asm`'s
  `box_part`) is outside this instrument; the document-door boxes are
  the brick rows' class, not this one.
- **Raised by**: the S-DUP insert-door unit's fix pass, 2026-09-24,
  naming the rectangle census's blind spots.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` with no single ground-owner; the class is one construction
spelled about a dozen times, which is this program's charter.

## Closed

Folded by the S-DUP `viewer-drain` lane, 2026-09-26, cut from
`0c1932667`.

- **Census re-taken**: `git grep -n 'ProfileShape::Rectangle'` with no
  path argument — 19 test sites, all under `crates/viewer/tests/`, plus
  the two `src/` sites that define the template. Each read.
- **The homes**, in `common` beside `xy_frame_in`:
  `rectangle_in(session, plane, w, h)` (the template through
  `SessionOp::AddProfile`), `box_in(session, plane, [w, h, d])` (that,
  then `SessionOp::AddExtrude`; answers profile and extrude) and
  `xy_box_in(session, size)` (on a fresh world xy frame; answers the
  extrude).
- **Routed**: both private builders (`combine_ops::boxed`,
  `blend_authoring::boxed`, 51 calls between them) are gone; the inline
  boxes in `combine_ops`, `creation_ops` (bracket block, boss),
  `docm1_face_frame`, `story_assembly::author_box_part` and all four of
  `story_authoring`'s rook boxes take the doors. The rook already
  authored its discs through a suite helper (`circle_at`), so a box
  door reads as that story's own style.
- **Beyond the row, same construction**: the template profiles drawn
  and not extruded (`combine_ops`' seat-kind and split-seat rows,
  `blend_authoring`'s not-a-body row, `creation_ops`'
  `new_document_derives_its_id_and_clears_the_session`) route to
  `rectangle_in`.
- **Kept longhand, with the reason**: `creation_ops`'
  `the_rectangle_template_is_the_centred_polygon` — its subject is the
  template's lowering, compared against a polygon spelled beside it;
  the multi-loop plate in
  `the_op_vocabulary_exceeds_the_chrome_templates_and_that_works`
  (a rectangle with two circles is not this construction);
  `profile_edit`'s template list and `creation_ops`' non-finite
  template, which never reach the session; `path_authoring`'s square,
  which is a `Path` profile and that suite's subject.
- The plants, and every row each one reddened, are listed below.

## Plant red lists

Every red row in the `viewer::all` binary (805 rows each run; passed + failed = 805; tree restored byte-exact and checked clean), grouped by suite. The instrument: a copy/restore harness over `cargo nextest run -p viewer --no-fail-fast`, reading every `FAIL` line.

- **`D_rectangle_wider`**, 14 red:
  - `blend_authoring`: `a_box_fillet_authors_from_picks_with_a_canonical_selection`
  - `combine_ops`: `a_duplicate_keeps_the_notes_promise`, `a_duplicates_copy_moves_on_its_own`, `a_part_indexes_a_patterns_instances_by_an_exact_count`, `a_part_of_a_pattern_takes_the_pattern_out_of_the_drawn_set`, `a_two_body_union_authors_evaluates_saves_and_reloads`, `duplicating_a_body_leaves_two_roots_and_two_drawn_copies`, `subtraction_is_not_commutative_in_the_authored_order`, `the_fused_door_mints_one_body_a_boolean_seat_takes`, `the_split_door_takes_a_body_and_a_datum_plane`, `the_transform_door_places_a_body_with_literal_slots`
  - `creation_ops`: `a_bracket_block_authors_saves_reloads_and_undoes`
  - `story_assembly`: `the_windmill_story`
  - `story_authoring`: `a_chess_rook_is_authored_probed_branched_and_reopened`
- **`D_box_extrude_deeper`**, 15 red:
  - `blend_authoring`: `a_box_fillet_authors_from_picks_with_a_canonical_selection`
  - `combine_ops`: `a_duplicate_keeps_the_notes_promise`, `a_duplicates_copy_moves_on_its_own`, `a_part_indexes_a_patterns_instances_by_an_exact_count`, `a_part_of_a_pattern_takes_the_pattern_out_of_the_drawn_set`, `a_two_body_union_authors_evaluates_saves_and_reloads`, `duplicating_a_body_leaves_two_roots_and_two_drawn_copies`, `subtraction_is_not_commutative_in_the_authored_order`, `the_fused_door_mints_one_body_a_boolean_seat_takes`, `the_split_door_takes_a_body_and_a_datum_plane`, `the_transform_door_places_a_body_with_literal_slots`
  - `creation_ops`: `a_boss_is_authored_on_a_picked_face`, `a_bracket_block_authors_saves_reloads_and_undoes`
  - `story_assembly`: `the_windmill_story`
  - `story_authoring`: `a_chess_rook_is_authored_probed_branched_and_reopened`
- **`D_rectangle_panic`**, 63 red:
  - `blend_authoring`: `a_blend_the_kernel_refuses_badges_on_the_authored_node`, `a_box_fillet_authors_from_picks_with_a_canonical_selection`, `a_held_set_marks_exactly_the_edges_it_names`, `a_pick_on_another_body_is_refused_and_keeps_the_held_edges`, `a_stranded_selection_refuses_typed_rather_than_shrinking`, `an_authored_blend_saves_and_reloads`, `an_emptied_set_releases_its_target`, `an_empty_set_refuses_at_the_tool_and_at_evaluation`, `an_upstream_edit_that_strands_held_edges_drops_them_and_says_so`, `losing_the_target_voids_the_whole_set_and_says_so`, `only_a_drawn_selection_names_a_body_for_the_all_edges_door`, `only_the_open_blend_tool_accumulates_edges`, `picking_a_held_edge_again_removes_it`, `the_all_edges_door_loads_the_set_twelve_clicks_would_have`, `the_all_edges_door_narrows_to_the_body_it_was_asked_about`, `the_all_edges_door_refuses_a_target_with_no_edges`, `the_blend_door_refuses_a_target_that_is_not_a_body`, `the_chamfer_twin_authors_the_other_node_from_the_same_picks`, `the_strand_check_is_not_asked_without_an_answer`
  - `combine_ops`: `a_duplicate_is_not_measured_off_a_picture_older_than_the_document`, `a_duplicate_keeps_the_notes_promise`, `a_duplicate_lands_clear_of_its_original`, `a_duplicates_copy_moves_on_its_own`, `a_fused_pattern_round_trips_through_save_and_open`, `a_non_positive_count_refuses_at_the_node_not_at_the_door`, `a_part_indexes_a_patterns_instances_by_an_exact_count`, `a_part_of_a_pattern_takes_the_pattern_out_of_the_drawn_set`, `a_part_projects_the_named_half_of_a_split`, `a_pick_both_seats_admit_and_a_pick_neither_does_follow_the_plain_rule`, `a_refusal_at_any_body_seated_door_leaves_no_history_state`, `a_second_body_re_targets_the_split_rather_than_displacing_its_plane`, `a_second_pick_replaces_a_one_seat_tools_body`, `a_two_body_union_authors_evaluates_saves_and_reloads`, `a_viewport_pick_seats_the_drawn_body_in_every_body_seat`, `duplicating_a_body_leaves_two_roots_and_two_drawn_copies`, `duplicating_a_moved_copy_picked_in_the_viewport_duplicates_the_copy`, `duplicating_a_several_body_value_is_refused`, `duplicating_before_anything_has_landed_is_refused`, `duplicating_is_one_undo`, `each_combining_tool_holds_its_picks_and_survives_a_vanished_one`, `every_seats_wanted_kind_is_the_one_its_door_refuses_by`, `overlapping_placements_refuse_on_the_fused_nodes_own_badge`, `several_bodies_are_not_one_body_at_a_seat`, `subtraction_is_not_commutative_in_the_authored_order`, `the_boolean_door_refuses_a_non_body_seat_and_a_self_boolean`, `the_fused_door_mints_one_body_a_boolean_seat_takes`, `the_open_tool_consumes_the_selection_stream`, `the_output_choice_names_both_doors_and_defaults_to_instances`, `the_part_door_refuses_the_crossed_selector`, `the_part_seats_track_the_evaluators_part_door`, `the_part_tool_seats_a_pattern_picked_in_the_viewport`, `the_part_tool_seats_a_split_picked_in_the_viewport`, `the_part_tools_seats_route_a_pick_to_the_one_that_can_hold_it`, `the_pattern_door_spells_its_count_structurally`, `the_split_door_takes_a_body_and_a_datum_plane`, `the_transform_door_places_a_body_with_literal_slots`, `undo_and_redo_walk_the_fused_pattern_out_and_back`
  - `creation_ops`: `a_boss_is_authored_on_a_picked_face`, `a_bracket_block_authors_saves_reloads_and_undoes`, `new_document_derives_its_id_and_clears_the_session`
  - `docm1_face_frame`: `a_pick_whose_node_an_undo_took_away_is_refused_as_gone`
  - `story_assembly`: `the_windmill_story`
  - `story_authoring`: `a_chess_rook_is_authored_probed_branched_and_reopened`
