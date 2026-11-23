---
id: main-red-nine-editor-core-receipt-rows-gained-two-decisions-per-circle-after-e1830a1c
kind: issue
title: Main red: nine editor-core slow-set receipt rows count two more numeric decisions per circle (least_turn_conditioning, mate_chart_pole) on dc263f85 than their pins, green on e1830a1c
status: open
opened: 2026-11-23
priority: P0
cost: E
---

Found by BOXFINISH's expression-question lane (PR 7488) on 2026-11-23,
re-merging `origin/main` at `dc263f855`. Each row below is red on
`dc263f855` itself (a clean checkout, `--ignore-default-filter`,
default ε, `CARGO_PROFILE_{DEV,TEST}_OPT_LEVEL=1`) and was green on
`e1830a1c` in that lane's slow-set run, so one of the 63 first-parent
merges in `e1830a1c..dc263f855` moved them. Not bisected. On PR 7488's
head the same rows are red with the same deltas on top of that PR's
own re-cut numbers.

`editor-core` (all in the slow set):

- `m10_bulge_interval::{m10_bulge_the_bosss_split_at_the_nominal, m10_bulge_the_d_tabs_literal_split_at_the_nominal, m10_bulge_the_d_tabs_parameter_split_at_the_nominal}`:
  `these decided but the pinned table does not list them:
  ["least_turn_conditioning [0, 1]", "mate_chart_pole [0, 1]"]`.
- `sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling`:
  `two_hole_plate: [979, 1051, 36], stored [979, 1047, 36]`,
  `r1_annulus: [511, 550, 36], stored [511, 548, 36]`, and on.
- `sym_9_retry_interval::sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured`:
  `two_hole_plate [979, 1047] -> [979, 1051]`,
  `r1_annulus [511, 548] -> [511, 550]`,
  `r1_segment_boss [489, 572] -> [489, 574]`, and on.
- `sym_9_retry_interval::sym_9_the_drive_writes_the_ladders_receipt`:
  `[1390, 1803, 0]` against the pinned `[1390, 1801, 0]`.
- `m10_9_pins_interval::m10_9_the_pad_at_both_rule_f_dials`:
  `(1483, 1711, 1967)` against `(1483, 1709, 1967)`.
- `m10_9_pins_interval::m10_9_what_the_registry_buys`:
  `r2_filleted_bracket: door shut (1290, 1616), stored (1290, 1612)`,
  `r2_rounded_pad: door shut (1483, 1711), stored (1483, 1709)`.
- `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned`:
  `checks_expanded: 1051` where the ledger pins 1047 assertion calls.

**The one shape:** every numeric count is up by two per circular
profile piece (the plate's two holes +4, the annulus +2, the boss +2),
the symbolic counts do not move, and the two predicates the bulge rows
name are new at the nominal: `least_turn_conditioning` and
`mate_chart_pole`, one numeric decision each. A merge that added those
two readings to a circle's build and re-pinned none of these rows is
the first guess.

**Owed:** bisect the range on the first parent; re-pin with the moving
PR's reason, or fix what it moved.
