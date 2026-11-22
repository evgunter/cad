---
id: a-door-reading-the-minted-axis-loses-identities-with-quotient-minted-partners
kind: issue
title: A door that reads the profile's minted axis on one wall loses identities with a partner whose director is the chord's quotient, and the projection span folds only under rule A
status: open
opened: 2026-11-22
---


Split from `the-sweep-doors-read-the-profiles-recorded-directions`
(2026-11-22), which shipped the validated type with a segment's
directions in `SegmentKind`'s arms and the sweep traversal
(`sweep::swept::Traversed`) carrying them, read by no door. This row
is the line half of the doors reading them; the arc half is
`a-verb-stores-a-radius-as-a-magnitude-and-the-symbolic-tier-cannot-use-the-sign-its-gate-decided`.
It waits on the mint's question, how validation decides that a
director lies along an axis
(`the-axis-mint-asks-for-proof-only-where-the-value-reads-zero`,
with Ev on `[ev]` PR 6993).

## Measured

On the doors' branch, head `32eb1f08`, against its merge base
`c4af8bae` (the frames and the proven-zero mint on main), the slow sets
of `profile`, `sweep`, `topo` and `editor-core` at the default ε and
`sweep` at 1e-6 and 1e-12. With `swept::sketch_direction`'s line arm
reading the recorded director and the line rim's span spelled
`(b − a)·d` (the chord read along it), the base is green everywhere and
the head reds 20 rows. Two costs, separable.

**(i) A minted axis beside a quotient-minted partner.** Validation
mints a side's director as the exact axis `(±1, 0)` where the tier
proves the other component zero, and as the chord over its length
otherwise. A wedge's near side `(ri, 0) → (ro, 0)` gets `(1, 0)`; its
far side `(ri·c, ri·s) → (ro·c, ro·s)` gets `(ro − ri)(c, s)/|ro − ri|`
(`√((ro − ri)²(c² + s²))` is `|ro − ri|` under Pythagoras). The
coincidence door's seat identity, one member's far wall against the
next member's near wall, then needs `(ro − ri)/|ro − ri| ≡ 1`: a sign
the tier cannot use, the arc half's missing fact on lines. When both
walls were the door's own quotients the factor cancelled. Rows:
- `editor-core` `g_ring_closure_measure::a_ring_stepped_by_an_exact_turn_proves_every_side_seat`,
  `…::a_ring_with_a_member_at_a_rational_cosine_proves_every_side_seat`,
  `…::a_ring_on_a_tilted_axis_proves_every_side_seat`,
  `…::a_far_side_written_through_the_half_angle_proves_every_seat`,
  `…::a_ring_stepped_by_a_free_step_leaves_the_closing_seat_alone_unproven`:
  every side seat unproven (e.g. N = 5: `[(0,1), (0,4), (1,2), (2,3),
  (3,4)]` against `[]`);
- `editor-core` `m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials`
  ("the drive must freeze": it froze nothing) and
  `coincidence_door::two_copies_on_one_axis_glue_their_walls_on_one_axis`
  (a rung list moved).

**(ii) The projection span folds only under rule A.** `(b − a)·d` for
the quotient `d = (b − a)/‖b − a‖` is `‖b − a‖` only through
`sqrt(X)² = X`, which the plain walks and some retry rungs do not have.
With the axis it folds with no rule, and the span `‖b − a‖` beside an
axis does not (`√((2 − r0)²)` against `2 − r0`: 72 decisions of the
washer revolve went numeric with it). Rows that moved with the
projection span on either director:
- `editor-core` `sym_9_retry_interval::sym_9_the_drive_writes_the_ladders_receipt`
  (`[1328, 1864, 14]` with the director read, `[1288, 1904, 0]` with
  the quotient and the projection span, pinned `[1390, 1802, 0]`);
- `editor-core` `coincidence_door::two_blocks_resting_on_one_floor_are_one_plane_at_their_bases`
  (three plane-ladder rungs `OneConstruction` → `Budget`);
- `editor-core` `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned`,
  `…::the_forms_the_slabs_walks_build_are_pinned_per_eps_row`,
  `m10_9_pins_interval::m10_9_the_pad_at_both_rule_f_dials`,
  `sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling`,
  `m10_5_r1_probes_interval::an_l_shaped_face_holds_where_it_has_no_material`
  (form digests, frozen counts, a receipt);
- `sweep` at 1e-12 `a_closed_conic_in_the_partners_face::short_arcs_at_a_crescents_tip`,
  `band_planar_cut_off_interval::a_sheared_fillets_foot_lines_certify_with_headroom`,
  `…::a_fillet_foot_on_atan2s_cut_at_an_oblique_end_carves_at_the_certified_scalar`;
  and `mass_props_are_thread_count_invariant`'s two golden rows at
  every ε (frozen counts only).

**The two re-spellings.** The line arm back on the chord's quotient
with the projection span kept: (i)'s rows go green, (ii)'s stay red and
`sym_9` gets worse. Both back to the base spelling (one
`direction_and_length` reading, director and `‖b − a‖`): all of them
green, which is what shipped.

## What reading the record buys, and what closes this row

A width-boxed slab's three moving walls store exact normals and
`u_ref`s, every face exact: measured with the director read, as
`sweep` `a_sweep_keeps_the_frame_it_is_handed::a_slab_over_a_width_box_stores_its_fixed_faces_exact_and_holds_the_moving_walls`
did then (it holds its members today). The closing tests, all written
and run on the doors' branch (commit `32eb1f08`):
- that slab row tightened to exact (red under the line arm dividing the
  chord);
- `editor-core` `a_driven_leaf_holds_its_members_wall_normals`: a leaf
  the driver certifies holds every member's wall normals, eager over
  the leaf and deferring at each member, on an identity hop and on a
  quotient hop between two variables equal at the nominal. With the
  director read, the quotient row is red under an axis minted for an
  escalated component; the identity row only under a sign mutant that
  the drive itself refuses (no leaf certifies). With no door reading
  the director they guard nothing, so they wait here;
- a table's written director `d` is accepted at `‖o + ℓ·d − p‖` within
  the band, which bounds `(|d| − 1)·ℓ`, so a door reading it stores a
  plane normal off unit by up to band/ℓ that nothing at rest reads:
  `work/restdatum/unlevered-frame-conventions-are-uncertified-at-rest.md`
  then holds it (the pin, a square whose side records `(1 + ε/5, 0)`,
  is on that commit too).
