---
id: main-red-the-identity-turns-decisions-moved-the-m10-slow-rows
kind: issue
title: Main red: the identity turn's decisions (PR 7434) moved nine editor-core slow rows
status: open
opened: 2026-11-23
priority: P0
cost: M
---


PR 7434 (merged as `ccabcfb`, "the least turn reads no exactness off the
nominal") took away `geom_core::RigidMotion::least_turn`'s identity
shortcut. A pair that is one direction now turns by the identity through
the Rodrigues arm's arithmetic, and the arm spells its last term
`k × (k × v)/(1 + c)`. So every identity chart decides its pole verdict
and its conditioning like any other pair: one `mate_chart_pole` and one
`least_turn_conditioning` numeric row a chart. 7434's gate never ran the
slow rows that read those counts, because its diff edited none of their
files.

## How each red was classed

The workspace slow set (2343 tests, opt-level 2 as the nightly builds it)
ran on `origin/main` `da61db71a` at the default ε, 1e-6 and 1e-12. Every
red was re-run on `ccabcfb^1` (`fa42c60`), the merge's first parent.
A row green there is 7434's. A row red there with the same reading is not.

**Attribution.** Each document's per-predicate split was replayed at its
`certifies_at` box, with the door open and shut, on `ccabcfb` and on its
first parent. On the plate, the annulus, the link, the bracket and the pad,
the whole difference is `least_turn_conditioning` [0, 1] and
`mate_chart_pole` [0, 1] per identity chart. The plate and the bracket
have two charts; the others have one. Nothing symbolic or frozen moves.
Restoring only 7434's geom-core files (the identity shortcut back, editor-core
left as main) puts back every editor-core pin below. So
`SolveScalar::zero_everywhere` moves none of them.

## 7434's, re-pinned

| row | reading | cause |
|---|---|---|
| `m10_bulge_interval` ×3 `*_split_at_the_nominal` | `least_turn_conditioning` [0, 1], `mate_chart_pole` [0, 1] new | the copy's one identity chart |
| `m10_9_pins_interval::m10_9_the_pad_at_both_rule_f_dials` | (1483, 1709, 1967) → (1483, 1711, 1967) | the pad's one chart |
| `m10_9_pins_interval::m10_9_what_the_registry_buys` | door shut: bracket (1290, 1612) → (1290, 1616), pad (1483, 1709) → (1483, 1711) | two charts and one |
| `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned` | `*/Assertion` calls 1047 → 1051, `Door/Decision` 550 → 554, and digests | the plate's two charts. With the arm's former spelling, `Plain/Decision`'s digest still moves: the identity's turn is now built forms where it was `world()`. |
| `sym11_exact_channel_rows::sym11_the_exact_channel_never_contradicts_past_the_ceiling` | numeric: plate +4, annulus +2, link +2, bracket +4, pad +2 | as above; no dispute fired |
| `sym_9_retry_interval::sym_9_the_kept_atom_ladder_recovers_what_phase_1_measured` | numeric: plate +4, annulus +2, boss +2, bracket +4, link +2, both ways | as above; the recovery holds at 6 |
| `sym_9_retry_interval::sym_9_the_drive_writes_the_ladders_receipt` | [1390, 1801, 0] → [1390, 1803, 0], shipped and bare | the pad's chart |
| `step-export` `export::committed_fixtures_are_byte_golden` | `swept_elbow.step`, 5 `DIRECTION` lines, an ulp or two of a 6.1e-17 component | the Rodrigues spelling: the former spelling on main's head writes every fixture back byte for byte |

These readings are the same at the default ε, 1e-6 and 1e-12.

## Not 7434's: red on its first parent with the same reading

- **`m10_p_fence::the_corpus_evaluation_is_bit_identical_at_interval`.**
  It reads `(9674677071270001226, 9561563393312185022)` on both. It is held
  by GREEN's
  `main-red-eleven-editor-core-and-sweep-rows-moved-on-main-after-c390e1f9`
  (SECTCONE's move and PR 7299's).
- **`m10_sym_drive_memo_interval::every_leaf_reports_one_column_under_every_schedule_and_both_dials`.**
  The receipt is `Receipt { certified: 1, refused: 7, splits: 7 }` on both,
  and on `787aed5` (the last nightly's sha). See the next section.

## The race row (item 6): not a loss, and not 7434's

The row was reported as "the drive now refuses where it certified". It
is the other way round. The `race` arm pins `certifies: false`, and its
8-leaf drive now certifies one leaf, `[-1.25e-8, 0]`. The receipt is the
same on 7434's first parent, on `787aed5`, with 7434's geom-core files
restored on main, and with the arm's former spelling. None of the three
candidates (the identity chart's conditioning, `turn::zero` at
`Sym<Interval>`, a moved certifying vector) is the cause. It has been red
since 2026-11-19 at least, at every ε (nightly 44870651337).

With `certifies: true` on `race`, the whole row passes on main. The
columns agree under every schedule and both memo dials. The 12-leaf
and 64-leaf drives certify two and eight leaves, where the `RACING`
doc says one and four. That is a gain. SCALAR's
`the-race-rows-leaves-need-two-amounts-where-they-needed-six` holds the
re-pin and now carries these measurements; it is not re-pinned here.
