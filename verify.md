# Verify PR 3985 (`reach/arc-from-pairing`), last fix pass

Frozen head `f69ec37c57`. The branch head is still `f69ec37c57`, so there is nothing newer to verify. It merges current main `a93cbcf5e`. The lane measured its differential on `b4490d579`; everything below is re-run on `f69ec37c57`.

**Verdict: VERIFIED.** No blocking point. There are two non-blocking prose findings (C7, C8 below).

## Method

- Worktrees for the head and for main `a93cbcf5e`, each with its own target dir, on a 4-core box.
- **Instrumentation** (local, never pushed). `chord_join::chord_spec` is wrapped by a function with the same signature, on both trees. When `VERIFY_CHORD_DUMP` is set, the wrapper appends one line per minted spec: the test, both end points, `carrier`, `param_start` and `param_end`. Values are `{:?}`-printed f64, which round-trip exactly. With the variable unset the wrapper changes nothing.
- On the head only, two more env-gated counters:
  - `UNDECIDED`: the one production arm that places an end `Placement::Undecided` (`vertex_on_curved_face_at`'s `_ => Placement::Undecided`).
  - `THIRD_SITE_*`: in `partners`' walk filter, each germ that is neither of the pair's own two (`VISIT`, the positive control), and each such germ that decides `bool_join_walk_site` `Zero` or in-band against an end (`AT_END`).
- **Touched-row set:** 218 rows. They are every test in the 13 touched sweep test files, `editor-core`'s `refusal_concision_chains` and `reach_slab_cut_sector_side`, mesh `r2_bool_door`, step-import `poleguard`, topo `m6_2_fitted_at_rest`, and topo's unit tests under `chord_join`, `boolean::{join, reduce, solid_contain, offer_rows}` and `splitting::{join, classify, rules}`. The lane did not publish its 271-row filter, so the counts below are mine, beside the lane's.

## Mutants (touched rows, ε 1e-9; each applied alone, rebuilt, run, reverted; control 0/218 red)

| mutant | smallest edit | my rows red | lane | F4 row |
|---|---|---|---|---|
| datum negated | `arc_leaving`: `along = -(leave.dir·t)/…` | 43 | 92 | red |
| partner dir | `germ_at`: each half paired with the other site's `dir` | 11 | 24 | red |
| germ dir flipped | `germ_at`: `(g.he, -g.dir)` | 31 | 50 | red |
| split_leave flipped | `split_leave`: branches swapped (walk and chord) | 10 | 40 | green (no split in it) |
| split chord datum only | both `split_leave(..)` in the split's `Leave` negated | 11 | 41 | green (no split in it) |
| walk_passes off | `walk_passes` returns `Ok(false)` first | 3 | 3 | red |
| site rung | the site loop never returns (`… == Sign::Zero && false`) | 38 | 69 | red |
| order rung | `== Sign::Positive` → `== Sign::Negative` | 5 | 13 | red |
| always ccw | `Sign::Negative => Ok(true)` | 44 | 91 | red |

- Every mutant is killed.
- `walk_passes` off reds exactly the lane's three rows: the four-crossing row, `a_pierce_off_the_seam_plane…` and the round boss.
- The partner-dir mutant reds `join1_r1_rows::a_pole_struts_halves_face_their_own_meridians` and the pierce rows.
- My counts are lower than the lane's because my row set is smaller (218 against 271). No mutant the lane calls killed stays green.

## ε

| run | result |
|---|---|
| touched rows, ε 1e-9 / 1e-6 / 1e-12 | 218/218 green at each |
| topo, sweep, mesh, editor-core, step-import, full suites (slow set included), ε 1e-9 | 7633/7633 green |
| main, the same suites, ε 1e-9 | 7636/7636 green |
| tour (release), head and main | 95/95 each |
| `arc_loft_natively_computes_its_rational_volume` at 1e-12, on main `a93cbcf5e` | **red**, so main's |
| `euler::tests::a_fan_split_past_a_paired_tear…` with `--features per-op-postcondition`, on main | **red**, so main's |

I did not re-run the head's full suites at 1e-6 or 1e-12, nor with `--all-features`. So the lane's "only reds" at those settings is confirmed only for the two named rows, both red on main.

## Claim checks

- **C1, differential. Holds.**
  - Five crates: 476 chord-minting tests on main (23,633 chords) and 488 on the head (25,020).
  - 463 tests are identical; 24 only add chords.
  - Five main tests lack their chords under the same name, because the head renamed or retired them: `snowman::a_bar_through_a_ball_crosses_the_sphere`, two `tilted_sphere_pair` rows, and two retired `chord_join` unit tests. Every one of their specs is minted on the head in the renamed row or its successor.
  - **No chord main mints is missing on the head.**
  - Re-running the head gives an identical dump (488/488), so the dump is deterministic.
- **C2, round boss. Holds.** I ran the six member orders one per process on both trees.
  - All 16 moved chords (8 per order) are in [1,2,0] and [2,1,0], the orders that union the plate last.
  - Both orders refuse `WallOutlineUnsupported` on both trees. [2,1,0] names a different face key, as review A's NOTE 3 already reports.
  - The four building orders are bit-identical: same volume bits and same vertex-point digest.
- **C3, tour. Holds.** 31 tests identical, with the three ε-pin subprocesses keyed without their PID. The only difference is `snowman::the_head_moved_out_of_the_seam_plane_builds_through_its_ring`, with 24 added chords against 0 on main.
- **C4, F4 row. Holds.** It goes red under `walk_passes` off, under the site rung alone and under the order rung alone. On main it is red with `SectionNotPolar`, both rows.
  - Also red on main: the collar gate pose and the 48-pose matrix (`BothContained`), and `germ_coplanar_conic`'s outcome row.
- **C5, F1. Holds.** `UNDECIDED` fires 0 times over the five full suites (7,633 tests) and the tour, so no end is ever placed `Undecided` outside the truth-table rows. The walk probe in the same build visited 18,312 third sites, so the build did log.
- **C6, new bodies. Holds.**
  - I ran review B's oracle probes on the head, with every op in both operand orders, tiers 2 and 3, volume against a closed form, and `point_in_solid` against an SDF. I added my own row for the F4 slab × sphere pose: both slabs, y-poled and turned ball, six op/order rows, against `∫seg dz` (Simpson, n = 20,000).
  - At 1e-9 there are 0 wrong bodies. Built: tilted sphere pairs 72, tilted plane×sphere 72, bar/notch pierces 18, star plates 72 + 216 on a full-turn face, and four crossings on a sphere section 24/24.
  - At 1e-6, 0 wrong volumes or tiers. B's harness flags 3 `point_in_solid` = `OnBoundary` at a point 8.4e-7 from the boundary at scale 1e-3. That is inside the 1e-6 zero band, and the harness's skip of 1e-6·s does not scale with ε. It is not a wrong body.
  - At 1e-12, the sphere rows panic in the fixture (`sweep/src/test_support.rs:586`, `ball_poled`'s pcurve certify escalates) before any boolean runs. The PR does not touch `sweep/src`. The plate rows and my row pass.
- **C7, F4's in-band branch. Partly holds.**
  - What holds: no third site ever decides in the escalation band (0 `Err`), in the five suites or the tour.
  - What is false: the row doc (`four_crossings_on_one_section_circle.rs` module doc) says a third site within the band of an end "reaches it from no row". `run_walls_built::revolved_runs_build_one_wall_each` decides `bool_join_walk_site` `Zero` 120 times for germs that are **not** the pair's own. Each such germ starts at a different vertex at distance exactly 0 from the end; 112 of them are unused germs.
  - Non-blocking: the outcome is right. But the doc's claim that only "the pair's own two germs" reach that arm is wrong, and it has a row that reaches it.
- **C8, F3 work item. Overstated.** `work/reach/arc-side-rule-has-two-predicates.md` says that in both disagreement classes "the datum's body [meets] its closed form". Class 1 (the die pips, and r1's y-poled ball on a cylinder cap) builds no body: its rows assert `SectionLoopUndecided`, as the PR body itself says. The sentence holds only for the tube strut. Non-blocking prose.
- **Other claims checked.**
  - `9922b0ac` exists in no fetched ref.
  - The S-h desync check is added at `bool_connect`. No row reaches it; it is a guard.
  - `the_chord_arc_rung_is_decided_in_one_place` counts both `chord_arc_leave_*` names and `split_join_conic_heading`.
  - The split's chord and its walk use the same lever: `curvature_lever_arm`.
  - The stale-prose grep is clean of the retired selector names outside `work/`. The audit's F8 names `bool_between_arc_window` only as retired. One leftover line of history stays in the wedge row's doc: "where a chord that asked the divided face's window found both candidate arcs inside it". That is comment style, not a false claim.
