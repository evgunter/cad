# BOXFINISH M4 — the cost row re-measured on the tree with the ray and the frames (2026-11-15)

Headline: with the frames built, the slab studies' forced bisection is nearly gone (docm9 3.5 s -> 0.5 s, driving_writes_nothing
5.4 s -> 0.44 s, all inside 1.0 s and passing), but the switch's box arm is still held by (1) the arc walls' and rim circles'
`u_ref` length (the unbuilt doors-read unit: the tour certifies 0), (2) a depth-boxed slab wider than +-0.215 still losing every
ray (winding/edge/advance), and (3) what a finish costs when nothing refuses: 55-100 ms on a planar slab, 0.85-2.9 s on a
cylinder-walled or revolved body.

## Trees (hand substitutions BOXFINISH_AIM / BOXFINISH_PLANE unset everywhere)
- ON-head: `origin/boxfinish/sweep-frames` 9ee3c1ea3f, unmodified.
- OFF-main: `boxfinish/probe-switch-off-main` b51b63682f = probe bba45062b9 + `origin/main` d00654ddde (pushed).
- OFF-head: `boxfinish/probe-switch-off` c9dd57b5ae = probe bba45062b9 + ON-head (pushed).
- Merge conflicts (same three in both merges, all in the probe's own edits): `solid_contain.rs` edge pass (kept the probe's
  `[bf-ray] edge` trace, took the incoming `Reach::of(...)` line); `quad_lane.rs::nurbs_face` (took the incoming
  `image.span_ends`, kept the `[bf-quad]` trace after it); `side_reader.rs::aimed_ray_inner` (took the built ray; the
  `BOXFINISH_AIM=dir|mid` direction experiment is gone with the code it replaced; a reach trace stays). Switch-off untouched.
- One more probe commit on each: `crates/sweep/tests/an_aimed_ray_keeps_its_line_over_a_box.rs` (PR 6958) names
  `topo::Replay<Boxed>` as the leaf and does not compile switch-off (E0308 x2); gated out in `crates/sweep/tests/all.rs`.
- Build: ci.yml's test-job env + `scripts/rustc-test-crates-opt0.sh`, one target dir per tree, 4 vCPU. `c.sh <tree> ...`,
  `rows.sh`, `srows.sh`, `census.py`, `attr.py`, `cmp.py` and all logs are in `~/.local/share/cad-work/boxfinish-m4/`.

## 1. The row's table (median of three runs of the test binary, `--exact`, quiet box)
| row | switch on (ON-head) | off, no frames (OFF-main) | off, frames (OFF-head) |
|---|---|---|---|
| m10_8 plain decisions | 460/504/0 (pin, passes) | 10974/12684/100 | 2424/3151/192 (5.3x / 6.3x) |
| m10_8 `..shipped_set_is_inert..` | 0.086 s PASS | 2.08 s FAIL (pin) | 0.83 s FAIL (pin) |
| m10_8 `..a0_set_is_a0_alone` | 0.87 s PASS | 6.76 s PASS | 7.46 s PASS |
| docm9 `a_certified_side_stops_at_the_seeds_edge` | 0.048 s PASS | 3.55 s PASS | 0.48 s PASS |
| docm9 `a_parameter_field_boxes_directly` | 0.054 s PASS | 3.39 s PASS | 0.54 s PASS |
| docm9 `the_query_does_not_touch_the_input_document` | 0.046 s PASS | 3.63 s PASS | 0.72 s PASS |
| m10_3 `driving_writes_nothing` | 0.196 s PASS | 5.41 s PASS | 0.44 s PASS |
| sweep `an_elbow_builds_from_every_spelling` (f64) | 20.1 s PASS | 18.9 s PASS | 22.0 s PASS |
| sweep `a_one_segment_strut_..negated_knots` (f64) | 15.9 s PASS | 15.7 s PASS | 17.7 s PASS |
| sweep `revolve_washer::*` (both rows) | 0.02 s PASS | 0.02 s PASS | 0.02 s PASS |
Replays (measured, trace): m10_8 plain per drive: OFF-main 15, all refusing at the finish (certified 0, refused 8); OFF-head 3
(root +-0.25 refuses at the finish, two halves certify). docm9 first row: OFF-main 31 replays (12 refusing finishes), OFF-head 3.
ON-head's 460/504 is one replay (inferred from the pin: it equals one leaf's build, 468/496 at the other rule set).
Establishes: the frames remove all but one bisection level on the +-0.25 slab. Does not establish ON-main (not built; OFF-main's
own pin text says main is 2926/2894/0).

## 2. What still refuses at a finish on OFF-head (BOXFINISH_TRACE=1, one test per process; `census.py`)
| study | rows pass/fail | Sym<Interval> finishes ok / refused | refusals by kind |
|---|---|---|---|
| m10_3_driver_interval | 19 / 3 | 462 (68 ms avg) / 97 (123 ms avg) | 62 URef length (A); 32 rays (B); 3 turning wall (C) |
| m10_derived_frame_interval | 2 / 1 | 18 / 0 | none (the cube finishes at 5e-2) |
| eval10_section_reads_the_nominal | 3 / 0 | 1 / 0 | none (the boxed loft finishes, 14 ms) |
| docm9_range | 21 / 0 | 326 (87 ms) / 7 (318 ms) | 6 rays (B); 1 turning wall (C) |
| m10_8_pins_interval | 1 / 1 | 12 / 2 | 2 rays (B) |
| tour `demo-tour certified` | exit 101, 83 s | 1023 / 2046 | 2046 URef length |
Red rows: m10_3 `a_band_parameter_certifies_normally_and_prices_nothing` and `the_two_parameter_drive_certifies_after_bisection..`
(Receipt certified 0, refused 16, splits 15; URef), `the_certification_width_is_no_longer_bounded_by_epsilon` (whole-certifying
half-width settles at 0.21516 of depth 1, where the row wants ~1; rays); m10_8 plain pin; derived-frame
`..refusal_is_not_a_freeze` (certifies, then `frozen == 0` fails with frozen 192).
Kinds, one payload each (all at `Sym<Interval>`, band (1e-9, 1e-8) unless said):
- **A. `u_ref` length, an end-derived direction (EXPECTED, counted apart; row `the-sweep-doors-read-the-profiles-recorded-directions`).**
  Tour, root box, both hole extrudes at every replay: `Extrude(NotFinished[SurfaceDatumUndecided{face 3v1, kind: Cylinder,
  datum: URef, measure: Length, end: Upper, Indeterminate{Enclosure{-2.971e-4, 2.412e-4}, predicate: None}}, same on 4v1,
  CurveDatumUndecided{edge 1v1, kind: Circle, datum: URef, measure: Length, ..same margin}, ..])`. m10_3 band-parameter doc at
  +-0.16: same shape, faces 7v1/8v1, `Enclosure{-1.4578, 0.3200}`. Raised by `topo::validate::surface_datum_errors` (check 1)
  over `Surface::representability_margins`. Identical to M1's tour payload: the frames unit does not touch it (measured).
- **B. Every aimed ray set aside on a depth-boxed slab, then the volume refuses (rows
  `an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice` for the rays, `check-7s-volume-reading-is-loose-over-a-box`
  for the fallback).** docm9/m10_8 root, depth [0.75, 1.25]: `Extrude(NotFinished[OrientationUndecided{reading: Volume{
  Indeterminate{Enclosure{-0.9699, 2.9699}, band (2.2e-308, 4.5e-308), "positive_volume_exact"}}, aimed: 6, set_aside: 6}])`.
  Stored normals are now exact (trace: every `normal(as built)` width 0; measured), reach width 0.35-0.50. Per ray: top cap
  `side_at_infinity_winding` `Enclosure{-3.827, 13.758}` and bottom cap `{-1.412, 8.154}`, raised by `turned_inside` /
  `crosses_by_turn` on face 1 (FAULT), after `side_at_infinity_edge` `{0, 0.177}`, `{0, 0.239}` and `side_at_infinity_advance`
  `{-1.199, 0.927}`; the four sides on `side_at_infinity_edge` `{0, 1.109..1.227}` of face 1 (`solid_contain`'s edge pass).
  The halves (+-0.125) read by rays, 75-80 ms per leaf; the root's refusing finish is 215 of its 266 ms.
- **C. A wall that turns over the box (denom), 4 finishes, e.g. `a_planted_flip_refuses_flip_crossing..` (passes).** 7-face
  body at +-0.3: rays set aside on `side_at_infinity_denom` of face 3 `{-0.0821, 0.1079}`, face 4 `{-0.0790, 0.1038}`; volume
  `{-3.350, 7.503}`. Face 3's stored normal is `([-0.984, -0.336], [-0.894, -0.673], 0)`: a line wall's normal from a moving
  vertex (measured, BOXFINISH_TRACE=2). End-derived like A, but here the wall really turns (inferred: recorded directions would
  still have width); no row names it apart from the doors-read row.
Not finish refusals (same on every tree's build, not counted): `NegativeDepth`, `DegenerateExtrusion`,
`ExtrusionEscalated/extrusion_normal_component`, `ProfileLaneReplay`, `Mate:SideUnevaluated/pose_unit_norm`.

## 3. What is left once nothing refuses (OFF-head)
- **A planar slab leaf** (`m10_3 a_macroscopic_box_certifies_at_the_shipped_depth_budget`, +-0.05, now PASSES in one leaf;
  OFF-main: root refuses, 2 leaves). Leaf 816 sym / 971 num, 89 ms; the finish is 55 ms and 348/475 of the decisions, i.e.
  1.77x / 1.93x ON-head's one-leaf 460/504. (a) Of the finish's 823 decisions, 732 (89%) carry a name and count the door's
  build decided in the same leaf: `dihedral_arm` 168(=168), `dihedral_wedge` 168(=168), `carrier_on_surface_1`/`_2` 108(=108)
  each, `dihedral_sign` 84(=84), `ssi_hull_sup` 24(=24). Finish-only: `pcurve_*_chart_incident` 48, `side_at_infinity_*` 36,
  `bool_ring_run_winding` 6, `session_box_reach` 1. Equal margins still inferred. It also freezes 96 forms (frozen 0 on ON-head's
  pin; which predicates, not traced).
- `driving_writes_nothing`: 127 replays, no finish refuses, 2 finishes (104 ms avg): 0.196 s -> 0.44 s is those two finishes.
- **m10_8** `a0_set_is_a0_alone` (three-axis plate with holes, +-5e-11, nothing refuses, 1 leaf per drive): 0.87 s -> 7.46 s.
  4 finishes per leaf: the plate 83-95 ms, three 4-face hole bodies 844-1073 ms each (2.7 of 3.4-3.7 s per leaf). (a) finish
  5018 decisions against the build's 4052; 61% repeat build names. (b) 8 box finishes over 4 fingerprints: none repeated in a
  drive. Same on OFF-main (6.76 s) and in M1 (6.77 s): not the frames'. Why a cylinder-walled finish costs 10x a slab's per
  decision is not decomposed (no profiler run); under the shipped set an ok hole finish was not observed (the tour's refuse).
- **A revolved circle** (`sweep a_sweep_keeps_the_frame_it_is_handed::a_revolved_circles_latitude_over_a_centre_box_is_exact`):
  one finish of a 3-face body, 2932 ms; row 0.29 s -> 3.13 s, passes.
- **Tour** (b), re-taken: 1023 replays, 3069 box finishes over 255 distinct vertex fingerprints: the plate is one body
  finished 1023 times (39 ms avg, all ok), the holes 2046 finishes over 254 (36 ms, all refused, kind A): 92% repeat. Finishes
  are 114 s of 330 s summed leaf time (35%). (a) 85% of finish decisions repeat build names (`dihedral_arm` 286440(=286440)).
  ON-head: 163 s, 211 certified / 301 refused, 94.9069% certified; OFF-head: 83 s, 0 certified (so the times do not compare).

## 4. The row's closing test: `nextest run --profile ci --no-fail-fast -p editor-core -p sweep -p topo -p mesh`, default eps
ON-head: 10299 run, 10299 pass (114.6 s). OFF-head: 10295 run, 10288 pass, 7 fail (141.7 s). Times are nextest's under -j4.
Past 1.0 s on OFF-head and not on ON-head (7; all pass):
`sweep a_sweep_keeps_the_frame_it_is_handed::a_revolved_circles_latitude_over_a_centre_box_is_exact` 3.40 (0.31);
`editor-core m10_5_clearance_interval::a_starved_cell_depth_refuses_on_its_own_axis` 1.37 (0.35) and
`::a_starved_cell_budget_refuses_typed` 1.32 (0.36); `coincidence_door::two_copies_meeting_by_the_same_face_are_not_one_construction`
1.13 (0.45); `intent_s5_c_at_rest_check::a_contact_is_quiet_under_its_gap_and_nothing_else` 1.07 (0.42);
`m10_5_r2_probes_interval::the_leaf_fold_answers_the_same_question_over_a_real_drive` 1.07 (0.37) and
`::the_combs_internal_gaps_answer_at_three_bounds` 1.01 (0.41). Crate sums on -> off: editor-core 134 -> 178 s, sweep
227 -> 275 s, topo 80 -> 95 s, mesh 14.3 -> 16.1 s (the last two inferred to be load: their code does not finish box leaves).
Fail on OFF-head and not on ON-head (first payload):
1. `editor-core m10_8_pins_interval::m10_8_the_shipped_set_is_inert_on_straight_geometry`: 2424/3151/192 vs 460/504/0.
2. `m10_derived_frame_interval::m10_the_derived_frames_refusal_is_not_a_freeze`: `frozen` 192 vs 0.
3. `m10_sym_profile_interval::the_forms_the_slabs_walks_build_are_pinned_per_eps_row`: largest form 17 terms, pinned 4.
4. `a_turned_discs_union_names_its_unfinished_member::..carried_through_the_turn`: expects `UnfinishedLeaf`, gets
   `NotFinished[SurfaceDatumUndecided{face 3v1, Cylinder, datum: Axis, measure: Length, Enclosure{-7.4996e-5, 7.6140e-5}}, ..]`
   (the switch's own arm; the deletion row removes `UnfinishedLeaf`).
5. `sweep a_sweep_keeps_the_frame_it_is_handed::a_revolves_latitude_identity_is_symbolic_without_a_registration`: the full
   washer `(true, 729, 1229)` vs pinned `(true, 501, 461)` (a decision pin).
6. `sweep a_wall_row_read_in_band_takes_the_lane_a_miss_takes::a_seam_in_the_band_takes_its_general_image_at_interval`:
   "the prism builds": `NotFinished[VolumeUncomputable{face 4v1, QuadratureUnsupported{"a NURBS-face pcurve endpoint is not
   exact structure .."}}]`, raised in `topo::props::quad_lane::nurbs_face`. A semicircle wall, so an arc rim (inferred:
   `an-iso-arc-rims-end-reads-through-its-map-at-interval`, parked; not checked against the row's fixture).
7. `mesh sizing::tests::each_operation_is_bitwise_the_bare_spelling_it_replaced` ("pad at band NaN, x NaN", NaN bits differ):
   passed 3/3 re-run alone on OFF-head; a flake under load, not the switch (inferred).
Does not establish the slow set: the row's own slow rows are item 1's.

## 5. The far-placement washer: it is the switch
Row: `crates/sweep/tests/sym11_far_placement_rows.rs`, `sym11_the_far_placement_never_contradicts_at_sym_interval` (and
`.._is_a_counted_dispute_at_sym_f64`), `washer_revolve` at `(d, d, d)`. Slow set, so not in item 4.
- ON-head: all 3 rows pass. OFF-main: 2 red. OFF-head: 2 red. So neither the frames unit nor main (measured on these three).
- OFF-main, `Sym<Interval>`, d = 1e6: stadium `MappedSource` (as its table), then the washer: `revolve: NotFinished{[
  OutsideSessionBox{margin: Enclosure{-999002.0, -999002.0}}]}` where TABLE says `built`: the reviewer's sighting.
- OFF-head: the stadium is asserted first and is the one that reads red: `extrude: NotFinished{[OutsideSessionBox{margin:
  Enclosure{-999001.2500000003, -999001.2499999997}}]}` (`Sym<f64>`: `Value(-999001.25)`); the washer cell is not reached.
- Raising site: `topo::session_box::in_session_box` (`session_box_reach`), inside the door's finish. The file's own header says
  why its table reads `built`: "The `Sym` lanes build a replay leaf .. whose finished-body gate, the box with it, runs where
  the leaf is taken, which no row here does." With the leaf finished at the door, D4's box answers first. `Sym<f64>` is red
  too because the probe turns the arm off for every `Sym<T>`.

## What stands between this tree and deleting the switch's box arm
1. Kind A, `u_ref` length on arc walls and rim circles: the tour's stop (0 certified against 94.9%), 2 red m10_3 rows, 2108
   refusing finishes. The doors-read unit, unbuilt.
2. Kind B, rays on a slab boxed wider than +-0.215 of its depth, with the loose volume behind them: 40 refusing finishes, one
   extra bisection level, 2 red rows (the m10_8 pin, the certification-width row). Two P2 rows hold it.
3. Kind C, a turning line wall: 4 finishes, no red row.
4. The Interval quadrature on an arc-rimmed wall: 1 red sweep row.
5. Pins that move with any finish at the door: m10_8 decisions, `frozen == 0` (96 forms per slab finish), largest form 17 vs
   4, the washer's (729, 1229), the far-placement TABLE (session box at the door), the `UnfinishedLeaf` fixture.
6. Cost with nothing refusing: +55-100 ms and x1.8-1.9 decisions per planar slab leaf (89% repeating the build's names);
   0.85-1.07 s per cylinder-walled finish at the a0 set (a0 row 7.5 s); 2.9 s per revolved-circle finish; 7 fast rows cross
   1.0 s; the tour finishes one plate 1023 times.

## Corrections to the rows' premises
1. Cost row: `revolve_washer::m10_9_…` is not a row on any of the three trees (`revolve_washer.rs` has two tests, 0.02 s).
   The elbow and strut rows are f64 and cost the same switch on or off (20 s, 16 s), as its second list already says.
2. Cost row: "with the two substitutions every row above is inside the 1.0 s budget" holds as built for docm9 and
   `driving_writes_nothing`; the m10_8 pin is 5.3x / 6.3x, not 1.49x / 1.68x, because the built ray reads to +-0.215 and the
   hand substitution's constant ray read the +-0.25 root.
3. M1 called `frozen != 0` on the derived-frame freeze row an artefact of the hand constants; it is there as built (192).
4. Cost row / M1: "one finish per finishing leaf" holds; the a0 plate runs 4 (one per sweep door), none repeated.
5. Deletion row: a second sweep file names the Replay leaf, `an_aimed_ray_keeps_its_line_over_a_box.rs`.
6. Extrude-mint row (in passing): on OFF-head the +-0.05 slab and the derived-frame cube at 5e-2 finish; its closing rows
   `a_macroscopic_box_certifies..` and the twin row pass switch-off.
7. Tour, switch on: 163 s here on ON-head (M1 took 114 s as given).

## Off-fence, noticed, not filed
- `mesh sizing::tests::each_operation_is_bitwise_the_bare_spelling_it_replaced` failed once under -j4 on a NaN's bits.
- Cargo's incremental caches for three trees filled the session's disk allowance (about 20 G) mid-run; `CARGO_INCREMENTAL=0`
  from then on (the ON-head four-crate build and everything after it).

## Not measured
ON-main; the slow set beyond item 1's rows; where a cylinder-walled or revolved finish spends its time; which forms a finish
freezes; whether the repeated rows' margins are bit-equal; the tour with kind A substituted by hand; M3's ignored probe rows.
