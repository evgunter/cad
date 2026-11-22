---
id: main-red-rows-two-lanes-found-red-on-their-merge-base-are-held-by-closed-rows-or-none
kind: issue
title: Main red: rows two lanes found red on their merge base at the default eps are held by closed rows, by rows that say 1e-12, or by none
status: open
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
| `editor-core m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials` | all three | `work/scalar/the-race-rows-leaves-need-two-amounts-where-they-needed-six.md` (P3) |
| `editor-core m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned` | none | green at all three ε on current main since PR 7389's re-pin; nothing to hold |
| `topo ring_path::spline_fuzz::a_path_across_a_carried_loop_reads_the_membership_parity` (PR 7344's first further red, 1e-6) | none | green at all three ε: PR 7383 pinned the sliver pose's refusal at 1e-6 against `work/sectarms/a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band.md` (P1), which names it |
| `topo::all a_seamless_pipe_cut_by_an_oblique_plane_measures::a_seamless_pipe_cut_by_an_oblique_plane_encloses_its_closed_form_at_interval` (PR 7344's second, 1e-12) | 1e-12 | `work/green/main-red-a-seamless-pipes-curved-junction-row-is-red-at-eps-1e-12.md` (P0): one payload with its junction sibling, added there |
| `editor-core split_pinch_keeps::the_kite_moats_subtract_names_its_edges_where_its_result_stands` | all three until this PR | re-blessed here (below) |

**The kite moat.** Its evaluation digest reads `0x5f8d_d6d3_ab59_87de`
at every ε where `0xa4d9_ab2a_eaf0_bd50` is pinned. Bisected on hosted
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
after. So the new reading is right, and it is re-blessed.
