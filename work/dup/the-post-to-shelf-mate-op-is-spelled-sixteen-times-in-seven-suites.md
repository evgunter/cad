---
id: the-post-to-shelf-mate-op-is-spelled-sixteen-times-in-seven-suites
kind: issue
title: The post-top to shelf-bottom SessionOp::AddMate is written out sixteen times across seven viewer suites
status: closed
closed: 2026-09-26
opened: 2026-09-24
priority: P4
cost: E
pr: 3285
---


## Finding

- **Where**, measured at `6db5b87f2` plus the insert-door unit's fold:
  the GUI-4 bench's seat mate, written as a literal
  `SessionOp::AddMate { a: common::head(asm::in_part(<post>,
  &bench.post_top)), b: common::head(asm::in_part(bench.shelf_i,
  &bench.shelf_bottom)), class, alignment }`, sixteen times:
  `assembly_display.rs` (4: `add_seat_mate` and three inline),
  `review_gui4_r1.rs` (4), `tree_badges.rs` (3), `msolve4_blame_rows.rs`
  (2: `add_seat`, `add_rest`), `frame_policy.rs`, `instance_authoring.rs`
  and `landing_gathers.rs` (1 each). What varies is the post, the
  `class` (`Rest` or `Tangent`) and the alignment, which already has a
  home (`asm::seat_alignment` / `asm::rest_alignment`); the op around
  it has none. Three of the sixteen are private helpers doing the
  same job under three names (`add_seat_mate`, `add_seat`, `add_rest`).
- **Also seen, not a member**: three sites (`assembly_display.rs`'s
  at-rest and instance-check rows, `landing_gathers.rs`) discard the
  op's outcome entirely. Each checks the mate landed downstream (a
  `mate_nodes(..)[0]` read, a badge the mate must produce), so it is
  not a silent pass; a door would still make it one line.
- **The home** is `common::asm`, beside `seat_alignment`: an op
  builder `seat_op(bench, post, class, alignment) -> SessionOp`, which
  a row then hands to `common::session_insert` or inspects itself. The
  rows that read the outcome's `withdrawn` half keep their inline
  `perform`; they need the op, not the insert.
- **Importance**: low. No oracle rides on the op's spelling.
- **Instrument, and its blind spot**: `git grep -n -A3
  'SessionOp::AddMate {' -- crates/viewer/tests`, filtered to the
  `post_top` / `shelf_bottom` pair. Line-window-shaped (three lines
  after the variant), so a site whose fields are reordered or built
  into a `let` above the op is missed; three `AddMate` sites that take
  a tool's proposal (`review_gui4_r2.rs`, `story_assembly.rs` twice)
  and one on a single node's faces (`gesture_table.rs`) are read and
  are not this construction. A second instrument on the required
  atom, every `&bench.shelf_bottom` under `crates/viewer/tests/`, finds
  the same sixteen plus two that are not members:
  `msolve5_read_below_a_root.rs` builds its mate as a document
  `Node::Mate` through `insert_into`, not as the op, and
  `review_gui4_r2.rs` (`oracle(&bench.shelf, &bench.shelf_bottom)`)
  reads the cap's frame for an expectation and builds no mate. Scope: `crates/viewer/tests/` only,
  because the bench is `common::asm`'s and nothing outside this binary
  can reach it.
- **Raised by**: the S-DUP lane on
  `three-doors-named-insert-mean-two-different-constructions`,
  2026-09-24, while folding `add_seat_mate` onto the session door.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` with no single ground-owner; the class is one construction
spelled sixteen times, which is this program's charter.

## Closed

Folded by the S-DUP `viewer-drain` lane, 2026-09-26, cut from
`0c1932667`.

- **Census re-taken on the atom**, every `in_part(<instance>,
  &bench.shelf_bottom)` under `crates/viewer/tests/` (`git grep -n
  shelf_bottom -- crates/viewer/tests`, read site by site): **17**
  op sites, not 16 — `frame_policy.rs` had two
  (`a_refusal_reached_through_a_mate_names_the_mate_the_tree_blames`
  as well as `a_superseded_free_move_is_news_the_ranking_shows`). The
  two non-members stand as the row read them.
- **The home**: `common::asm::seat_op(bench, post, class, alignment)`,
  and `seat_op_under` for `instance_authoring`'s row, which mates a
  shelf instance it authored itself. Every one of the 17 routes
  through it; `add_seat_mate` and `add_seat` are gone, `tree_badges`'
  three `add_mate` closures are gone, and `msolve4`'s `add_rest`
  stays as the named offender over the door.
- **Commit and pump** has one home too, `common::commit_mate` (the
  insert, the `Node::Mate` kind check, the pump), moved out of
  `mate_tool_flow`. Every site that inserted a seat mate and pumped
  takes it; the two `tree_badges` rows that author two mates into ONE
  evaluation keep `session_insert` and say why in the door's doc.
- **X4 twins folded in the same files**: the middle-of-the-shelf seat
  alignment written out longhand twice (`review_gui4_r1::seat`,
  `instance_authoring::seat_alignment`) and once as a local wrapper
  (`assembly_display::seat_alignment`) is `asm::middle_seat_alignment`; the mate
  tool's seat choice written out three times beside `asm::seat_choice`
  (`review_gui4_r1::rest_choice`, `review_gui4_r2::seat`, an inline
  one in `assembly_walk`) routes to `asm::seat_choice`; and the face pick the
  tool rows start from — a private `pick_at` / `pick` written
  byte-identically in `mate_tool_flow`, `review_gui4_r2` and
  `rv_matehead_probes`, with an index-taking variant in
  `review_gui4_r1` and `story_assembly` — is `common::displayed_face_at` /
  `asm::pick_face`, the two seat picks (`two_picks` twice,
  `seat_picks`) are `asm::seat_picks`, and `mate_tool_flow`'s
  `shelf_underside` helper, which that file also spelled inline five
  more times, is `asm::shelf_underside`.
- **Fix pass** (style review at `dacc31e95`): the shelf-middle and
  post_b-middle rays are `asm::under_shelf` / `asm::over_post_b`, and
  their last copies in `assembly_display`, `assembly_walk`,
  `mate_tool_flow`, `review_gui4_r1` and `review_gui4_r2` route to
  them; `assembly_walk`'s hand-built pick is `displayed_face_at`; the
  proposal commits in `review_gui4_r1`/`_r2` and the three Tangent
  seats take `commit_mate`; the contradiction pair's second seat is
  `asm::contradicting_seat_alignment`.
- The plants, and every row each one reddened, are listed below.

## Plant red lists

Every red row in the `viewer::all` binary (805 rows each run; passed + failed = 805; tree restored byte-exact and checked clean), grouped by suite. The instrument: a copy/restore harness over `cargo nextest run -p viewer --no-fail-fast`, reading every `FAIL` line.

- **`A_seat_op_swap`**, 1 red:
  - `instance_authoring`: `an_assembly_authored_into_a_directory_of_parts_round_trips`
- **`A_seat_op_panic`**, 16 red:
  - `assembly_display`: `a_landing_mate_discards_the_probe_value`, `free_move_accepts_only_completely_unconstrained_instances`, `instance_check_tells_an_absent_node_from_a_wrong_kind`, `the_at_rest_badge_lands_with_the_evaluation`
  - `frame_policy`: `a_refusal_reached_through_a_mate_names_the_mate_the_tree_blames`, `a_superseded_free_move_is_news_the_ranking_shows`
  - `instance_authoring`: `an_assembly_authored_into_a_directory_of_parts_round_trips`
  - `landing_gathers`: `a_refused_a5_gate_eats_the_body_and_says_so_by_its_absence`
  - `msolve4_blame_rows`: `a_cluster_refusal_reaches_the_mate_that_evaluated_before_it`, `two_faults_in_succession_on_one_mate_never_serve_a_stale_one`
  - `review_gui4_r1`: `r1_every_offered_class_is_executable_and_a_tangent_commit_is_unassemblable`, `r1_hide_probe_and_mate_compose_without_a_silent_state`, `r1_the_probe_gestures_order_and_identity_edges`
  - `tree_badges`: `a_boolean_over_a_refused_clusters_instances_points_at_the_mate`, `a_contradiction_points_downstream_rows_at_a_row_that_is_actually_failing`, `a_refused_mate_solve_names_the_mate_and_reads_every_other_row_downstream`
- **`B_commit_mate_wrong_id`**, 6 red:
  - `assembly_display`: `free_move_accepts_only_completely_unconstrained_instances`, `instance_check_tells_an_absent_node_from_a_wrong_kind`
  - `frame_policy`: `a_refusal_reached_through_a_mate_names_the_mate_the_tree_blames`
  - `msolve4_blame_rows`: `a_cluster_refusal_reaches_the_mate_that_evaluated_before_it`, `two_faults_in_succession_on_one_mate_never_serve_a_stale_one`
  - `tree_badges`: `a_contradiction_points_downstream_rows_at_a_row_that_is_actually_failing`
- **`B_commit_mate_panic`**, 12 red:
  - `assembly_display`: `free_move_accepts_only_completely_unconstrained_instances`, `instance_check_tells_an_absent_node_from_a_wrong_kind`
  - `frame_policy`: `a_refusal_reached_through_a_mate_names_the_mate_the_tree_blames`
  - `mate_tool_flow`: `a_circular_pattern_copy_authors_the_masters_unrotated_frame`, `a_nested_copy_pick_reads_the_master_and_seats`, `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_pattern_placed_pick_mates_through_an_instance_headed_reference`, `a_pick_on_a_moved_instance_authors_the_transform_and_seats`, `two_picks_one_choice_one_committed_edit`
  - `msolve4_blame_rows`: `a_cluster_refusal_reaches_the_mate_that_evaluated_before_it`, `two_faults_in_succession_on_one_mate_never_serve_a_stale_one`
  - `tree_badges`: `a_contradiction_points_downstream_rows_at_a_row_that_is_actually_failing`
- **`L_commit_mate_panic`**, 18 red:
  - `assembly_display`: `free_move_accepts_only_completely_unconstrained_instances`, `instance_check_tells_an_absent_node_from_a_wrong_kind`, `the_at_rest_badge_lands_with_the_evaluation`
  - `frame_policy`: `a_refusal_reached_through_a_mate_names_the_mate_the_tree_blames`
  - `landing_gathers`: `a_refused_a5_gate_eats_the_body_and_says_so_by_its_absence`
  - `mate_tool_flow`: `a_circular_pattern_copy_authors_the_masters_unrotated_frame`, `a_nested_copy_pick_reads_the_master_and_seats`, `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_pattern_placed_pick_mates_through_an_instance_headed_reference`, `a_pick_on_a_moved_instance_authors_the_transform_and_seats`, `two_picks_one_choice_one_committed_edit`
  - `msolve4_blame_rows`: `a_cluster_refusal_reaches_the_mate_that_evaluated_before_it`, `two_faults_in_succession_on_one_mate_never_serve_a_stale_one`
  - `review_gui4_r1`: `r1_every_offered_class_is_executable_and_a_tangent_commit_is_unassemblable`, `r1_the_minted_alignment_is_the_placement_inverse_of_the_picked_world_pose`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `the_solved_seat_hangs_the_post_under_the_shelf`
  - `tree_badges`: `a_contradiction_points_downstream_rows_at_a_row_that_is_actually_failing`
- **`C_middle_seat_off_shelf`**, 1 red:
  - `instance_authoring`: `an_assembly_authored_into_a_directory_of_parts_round_trips`
- **`C_middle_seat_panic`**, 16 red:
  - `assembly_display`: `a_landing_mate_discards_the_probe_value`, `free_move_accepts_only_completely_unconstrained_instances`, `instance_check_tells_an_absent_node_from_a_wrong_kind`, `the_at_rest_badge_lands_with_the_evaluation`
  - `frame_policy`: `a_superseded_free_move_is_news_the_ranking_shows`
  - `instance_authoring`: `an_assembly_authored_into_a_directory_of_parts_round_trips`
  - `landing_gathers`: `a_refused_a5_gate_eats_the_body_and_says_so_by_its_absence`
  - `msolve4_blame_rows`: `a_cluster_refusal_reaches_the_mate_that_evaluated_before_it`, `two_faults_in_succession_on_one_mate_never_serve_a_stale_one`
  - `msolve5_read_below_a_root`: `the_badge_names_the_operand_of_a_mate_read_below_a_root`
  - `review_gui4_r1`: `r1_every_offered_class_is_executable_and_a_tangent_commit_is_unassemblable`, `r1_hide_probe_and_mate_compose_without_a_silent_state`, `r1_the_probe_gestures_order_and_identity_edges`
  - `tree_badges`: `a_boolean_over_a_refused_clusters_instances_points_at_the_mate`, `a_contradiction_points_downstream_rows_at_a_row_that_is_actually_failing`, `a_refused_mate_solve_names_the_mate_and_reads_every_other_row_downstream`
- **`M_contradiction_agrees`**, 2 red:
  - `msolve4_blame_rows`: `two_faults_in_succession_on_one_mate_never_serve_a_stale_one`
  - `tree_badges`: `a_contradiction_points_downstream_rows_at_a_row_that_is_actually_failing`
- **`G_seat_choice_clocked`**, 13 red:
  - `assembly_walk`: `the_exit_demo_walk`
  - `mate_tool_flow`: `a_circular_pattern_copy_authors_the_masters_unrotated_frame`, `a_nested_copy_pick_reads_the_master_and_seats`, `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_pattern_placed_pick_mates_through_an_instance_headed_reference`, `a_pick_on_a_moved_instance_authors_the_transform_and_seats`, `two_picks_one_choice_one_committed_edit`
  - `review_gui4_r1`: `r1_the_minted_alignment_is_the_placement_inverse_of_the_picked_world_pose`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `a_landing_mate_kills_an_in_flight_gesture`, `hide_survives_the_mate_that_discards_the_probe`, `the_solved_seat_hangs_the_post_under_the_shelf`
  - `story_assembly`: `the_windmill_story`
- **`G_seat_choice_panic`**, 21 red:
  - `assembly_walk`: `the_exit_demo_walk`
  - `mate_tool_flow`: `a_circular_pattern_copy_authors_the_masters_unrotated_frame`, `a_nested_copy_pick_reads_the_master_and_seats`, `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_pattern_copy_over_a_transform_is_an_instance_pick`, `a_pattern_placed_pick_mates_through_an_instance_headed_reference`, `a_pick_on_a_fused_body_is_not_an_instance_pick`, `a_pick_on_a_moved_instance_authors_the_transform_and_seats`, `the_tool_refuses_the_tables_static_gaps_before_any_geometry`, `the_tool_refuses_typed_what_the_picks_do_not_admit`, `two_picks_one_choice_one_committed_edit`
  - `review_gui4_r1`: `r1_the_minted_alignment_is_the_placement_inverse_of_the_picked_world_pose`, `r1_two_faces_of_one_instance_refuse_before_any_edit`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `a_landing_mate_kills_an_in_flight_gesture`, `hide_survives_the_mate_that_discards_the_probe`, `proposal_frames_agree_with_the_standalone_part_documents`, `the_solved_seat_hangs_the_post_under_the_shelf`, `two_different_faces_of_one_instance_refuse_same_pick`
  - `rv_matehead_probes`: `an_edge_named_pick_is_refused_by_the_mate_tool`
  - `story_assembly`: `the_windmill_story`
- **`F_face_at_panic`**, 26 red:
  - `assembly_walk`: `the_exit_demo_walk`
  - `combine_ops`: `a_viewport_pick_seats_the_drawn_body_in_every_body_seat`, `duplicating_a_moved_copy_picked_in_the_viewport_duplicates_the_copy`, `the_part_tool_seats_a_pattern_picked_in_the_viewport`, `the_part_tool_seats_a_split_picked_in_the_viewport`
  - `mate_tool_flow`: `a_circular_pattern_copy_authors_the_masters_unrotated_frame`, `a_nested_copy_pick_reads_the_master_and_seats`, `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_pattern_copy_over_a_transform_is_an_instance_pick`, `a_pattern_placed_pick_mates_through_an_instance_headed_reference`, `a_pick_on_a_fused_body_is_not_an_instance_pick`, `a_pick_on_a_moved_instance_authors_the_transform_and_seats`, `a_vanished_pick_degrades_the_tool_one_step_typed`, `the_tool_refuses_the_tables_static_gaps_before_any_geometry`, `the_tool_refuses_typed_what_the_picks_do_not_admit`, `two_picks_one_choice_one_committed_edit`
  - `review_gui4_r1`: `r1_the_minted_alignment_is_the_placement_inverse_of_the_picked_world_pose`, `r1_two_faces_of_one_instance_refuse_before_any_edit`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `a_landing_mate_kills_an_in_flight_gesture`, `hide_survives_the_mate_that_discards_the_probe`, `proposal_frames_agree_with_the_standalone_part_documents`, `the_solved_seat_hangs_the_post_under_the_shelf`, `two_different_faces_of_one_instance_refuse_same_pick`
  - `rv_matehead_probes`: `an_edge_named_pick_is_refused_by_the_mate_tool`
  - `story_assembly`: `the_windmill_story`
- **`K_displayed_face_at_panic`**, 26 red:
  - `assembly_walk`: `the_exit_demo_walk`
  - `combine_ops`: `a_viewport_pick_seats_the_drawn_body_in_every_body_seat`, `duplicating_a_moved_copy_picked_in_the_viewport_duplicates_the_copy`, `the_part_tool_seats_a_pattern_picked_in_the_viewport`, `the_part_tool_seats_a_split_picked_in_the_viewport`
  - `mate_tool_flow`: `a_circular_pattern_copy_authors_the_masters_unrotated_frame`, `a_nested_copy_pick_reads_the_master_and_seats`, `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_pattern_copy_over_a_transform_is_an_instance_pick`, `a_pattern_placed_pick_mates_through_an_instance_headed_reference`, `a_pick_on_a_fused_body_is_not_an_instance_pick`, `a_pick_on_a_moved_instance_authors_the_transform_and_seats`, `a_vanished_pick_degrades_the_tool_one_step_typed`, `the_tool_refuses_the_tables_static_gaps_before_any_geometry`, `the_tool_refuses_typed_what_the_picks_do_not_admit`, `two_picks_one_choice_one_committed_edit`
  - `review_gui4_r1`: `r1_the_minted_alignment_is_the_placement_inverse_of_the_picked_world_pose`, `r1_two_faces_of_one_instance_refuse_before_any_edit`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `a_landing_mate_kills_an_in_flight_gesture`, `hide_survives_the_mate_that_discards_the_probe`, `proposal_frames_agree_with_the_standalone_part_documents`, `the_solved_seat_hangs_the_post_under_the_shelf`, `two_different_faces_of_one_instance_refuse_same_pick`
  - `rv_matehead_probes`: `an_edge_named_pick_is_refused_by_the_mate_tool`
  - `story_assembly`: `the_windmill_story`
- **`F_seat_picks_panic`**, 10 red:
  - `mate_tool_flow`: `a_vanished_pick_degrades_the_tool_one_step_typed`, `the_tool_refuses_the_tables_static_gaps_before_any_geometry`, `the_tool_refuses_typed_what_the_picks_do_not_admit`, `two_picks_one_choice_one_committed_edit`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `a_landing_mate_kills_an_in_flight_gesture`, `hide_survives_the_mate_that_discards_the_probe`, `proposal_frames_agree_with_the_standalone_part_documents`, `the_solved_seat_hangs_the_post_under_the_shelf`
  - `rv_matehead_probes`: `an_edge_named_pick_is_refused_by_the_mate_tool`
- **`H_shelf_underside_panic`**, 17 red:
  - `mate_tool_flow`: `a_circular_pattern_copy_authors_the_masters_unrotated_frame`, `a_nested_copy_pick_reads_the_master_and_seats`, `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_pattern_copy_over_a_transform_is_an_instance_pick`, `a_pattern_placed_pick_mates_through_an_instance_headed_reference`, `a_pick_on_a_fused_body_is_not_an_instance_pick`, `a_pick_on_a_moved_instance_authors_the_transform_and_seats`, `a_vanished_pick_degrades_the_tool_one_step_typed`, `the_tool_refuses_the_tables_static_gaps_before_any_geometry`, `the_tool_refuses_typed_what_the_picks_do_not_admit`, `two_picks_one_choice_one_committed_edit`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `a_landing_mate_kills_an_in_flight_gesture`, `hide_survives_the_mate_that_discards_the_probe`, `proposal_frames_agree_with_the_standalone_part_documents`, `the_solved_seat_hangs_the_post_under_the_shelf`
  - `rv_matehead_probes`: `an_edge_named_pick_is_refused_by_the_mate_tool`
- **`I_under_shelf_off_shelf`**, 19 red:
  - `assembly_walk`: `the_exit_demo_walk`
  - `mate_tool_flow`: `a_circular_pattern_copy_authors_the_masters_unrotated_frame`, `a_nested_copy_pick_reads_the_master_and_seats`, `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_pattern_copy_over_a_transform_is_an_instance_pick`, `a_pattern_placed_pick_mates_through_an_instance_headed_reference`, `a_pick_on_a_fused_body_is_not_an_instance_pick`, `a_pick_on_a_moved_instance_authors_the_transform_and_seats`, `a_vanished_pick_degrades_the_tool_one_step_typed`, `the_tool_refuses_the_tables_static_gaps_before_any_geometry`, `the_tool_refuses_typed_what_the_picks_do_not_admit`, `two_picks_one_choice_one_committed_edit`
  - `review_gui4_r1`: `r1_the_minted_alignment_is_the_placement_inverse_of_the_picked_world_pose`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `a_landing_mate_kills_an_in_flight_gesture`, `hide_survives_the_mate_that_discards_the_probe`, `proposal_frames_agree_with_the_standalone_part_documents`, `the_solved_seat_hangs_the_post_under_the_shelf`
  - `rv_matehead_probes`: `an_edge_named_pick_is_refused_by_the_mate_tool`
- **`I_under_shelf_panic`**, 19 red:
  - `assembly_walk`: `the_exit_demo_walk`
  - `mate_tool_flow`: `a_circular_pattern_copy_authors_the_masters_unrotated_frame`, `a_nested_copy_pick_reads_the_master_and_seats`, `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_pattern_copy_over_a_transform_is_an_instance_pick`, `a_pattern_placed_pick_mates_through_an_instance_headed_reference`, `a_pick_on_a_fused_body_is_not_an_instance_pick`, `a_pick_on_a_moved_instance_authors_the_transform_and_seats`, `a_vanished_pick_degrades_the_tool_one_step_typed`, `the_tool_refuses_the_tables_static_gaps_before_any_geometry`, `the_tool_refuses_typed_what_the_picks_do_not_admit`, `two_picks_one_choice_one_committed_edit`
  - `review_gui4_r1`: `r1_the_minted_alignment_is_the_placement_inverse_of_the_picked_world_pose`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `a_landing_mate_kills_an_in_flight_gesture`, `hide_survives_the_mate_that_discards_the_probe`, `proposal_frames_agree_with_the_standalone_part_documents`, `the_solved_seat_hangs_the_post_under_the_shelf`
  - `rv_matehead_probes`: `an_edge_named_pick_is_refused_by_the_mate_tool`
- **`J_over_post_b_onto_post_a`**, 15 red:
  - `assembly_display`: `hiding_drops_scene_and_picks_but_keeps_tree_and_document`, `the_probe_gesture_previews_commits_and_draws_visibly_distinct`
  - `mate_tool_flow`: `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_vanished_pick_degrades_the_tool_one_step_typed`, `the_tool_refuses_the_tables_static_gaps_before_any_geometry`, `the_tool_refuses_typed_what_the_picks_do_not_admit`, `two_picks_one_choice_one_committed_edit`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `a_landing_mate_kills_an_in_flight_gesture`, `a_rotating_probe_is_picked_at_its_drawn_position`, `hide_survives_the_mate_that_discards_the_probe`, `proposal_frames_agree_with_the_standalone_part_documents`, `the_solved_seat_hangs_the_post_under_the_shelf`, `two_different_faces_of_one_instance_refuse_same_pick`
  - `rv_matehead_probes`: `an_edge_named_pick_is_refused_by_the_mate_tool`
- **`J_over_post_b_panic`**, 17 red:
  - `assembly_display`: `a_landing_mate_discards_the_probe_value`, `hiding_drops_scene_and_picks_but_keeps_tree_and_document`, `the_probe_gesture_previews_commits_and_draws_visibly_distinct`
  - `mate_tool_flow`: `a_part_over_a_pattern_pick_is_a_member_and_seats`, `a_pick_on_a_fused_body_is_not_an_instance_pick`, `a_vanished_pick_degrades_the_tool_one_step_typed`, `the_tool_refuses_the_tables_static_gaps_before_any_geometry`, `the_tool_refuses_typed_what_the_picks_do_not_admit`, `two_picks_one_choice_one_committed_edit`
  - `review_gui4_r2`: `a_contradictory_second_mate_fails_typed_and_undo_recovers`, `a_landing_mate_kills_an_in_flight_gesture`, `a_rotating_probe_is_picked_at_its_drawn_position`, `hide_survives_the_mate_that_discards_the_probe`, `proposal_frames_agree_with_the_standalone_part_documents`, `the_solved_seat_hangs_the_post_under_the_shelf`, `two_different_faces_of_one_instance_refuse_same_pick`
  - `rv_matehead_probes`: `an_edge_named_pick_is_refused_by_the_mate_tool`
