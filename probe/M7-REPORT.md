# BOXFINISH M7 — the after-the-fact walks skipped by hand (S1), and symbolic Zeros against their enclosures (S2) (2026-11-22)

Headline: with nothing asked of the tier after a definite numeric sign, a finish that refuses nothing costs 33 ms (slab, was 52),
71 ms (cylinder-walled, was 796) and 1.0 s (revolved circle, was 2.66 s); every M4 row but two is inside 1.0 s; no verdict
changes in 13,636 rows (the rows that go red pin the skipped walk's own counters); and over about 9.97 million symbolic Zeros
at `Sym<Interval>` none has an enclosure excluding 0, `free_identity` margins included.

## Tree and probe
- `boxfinish/probe-switch-off` at the commit holding this file; base 88d7076bab = the M5 probe 2f37154e56 + `origin/main`
  ae076e0507 (4,137 commits). The merge was cheap: three textual conflicts, all in the probe's traces
  (`quad_lane.rs::nurbs_face` and `side_reader.rs`: took main's, the `[bf-quad]` and `[bf-side]` traces are gone;
  `validate.rs`: kept the probe's finish trace around main's `k_stats::checking(..)`). One compile repair: `MarginTerms`
  gained a `Debug` supertrait for the M5 margin text. Switch still off (`AtRestPolicy for Interval` / `Sym<T>`: `Leaf =
  AtRestBody`, `TAKE_FINISHES = false`); the two gated sweep files stay gated. Every test target of geom-core, profile, topo,
  sweep, editor-core and the tour compiled with no other change.
- Build as M4 (`c.sh`: ci.yml's env, opt-0 wrapper, `CARGO_INCREMENTAL=0`, own target dir). S1 on and off are ONE binary.
- **S1** (`BOXFINISH_S1=1`), in `Sym::decide_sign` (`crates/geom-core/src/sym.rs`), the `definitely_nonzero` arm. Skipped:
  (a) `door_zero(self.node)` and its `count_registration_contradicted()`; (b) the `match T::WITNESS` block: at `Exact` the
  `debug_assert!(!contradicts(..))`, at `Inexact` `contradicts(..)` and `count_theorem_disputed()`. `contradicts` is
  `discharge` (the whole ladder on `Path::Contradiction`) plus `count_check`, so the `checks_expanded` / `checks_frozen`
  columns go to 0 with it. Kept: `count_decision(false)`, `report::record(..)`, the return of the numeric outcome.
- **What else runs at a definite sign** (read from the code; measured where it shows): before the arm, `note_witness`,
  `T::sign_within`, `deferred::defer` (a session flag; acts only on a numeric Zero); after it, `count_decision` and
  `report::record` (returns at once unless the shape report is installed). Under `sym-revalue-testing` only, `revalue::note`
  / `settle`. Outside `Sym`, `k_stats::classify` records the verdict. With S1 on, the time inside `sign_within` for a definite
  decision reads 0.0 ms in every traced row (measured), so nothing else there has a cost that shows.
- **DOCTAIL's check role** (`k_stats::checking`, `Role::Check`): the symbolic tier reads the role in one place,
  `sym/revalue.rs` (feature `sym-revalue-testing`); `decide_sign` does not branch on it. Nothing new runs after a definite
  sign because of it (read from the code).
- **S2** (`BOXFINISH_S2_FILE=<path>`): where `decide_sign` returns the tier's `Zero` (`discharge_retried(..).is_some()`, the
  one site every rung, retry and the registry door reach on the decision path), it tallies per thread (scalar, rung, attempt,
  whether the node's DAG reaches a `SymOp::Free`, whether `enclosure_probe()` contains 0, whether the numeric channel had read
  Zero or was indeterminate, predicate name) and appends the tally when the thread ends; a zero-excluding case also writes a
  line with the enclosure and band. Not covered: `deferred::discharged` (a deferred session's later discharge, which has no
  value to hand; it only ever follows a numeric Zero), and a thread whose destructors do not run.
- `BOXFINISH_TRACE=3` now also prints `[bf-slow]` for a decision of 20 ms or more: forms built, largest form (numerator +
  denominator terms), time in nodes that made a form against nodes refused, `Refusal::Ring` count and time to the first one,
  ladders asked on the decision path and on the contradiction path.

## 1. M4's table, S1 off against S1 on (median of 3, `--exact`, quiet box)
| row | S1 off | S1 on | inside 1.0 s |
|---|---|---|---|
| docm9 `a_certified_side_stops_at_the_seeds_edge` | 0.423 s | 0.296 s | yes (was) |
| docm9 `a_parameter_field_boxes_directly` | 0.439 | 0.232 | yes (was) |
| docm9 `the_query_does_not_touch_the_input_document` | 0.518 | 0.223 | yes (was) |
| m10_3 `driving_writes_nothing` | 0.457 | 0.260 | yes (was) |
| m10_8 `m10_8_the_a0_set_is_a0_alone` | 5.344 | 1.605 | NO |
| m10_8 plain decisions (pin 460/504/0; red both ways) | 2424/3151/192, 0.759 s | 2424/3151/24, 0.449 s | yes |
| m10_5_clearance `a_starved_cell_depth_refuses_on_its_own_axis` | 1.175 | 0.336 | now |
| m10_5_clearance `a_starved_cell_budget_refuses_typed` | 1.202 | 0.360 | now |
| coincidence_door `two_copies_meeting_by_the_same_face..` | 0.591 | 0.613 | yes (was, on this tree) |
| intent_s5_c `a_contact_is_quiet_under_its_gap_and_nothing_else` | 0.708 | 0.768 | yes (was, on this tree) |
| m10_5_r2 `the_leaf_fold_answers_the_same_question_over_a_real_drive` | 0.922 | 0.457 | yes |
| m10_5_r2 `the_combs_internal_gaps_answer_at_three_bounds` | 1.023 | 0.402 | now |
| sweep `a_revolved_circles_latitude_over_a_centre_box_is_exact` | 3.403 | 1.332 | NO |
All pass their assertions both ways except the m10_8 pin. The symbolic and numeric counts do not move with S1; `frozen` does.
The m10_8 slab still bisects once at +-0.25 (M4's kind B), which is why its `frozen` reads 24 and not 0 there.

## 2. A finish that refuses nothing (BOXFINISH_TRACE=3; ms inside `sign_within` unless said)
| finish at Sym<Interval> | S1 off | S1 on | what S1 removes | what is left |
|---|---|---|---|---|
| six-face slab, 824 decisions | 52 (32 inside) | 33 (5 inside) | rays 20.2 -> 0.0, session box 4.9 -> 0, dihedral 2.6 -> 0.2 | 27 ms building margins (pcurve/chart 11.8, rays 7.1, carrier 4.0) |
| cylinder-walled hole body, 619 | 796 (736) | 71 (16) | rays 634 -> 0.0, `bool_*` 48 -> 0, `curve_carrier_clear` 20.5 -> 0 | 55 ms building margins (`ssi_hull_sup` 27), `props_*` 9.7, pcurve 6.5 |
| revolved circle, 1429 | 2659 (2599) | 1011 (953) | rays 304, `azimuth_near_cut_side` 271, `period_near_mark` 199, `line_quadric_torus_split*` 240, `bool_torus_region_at` -> 0 | `curve_pair_span_end_chord` 454, `bool_torus_region_component` 310, `bool_torus_region_corner` 116 |
- **The revolved circle's remainder is on the DECISION path** (measured): each slow one reports `decision_ladders=1
  contradiction_ladders=0` and `outcome=Indeterminate`: the numeric margin is indeterminate, the tier is asked, and it does
  not prove. `curve_pair_span_end_chord`: 100 decisions, 454 ms with S1 on (600 off): so about a quarter of M5's 559 ms was
  the check and three quarters is the decision's.
- **Above about 50 ms per decision, S1 on:**
  - `curve_pair_span_end_chord` (8 decisions of 46 to 67 ms): 150 to 390 forms per decision, largest form 1,970 terms, no
    `Refusal::Ring` at all, 41 to 62 ms in nodes that made a form against 4.5 ms in the 15 or 16 refused (budget).
  - `bool_torus_region_component` (131, 55, 38, 37 ms): 280 to 1,744 forms, largest 988 to 1,890 terms; the 131 ms one has 32
    `Ring` freezes, the first at 14 ms, 121 ms in made forms against 7.9 ms refused.
  - `bool_torus_region_corner` (54, 62 ms): 111 forms, largest 2,089 terms, no `Ring`, 44 to 52 ms made, 10 ms refused.
  So the time goes in building large forms BELOW the frozen nodes (forms of one to two thousand terms that are kept), not in
  products that are then refused, and not before a `Ring` freeze (measured).
- S1 off, for reference (hole body): `side_at_infinity_denom` 211 ms = one contradiction ladder, 11,195 forms, largest 159
  terms, 38 `Ring`; `side_at_infinity_winding` 98 ms = one contradiction ladder, 2,071 forms, largest 805 terms, 90 `Ring`.

## 3. Receipts
- **Slab finish** (`a_macroscopic_box_certifies..`): 816 / 971 / frozen 96 -> 816 / 971 / **0**. Yes: all 96 were the
  contradiction walk's on `side_at_infinity_winding`, and no `[bf-froze-why]` line remains.
- **a0 plate drive**: 1819 / 2154 / 544 -> 1819 / 2154 / 36 (left: `carrier_pose_axes_parallel` 48, `pcurve_envelope` 24).
- **Tour** drive: frozen 146 -> 12; symbolic 1,782,066 and numeric 2,020,425 unchanged.
- **Pinned rows that move with S1** (full suite, geom-core + profile + topo + sweep + editor-core, default eps, switch off):
  - red only with S1 (11), each pinning a counter of the skipped walk: `geom-core sym::profile::tests::
    the_walks_and_the_ring_are_counted_by_origin` ("no entry found for key": no `Assertion` origin);
    `m10_9_r1_sym_probes::r1_two_registrations_make_the_two_channels_contradict_and_it_is_counted`
    (`registrations_contradicted` 1 -> 0); `sym11_witness_kind_rows::` eight rows: `..a_check_the_early_walk_unfreezes_is_
    counted_expanded` (1,0) -> (0,0), `..a_contradicted_registration_is_not_a_disputed_theorem` (1,0,0,1) -> (0,0,0,1),
    `..a_frozen_check_is_counted_frozen_and_disputes_nothing` (1,0,1) -> (0,0,0), `..a_poisoned_check_reads_frozen` (0,1) ->
    (0,0), `..a_zero_that_rests_on_no_registration_is_not_a_contradicted_one` (0,1) -> (0,0), `..an_opaque_value_and_a_hull_
    read_expanded` (1,0) -> (0,0), `..the_adversary_is_a_counted_dispute_at_sym_f64` 1 -> 0, `..values_built_outside_the_
    session_read_frozen` (0,1,0) -> (0,0,0); `sweep mass_props_are_thread_count_invariant::bodies_built_before_the_session_
    trip_the_guard_while_disp_still_reads_zero` (doors blinded: `validate_geometric` -> also `mass_properties`).
  - green only with S1 (1): `m10_derived_frame_interval::m10_the_derived_frames_refusal_is_not_a_freeze` (frozen 192 -> 0).
  - red both ways, payload moved: `m10_8` plain frozen 192 -> 24; `m10_9_pins::m10_9_no_registrant_lies_on_any_measured_
    document` (two_hole_plate `checks_expanded` 2067 -> 0, `checks_frozen` 87 -> 0, `frozen` 544 -> 36; r1_annulus 1098 -> 0,
    82 -> 0, 474 -> 36); `m10_9_pins::m10_9_the_pad_at_both_rule_f_dials` (2801, 3220, 4290) -> (2801, 3220, 1769);
    `m10_sym_profile::the_forms_the_plates_walks_build_are_pinned` largest form 805 -> 46 (pin 38);
    `..the_forms_the_slabs_walks_build_are_pinned_per_eps_row` from "largest form 17 terms, pinned 4" to "the walks built
    different forms": the ledger loses its two `Assertion` lines (`Plain/Assertion calls 992`, `Early/Assertion calls 992`); `sym11_exact_channel_rows::sym11_the_exact_channel_never_
    contradicts_past_the_ceiling` two_hole_plate [1742, 1975, 146] -> [1742, 1975, 12]; the two thread-count digests
    (`mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_one_thread` / `_at_four_threads`) change
    from a golden mismatch to "no contradiction check ran, so disp=0 speaks for nothing".
  - `revalue_corpus` (15 rows): pass both ways. The other 34 rows red both ways carry the same payload (switch-off reds).
- **No verdict changed**: no row's certified / refused outcome, refusal kind or decision count moved (measured over the two
  full runs: 13,588 pass / 48 fail off, 13,578 / 58 on; the differences are exactly the lists above; `topo
  boolean::section_cert::square_search` is a random-seed fuzz row, red both ways with different seeds).
- Suite cost, summed per-test time off -> on: editor-core 8,025 -> 3,143 s; sweep 4,181 -> 4,169; topo 1,424 -> 1,344
  (runs at -j4 and -j3, so indicative). Largest: `m10_3_r2_probes_interval::a_consumer_drives_a_two_parameter_document_at_
  four_widths` 4,261 -> 1,013 s; `m10_sym_drive_memo_interval::the_plain_memo_moves_no_decision` 216 -> 59 s.

## 4. The tour (`demo-tour certified`, BOXFINISH_TRACE=1, switch off; still 0 certified, kind A, on this main)
| | S1 off | S1 on |
|---|---|---|
| wall | 95 s | 56 s |
| summed leaf time (1023 replays) | 375.8 s | 214.9 s |
| box finishes' share | 130.6 s, 34.7% | 72.1 s, 33.6% |
| the plate's 1023 finishes (one value) | 49.6 s (48.4 ms each) | 22.1 s (21.6 ms each) |
| the holes' 2046 refusing finishes | 81.0 s (39.6 ms) | 50.0 s (24.5 ms) |
So the share does not move: a body the leaf does not change is still 22 s of a 215 s drive (10%) finished again per leaf, and
the build outside the finishes sped up by the same factor. Does not establish the certifying tour (it refuses at check 1 here).

## 5. S2: symbolic Zeros against their enclosures
Full suite as in item 3 (slow set included, default eps), tallied on the S1-off run and again on the S1-on run:
- 9,965,060 Zeros at `Sym<Interval>` (S1 on: 9,966,708). By rung: Plain 8,252,258; Early 1,709,475 (2 on a second retry);
  Door 3,327 (7 on a first retry). 7.20 million followed a numeric Zero, 2.77 million a numeric indeterminate.
- **Enclosure excluding 0: none**, on either run, at any rung. No example exists to trace.
- **`free_identity` margins** (the node's DAG reaches a `Free`): 441,801, every one under `ssi_hull_sup`, every one with an
  enclosure containing 0 (Plain 401,795; Early 39,938; Door 68). So no executed row produces the `[1e-12, 3e-12]` case.
- `Sym<f64>`: 20,863 Zeros (no enclosure to read).
Establishes: the proposed assertion would not have fired in these five crates' rows at default eps on this tree, switch off.
Does not establish other eps rows, the tour and wild demos, pncad-py, or a deferred session's discharges.

## Not measured, and why
- `sym11` and `revalue_corpus` at 1e-6 and 1e-12, and S2 at those rows: the two full runs took 76 and 49 minutes.
- Item 1 on a switch-ON tree, and ON-head's figures on this main: only one tree was built.
- Why the two S2 runs differ by 1,648 Zeros (inferred: the random-seed fuzz rows).
- Which ring guard refuses; form degree (only term counts were captured).
