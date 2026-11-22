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
