---
id: main-red-rows-two-lanes-found-red-on-their-merge-base-are-held-by-closed-rows-or-none
kind: issue
title: Main red: rows two lanes found red on their merge base at the default eps are held by closed rows, by rows that say 1e-12, or by none
status: closed
opened: 2026-11-22
priority: P0
cost: E
pr: 7400
closed: 2026-11-22
---


Reported by two BOXFINISH lanes and a reviewer on 2026-11-22, each
running a slow set on its merge base (`main` at `ae076e05`), at the
default ε. Filed by the BOXFINISH orchestrator from their reports;
**not re-run by the filer**. A red row held by a closed row is held by
nothing, so these are listed for GREEN to confirm and re-home.

**Held by a row whose status is `closed`:**

- `sweep::all germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`,
  cited to `main-red-eighteen-sweep-analytic-rows-red-on-main-at-the-default-eps`
  (closed 2026-11-02). Red on base and head of PR 7370 and of PR 7344.
- `topo boolean::refusal_routes::offer_rows::every_offered_tolerance_passes_just_below_it`,
  cited to `main-red-two-offered-tolerances-fail-just-below-their-margin`
  (closed 2026-11-14). Red on base and head of PR 7370 and of PR 7344.
- PR 7344's body cites two more inherited reds to closed rows (the
  sectarms row; `quadrature-interval-floor-grows…`); its fix pass is
  asked to name the test and payload of each in the PR body.

**Held by rows that say 1e-12, red here at the default ε**
(`editor-core` slow set, head and base of PR 7344 identical):

- `m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials`
- `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned`

(`main-red-three-editor-core-rows-red-at-1e-12-since-before-11-19`,
`main-red-the-plates-sym-forms-ledger-moved-at-1e-12-since-the-ellipse-speed-magnitudes`.)

**Named by no file under `work/`:**

- `editor-core` `split_pinch_keeps::the_kite_moats_subtract_names_its_edges_where_its_result_stands`,
  red on head and base of PR 7344 with identical payloads.

Other inherited reds the same reports list are held by live rows and
are not repeated here (PR 7370's body has the table).

## Seen by `oneround/unsourced-unions` (PR 7294), 2026-11-21

A differential ran the slow set (release build) on that branch, at its base (PR 7227 with main at `b80352cd`, then `2e721a928`), with every site under test switched back to its old read and again with the new read. These were red in both modes, with identical payloads in each, so neither the branch nor the base's lever changes caused them:

- At 1e-9 and 1e-12: sweep `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs` (above); topo `boolean::refusal_routes::offer_rows::every_offered_tolerance_passes_just_below_it` (above).
- At 1e-9 and 1e-12: editor-core `m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials` and `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned` (above). On the branch's final tree, in a debug build at the default ε, the plate's digests differed by two forms (`Plain/Decision ... forms 39705` against the pinned `39703`).
- Held by a closed row (`main-red-seven-sweep-and-topo-rows-red-at-1e-6-since-11-17`), at 1e-9 and 1e-12: sweep `offer_rows::every_sphere_offer_passes_just_below_it`. Its false offers are `nested_off_axis_in_the_zero_band_{union,subtract}` ("quotes None and offers Some(4.999e-11)") and `ball_beside_a_slab_in_band_{union,subtract}`.
- Held by a closed row (`main-red-a-ball-r-0-1-across-the-cylinder-rim-refuses-b-minus-a-at-edge-in-face`), at 1e-9 and 1e-12: sweep `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_r_0_1_across_the_rim`. It was red again on the final tree in a debug build.

## Re-run on PR 7384 (2026-11-22)

PR 7384's head (`dcc3610d` and its fix pass) and its merge base
(`c390e1f9`), slow sets at the default ε, `editor-core` and `profile`
also at 1e-12; red on both with the same payload. Each was cited to a
closed row in that PR's first body; each is also named by an open row,
given here.

- `topo` `offer_rows::every_offered_tolerance_passes_just_below_it`:
  `lever_arm_sector_side_in_the_zero_band: offers Some(3.535e-11), its margin gives Some(3.535533905932738e-11)`.
  Open: `scalar/main-red-offer-rows-false-offers-since-the-told-quantity-type`.
- `topo` `section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section`:
  fuzz, seed-dependent; `CAD_FUZZ_SEED=0x8fa0b9fb2e8c03f4` replays red on
  the base too (`a strip crossing at a tube circle's top … R-tan … Tangent("section_torus_square_wall_fold_half")`).
  Open: `germ/section-cert-square-wall-reads-its-trace-beside-the-square-tilt`.
- `topo` `recourse_roster::every_decided_name_has_its_door_and_every_site_is_read`:
  `an indirect site not listed: chain_parity.rs: decide: name` (and `spline_trim.rs`).
  Open: `green/main-red-recourse-roster-misses-chain-parity-and-spline-trim-decide-sites`.
- `topo` `face_normal::tests::no_source_file_folds_the_bit_by_hand`:
  the hand-fold list differs from the sanctioned one (`dihedral.rs` `kappa_rel`, `props/curved.rs` `enclosed`, …).
  Open: `onedoor/the-fold-census-keys-on-file-and-text-not-on-a-site`.
- `editor-core` `m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials`,
  red at the default ε: `race: the row's certified arm is vacuous unless this drive certifies: Receipt { certified: 1, refused: 7, splits: 7 }`.
  Open: `scalar/the-race-rows-leaves-need-two-amounts-where-they-needed-six`.
- `editor-core` `union_fold_face_face_touch::a_carrier_whose_origin_stands_far_off_hands_on_the_pass_s_line`,
  at 1e-12: `back 300: plate, rod, block: refused: … VolumeUncomputable … Escalated { cause: Indeterminate …`.
  Open: `nearpoint/a-cylinder-circle-and-a-chart-junction-escalate-in-band-at-1e-12`.

`m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned`
and `sweep` `cylinder_sphere_frame::a_ball_across_a_cylinder_wall_r_0_1_across_the_rim`
were green on both sides of this run. `germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`
was left out (30 minutes). The kite moat is unchanged:
`the kite moat's evaluation, bodies and name tables` digest
`6885395609920571358` against `11878713692985015632`.

**One more, from PR 7488's two reviewers (2026-11-23), run by both on
merge base and head with the same payload:** `sweep`
`a_ring_on_a_cone_face::the_rings_volume_brackets_its_closed_form_at_the_certified_scalar`
at ε 1e-12. The only rows that name it are closed
(`work/oneround/an-anchored-cone-section-encloses-wider-at-interval.md`,
`work/onecopy/meet-at-interval-refuses-on-its-basin-meter-and-reads-an-ends-distance-at-rounding.md`).
The other seven `sweep` rows the reviewers could not match at 1e-12
(the mixed-chain band shapes, the two section-ellipse seam poses) are
held by `main-red-seven-slow-and-1e-12-rows-no-row-holds-on-454c3e54`.
(BOXFINISH orchestrator; not re-run here.)

## Where each red is held now (GREEN U9, 2026-11-22)

(GREEN, `green/held-reds`) Each red re-read on the nightly of main
`1942290ead` (run 44989234943) and again hosted on current main
`5ea1ac0a70`, the slow-set names taken out of the slow set on throwaway
branches so they ran at every ε (runs 45051569400 and 45051677551).
The tilted rod is the exception: on current main it ran only in the
slow-set step, at the default ε (run 45051677551), so its 1e-6 and
1e-12 reds are read on the nightly alone.

| test | ε red on main | held by |
|---|---|---|
| `sweep::all germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs` | all three | `work/sectarms/main-red-the-tilted-rods-other-spins-row-asks-a-periodic-view-past-its-window.md` (P2), per-ε payloads added |
| `topo boolean::refusal_routes::offer_rows::every_offered_tolerance_passes_just_below_it` | all three | `work/scalar/main-red-offer-rows-false-offers-since-the-told-quantity-type.md` (P1); fix PR 7267 open |
| `editor-core m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials` | all three | `work/certread/the-race-rows-leaves-need-two-amounts-where-they-needed-six.md` (P3) |
| `editor-core m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned` | none | green at all three ε on current main since PR 7389's re-pin; nothing to hold |
| `topo ring_path::spline_fuzz::a_path_across_a_carried_loop_reads_the_membership_parity` (PR 7344's first further red, 1e-6) | none | green at all three ε: PR 7383 pinned the sliver pose's refusal at 1e-6 against `work/sectarms/a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band.md` (P1), which names it |
| `topo::all a_seamless_pipe_cut_by_an_oblique_plane_measures::a_seamless_pipe_cut_by_an_oblique_plane_encloses_its_closed_form_at_interval` (PR 7344's second, 1e-12) | 1e-12 | `work/green/main-red-a-seamless-pipes-curved-junction-row-is-red-at-eps-1e-12.md` (P0): one payload with its junction sibling, added there |
| `editor-core split_pinch_keeps::the_kite_moats_subtract_names_its_edges_where_its_result_stands` | all three | `work/green/main-red-the-kite-moats-digest-moved-in-value-at-pr-7411.md` (P0): spelling only at PR 7131, values at PR 7411 (below) |

**The kite moat, at PR 7131.** Its evaluation digest read
`0x5f8d_d6d3_ab59_87de` at every ε where `0xa4d9_ab2a_eaf0_bd50` is
pinned. Bisected on hosted
CI: green at `e30349eab8` (PR 7131's first parent), red at
`9bc9d6517b` (the merge of PR 7131, `onehome/carrier-frames`) with
main's digest to the bit, so no later merge moved it. PR 7206 added the
row and its pin, and 7131 had merged that main before landing, but it
did not re-pin this digest. The digest takes geometry
through `Debug` (`fixture::digest`). PR 7131's variants hold their frame,
so `Plane` renders as `Plane { frame: Frame { .. } }`. Its body re-pins
every other evaluation digest it moved for that spelling, with the
values bit-identical everywhere except a blend's squaring. The kite
fixture has no blend: a block, a pocket and a moat extruded, subtracted
and split. The placed-and-moved name guard on the same fixture
(`the_kite_moats_cuts_name_alike_placed_and_moved`) is green before and
after. So the reading 7131 moved is right.

**The kite moat, at PR 7411** (GREEN U14, 2026-11-23). By the time this
PR's head merged main `9eb483e0c7` the digest read `0xfe07_0805_06b3_9e32`
instead, and a chunk dump bisected that second move to PR 7411
(`germ/plane-pose-far-frame`, `3547fe5764`, the re-posed
`plane_offset`). It moved values, not spelling: certificate residuals
and last-ulp line origins, directions and ends. So this PR does not
re-pin the row. It stays red on main, held by
`main-red-the-kite-moats-digest-moved-in-value-at-pr-7411`, which
carries both moves' evidence.

## Closed (GREEN orchestrator, 2026-11-22)

Every red the reports listed is now held by a live row that names it,
or is green on main (the table above). PR 7400's STYLE review confirmed
the kite moat's PR 7131 move is spelling only, by dumping the digest's
chunks on both sides of PR 7131; its PR 7411 move is not, and its own
row holds it.
