---
id: main-red-the-identity-turns-decisions-moved-the-m10-slow-rows
kind: issue
title: "Main red: the identity turn's decisions (PR 7434) moved nine editor-core slow rows and step-export's swept_elbow fixture"
status: closed
opened: 2026-11-23
priority: P0
cost: M
closed: 2026-11-23
pr: 7549
branch: doctail/the-identity-turns-decisions-repinned-on-the-m10-rows
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

**After PR 7501** (CIW's `census-digest-goldens`). It merged while this PR was open. It narrowed `m10_9_pins_interval`, `m10_sym_profile_interval`, `sym11_exact_channel_rows` and `sym_9_retry_interval` to what an intended change does not move: their receipts now pin the symbolic and frozen columns, not the numeric one. 7434's moves are numeric only, so the re-pins of those four files are superseded, and main's versions are taken as they stand. They pass on the merged tree (`d2c0cf3356`) at the default ε, 1e-6 and 1e-12, with the bulge rows and the step-export fixture row: 17 of 17 at each ε. What this PR still changes is the bulge tables and `swept_elbow.step`.

**After PR 7264.** That PR merged while this one was open. It re-pinned
the same rows for validation no longer reading an adjacent line pair's
shared vertex, and it measured them on a base without 7434's move. Its
moves are symbolic and frozen; 7434's are numeric. The two compose
without interaction: the pins here are 7264's symbolic and frozen
columns with the numeric moves above. The plate ledger moves by the
same lines as before 7264: `*/Assertion` and `Door/Decision` +4 calls
and +146 forms, `Door/Decision`'s digest identical, and `Early/Decision`
unmoved. Measured on the merged tree at all three ε.

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

## The rest of main's slow-set reds: not 7434's

Each of these is red on `fa42c60` with the same reading. The exceptions
are the varying-seed fuzz rows, which are red there under other seeds.
None is touched here. The row that holds each is named where one is open.

- **Red at the default ε:**
  - `seat4_verb_lowering`, `seat7_sweep_lowering`, `seat8_split_lowering`:
    GREEN's `main-red-eleven-…-after-c390e1f9`.
  - `split_pinch_keeps::the_kite_moats_subtract_…`: GREEN's
    `main-red-rows-two-lanes-found-red-on-their-merge-base-…`.
  - The source censuses: geom-core's `bounds_census`,
    `certified_endpoint_census` and `sym_rule_f_rows`, test-utils'
    `reader_census`, and topo's `face_normal::…no_source_file_folds_…`.
    Held by CIW's
    `tree-wide-source-censuses-in-the-slow-set-run-on-no-pr-that-moves-a-site`,
    GREEN's `reader-ledger-is-keyed-on-the-file-not-the-census` and ONEDOOR's
    fold-census row.
  - The recourse rosters (geom-brep and topo): GREEN's two
    `main-red-…-recourse-roster-…` rows.
  - step-import's nist09s snap, `wild` and `tier_gate`: GERM's
    `main-red-axis-normal-snap-refuses-nist-ftc-09`.
  - viewer's `review_pick_r2`: GREEN's
    `main-red-the-viewers-pick-tally-reads-six-grazes-short-of-its-pin`.
  - sweep's `bisector_nodes`, `section_end_poses` and `germ_tilted_rod`:
    GREEN's rows.
  - **test-utils' `predicate_name_coverage::every_deciding_crate_runs_a_roster_…`:**
    `ct_critical_gap_straddle` (`pair_section/gap.rs`) and `pc_parallel_gap`
    (`locus.rs`, `offer_rows.rs`) are "a predicate literal no crate
    decides". No open row held it, so it is filed here as GREEN's
    `main-red-two-slow-rows-no-row-holds-at-da61db71a`.
- **Red at 1e-6 only:**
  - geom-brep's `ellipse_torus` and `circle_lane_rows` fuzz rows:
    SPLICE's and PCERT's seed rows.
  - `pair_section::one_loop_keeps_its_order_near_the_node`.
  - sweep's `verbs_cylcyl_r1_review_probes` and `verbs_germarms2` ×2:
    SPIRICARM's 1e-6 row.
  - topo's `square_search`: GERM's square-wall row.
  - **sweep's `a_plane_through_a_cones_apex` 30° and 45°:** no open row
    named them; they are filed on the same GREEN row.

- **Red at 1e-12 only:** 18 rows, all red on `fa42c60` at 1e-12. They are
  sweep's mixed-chain fillets, `parallel_cylinder_join`, the
  `section_ellipse_seam_poses`, `torus_touch_off_faces`, `a_ring_on_a_cone_face`,
  `spheres_crossing_off_every_edge`, `curved_notch_pinch` and
  `verbs_germarms2_interval`; topo's `a_seamless_pipe_cut_by_an_oblique_plane_measures`
  ×2; and editor-core's `union_fold_face_face_touch`. Each is named on an
  open row: GREEN's `main-red-seven-slow-and-1e-12-rows-…` and
  `…found-red-on-their-merge-base…`, ONEHOME's
  `pr-7131s-frame-cost-at-1e-12-…` and `main-red-a-rod-tipped-…`, BAND's,
  ORBIT's and REFS' rows.
- **Red after `da61db71a`:** topo's
  `face_normal::tests::the_planar_sense_flip_lives_in_one_place`, a
  `ring_path.rs` site. It is filed on the same GREEN row.

## The CI gap

7434's gate ran none of these rows. They sit in the `ci` profile's slow
set, and a slow row runs on a PR only when the diff spells its name or
edits its file (`scripts/slow_set.py`'s `selected`). Ev ruled out running
a seeded crate's whole slow set per PR (PR 6731). CIW's
`dependents-slow-sets-skip-a-kernel-pr` holds that ruling. It closed with
the nightly attributing each newly red test to its merge, so the next red
of this shape is bisected by the nightly rather than by hand, as these
were. The per-PR canary that would have caught it before merge is CIW's
open `a-stored-data-change-runs-none-of-the-slow-rows-that-read-it`.
