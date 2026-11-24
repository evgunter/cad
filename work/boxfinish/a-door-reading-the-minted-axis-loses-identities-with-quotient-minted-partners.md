---
id: a-door-reading-the-minted-axis-loses-identities-with-quotient-minted-partners
kind: issue
title: A door that reads the profile's minted axis on one wall loses identities with a partner whose director is the chord's quotient, and the projection span folds only under rule A
status: parked
opened: 2026-11-22
priority: P1
cost: H
blocked_on: [the-one-plane-reading-multiplies-a-normal-by-a-decided-sign, a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided]
---


Split from `the-sweep-doors-read-the-profiles-recorded-directions`
(2026-11-22, PR 7384), which shipped the validated type with a
segment's directions in `SegmentKind`'s arms and the sweep traversal
(`sweep::swept::Traversed`) carrying them, read by no door. This row
is the line half of the doors reading them; the arc half is
`a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided`.
The mint's question (how validation decides a director lies along an
axis: [ev] PR 6993, the structural question) is answered on main now;
the measurement below is against that mint.

## Measured (2026-11-22, PR 7384's head `dcc3610d`, against its merge base `c390e1f9`)

**The sites a door reads a line's direction at today.** Every door
reads it from the segment's points, in `sweep/src/swept.rs`:
- `sketch_vector`'s line arm, `b − a`: the wall frame turned onto it
  (`Placing::frame_toward`, which normalizes it) and
  `sketch_right_vector`, its right normal (the extruded line wall);
- `sketch_chord`, `(b − a).direction_and_length()`: a line rim's
  carrier direction and span (`placed_segment_spec_about`'s line arm)
  and the line wall's origin and the vector course's wall normal
  (`extrude::side_surface`).

**The mutant measured: both read the recorded director.**
`sketch_vector`'s line arm returns `director`, and `sketch_chord`
returns `(director, (b − a)·director)`, the chord read along it. With
it, the slow sets of `editor-core` and `sweep` at the default ε went 20
rows red beyond the head's own reds (head: `editor-core` 2, `sweep` 1,
both inherited), and `sweep` at 1e-12 ten more rows beyond the earlier
head's 1e-12 set.

**(i) A minted axis beside a quotient-minted partner.** Validation
mints a side's director as the exact axis `(±1, 0)` where the other
component is identically zero, and as the chord over its length
otherwise. A wedge's near side `(ri, 0) → (ro, 0)` gets `(1, 0)`; its
far side `(ri·c, ri·s) → (ro·c, ro·s)` gets `(ro − ri)(c, s)/|ro − ri|`.
The coincidence door's seat identity, one member's far wall against the
next member's near wall, then needs `(ro − ri)/|ro − ri| ≡ 1`: a sign
the tier cannot use, the arc half's missing fact on lines. When both
walls were the door's own quotients the factor cancelled. Rows
(`editor-core`):
- `g_ring_closure_measure::a_ring_stepped_by_an_exact_turn_proves_every_side_seat`
  (N = 5: unproven `[(0,1), (0,4), (1,2), (2,3), (3,4)]` against `[]`),
  `…::a_ring_with_a_member_at_a_rational_cosine_proves_every_side_seat`,
  `…::a_ring_on_a_tilted_axis_proves_every_side_seat`,
  `…::a_far_side_written_through_the_half_angle_proves_every_seat`
  (all twelve seats unproven), and
  `…::a_ring_stepped_by_a_free_step_leaves_the_closing_seat_alone_unproven`;
- `coincidence_door::two_copies_on_one_axis_glue_their_walls_on_one_axis`
  (a rung list moved).

**(ii) The projection span folds only under rule A.** `(b − a)·d` for
the quotient `d = (b − a)/‖b − a‖` is `‖b − a‖` only through
`sqrt(X)² = X`, which the plain walks and some retry rungs do not have;
with the axis it folds with no rule, and `‖b − a‖` beside an axis does
not (`√((2 − r0)²)` against `2 − r0`). Rows:
- `editor-core` `sym_9_retry_interval::sym_9_the_drive_writes_the_ladders_receipt`
  (`[1328, 1863, 14]` against `[1390, 1801, 0]`: 62 decisions symbolic
  to numeric);
- `editor-core` `coincidence_door::two_blocks_resting_on_one_floor_are_one_plane_at_their_bases`
  (three plane-ladder rungs `unproven/Numeric/OneConstruction` →
  `unproven/Numeric/Budget`);
- `editor-core` form and freeze pins:
  `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned`,
  `…::the_forms_the_slabs_walks_build_are_pinned_per_eps_row`,
  `m10_9_pins_interval::m10_9_the_pad_at_both_rule_f_dials` (frozen
  1967 → 1688),
  `sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling`
  (`r2_link` frozen 2600 → 2548, `r2_filleted_bracket` 57 → 48,
  `r2_rounded_pad` 1967 → 1688),
  `m10_5_r1_probes_interval::an_l_shaped_face_holds_where_it_has_no_material`
  (its receipt), `m10_6_ci_rows_interval::the_margin_thin_accounting_is_goldened_bit_exact`
  (a certified mass's low bits), `m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval`
  (its digest), `docm9_range::a_budget_too_small_is_indeterminate_and_never_a_bound`
  ("certified to 0");
- `sweep` `swept::tests::the_placed_lines_far_end_folds_to_the_far_vertex_in_the_plain_tier`
  (`[None, None, Some(Zero)]`: with no rule the quotient's projection
  span does not fold), `band_mixed_corner_odd_plus_one::the_odd_edge_with_one_pair_edge_chamfers_at_a_non_right_plan_angle`
  (a minted edge strays 1.16e-12 from a face it joins, at `Interval`),
  `mass_props_are_thread_count_invariant`'s two golden rows (frozen
  counts);
- `sweep` at 1e-12: `a_closed_conic_in_the_partners_face::short_arcs_at_a_crescents_tip`,
  `band_closed_mixed_chain_shapes::arcs_flanking_a_short_flat_fillet_whole`,
  `band_planar_cut_off_interval::a_fillet_foot_line_carries_the_bands_axis`,
  `…::a_fillet_foot_on_atan2s_cut_at_an_oblique_end_carves_at_the_certified_scalar`,
  `…::a_sheared_fillets_foot_lines_certify_with_headroom`,
  `band_turn_overrun_chamfer::the_overrun_carves_at_the_certified_scalar`,
  `band_turn_overrun_fillet::the_overrun_carves_at_the_certified_scalar`,
  and the 1e-12 copies of the plain-tier, `band_mixed_corner` and golden
  rows.

**The two re-spellings** (measured 2026-11-16 on `32eb1f08`, before
main's frame re-spelling): the line arm back on the chord's quotient
with the projection span kept cured (i) and not (ii), and `sym_9` got
worse (`[1288, 1904, 0]`); both back to the base spelling cured all of
them, which is what PR 7384 shipped.

## What reading the record buys, and the tests that close this row

A width-boxed slab's three moving walls store exact normals and
`u_ref`s, every face exact, when the director is read. The tests,
written and run with the director read, are in git history (none is on
the tree):
- `crates/sweep/tests/a_sweep_keeps_the_frame_it_is_handed.rs` at
  `32eb1f08`, `a_slab_over_a_width_box_stores_its_fixed_faces_exact_and_holds_the_moving_walls`:
  the caps and all four walls' normals and `u_ref`s exact unit vectors
  (`assert_exact_unit`, width ≤ 1e-14, length 1 ± 1e-14), and the
  enclosure holds every member's normal. Red when the line arm divides
  the chord (`width box moving wall 0 normal.y is exact to rounding:
  [-1.0000000000312506, -0.9999999999687497]`).
- `crates/editor-core/tests/a_driven_leaf_holds_its_members_wall_normals.rs`
  at `32eb1f08` (mounted in that commit's `tests/all.rs`), two rows: a
  leaf the driver certifies (`DriveConfig { max_leaves: 16, .. }`)
  holds every member's wall normals (its corners and middle, built at
  `f64`), face by face to 4 ulps, in what the drive's eager session
  stores over the whole leaf and in what a deferring session (the
  coincidence door's) stores at the member. The identity hop: a slab
  `(0,0) (w,0) (w,1) (0,1)`, `w = 1 ± 0.05`. The quotient hop:
  `(0,0) (w,0) (v,1) (0,1)`, `w` and `v` both `1 ± 1e-10`. The quotient
  row is red under an axis minted for an escalated component
  (`seg::axis_or`): the stored normal reads exact `[0, 0]` in `y` where
  a corner member's is `2.0e-10`.
- `crates/sweep/tests/a_sweep_keeps_the_frame_it_is_handed.rs` at
  `7d6a48d5`, `a_tables_director_is_stored_as_a_walls_normal_at_its_own_length`:
  the unit square whose bottom side records the director
  `(1 + ε/5, 0)` validates, and exactly one wall stores a normal
  `1 + ε/5` long (4 ulps), the minted sides' walls unit. A table's
  written director is accepted where `‖o + ℓ·d − p‖` is within the
  band, which bounds `(|d| − 1)·ℓ`, so a door that reads it stores a
  plane normal off unit by up to band/ℓ, and nothing at rest reads a
  plane normal's length. This row holds that evidence itself; the
  convention is
  `work/restfront/unlevered-frame-conventions-are-uncertified-at-rest.md`'s.

## Weighed (2026-11-24): two designers agree after two rounds; the build goes ahead in two units

The question as this row asked it (which spelling of a direction a
door reads) is not what either measured failure is about, and both
designers said so at their first reports. Failure (i) is in a reader:
the one-plane reading multiplies a normal by a decided sign. Failure
(ii) is that a line's extent is stored nowhere, so each reader
re-derives it (the chord's norm, the chord projected on the director),
and each re-derivation is a new spelling.

**The agreed final state.**
- **A validated line side records its director and its signed length
  together**, minted at one site so that start + length·director is the
  end as a formula: the exact axis with the signed other component
  where a component is proved zero as an expression; the chord's unit
  and norm from one read otherwise; a verb that states a side by
  direction and length records what it was given (an angle leg's
  authored length; for a rectangle the authored width and the projected
  chord are one polynomial).
- **Every reader reads the pair and none reads the two points:** the
  sweep doors (their line arms go; no validated line lacks the pair, so
  no fallback is written), the edge's own description
  (`start + t·director`, `t` in `[0, length]`), the verbs' junction
  checks, and `emit_straight_leg_at`, which recovers a length it was
  handed through a root. A run of collinear sides joined into one takes
  its first member's director and the summed lengths.
- **The one-plane reading** is its own unit, which lands first:
  `the-one-plane-reading-multiplies-a-normal-by-a-decided-sign`.
- Nothing is registered, the symbolic tier is unchanged, no ratified
  text changes. One sentence for the profile README: a line records its
  director and its length, minted together so that its start plus its
  length along its director is its end as a formula.

**What it does not reach:** a sloped side built from points whose
direction does not move over the box (a wedge's far side) keeps the
width of its quotient; only a construction that holds the direction
can record it exactly
(`fillet-side-and-centre-arcs-and-off-axis-hops-record-no-direction`).

**Not put to Ev** (logged): the designers agree, no ratified decision
changes, and the answer is E12's own principle applied as the arc half
applied it. The draws, both reports and both rounds are on
`analysis/design-fork/a-door-reading-the-minted-axis-loses-identities-with-quotient-minted-partners`.

**This row is now the record half** (cost H): the pair in
`SegmentKind::Line`, and every reader moved onto it. It waits on the
reader half and on the signed-radius unit (PR 7686 edits the same
files: `seg.rs`, `swept.rs`, the doors' arms). Its closing tests are
the three named above under "What reading the record buys", with the
edge's description reading the pair and `sym_9`'s receipt accounted.
Letter H; tier dual (a concurrent pair), as the arc half: it changes
what every line stores and what every door reads.
