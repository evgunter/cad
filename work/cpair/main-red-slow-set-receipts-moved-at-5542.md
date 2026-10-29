---
id: main-red-slow-set-receipts-moved-at-5542
kind: issue
title: Main red, pinned: #5542's segment_gap reading moved five editor-core slow-set receipts (m10_6 planted flip, m10_sym_profile ×2, sym_9 ×2) and the containment row; pinned at main's reading 2026-10-28
status: open
opened: 2026-10-28
---


Filed by GREEN (`green/slow-set-reds-pins`, 2026-10-28). The rows below
are pinned at what main measures, each with a comment naming this
row, so the nightly reads green on them while the mechanism stays
open here. Moved from `work/green/main-reds-editor-core-slow-set-and-the-k-probe-sweep.md`.

**The bisect.** `42af7c0b48` (#5548, the merge before #5542) holds every
old pin below. `fa901e4308` (the merge of #5542, `nurbs/seg-one-line`:
`profile::seg`'s `line_line` reads `segment_gap` first) moves them.
Measured locally at the default ε. The containment row (~2000 s) was not
re-run at either commit. It was green on the nightly of 2026-10-21
(`8c7db2ae37`, before #5542) and red from the 10-22 nightly on, the same
as the others.

| row | before #5542 | at #5542 | main 2026-10-28 (pinned) |
|---|---|---|---|
| `m10_6_ci_rows_interval::the_margin_thin_accounting_is_goldened_bit_exact`, planted flip | certified 0.917, `flip_crossing` 0.083 | certified 0, all budget | the same as at #5542, every ε |
| `m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row`, slab | `Plain/Decision` 928 calls, `*/Assertion` 524, no `*/Report` | 944, 532, `*/Report` 16 | 992, 532, 16; every digest moved, every ε |
| the same row, plate | `Plain/Decision` 1253 | — | 1157, with the plate's other moves (below) |
| `m10_sym_profile_interval::the_plains_ledger_lines_are_the_same_under_every_dial_set` | `Plain/Report` None / 16 / None / 16 | shipped `Some(8)` | 8 / 20 / 8 / 20 |
| `sym_9_retry_interval::sym_9_the_drive_writes_the_ladders_receipt` | `[1421, 1386, 34]`, bare `[1387, 1420, 0]` | `[1421, 1378, 34]` | `[1463, 1380, 34]`, bare `[1429, 1414, 0]`, every ε |
| `sym_9_retry_interval::sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured` | the plate `[1253, 979]` | `[1253, 991]` | the plate `[1149, 832]`, the annulus `[654, 555]`, the boss `[651, 531]`, the bracket `[1535, 1167]`, the link `[954, 813]` |
| `m10_3_r2_probes_interval::containment_fires_when_the_chamber_is_strictly_inside_the_box` | boundary leaves all `FlipCrossing`, containment fires | (not re-run) | boundary classes `[Budget, Budget]`, containment does not fire |

What moved after #5542 is unbisected. The two-hole plate's counts in the
same studies moved with PR 5728 and PR 5811 as well
(`work/offset/main-red-three-m10-interval-pins-moved-at-5728.md`), so the
plate's and the ladder's later moves are probably partly theirs.

**A fix in flight.** PR 5952 (`green/slow-set-reds`, held by the GREEN
orchestrator) fixes two defects of the #5542 reading:
- a pair that shares a vertex was read, though its gap is zero by
  construction;
- `geom_core::one_line::apart()` read an overlap by its length, so a
  perpendicular span sat at the gap's zero.

On it, at the default ε:
- the planted flip certifies 0.5 and refuses 0.5 as `flip_crossing`
  (containment fires);
- the slab's `Plain/Decision` reads 976;
- the plate's SymCounts read `[1149, 824, 229]`;
- the `sym_9` receipt does not move.

Whichever of PR 5952 and these pins lands second re-pins the rows the
other moves.

**Un-pinning owes a re-measure.** Each pinned row stops at its pin, so
what the row checks past that point has not run on main. In particular,
`m10_3_r2`'s containment checks (the body under its
`test_utils::pinned_red::row` pin) have not run since #5542. Whoever
un-pins a row re-measures what ran past its old stop.

**Closing this row** takes each move's mechanism: the #5542 part (largely
PR 5952's), and the post-#5542 moves, bisected. Then each pin is
re-blessed with its move explained, or its row restored.

Reproduce: `cargo nextest run -p editor-core --profile ci
--ignore-default-filter -E 'test(=m10_6_ci_rows_interval::the_margin_thin_accounting_is_goldened_bit_exact)
| test(/^m10_sym_profile_interval::the_/) | test(/^sym_9_retry_interval::sym_9_the_/)
| test(=m10_3_r2_probes_interval::containment_fires_when_the_chamber_is_strictly_inside_the_box)'`,
at a checkout before and at `fa901e4308`.

## 2026-10-29 — after PR 5952 landed

PR 5952 merged before these pins, so they are re-taken on main with it,
at all three ε:

| row | on main with PR 5952 |
|---|---|
| `m10_6` accounting, planted flip | certified 0.5, `flip_crossing` 0.5, containment fires (old golden: 0.917 / 0.083); re-pinned |
| `m10_sym_profile` slab ledger | `Plain/Decision` 976, `*/Assertion` 532, no `*/Report` lines; re-pinned |
| the plate ledger | `Plain/Decision` 1149; re-pinned |
| `Plain/Report` per dial set | None / 12 / None / 12 (old: None / 16 / None / 16); re-pinned |
| `sym_9` ladder | the plate `[1149, 824]`, the bracket `[1535, 1157]`; re-pinned |
| `sym_9` leaf receipt | unmoved by PR 5952, `[1463, 1380, 34]` |
| `m10_3_r2` containment | passes as written, so its pin is gone |

What is left is the post-#5542 drift these rows carry beside PR 5952's
fix, which is unbisected.
