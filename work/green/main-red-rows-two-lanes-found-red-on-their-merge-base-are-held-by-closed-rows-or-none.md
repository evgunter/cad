---
id: main-red-rows-two-lanes-found-red-on-their-merge-base-are-held-by-closed-rows-or-none
kind: issue
title: Main red: rows two lanes found red on their merge base at the default eps are held by closed rows, by rows that say 1e-12, or by none
status: dispatched
opened: 2026-11-22
priority: P0
cost: E
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
