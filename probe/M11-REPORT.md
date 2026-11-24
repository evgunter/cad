# BOXFINISH M11 — the hole finish's undecided decisions, the rectangle's memory, the tour's exit (2026-11-24)

## Corrections to the rows' premises, first (all measured)
1. **The hole finish's cost is not the eight `curve_face_pose_disc` decisions. M10's item 1 was a sample of the drive's first
   100 s (its widest leaves) and I reported it as the finish.** Over the whole tour drive (2,128 hole finishes, 523 ms each,
   `BOXFINISH_TRACE=3`): `curve_carrier_clear` 314 ms (60%), `side_at_infinity_winding` 77 ms (15%), `curve_face_pose_disc`
   47 ms (9%), `side_at_infinity_denom` 26 ms (margin building), `curve_carrier_gap` 17 ms, `azimuth_near_cut_side` 10 ms.
   The pose-disc is undecided in 206 of the 1,023 replays only (the wide ones); the other three are undecided throughout.
2. **The a0 plate's hole finish has no such decision undecided**: its 8 pose-disc decisions read `[4.8927e-4, 4.8927e-4]`,
   Positive, 0.00 ms, no ladder asked. The row's a0 half has nothing to explain.
3. **The rectangle's memory is not the drive.** The three drives before it finish (1 leaf; 3 leaves, 2 certified after one
   split; 1 leaf). What grows is one call of the test's `replay` helper (`m10_8_arc_family_interval::replay`), which installs
   the shape report (`geom_core::sym::start_shape_report`): on the first undecided ray decision of the extrude's finish, the
   report renders the blocking form as text and that rendering does not end within 5 GB.
4. **M11-c: the tour's exit at `chaintol.rs:297` is the box arm's.** Same tree, switch on: the chain's 1-link `Interval` row
   reads "CERTIFIES 0.02 s", the command runs to its end (109 s). Switch off: "refuses 0.05 s, node 111e1e667832 ... an edge's
   stored curve does not certify against its faces".

## M11-a. The tour's hole finish, by undecided predicate (whole first drive, 1,023 replays; all measured unless said)
| predicate (home) | decided / undecided | undecided reading (enclosure, decoration) | witness at f64 | ladder time, whole drive | per slow climb |
|---|---|---|---|---|---|
| `curve_carrier_clear` (`geom::curves::on_carrier`, `Foot::on_carrier`) | 143,058 Positive / 26,948 | `[0, +inf]` `Trv` (13,704), `Def` (10,788), `Com` (2,456) | 80 readings, 0.32 to 8.56 mm, all Positive | 682.6 s | 3,117 climbs, 211 ms, 17,050 forms, largest 173 terms |
| `side_at_infinity_winding` (`solid_contain`, `turned_inside`) | 17,208 definite (9,836 +, 7,372 −) / 11,436 | `[-0.0083, 0.0303]` `Trv` (8,405) | ±0.024227, never 0 | 167.2 s | 2,743 climbs, 57 ms, 5,915 forms, largest 156 |
| `curve_face_pose_disc` (`geom_brep::curve_face::pose::discriminant`) | 15,196 Positive / 1,760 | `[-0.0013783, 0.0013783]` `Def` | 0.000489268 ×8 | 100.0 s | 880 climbs, 113 ms, 10,796 forms, largest 173 |
| `curve_carrier_gap` (same door as the first) | 8,344 Positive / 9,710 | `[0, 0.001672]` `Com` | (none taken) | 33.6 s | 620 climbs, 26 ms, 1,583 forms, largest 105 |
Together 983 s of the drive's 1,306 s of leaf time. No decision of the four is ever decided Zero at the box scalar.
1. **Why the numeric channel does not decide** (pose-disc, the root replay's inputs): the band is (1e-9, 1e-8) and the true
   value is 0.49 mm, so it is not in band. `a = a2 = 0.0301537` exact; `radius = [0.00122, 0.00128]`, the box's own 6e-5;
   but `b = [-1.69e-4, 2.28e-4]` (the a0 plate reads ±2.1e-10), `c = [-1.638e-6, 1.614e-6]` (a0: −1.2232e-6),
   `shift = [-3.13e-3, 2.49e-3]`, 5.6 mm wide over a box 0.1 mm wide, decoration `Def`; so `disc = [-4.87e-8, 1.01e-7]`
   straddles 0. The reading is wide because the ray's offset from the wall's axis is, about 56 times the box (measured);
   that this is the box entering the offset more than once, through a quotient, is inferred from the width and the `Def`
   decoration, and is the shape of `an-aimed-rays-other-readings-still-carry-a-moving-faces-box-twice`. `curve_carrier_clear`
   reads `[0, +inf]` with decoration `Trv`: an operation inside the clearance lost its domain over the box (measured: the
   decoration; which operation: not traced).
2. **What the ladder is asked and does** (`sym::ladder`, a slow pose-disc of 536 ms): the plain form, 1 numerator term over
   2, not zero (20 ms); attempt 0: the early walk (62 ms) and the top reduction, neither zero; both retries the session
   offers are then climbed (`retry_attempts_offered=2`): early walk 282 ms, then 32 ms. Every stage ends in a small form
   that is NOT zero; no budget stop and no poisoned form on the path. It stops because it has run out of attempts.
   The partner decision on the same node then costs 0.12 ms (the session's memo).
3. **Is the value zero, nonzero, or sign-changing over the box?** Nonzero, for all four: the f64 witness is definite, and
   every sub-box narrow enough to decide decides the same way (15,196 pose-disc Positive with lower bounds from 3e-6;
   143,058 clearances Positive; the winding definite ±, by ray). No member was sampled at f64 beyond the witness, so
   "over the whole box" is inferred from the sub-boxes, not sampled.
4. **With the switch on (same tree, `props.rs` as main's, scratch build):** not asked at all at the box scalar. Tour: 0
   `Sym<Interval>` finishes in the 1,023 replays (the leaf's finishes are at f64: 4,151 four-face, 2,072 six-face); a0
   plate: `finishes=0` per leaf, 0 of the four predicates at `Sym<Interval>`. Tour's first drive: 38 s wall, leaf time
   148.9 s (off: 328 s, 1,306 s).
5. **What the finish does with the undecided answer:** it is not a refusal. `discriminant` returns
   `PoseRefusal::in_band(PoseRung::Discriminant)` and `Foot::on_carrier` a `CarrierRefusal`; the aimed ray is set aside
   (`[bf-ray] => .. SetAside(Crossing(Escalated ..))` 8,400 times against 1,057 `Side(Out)` over the drive) and every hole
   finish still ends `ok` (2,100 of 2,100): another ray or the volume reads the side.
   **Skipping the climb** (scratch, `BOXFINISH_SKIP_DISC=1`: an undecided decision of a named predicate returns the numeric
   answer and asks the tier nothing), same verdicts each time (211 certified, 301 refused, 511 splits; symbolic 1,832,193
   and numeric 2,541,916 unchanged):
   | skipped | hole finish | drive leaf time | frozen |
   |---|---|---|---|
   | nothing | 514.5 ms | 1,306.5 s | 1,490 |
   | the pose-disc alone | 468.2 ms (−9%) | 1,182.6 s | 1,490 |
   | all four | 218.7 ms (−57%) | 635.9 s | 658 |
   With all four skipped the remainder is `azimuth_near_cut_side`: 151 ms per finish (it was 10 ms), 2,498 climbs, all
   undecided. So the climb's cost is the forms the undecided margins share, paid by whichever asks first in the session;
   skipping by name moves it.
**One line:** both, in that order. The defect is the reading: four distances and a discriminant that are definitely nonzero
read as straddling zero over a 0.1 mm box. The climb is then a price on a question it cannot close, because the tier only
ever answers Zero and none of these is zero; it is 75% of the drive's leaf time.

## M11-b. The boxed rectangle (`a_boxed_rectangle_certifies_over_its_box::a_pen_drawn_and_a_cornered_rectangle_certify_over_a_boxed_width`)
Run under `ulimit -v 5000000`: aborts at 85 to 90 s, "memory allocation of 12240469 bytes failed" (measured).
| time | resident | leaves replayed | session tables at the last leaf | drive memo |
|---|---|---|---|---|
| leaf 1 (PenTurns, ±1e-6: certified) | 77 MB | 1 | 28,435 nodes, 8,186 forms, 6 atoms | 0 |
| leaf 2 (PenTurns, ±0.05: bisect) | 97 MB | 2 | 46,097 / 13,274 / 842 | 0 |
| leaves 3, 4 (the halves: certified) | 105, 111 MB | 4 | 54,073 / 460 / 941 | 14,842 forms, 649 atoms, 108 frozen, 11.3 MB est. |
| leaf 5 (PenRadians, ±1e-6: certified) | 111 MB | 5 | 28,432 / 8,181 / 16 | 0 (a new drive) |
| 5 s after | 321 MB | 5 | | |
| 25 s | 1.48 GB | 5 | | |
| 55 s | 2.85 GB | 5 | | |
| 85 s | 4.15 GB | 5 | | |
- **What grows:** about 45 MB/s, linearly, with the leaf count, the session's tables and the drive memo all standing still.
  Two stack samples (gdb, 30 s and 60 s) are the same frame chain: `report::render_poly <- render_form <- render_indet <-
  render_poly <- ... <- report::render_node <- Sym::decide_sign <- solid_contain::edge_meets <- ray_crossings <-
  side_reader::aimed_ray_inner <- cast_aims`. The renderer recurses through each atom's argument and builds one `String`.
- **Which node and check:** the extrude's finish (check 7, an aimed ray's edge pass, `side_at_infinity_edge`), inside
  `replay(&doc, &ParamBox::of(&analyzed), SymRules::shipped(), ..)` for `PenRadians` at half 0.05 (the test's line 99),
  not inside a drive. The last trace line before the stall: `[bf-ray] edge 1v1 faces=(3v1, 2v1) gap_lo=[0, 1.850]
  gap_hi=[0, 1.665] along=[0.943, 21.617]`.
- **Does the drive converge?** Yes, where it runs: 1 certified; 2 certified after 1 split; 1 certified. No drive is running
  when the memory goes.
- **Why only with the box arm off** (measured for the tour and a0, item 4; inferred here): with the switch on that replay
  runs no finish at `Sym<Interval>`, so no ray is cast and the report has nothing to render; the row passes in 0.18 s.
- Not determined: whether the rendering ends at all above 5 GB; the render's depth (`explain_depth` is the report's own
  limit for its explanation, not for `render_node`'s forms).

## M11-c. The tour command, both ways, one tree
| | switch off | switch on |
|---|---|---|
| plate study, first drive | 328 s wall (M10), 211 certified / 301 refused | 38 s, 211 certified / 301 refused |
| chain, 1 link, `Interval` | "refuses 0.05 s .. node 111e1e667832 .. an edge's stored curve does not certify against its faces" | "CERTIFIES 0.02 s, 0 refused" |
| command | panics at `chaintol.rs:297` (710 s at M10) | runs to its end, 109 s |

## Method
- Probe `boxfinish/probe-switch-off`: merge `cbb02ee39a` = `a0cba9afe1` + `origin/main` (410 commits, no conflict), then
  two probe commits: `BOXFINISH_M11=1` prints, for the named predicates (`BOXFINISH_M11_NAMES`, default the pose-disc), each
  decision's margin and outcome and the ladder's stages, the pose-disc's coefficients, and per replay the resident memory
  and table sizes; `BOXFINISH_SKIP_DISC=1` is the scratch substitution of item 5. Build as before.
- The switch-on runs: the same tree with `crates/topo/src/props.rs` replaced by main's, built in place and NOT committed;
  the file was restored afterwards (the worktree is clean; the target directory now holds the switch-on binaries).
- Tour figures are one run each on 4 workers, times summed over workers where they are leaf or finish times.

## Not measured, and why
- Which operation inside the clearance returns `[0, +inf]`, and where the pose-disc's offset takes the box twice.
- Members of the box sampled at f64 (only the witness and the deciding sub-boxes).
- The climb skipped for every undecided at-rest decision at once (it would need a rule for which decisions the tier's Zero
  is needed on); so the finish's floor without the climb is not established: 219 ms is an upper bound on it.
- The rectangle above 5 GB; the row's other two forms (`Corners`) are never reached.
