STAGE 1 REPORT — tesslane/certificate-decides, head 5e1460889f (confirmed with ls-remote). Nothing of stage 2 is built. I am stopped until you answer.

WHAT I BUILT TO MEASURE (does not ship; commits a3a849e2ce, b4abff04d8)
`crates/mesh/src/cd_probe.rs` plus hooks in `trimmed.rs`, `curved.rs`, `tessellate.rs`: env switches for the share of δ a chart-lane analytic face sizes at (all charts or per chart), the insertion rule, the correction's own round counter (fold and shared-side rebuilds stay on `MAX_GRID_RETRIES`), the same correction in `tessellate_curved`, and one log line per face meshed (lane, chart, how bounded, δ, outcome, triangles, time, passes, rounds by cause, points triangulated summed over passes, worst certificate). Unset, the code path is today's. To ship: delete the module and hooks, keep the loop's two counters and the rule.
The brief's `iso_probe.rs` / `ISO_*` are not in this branch's tree or history; I wrote my own.

SCOPE RUN
`mesh` whole suite with its slow set (550 tests); `sweep`, the 131 modules that name the mesher (934 tests; at ε 1e-12 the baseline in full and the rule runs on the 50 tests that hold a chart-lane analytic face, the only ones the switches can move); `step-import` (478). ε 1e-9, 1e-6, 1e-12. Chart-lane analytic meshings compared at 1e-9: 1,159 + 2,139 + 12. The tour is not run. Reds seen are yours-listed ones plus pins that move.

1. THE RULE (every chart-lane analytic face at δ; budget 40 so the rounds needed show)
rule | faces lost on the certificate | most rounds: mesh / sweep / step | faces taking ≥1 round
centroid | 0 | 3 / 7 / 0 | 39 / 41 / 0
midpoint of the widest free side (3-D length) | 0 | 3 / 7 / 0 | 39 / 41 / 0
side-sag (below) | 0 | 3 / 2 / 0 | 39 / 41 / 0
Identical at 1e-6 and 1e-12 (same maxima; 36–39 and 33–41 faces). Worst certificate on any returned face 0.998 δ. No ε is read: the rule reads the verdict `Exceeded`, which is `bound > δ + rounding allowance`.
- Centroid: 7 rounds on one sphere face, `sweep` `snowman::two_cut_ins_on_one_face_build_in_either_order` (12 meshings, 2 triangles over δ at first, 11 points added, 4.8× today's points triangulated for 0.63× today's triangles). At a budget of 6 it is lost.
- Midpoint: 5 and 7 rounds on bored cylinder walls (`sweep` `bisector_nodes`, 6 meshings); the centroid takes 2 there. Lost at 6 too.
- Side-sag, the rule the two failures suggested: the middle of the refused triangle's side, no constraint side, whose own chord stands farthest from the surface (one `surface.eval` per free side); the inside point where all three sides are constraints. Reason: on a ruled chart the widest side can lie along a ruling, where splitting moves no certificate.
- Without any correction at δ: 10 matched meshings lost on the certificate (worst first-pass read 1.196 δ, 1.115, 1.083).
- Spline rows: no spline face in any suite takes a round (2,465 faces); the rules differ only on `a_face_left_slivered_refuses_typed_through_tessellate`, whose subject is exhaustion (centroid 9 rounds, the others 12).
- Rectangle lane: of 24,767 faces one body ever reads over δ, `mesh` `loops_the_meridian_guard_admits::a_spur_on_a_rim_only_cap…` (refuses today at δ 0.1, 1.20 δ, and at 0.01, 7.6 δ). δ 0.1: centroid 36 rounds, midpoint 5, side-sag not in 40. δ 0.01: centroid not in 40, midpoint not in 60, side-sag 8. No rule and small budget cures both. The spec's witness (the filleted one-segment hole) cannot be built here: `circle()` still lowers to two arcs on this branch.

2. FOLDS (not deviations; the correction does not reach them)
Five meshings that mesh today fold at δ, all a tilted circle near a sphere's pole, same at all three ε, under every rule:
- `sweep` `tilted_sphere_pair::a_circle_near_the_pole_meshes_at_a_coarse_delta`, second body, δ 4e-2, all 4 sphere faces (5 fold rounds, still folded; today 1 pass + 1 shared-side round).
- `mesh` `reach_probes::ringed_curved_faces_that_still_refuse_answer_by_cause`, the near-pole tube on the ball, ringed sphere face, δ 5e-3 (today folds at 2e-2, meshes at 5e-3).
Not monotone in the target: the tilted pair by share of δ: 1.0 folds, 0.9 and 0.8 mesh, 0.7 folds, 0.6 and 0.5 mesh. 20 and 60 fold rounds do not clear it. Filed as evidence on the existing row `work/tesslane/a-tilted-circle-near-a-sphere-pole-refuses-at-the-sphere-arm.md` (commit 5e1460889f), not a new row.
Also: `reach_probes::a_needle_funnel_cut_at_its_apex_refuses_typed` δ 2e-4, two cone faces refuse either way, `UnclearedSharedSide` today, `FoldedTriangle` at δ.

3. WORK, δ + side-sag against today (`mesh` + `sweep`, ε 1e-9)
Time is the lane's time in the face, release, one test at a time, one rayon thread, nothing else running, two repeats (repeats agree within 7%). 3,238 faces ok in both.
chart | n | ms today | ms δ+sag | time | points triangulated | passes | triangles | time at a 0.9 δ first guess
cone | 452 | 3,051 | 1,576 | 0.52 | 0.51 | 478→502 | 0.50 | 0.56
cylinder | 1,819 | 873 | 647 | 0.74 | 0.71 | 2,173→2,190 | 0.71 | 0.75
sphere | 858 | 6,902 | 3,879 | 0.56 | 0.56 | 875→905 | 0.53 | 0.60
torus | 109 | 1,585 | 1,100 | 0.69 | 0.63 | 109→115 | 0.51 | 0.53
By family (chart × rings × boundary carriers × pole on the loop; 22 families): every one costs less time than today except
- cylinder, 2 rings, circles + lines (holed pipe walls, n = 9): time 1.16, points 1.03, 6 of 9 take one round, triangles 0.76. In absolute terms 5 ms → 6 ms.
Closest others: torus no ring (n = 73) 0.87; cylinder 1 ring circles + lines (n = 13) 0.84, 6 of 13 take a round.
AT FACE LEVEL, which is where the owner's sentence bites: 80 faces (2.4%) take a round; 55 of them cost more time than today and 53 of those also end coarser than today's δ/2 mesh. Median 1.19× today's time, worst 2.6× (a wide tilted cylinder, 3 rounds, 0.77× the triangles). Summed over the 80: 459 ms → 523 ms. The other 3,158 faces: 11,952 ms → 6,680 ms.
THE PAGE'S ANSWER, MEASURED, DOES NOT WORK AS STATED. A first guess of 0.9 δ on every arm does not bring the correction rate to nothing: 65 faces take a round against 80, and they are mostly different faces. Which face reads over δ is an alignment of chord points, not a property of a family: at δ the over-δ faces are over by at most 20%, but 0.85 δ on the cylinder arm still corrects 5 of 451 (different ones), and only 0.8 δ corrects none in `mesh`, at 0.62–0.82 of today's points instead of 0.51–0.78. Total work at 0.9 δ is higher than at δ on cone, cylinder and sphere and lower on the torus (0.53 against 0.69), where all six corrected faces are two bodies at δ 1e-3 that 0.9 δ happens to clear. It also changes the folds: one of the five at 0.9 δ.
What a round costs: every pass triangulates and certifies the whole face again. Measured split of a pass (dev profile): triangulation and constraints 57% sphere, 56% torus, 48% cylinder, 24% cone; the rest is emit and certificates. So inserting into the standing triangulation saves about half a round; only re-certifying the changed triangles alone would make a round near free. Neither is in the spec.

4. NEIGHBOURS (rims shared with a chart-lane face chorded at δ)
Faces whose triangle count moves, and the ratio over those: planar 1,628 of 36,009 (`mesh`, 0.72) and 1,378 of 22,923 (`sweep`, 0.73); rectangle-lane cylinder 26 of 8,062 and 34 of 3,130 (0.75, 0.74); rectangle-lane sphere 26 of 3,101 (0.99) and 87 of 2,339 (0.76); cone, torus, spindle rectangles and every walked face: none. No rectangle-lane, planar or walked face newly refuses in any suite at any ε, and none takes a correction round.

5. THE THREE ROWS
- `the-trimmed-cylinder-lane-has-no-worst-triangle-theorem`, `the-trimmed-sphere-arm-has-no-worst-triangle-theorem`, `one-containment-lemma-for-an-oblique-boundary`: each asks for a bound on the triangle beside a trim chord so that the step alone holds it. With the correction, no face in three suites at three ε is lost for want of that bound, so as debts on the sizing they are not owed. Two things stay true and should be said where they close: the correction's termination has no proof (it is a budget), and the lemma row also names `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`, a walked face, which this unit does not touch.
- `the-boundary-clearance-goes-to-half-a-step…`: unblocked, a measurement the same probe shape can take; I did not take it.

WHAT THE SPEC OR THE BRIEF HAD WRONG
1. The probes said to be on the branch are not.
2. "Centroid loses none within 6, at most 4 used": 7 on `sweep`.
3. "Midpoint needs 40 on one wide tilted cylinder wall": 3 now (the cylinder arm lays no rows since unit 4b); its bad case is elsewhere and is 7.
4. "Five sphere faces that refuse today mesh": those are `trimmed::tests::an_over_delta_sphere_arm_triangle_refuses_typed`, four faces of a row that scales the steps to force the refusal (and the spline row above). No face that refuses on the certificate in ordinary use is cured, in any suite.
5. "One sphere face folds at δ": five meshings, two bodies, and it is a lottery over the target.
6. "Size the family finer and its correction rate goes to nothing": measured false above.
7. The rectangle lane's correction has no subject in these suites that it cures within a small budget.

RECOMMENDATION
- Rule: side-sag. Budget: its own constant at 6, twice the most any face used at any ε (3). The spline rows take the same rule.
- First guess: δ on every chart, no family sized finer. The measurement gives no family a finer guess would fix; the torus at 0.9 δ is six faces of two bodies.
- The owner's condition holds per chart and per family (one family of nine faces at 1.16, 1 ms), and does not hold on 53 faces of 3,238 (1.6%), which nothing identifies before they are meshed. That is the open decision and I think it is the owner's or yours: accept that residue as stated, or have stage 2 make a round cheap (insert into the standing triangulation, about half a round saved; local re-certification on top for the rest), which I would cost as a unit of its own.
- Rectangle lane: build the same correction as the spec says, knowing it cures nothing here; under side-sag at 6 the spur cap keeps refusing typed at both δ, so its pin stays. If you would rather it wait for a witness, say so.
- Folds: stage 2 as specified loses five meshings that mesh today, one of them a `sweep` row named for meshing at a coarse δ. I see no cure inside this unit's fence. You decide whether that blocks.

WHAT WOULD CHANGE IT: a suite I did not run (the tour, `editor-core`, the viewer) showing a face past 3 rounds under side-sag or lost at 6; the one-segment hole's band, once buildable, needing a different rule on the rectangle lane; a ruling that the 1.6% is not acceptable.

Slowed me down: `sweep`'s single test binary takes about 20 minutes to build in release and every probe change rebuilds it; some `sweep` tests take minutes at ε 1e-12; I edited a runner script while it ran and it re-ran one baseline (caught, the first pass's log is complete).
