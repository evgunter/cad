# Delta re-read of PR 7329 `embed/remeter` at `2b01b4142` (reviewer lane `embed/remeter-review-5bf3bb`)

## Verdict: APPROVE-WITH-FIXES

One fix before the gate can be green: the new contact9 row runs four times over the fast-set time bar (MAJOR-1). Everything else from my first report is fixed or answered, and the answers hold. No decision of the engine changed in the fix pass (control and all thirteen mutants reproduce the lane's table to the count). Hosted CI not consulted, as asked; what I ran is at the end.

## MAJOR

**MAJOR-1, DEMONSTRATED, test/CI.** `crates/topo/tests/contact9_side_codes.rs`, `the_sector_slivers_meet_is_read_at_its_faces_reach` (commit `7c6cd14db`) takes **2.16 s, 2.12 s, 1.85 s** in my build (CI's opt-1 profile), 1.8 to 2.0 s at ε = 1e-6 and 1e-12. It exhausts a 65,536-pair budget by design. Its name is not in `.config/nextest.toml`'s slow set (0 matches for the test, 0 for `contact9_side_codes`), and the per-PR default-ε step fails a test the diff adds at 0.5 s (`scripts/check-fast-set-times.py`, NEW_BAR). The queued run should go red on it. Not an assertion failure; the row passes.

## 1. My first report's findings

| finding | state |
|---|---|
| Claim 2: the reach-alone row understated the gap (classes differ, no pinch needed) | **Fixed.** Row retitled and rewritten with the table and the sentence; pinned by `the_reach_alone_signs_an_edge_check_4_classes_smooth` (reads `classify_dihedral` beside `sign_on`, as my probe did). Red under M1 and M2. |
| `wedge_reach.rs` header said every reader folds | **Fixed** (lines 1-8, 29-33), and the doc above `folded_lever_arm` names the one reader that does not. Both cite the row. |
| ONEROUND row tallies stale | **Fixed.** 43 / 12, the site listed under `reach_from_vertex` as a pass-through. |
| Claim 10: no row through `departure` feels the arm | **Fixed.** `a_fold_certifies_by_its_sine_at_the_least_reach` goes through `certify_pair`; see 5. |
| Claim 11: `_column` and `leaves_line` are not cell-levered; "26 sites" | **Fixed**, see 4. `PIECES`' doc corrected; five audit rows carry the pointer. |
| Stale sliver rows, `program.md` | **Fixed** where I sampled (the drum row, `program.md:43-45`, the unit row's status now `dispatched`). I did not re-read all five. |
| Style: `.is_some()` on the line-corner row | **Fixed** (`Some(Sign::Positive)`). |
| Style: dropped half of the refused-lever row; no zero-arm row | **Fixed** by `a_zero_arm_and_an_uncertified_arm_decide_nothing`. |
| Style: `metre_arm` in the production region; `arm_of` before the early returns; pole bound far from its lines; unbounded "sine" | **Fixed**, each as J says. |
| Style: the sine computed in two crates with a comment saying they agree | **Answered, holds.** The new row compares them on one edge; one home is the stacked unit's. |
| Style: `arm_of` per cell pair; `Option` arm threaded by hand | **Answered** (not taken); see 7. |
| Claim 12: pole's rows red without this PR | **Answered** under Seams. It says "two or three"; I measured two. |
| The filed cone row's facts (1) and (2) | **Still not re-measured** by anyone I can see. Not blocking. |

## Findings that were only the other review's (from the letters and the diff)

- **A, the run along the edge: fixed as far as this PR goes.** Stated at `sign_on` (5540-5551) and in the body; pinned (see 3). I had noted the over-statement as inherited and did not probe it; the other review's strip is the stronger finding and the lane's account of it is accurate (`sine × arm` 3.6e-8 against 1e-8 at reach 2.0006, undecided at 0.05).
- **D, PR 7262 coupling: answered.** Present under Seams. I did not read ONEROUND's log line.
- **G: fixed.** `a_sine_that_changes_sign_over_the_rectangle_decides_nothing` is red under my M10, at unit level.
- **K: answered.** Each item is under Seams. I did not build `embed/tangent-corner` or `embed/apex-separator` against this head.

## 2. The text against the ruling: HELD, one NOTE

- The PR body's top, `sign_on`'s docs, `arm_of`'s docs, `wedge_reach.rs`'s header and `SELF_INTERSECTION_DEPARTURE`'s doc each say the lever is the reach as `wedge_reach` builds it. None claims the DESIGN sentence's reading; the body says outright that the clause on main "says more than this PR does".
- NOTE: the body's section heading still reads "What the margin is now, and why it is the tolerance Ev described". The section under it then lists two ways the arm is not that tolerance. The heading is the one sentence left that over-claims.
- NOTE: `sign_on` (5534-5539) still opens "an angle metered where it displaces the geometry it governs" before the next sentence takes it back. Truthful as a whole; the opening clause is the old claim.

## 3. The two rows that pin a known-wrong reading

- **`the_arm_counts_a_strips_run_along_its_edge`: tied.** It asserts `arm_of`'s number (√4.0025, to 1e-9), that the strips never part by the band's zero, `Some(Negative)` at that arm and `None` at 0.05. A reach read across the edge reddens the first assert; nothing else can keep it green. Owner row named in the doc (HONEONE's).
- **The contact9 row: not tied. MINOR, test quality.** It asserts only `Refused(Budget(_))`: either budget, any cause. My trace on it: 295 departure reads, all decided, arms 5.03 and 10.05, so the lane's account (the margin is not what refuses) is right today. But the row stays green if the faces become presented and the ask then runs out of pairs for a new reason, and it does not assert that no departure is undecided. Its name, `…_is_read_at_its_faces_reach`, states what its doc says does not happen. Owner row named in the doc (EMBED's thin-face row, which exists).
- NOTE: `the_reach_alone_signs…` asserts `engine.is_some()`, not the sign: the pattern fixed on the line-corner row, re-minted here.

## 4. H: the split is right at every site I sampled

Sampled (with the first pass's read): `graph_of` (span of the piece the cell pair reaches: cell), `leaves_graph` (the cell's columns' span: cell), `cusp` (the cell ball's reach, `Zero`: cell), `monotone` (the rectangle's spans: cell), `monotone_run` (1/64 of the edge's span), `leaves_line` (a piece under its own halving). The row's table and the PR's Sweep table agree with the code at each. 21 call sites reproduces.
- NOTE: the row's **title** says "three by a cell's … two by a piece of an edge's span (the column, a corner's turn)". Its table and its text put four functions at a cell: `leaves_graph` is a corner's turn and is cell-levered. The title files the whole `_corner_departs` site under the piece kind.

## 5. Mutants: "all thirteen red" is TRUE; F is a real pin

Re-applied my switch to `2b01b4142` (patch on my branch), ran control and all thirteen over the 170: control 170/170; reds 11, 2, 2, 2, 2, 2, 5, 1, 1, 4, 2, 7, 4 for span, unnorm, greatest, first, dropother, metre, naivepole, guard, oneface, mid, double, exact, zeroband. That is the lane's table exactly.
- `a_fold_certifies…` is red under greatest, first, dropother and double. It asserts `arm_of`'s number and then a `certify_pair` outcome either side of the band, so the least reach is pinned at the decision, not only as a number.
- `oneface` is still killed by one row only (`the_arm_is…`, a number, not a decision). The vertex-alone arm has no row through `certify_pair`. NOTE.

## 6. The census test that moved: main's

`sheets_part_on_slanted_rims::a_rim_crossing_with_the_inner_plane_steeper_builds`, the gate asking: on `origin/embed/gate-measure` merged with main `a3d5ce89c` alone (no remeter), it **passes**, every ask `Embedded`. So the move is main's, as the lane guessed. I did not re-run it on the head (the lane's figure).
- MINOR, doc/claim: the "24 of 275" was taken on `embed/remeter-measure` at `8143a77a4`, which contains the merge of `embed/vertex-bisector` (`449c58f18` is its ancestor); the first measurement's tree (`a246952d3`) did not. The body compares the two without saying so. The 275 are edge refusals, so I expect no count to move by it, but the measurement is not like for like and the body should say which tree holds what.

## 7. The three not taken under J

- **`Along::collapsed` documented, not a variant: holds.** The doc's claim checks: `Along::at` is the only non-test site that sets `vertex`, and it sets no edge (the literals at 3116 and 3482 set `None`).
- **`SELF_INTERSECTION_DEPARTURE` not split: holds as a cost reason.** No row schedules the split; by the style lane's Q6 a disclosed deviation owes one. NOTE.
- **`arm_of` not memoised: the conclusion holds, one stated reason does not.** `departure` does hold a key (`x.face`, `y.face`, the near edges' indices); what it lacks is somewhere to keep a memo. Moot once the stacked unit reads extents from the verdict.

## 8. New in the fix pass

- MAJOR-1 above.
- `a_quadrant_of_no_width_departs_at_its_arm` is red under M1, as the body says.
- Nothing else: `definite` is a pure refactor (both callers, same match), and `arm_of` moved below returns that never read it.

## Style (new only)

- `crates/topo/src/injective.rs:5917-5920`. `arm_of`'s doc now carries a measurement ("107.8 s without, 102.5 s with"). Those two numbers are main against the branch, not the walk with and without the per-pair read, and nothing re-takes them. A claim resting on a measurement, written into a doc comment, with no guard and no schedule. **sure**
- `crates/topo/tests/contact9_side_codes.rs` (the new row). The name promises a reading the test never makes, and the assertion is a refusal of any kind. A row named for the state it waits for reads as green proof of that state. **likely**
- `crates/topo/src/injective.rs:13169` on. A third test module (`margin_reach_rows`) re-declares `iv0`, `band`, `key`, `along`, `on`, `floor`, `piece`, `boundary` beside `margin_rows`' copies, with different signatures (`v3([f64; 3])` against `v3(x, y, z)`). Two near-parallel fixture vocabularies forty lines apart. **sure**
- `work/embed/five-more…md:4`. The title and the table disagree on how many sites are of each kind (see 4). A count written by hand in a title, corrected in the body only. **sure**
- `injective.rs:5534-5551`. The margin paragraph states the old claim and then retracts it in the next sentence. It would read truer the other way round. **unsure**

## Executed, as against read

- Executed on `2b01b4142` with my switch: control and thirteen mutants over 170; the contact9 row three times at default ε and once each at 1e-6 and 1e-12, with a margin trace; the `injective::margin*` rows at 1e-6 and 1e-12 (11 of 11).
- Executed on `gate-measure` ⊕ main `a3d5ce89c`: the moved census test, the gate logging.
- Read: the lane's four commits in full for `injective.rs`, `wedge_reach.rs`, `dihedral.rs`, `contact9_side_codes.rs`; the PR body; the five-margins, reach-alone and ONEROUND rows; `check-fast-set-times.py`'s header.
- Not run: the suites at three ε (the lane's 266), the tour, the 275, the head through the gate, any Seams branch. Not read: the other review's report itself (only the letters), ONEROUND's log, HONEONE's row, three of the five corrected EMBED rows.
- Probe patch: `review/remeter-delta-mutants-on-2b01b4142.patch` on `embed/remeter-review-5bf3bb`.
